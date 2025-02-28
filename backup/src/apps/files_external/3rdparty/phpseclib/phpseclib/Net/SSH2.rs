// Pure-Rust implementation of SSHv2.
// 
// This is a port of phpseclib's Net_SSH2 class to Rust.
// 
// # Examples
// 
// ```no_run
// use ssh2::Ssh2;
// 
// # fn main() -> Result<(), Box<dyn std::error::Error>> {
// let mut ssh = Ssh2::new("www.domain.tld", 22, Some(10.0))?;
// 
// if !ssh.login("username", "password")? {
//     panic!("Login Failed");
// }
// 
// println!("{}", ssh.exec("pwd")?);
// println!("{}", ssh.exec("ls -la")?);
// # Ok(())
// # }
//
// 
// Using a private key for authentication:
// 
// ```no_run
// use ssh2::{Ssh2, key::RsaKey};
// 
// # fn main() -> Result<(), Box<dyn std::error::Error>> {
// let key = RsaKey::load_key(std::fs::read("privatekey")?)?;
// 
// let mut ssh = Ssh2::new("www.domain.tld", 22, Some(10.0))?;
// if !ssh.login_with_key("username", &key)? {
//     panic!("Login Failed");
// }
// 
// println!("{}", ssh.read("username@username:~$")?);
// ssh.write("ls -la\n")?;
// println!("{}", ssh.read("username@username:~$")?);
// # Ok(())
// # }
//

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};
use thiserror::Error;
use regex::Regex;
use log::{trace, debug, warn, error};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream as TokioTcpStream;
use tokio::time::timeout as tokio_timeout;
use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};
use num_traits::{Zero, One};
use rand::{thread_rng, Rng};
use ring::digest::{Context, SHA1_FOR_LEGACY_USE_ONLY};
use ring::hmac;
use aes::{Aes128, Aes192, Aes256};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use des::TdesEde3;
use ctr::Ctr64BE;
use rc4::{KeyInit, StreamCipher};

// SSH message constants
const SSH_MSG_DISCONNECT: u8 = 1;
const SSH_MSG_IGNORE: u8 = 2;
const SSH_MSG_UNIMPLEMENTED: u8 = 3;
const SSH_MSG_DEBUG: u8 = 4;
const SSH_MSG_SERVICE_REQUEST: u8 = 5;
const SSH_MSG_SERVICE_ACCEPT: u8 = 6;
const SSH_MSG_KEXINIT: u8 = 20;
const SSH_MSG_NEWKEYS: u8 = 21;
const SSH_MSG_KEXDH_INIT: u8 = 30;
const SSH_MSG_KEXDH_REPLY: u8 = 31;
const SSH_MSG_USERAUTH_REQUEST: u8 = 50;
const SSH_MSG_USERAUTH_FAILURE: u8 = 51;
const SSH_MSG_USERAUTH_SUCCESS: u8 = 52;
const SSH_MSG_USERAUTH_BANNER: u8 = 53;
const SSH_MSG_USERAUTH_PASSWD_CHANGEREQ: u8 = 60;
const SSH_MSG_USERAUTH_PK_OK: u8 = 60;
const SSH_MSG_USERAUTH_INFO_REQUEST: u8 = 60;
const SSH_MSG_USERAUTH_INFO_RESPONSE: u8 = 61;
const SSH_MSG_GLOBAL_REQUEST: u8 = 80;
const SSH_MSG_REQUEST_SUCCESS: u8 = 81;
const SSH_MSG_REQUEST_FAILURE: u8 = 82;
const SSH_MSG_CHANNEL_OPEN: u8 = 90;
const SSH_MSG_CHANNEL_OPEN_CONFIRMATION: u8 = 91;
const SSH_MSG_CHANNEL_OPEN_FAILURE: u8 = 92;
const SSH_MSG_CHANNEL_WINDOW_ADJUST: u8 = 93;
const SSH_MSG_CHANNEL_DATA: u8 = 94;
const SSH_MSG_CHANNEL_EXTENDED_DATA: u8 = 95;
const SSH_MSG_CHANNEL_EOF: u8 = 96;
const SSH_MSG_CHANNEL_CLOSE: u8 = 97;
const SSH_MSG_CHANNEL_REQUEST: u8 = 98;
const SSH_MSG_CHANNEL_SUCCESS: u8 = 99;
const SSH_MSG_CHANNEL_FAILURE: u8 = 100;

// Bitmap masks
const SSH2_MASK_CONSTRUCTOR: u32 = 0x00000001;
const SSH2_MASK_LOGIN: u32 = 0x00000002;
const SSH2_MASK_SHELL: u32 = 0x00000004;

// Channel constants
const SSH2_CHANNEL_EXEC: usize = 0;
const SSH2_CHANNEL_SHELL: usize = 1;

// Log constants
const SSH2_LOG_SIMPLE: u8 = 1;
const SSH2_LOG_COMPLEX: u8 = 2;
const SSH2_LOG_REALTIME: u8 = 3;
const SSH2_LOG_REALTIME_FILE: u8 = 4;
const SSH2_LOG_MAX_SIZE: usize = 1024 * 1024;

// Read constants
const SSH2_READ_SIMPLE: u8 = 1;
const SSH2_READ_REGEX: u8 = 2;

// Disconnect reason codes
#[derive(Debug, Clone, Copy)]
enum DisconnectReason {
    HostNotAllowedToConnect = 1,
    ProtocolError = 2,
    KeyExchangeFailed = 3,
    Reserved = 4,
    MacError = 5,
    CompressionError = 6,
    ServiceNotAvailable = 7,
    ProtocolVersionNotSupported = 8,
    HostKeyNotVerifiable = 9,
    ConnectionLost = 10,
    ByApplication = 11,
    TooManyConnections = 12,
    AuthCancelledByUser = 13,
    NoMoreAuthMethodsAvailable = 14,
    IllegalUserName = 15,
}

// Channel open failure reasons
#[derive(Debug, Clone, Copy)]
enum ChannelOpenFailureReason {
    AdministrativelyProhibited = 1,
}

// SSH errors
#[derive(Error, Debug)]
pub enum SshError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Connection closed by server")]
    ConnectionClosed,
    
    #[error("Connection timeout")]
    Timeout,
    
    #[error("Banner timeout")]
    BannerTimeout,
    
    #[error("Invalid SSH version: {0}")]
    InvalidSshVersion(String),
    
    #[error("Invalid size")]
    InvalidSize,
    
    #[error("Invalid HMAC")]
    InvalidHmac,
    
    #[error("Expected {0}")]
    UnexpectedMessage(String),
    
    #[error("Unable to connect: {0}")]
    ConnectionError(String),
    
    #[error("Key exchange failed: {0}")]
    KeyExchangeFailed(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Channel error: {0}")]
    ChannelError(String),
    
    #[error("Server disconnected: {0}")]
    ServerDisconnect(String),
    
    #[error("Invalid signature")]
    InvalidSignature,
    
    #[error("Operation disallowed prior to login")]
    NotLoggedIn,
    
    #[error("Unable to initiate shell session")]
    ShellInitFailed,
    
    #[error("Unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),
    
    #[error("Crypto error: {0}")]
    CryptoError(String),
    
    #[error("SSH error: {0}")]
    Other(String),
}

pub type SshResult<T> = Result<T, SshError>;

// Cryptographic primitives

trait SshCipher {
    fn encrypt(&mut self, data: &[u8]) -> Vec<u8>;
    fn decrypt(&mut self, data: &[u8]) -> Vec<u8>;
}

trait SshHmac {
    fn hash(&self, data: &[u8]) -> Vec<u8>;
    fn set_key(&mut self, key: &[u8]);
    fn size(&self) -> usize;
}

struct NullCipher;
impl SshCipher for NullCipher {
    fn encrypt(&mut self, data: &[u8]) -> Vec<u8> { data.to_vec() }
    fn decrypt(&mut self, data: &[u8]) -> Vec<u8> { data.to_vec() }
}

struct NullHmac;
impl SshHmac for NullHmac {
    fn hash(&self, _data: &[u8]) -> Vec<u8> { vec![] }
    fn set_key(&mut self, _key: &[u8]) {}
    fn size(&self) -> usize { 0 }
}

struct HmacSha1 {
    key: Vec<u8>,
}

impl HmacSha1 {
    fn new() -> Self {
        Self { key: vec![] }
    }
}

impl SshHmac for HmacSha1 {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &self.key);
        hmac::sign(&key, data).as_ref().to_vec()
    }
    
    fn set_key(&mut self, key: &[u8]) {
        self.key = key.to_vec();
    }
    
    fn size(&self) -> usize {
        20 // SHA1 digest size
    }
}

struct HmacSha1_96 {
    inner: HmacSha1,
}

impl HmacSha1_96 {
    fn new() -> Self {
        Self { inner: HmacSha1::new() }
    }
}

impl SshHmac for HmacSha1_96 {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        self.inner.hash(data)[..12].to_vec()
    }
    
    fn set_key(&mut self, key: &[u8]) {
        self.inner.set_key(key);
    }
    
    fn size(&self) -> usize {
        12 // Truncated to 96 bits
    }
}

// The main SSH client structure
pub struct Ssh2 {
    identifier: String,
    stream: TcpStream,
    bitmap: u32,
    errors: Vec<String>,
    server_identifier: String,
    
    // Key exchange results
    kex_algorithms: Vec<String>,
    server_host_key_algorithms: Vec<String>,
    encryption_algorithms_client_to_server: Vec<String>,
    encryption_algorithms_server_to_client: Vec<String>,
    mac_algorithms_client_to_server: Vec<String>,
    mac_algorithms_server_to_client: Vec<String>,
    compression_algorithms_client_to_server: Vec<String>,
    compression_algorithms_server_to_client: Vec<String>,
    languages_client_to_server: Vec<String>,
    languages_server_to_client: Vec<String>,
    
    // Crypto context
    encrypt_block_size: usize,
    decrypt_block_size: usize,
    encrypt: Option<Box<dyn SshCipher>>,
    decrypt: Option<Box<dyn SshCipher>>,
    hmac_create: Option<Box<dyn SshHmac>>,
    hmac_check: Option<Box<dyn SshHmac>>,
    hmac_size: usize,
    
    // Session data
    server_public_host_key: Vec<u8>,
    session_id: Option<Vec<u8>>,
    exchange_hash: Vec<u8>,
    signature: Vec<u8>,
    signature_format: String,
    signature_validated: bool,
    
    // Sequence numbers
    send_seq_no: u32,
    get_seq_no: u32,
    
    // Channel data
    server_channels: HashMap<usize, u32>,
    channel_buffers: HashMap<usize, Vec<String>>,
    channel_status: HashMap<usize, u8>,
    packet_size_client_to_server: HashMap<usize, usize>,
    window_size_client_to_server: HashMap<usize, u32>,
    
    // Logging
    message_number_log: Vec<String>,
    message_log: Vec<String>,
    log_size: usize,
    
    // Window size
    window_size: u32,
    
    // Interactive session
    interactive_buffer: String,
    
    // Timeout configuration
    timeout: Option<f64>,
    cur_timeout: Option<f64>,
    last_packet_time: Instant,
    
    // Exit status
    exit_status: Option<i32>,
    
    // Quiet mode
    quiet_mode: bool,
}

impl Ssh2 {
    /// Creates a new SSH2 client and connects to the specified host
    pub fn new(host: &str, port: u16, timeout: Option<f64>) -> SshResult<Self> {
        let timeout_duration = timeout.map(|t| Duration::from_secs_f64(t));
        let start = Instant::now();
        
        // Connect to the SSH server
        let stream = match timeout_duration {
            Some(duration) => {
                let stream = TcpStream::connect_timeout(&format!("{}:{}", host, port).parse().unwrap(), duration)?;
                stream.set_read_timeout(Some(duration))?;
                stream.set_write_timeout(Some(duration))?;
                stream
            },
            None => TcpStream::connect(format!("{}:{}", host, port))?,
        };
        
        let elapsed = start.elapsed();
        
        // Calculate remaining timeout if we have one
        let remaining_timeout = timeout.map(|t| {
            let remaining = t - elapsed.as_secs_f64();
            if remaining <= 0.0 {
                return Err(SshError::Timeout);
            }
            Ok(remaining)
        }).transpose()?;
        
        // Initialize base SSH structure
        let mut ssh = Self {
            identifier: format!("SSH-2.0-rust_ssh2_0.1"),
            stream,
            bitmap: 0,
            errors: Vec::new(),
            server_identifier: String::new(),
            
            kex_algorithms: Vec::new(),
            server_host_key_algorithms: Vec::new(),
            encryption_algorithms_client_to_server: Vec::new(),
            encryption_algorithms_server_to_client: Vec::new(),
            mac_algorithms_client_to_server: Vec::new(),
            mac_algorithms_server_to_client: Vec::new(),
            compression_algorithms_client_to_server: Vec::new(),
            compression_algorithms_server_to_client: Vec::new(),
            languages_client_to_server: Vec::new(),
            languages_server_to_client: Vec::new(),
            
            encrypt_block_size: 8,
            decrypt_block_size: 8,
            encrypt: None,
            decrypt: None,
            hmac_create: None,
            hmac_check: None,
            hmac_size: 0,
            
            server_public_host_key: Vec::new(),
            session_id: None,
            exchange_hash: Vec::new(),
            signature: Vec::new(),
            signature_format: String::new(),
            signature_validated: false,
            
            send_seq_no: 0,
            get_seq_no: 0,
            
            server_channels: HashMap::new(),
            channel_buffers: HashMap::new(),
            channel_status: HashMap::new(),
            packet_size_client_to_server: HashMap::new(),
            window_size_client_to_server: HashMap::new(),
            
            message_number_log: Vec::new(),
            message_log: Vec::new(),
            log_size: 0,
            
            window_size: 0x7FFFFFFF,
            
            interactive_buffer: String::new(),
            
            timeout,
            cur_timeout: remaining_timeout,
            last_packet_time: Instant::now(),
            
            exit_status: None,
            quiet_mode: false,
        };
        
        // Read and process SSH banner
        let mut banner = String::new();
        let mut extra = String::new();
        
        loop {
            let mut buf = [0u8; 1];
            ssh.stream.read_exact(&mut buf)?;
            
            let ch = buf[0] as char;
            banner.push(ch);
            
            if banner.ends_with("\r\n") {
                if banner.starts_with("SSH-") {
                    break;
                } else {
                    extra.push_str(&banner);
                    banner.clear();
                }
            }
        }
        
        // Check if we got a valid SSH-2.0 banner
        let re = Regex::new(r"^SSH-(\d\.\d+)").unwrap();
        let caps = re.captures(&banner).ok_or_else(|| SshError::InvalidSshVersion(banner.clone()))?;
        let version = caps.get(1).unwrap().as_str();
        
        if version != "1.99" && version != "2.0" {
            return Err(SshError::InvalidSshVersion(version.to_string()));
        }
        
        ssh.server_identifier = banner.trim_end().to_string();
        if !extra.is_empty() {
            ssh.errors.push(extra);
        }
        
        // Send our own banner
        writeln!(ssh.stream, "{}", ssh.identifier)?;
        
        // Get the server KEXINIT packet
        let response = ssh.get_binary_packet()?;
        if response[0] != SSH_MSG_KEXINIT {
            return Err(SshError::UnexpectedMessage(format!("Expected SSH_MSG_KEXINIT, got {}", response[0])));
        }
        
        // Perform key exchange
        ssh.key_exchange(&response)?;
        
        ssh.bitmap |= SSH2_MASK_CONSTRUCTOR;
        
        Ok(ssh)
    }
    
    /// Perform key exchange with the server
    fn key_exchange(&mut self, kexinit_payload_server: &[u8]) -> SshResult<()> {
        // Supported algorithms
        let kex_algorithms = vec![
            "diffie-hellman-group1-sha1",
            "diffie-hellman-group14-sha1",
        ];
        
        let server_host_key_algorithms = vec![
            "ssh-rsa",
            "ssh-dss",
        ];
        
        let encryption_algorithms = vec![
            "arcfour256",
            "arcfour128",
            "arcfour",
            "aes128-cbc",
            "aes192-cbc",
            "aes256-cbc",
            "aes128-ctr",
            "aes192-ctr",
            "aes256-ctr",
            "3des-ctr",
            "3des-cbc",
            "none",
        ];
        
        let mac_algorithms = vec![
            "hmac-sha1-96",
            "hmac-sha1",
            "hmac-md5-96",
            "hmac-md5",
            "none",
        ];
        
        let compression_algorithms = vec![
            "none",
        ];
        
        // Prepare our client KEXINIT payload
        let client_cookie: Vec<u8> = (0..16).map(|_| thread_rng().gen()).collect();
        
        // Parse server KEXINIT
        let mut response = kexinit_payload_server.to_vec();
        let server_cookie = response[1..17].to_vec();
        
        // Skip message number and cookie
        let mut pos = 17;
        
        // Parse key exchange algorithms
        let len = u32::from_be_bytes([response[pos], response[pos+1], response[pos+2], response[pos+3]]) as usize;
        pos += 4;
        let kex_algos_str = String::from_utf8_lossy(&response[pos..pos+len]).to_string();
        self.kex_algorithms = kex_algos_str.split(',').map(String::from).collect();
        pos += len;
        
        // Parse server host key algorithms
        let len = u32::from_be_bytes([response[pos], response[pos+1], response[pos+2], response[pos+3]]) as usize;
        pos += 4;
        let host_key_algos_str = String::from_utf8_lossy(&response[pos..pos+len]).to_string();
        self.server_host_key_algorithms = host_key_algos_str.split(',').map(String::from).collect();
        pos += len;
        
        // Parse encryption algorithms (client to server)
        let len = u32::from_be_bytes([response[pos], response[pos+1], response[pos+2], response[pos+3]]) as usize;
        pos += 4;
        let enc_algos_c2s_str = String::from_utf8_lossy(&response[pos..pos+len]).to_string();
        self.encryption_algorithms_client_to_server = enc_algos_c2s_str.split(',').map(String::from).collect();
        pos += len;
        
        // Parse encryption algorithms (server to client)
        let len = u32::from_be_bytes([response[pos], response[pos+1], response[pos+2], response[pos+3]]) as usize;
        pos += 4;
        let enc_algos_s2c_str = String::from_utf8_lossy(&response[pos..pos+len]).to_string();
        self.encryption_algorithms_server_to_client = enc_algos_s2c_str.split(',').map(String::from).collect();
        pos += len;
        
        // Parse MAC algorithms (client to server)
        let len = u32::from_be_bytes([response[pos], response[pos+1], response[pos+2], response[pos+3]]) as usize;
        pos += 4;
        let mac_algos_c2s_str = String::from_utf8_lossy(&response[pos..pos+len]).to_string();
        self.mac_algorithms_client_to_server = mac_algos_c2s_str.split(',').map(String::from).collect();
        pos += len;
        
        // Parse MAC algorithms (server to client)
        let len = u32::from_be_bytes([response[pos], response[pos+1], response[pos+2], response[pos+3]]) as usize;
        pos += 4;
        let mac_algos_s2c_str = String::from_utf8_lossy(&response[pos..pos+len]).to_string();
        self.mac_algorithms_server_to_client = mac_algos_s2c_str.split(',').map(String::from).collect();
        pos += len;
        
        // Parse compression algorithms (client to server)
        let len = u32::from_be_bytes([response[pos], response[pos+1], response[pos+2], response[pos+3]]) as usize;
        pos += 4;
        let comp_algos_c2s_str = String::from_utf8_lossy(&response[pos..pos+len]).to_string();
        self.compression_algorithms_client_to_server = comp_algos_c2s_str.split(',').map(String::from).collect();
        pos += len;
        
        // Parse compression algorithms (server to client)
        let len = u32::from_be_bytes([response[pos], response[pos+1], response[pos+2], response[pos+3]]) as usize;
        pos += 4;
        let comp_algos_s2c_str = String::from_utf8_lossy(&response[pos..pos+len]).to_string();
        self.compression_algorithms_server_to_client = comp_algos_s2c_str.split(',').map(String::from).collect();
        pos += len;
        
        // Parse languages (client to server)
        let len = u32::from_be_bytes([response[pos], response[pos+1], response[pos+2], response[pos+3]]) as usize;
        pos += 4;
        let langs_c2s_str = String::from_utf8_lossy(&response[pos..pos+len]).to_string();
        self.languages_client_to_server = langs_c2s_str.split(',').map(String::from).collect();
        pos += len;
        
        // Parse languages (server to client)
        let len = u32::from_be_bytes([response[pos], response[pos+1], response[pos+2], response[pos+3]]) as usize;
        pos += 4;
        let langs_s2c_str = String::from_utf8_lossy(&response[pos..pos+len]).to_string();
        self.languages_server_to_client = langs_s2c_str.split(',').map(String::from).collect();
        pos += len;
        
        // Check if first_kex_packet_follows flag is set
        let first_kex_packet_follows = response[pos] != 0;
        
        // Send our KEXINIT message
        let str_kex_algorithms = kex_algorithms.join(",");
        let str_server_host_key_algorithms = server_host_key_algorithms.join(",");
        let encryption_algorithms_client_to_server = encryption_algorithms.join(",");
        let encryption_algorithms_server_to_client = encryption_algorithms.join(",");
        let mac_algorithms_client_to_server = mac_algorithms.join(",");
        let mac_algorithms_server_to_client = mac_algorithms.join(",");
        let compression_algorithms_client_to_server = compression_algorithms.join(",");
        let compression_algorithms_server_to_client = compression_algorithms.join(",");
        
        let mut kexinit_payload_client = vec![SSH_MSG_KEXINIT];
        kexinit_payload_client.extend(&client_cookie);
        
        // Key exchange algorithms
        kexinit_payload_client.extend(&(str_kex_algorithms.len() as u32).to_be_bytes());
        kexinit_payload_client.extend(str_kex_algorithms.as_bytes());
        
        // Server host key algorithms
        kexinit_payload_client.extend(&(str_server_host_key_algorithms.len() as u32).to_be_bytes());
        kexinit_payload_client.extend(str_server_host_key_algorithms.as_bytes());
        
        // Encryption algorithms (client to server)
        kexinit_payload_client.extend(&(encryption_algorithms_client_to_server.len() as u32).to_be_bytes());
        kexinit_payload_client.extend(encryption_algorithms_client_to_server.as_bytes());
        
        // Encryption algorithms (server to client)
        kexinit_payload_client.extend(&(encryption_algorithms_server_to_client.len() as u32).to_be_bytes());
        kexinit_payload_client.extend(encryption_algorithms_server_to_client.as_bytes());
        
        // MAC algorithms (client to server)
        kexinit_payload_client.extend(&(mac_algorithms_client_to_server.len() as u32).to_be_bytes());
        kexinit_payload_client.extend(mac_algorithms_client_to_server.as_bytes());
        
        // MAC algorithms (server to client)
        kexinit_payload_client.extend(&(mac_algorithms_server_to_client.len() as u32).to_be_bytes());
        kexinit_payload_client.extend(mac_algorithms_server_to_client.as_bytes());
        
        // Compression algorithms (client to server)
        kexinit_payload_client.extend(&(compression_algorithms_client_to_server.len() as u32).to_be_bytes());
        kexinit_payload_client.extend(compression_algorithms_client_to_server.as_bytes());
        
        // Compression algorithms (server to client)
        kexinit_payload_client.extend(&(compression_algorithms_server_to_client.len() as u32).to_be_bytes());
        kexinit_payload_client.extend(compression_algorithms_server_to_client.as_bytes());
        
        // Languages (client to server) - empty
        kexinit_payload_client.extend(&(0u32).to_be_bytes());
        
        // Languages (server to client) - empty
        kexinit_payload_client.extend(&(0u32).to_be_bytes());
        
        // first_kex_packet_follows = false
        kexinit_payload_client.push(0);
        
        // Reserved
        kexinit_payload_client.extend(&(0u32).to_be_bytes());
        
        self.send_binary_packet(&kexinit_payload_client)?;
        
        // Decide which encryption algorithms to use
        let mut decrypt = "none";
        for algo in &encryption_algorithms {
            if self.encryption_algorithms_server_to_client.contains(&algo.to_string()) {
                decrypt = algo;
                break;
            }
        }
        
        let decrypt_key_length = match decrypt {
            "3des-cbc" | "3des-ctr" | "aes192-cbc" | "aes192-ctr" => 24,
            "aes256-cbc" | "aes256-ctr" | "arcfour256" => 32,
            "aes128-cbc" | "aes128-ctr" | "arcfour" | "arcfour128" => 16,
            "none"

}}} // Añadido por reparador automático