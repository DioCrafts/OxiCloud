//! Pure-Rust implementation of SSHv1.
//!
//! Here's a short example of how to use this library:
//!
//! use ssh1::SSH1;
//!
//! let mut ssh = SSH1::new("www.domain.tld", 22).unwrap();
//! if !ssh.login("username", "password").unwrap() {
//!     panic!("Login Failed");
//! }
//!
//! println!("{}", ssh.exec("ls -la").unwrap());
//!
//!
//! Here's another short example:
//!
//! use ssh1::SSH1;
//!
//! let mut ssh = SSH1::new("www.domain.tld", 22).unwrap();
//! if !ssh.login("username", "password").unwrap() {
//!     panic!("Login Failed");
//! }
//!
//! println!("{}", ssh.read("username@username:~$").unwrap());
//! ssh.write("ls -la\n").unwrap();
//! println!("{}", ssh.read("username@username:~$").unwrap());
//!
//!
//! More information on the SSHv1 specification can be found by reading
//! protocol-1.5.txt at http://www.snailbook.com/docs/protocol-1.5.txt.
//!
//! LICENSE: Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in
//! all copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
//! THE SOFTWARE.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use num_bigint::BigInt;
use rand::RngCore;
use rand::rngs::OsRng;
use regex::Regex;
use std::io;
use std::fmt;
use crypto::des::Des;
use crypto::des::TripleDes;
use crypto::rc4::Rc4;
use crypto::crypter::{Mode, Crypter};
use crypto::buffer::{RefReadBuffer, RefWriteBuffer, BufferResult};
use md5::{Md5, Digest};

/// No encryption
pub const CIPHER_NONE: u32 = 0;
/// IDEA in CFB mode - Not supported
pub const CIPHER_IDEA: u32 = 1;
/// DES in CBC mode
pub const CIPHER_DES: u32 = 2;
/// Triple-DES in CBC mode (all implementations are required to support this)
pub const CIPHER_3DES: u32 = 3;
/// TRI's Simple Stream encryption CBC - Not supported
pub const CIPHER_BROKEN_TSS: u32 = 4;
/// RC4 - Not supported
pub const CIPHER_RC4: u32 = 5;
/// Blowfish - Not supported
pub const CIPHER_BLOWFISH: u32 = 6;

/// .rhosts or /etc/hosts.equiv
pub const AUTH_RHOSTS: u32 = 1;
/// pure RSA authentication
pub const AUTH_RSA: u32 = 2;
/// password authentication
pub const AUTH_PASSWORD: u32 = 3;
/// .rhosts with RSA host authentication
pub const AUTH_RHOSTS_RSA: u32 = 4;

// Terminal Modes
const TTY_OP_END: u8 = 0;

// Message Response Types
const RESPONSE_TYPE: usize = 0;
const RESPONSE_DATA: usize = 1;

// Execution Bitmap Masks
const MASK_CONSTRUCTOR: u32 = 0x00000001;
const MASK_LOGIN: u32 = 0x00000002;
const MASK_SHELL: u32 = 0x00000004;

// Logging constants
const LOG_SIMPLE: u32 = 1;
const LOG_COMPLEX: u32 = 2;
const LOG_REALTIME: u32 = 3;
const LOG_REALTIME_FILE: u32 = 4;

// Read modes
const READ_SIMPLE: u32 = 1;
const READ_REGEX: u32 = 2;

// Message numbers
const MSG_DISCONNECT: u8 = 1;
const SMSG_PUBLIC_KEY: u8 = 2;
const CMSG_SESSION_KEY: u8 = 3;
const CMSG_USER: u8 = 4;
const CMSG_AUTH_PASSWORD: u8 = 9;
const CMSG_REQUEST_PTY: u8 = 10;
const CMSG_EXEC_SHELL: u8 = 12;
const CMSG_EXEC_CMD: u8 = 13;
const SMSG_SUCCESS: u8 = 14;
const SMSG_FAILURE: u8 = 15;
const CMSG_STDIN_DATA: u8 = 16;
const SMSG_STDOUT_DATA: u8 = 17;
const SMSG_STDERR_DATA: u8 = 18;
const CMSG_EOF: u8 = 19;
const SMSG_EXITSTATUS: u8 = 20;
const CMSG_EXIT_CONFIRMATION: u8 = 33;

const LOG_MAX_SIZE: usize = 1048576; // 1MB

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ProtocolError(String),
    CryptoError(String),
    AuthenticationError(String),
    ConnectionError(String),
    Timeout,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "I/O error: {}", err),
            Error::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
            Error::CryptoError(msg) => write!(f, "Cryptography error: {}", msg),
            Error::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            Error::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            Error::Timeout => write!(f, "Operation timed out"),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

// Crypto trait for different encryption methods
trait Crypto {
    fn encrypt(&mut self, data: &[u8]) -> Vec<u8>;
    fn decrypt(&mut self, data: &[u8]) -> Vec<u8>;
}

struct DesCrypto {
    crypter: Crypter,
}

impl DesCrypto {
    fn new(key: &[u8]) -> Self {
        let mut crypter = Crypter::new(crypto::symm::Type::Des, 
                                      Mode::Cbc, 
                                      key, 
                                      Some(&[0; 8])).unwrap();
        crypter.pad(false);
        Self { crypter }
    }
}

impl Crypto for DesCrypto {
    fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
        let mut read_buffer = RefReadBuffer::new(data);
        let mut buffer = vec![0; data.len() + self.crypter.block_size()];
        let mut write_buffer = RefWriteBuffer::new(&mut buffer);
        
        self.crypter.reset();
        let _ = self.crypter.update(&mut read_buffer, &mut write_buffer).unwrap();
        let _ = self.crypter.finalize(&mut write_buffer).unwrap();
        
        buffer.truncate(write_buffer.position());
        buffer
    }

    fn decrypt(&mut self, data: &[u8]) -> Vec<u8> {
        let mut read_buffer = RefReadBuffer::new(data);
        let mut buffer = vec![0; data.len() + self.crypter.block_size()];
        let mut write_buffer = RefWriteBuffer::new(&mut buffer);
        
        self.crypter.reset();
        let _ = self.crypter.update(&mut read_buffer, &mut write_buffer).unwrap();
        let _ = self.crypter.finalize(&mut write_buffer).unwrap();
        
        buffer.truncate(write_buffer.position());
        buffer
    }
}

struct TripleDesCrypto {
    crypter: Crypter,
}

impl TripleDesCrypto {
    fn new(key: &[u8]) -> Self {
        let mut crypter = Crypter::new(crypto::symm::Type::TripleDes, 
                                      Mode::Cbc, 
                                      key, 
                                      Some(&[0; 8])).unwrap();
        crypter.pad(false);
        Self { crypter }
    }
}

impl Crypto for TripleDesCrypto {
    fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
        let mut read_buffer = RefReadBuffer::new(data);
        let mut buffer = vec![0; data.len() + self.crypter.block_size()];
        let mut write_buffer = RefWriteBuffer::new(&mut buffer);
        
        self.crypter.reset();
        let _ = self.crypter.update(&mut read_buffer, &mut write_buffer).unwrap();
        let _ = self.crypter.finalize(&mut write_buffer).unwrap();
        
        buffer.truncate(write_buffer.position());
        buffer
    }

    fn decrypt(&mut self, data: &[u8]) -> Vec<u8> {
        let mut read_buffer = RefReadBuffer::new(data);
        let mut buffer = vec![0; data.len() + self.crypter.block_size()];
        let mut write_buffer = RefWriteBuffer::new(&mut buffer);
        
        self.crypter.reset();
        let _ = self.crypter.update(&mut read_buffer, &mut write_buffer).unwrap();
        let _ = self.crypter.finalize(&mut write_buffer).unwrap();
        
        buffer.truncate(write_buffer.position());
        buffer
    }
}

/// Main SSH1 client implementation
pub struct SSH1 {
    identifier: String,
    fsock: Option<TcpStream>,
    crypto: Option<Box<dyn Crypto>>,
    bitmap: u32,
    server_key_public_exponent: Option<BigInt>,
    server_key_public_modulus: Option<BigInt>,
    host_key_public_exponent: Option<BigInt>,
    host_key_public_modulus: Option<BigInt>,
    supported_ciphers: HashMap<u32, String>,
    supported_authentications: HashMap<u32, String>,
    server_identification: String,
    protocol_flags: HashMap<u8, String>,
    protocol_flag_log: Vec<String>,
    message_log: Vec<Vec<u8>>,
    realtime_log_file: Option<File>,
    realtime_log_size: usize,
    realtime_log_wrap: bool,
    interactive_buffer: String,
    timeout: Option<Duration>,
    cur_timeout: Option<Duration>,
}

impl SSH1 {
    /// Creates a new SSH1 client and connects to the specified host.
    ///
    /// # Arguments
    ///
    /// * `host` - Hostname or IP address to connect to
    /// * `port` - Port number (default: 22)
    /// * `timeout` - Connection timeout in seconds (default: 10)
    /// * `cipher` - Encryption cipher to use (default: CIPHER_3DES)
    ///
    /// # Returns
    ///
    /// A Result containing either the connected SSH1 client or an error
    pub fn new(host: &str, port: u16, timeout_secs: u64, cipher: u32) -> Result<Self> {
        let mut ssh = SSH1 {
            identifier: String::from("SSH-1.5-rustseclib"),
            fsock: None,
            crypto: None,
            bitmap: 0,
            server_key_public_exponent: None,
            server_key_public_modulus: None,
            host_key_public_exponent: None,
            host_key_public_modulus: None,
            supported_ciphers: HashMap::new(),
            supported_authentications: HashMap::new(),
            server_identification: String::new(),
            protocol_flags: HashMap::new(),
            protocol_flag_log: Vec::new(),
            message_log: Vec::new(),
            realtime_log_file: None,
            realtime_log_size: 0,
            realtime_log_wrap: false,
            interactive_buffer: String::new(),
            timeout: Some(Duration::from_secs(timeout_secs)),
            cur_timeout: Some(Duration::from_secs(timeout_secs)),
        };
        
        // Initialize protocol flags map
        ssh.protocol_flags.insert(1, "NET_SSH1_MSG_DISCONNECT".to_string());
        ssh.protocol_flags.insert(2, "NET_SSH1_SMSG_PUBLIC_KEY".to_string());
        ssh.protocol_flags.insert(3, "NET_SSH1_CMSG_SESSION_KEY".to_string());
        ssh.protocol_flags.insert(4, "NET_SSH1_CMSG_USER".to_string());
        ssh.protocol_flags.insert(9, "NET_SSH1_CMSG_AUTH_PASSWORD".to_string());
        ssh.protocol_flags.insert(10, "NET_SSH1_CMSG_REQUEST_PTY".to_string());
        ssh.protocol_flags.insert(12, "NET_SSH1_CMSG_EXEC_SHELL".to_string());
        ssh.protocol_flags.insert(13, "NET_SSH1_CMSG_EXEC_CMD".to_string());
        ssh.protocol_flags.insert(14, "NET_SSH1_SMSG_SUCCESS".to_string());
        ssh.protocol_flags.insert(15, "NET_SSH1_SMSG_FAILURE".to_string());
        ssh.protocol_flags.insert(16, "NET_SSH1_CMSG_STDIN_DATA".to_string());
        ssh.protocol_flags.insert(17, "NET_SSH1_SMSG_STDOUT_DATA".to_string());
        ssh.protocol_flags.insert(18, "NET_SSH1_SMSG_STDERR_DATA".to_string());
        ssh.protocol_flags.insert(19, "NET_SSH1_CMSG_EOF".to_string());
        ssh.protocol_flags.insert(20, "NET_SSH1_SMSG_EXITSTATUS".to_string());
        ssh.protocol_flags.insert(33, "NET_SSH1_CMSG_EXIT_CONFIRMATION".to_string());
        
        // Initialize supported ciphers map
        ssh.supported_ciphers.insert(CIPHER_NONE, "No encryption".to_string());
        ssh.supported_ciphers.insert(CIPHER_IDEA, "IDEA in CFB mode".to_string());
        ssh.supported_ciphers.insert(CIPHER_DES, "DES in CBC mode".to_string());
        ssh.supported_ciphers.insert(CIPHER_3DES, "Triple-DES in CBC mode".to_string());
        ssh.supported_ciphers.insert(CIPHER_BROKEN_TSS, "TRI's Simple Stream encryption CBC".to_string());
        ssh.supported_ciphers.insert(CIPHER_RC4, "RC4".to_string());
        ssh.supported_ciphers.insert(CIPHER_BLOWFISH, "Blowfish".to_string());
        
        // Initialize supported authentications map
        ssh.supported_authentications.insert(AUTH_RHOSTS, ".rhosts or /etc/hosts.equiv".to_string());
        ssh.supported_authentications.insert(AUTH_RSA, "pure RSA authentication".to_string());
        ssh.supported_authentications.insert(AUTH_PASSWORD, "password authentication".to_string());
        ssh.supported_authentications.insert(AUTH_RHOSTS_RSA, ".rhosts with RSA host authentication".to_string());
        
        // Connect to host
        let timeout = Duration::from_secs(timeout_secs);
        let mut stream = TcpStream::connect_timeout(&format!("{}:{}", host, port).parse().unwrap(), timeout)?;
        stream.set_read_timeout(Some(timeout))?;
        stream.set_write_timeout(Some(timeout))?;
        
        // Read server identification
        let mut init_line = String::new();
        let mut reader = io::BufReader::new(&stream);
        reader.read_line(&mut init_line)?;
        ssh.server_identification = init_line.clone();
        
        if let Some(logging) = option_env!("SSH1_LOGGING") {
            ssh.append_log("<-".to_string(), ssh.server_identification.as_bytes().to_vec());
            ssh.append_log("->".to_string(), format!("{}\r\n", ssh.identifier).as_bytes().to_vec());
        }
        
        // Check for SSH protocol
        let re = Regex::new(r"SSH-([0-9\.]+)-(.+)").unwrap();
        let caps = re.captures(&init_line).ok_or_else(|| 
            Error::ProtocolError("Can only connect to SSH servers".to_string()))?;
        
        let version = caps.get(1).unwrap().as_str();
        if !version.starts_with('1') {
            return Err(Error::ProtocolError(format!("Cannot connect to SSH {} servers", version)));
        }
        
        // Send client identification
        writeln!(stream, "{}", ssh.identifier)?;
        
        ssh.fsock = Some(stream);
        
        // Get server's public key
        let response = ssh.get_binary_packet()?;
        if response[RESPONSE_TYPE] != SMSG_PUBLIC_KEY as u8 {
            return Err(Error::ProtocolError("Expected SSH_SMSG_PUBLIC_KEY".to_string()));
        }
        
        let mut data = &response[RESPONSE_DATA];
        
        // Extract cookie
        let anti_spoofing_cookie = Self::extract_bytes(&mut data, 8);
        
        // Skip 4 bytes
        Self::extract_bytes(&mut data, 4);
        
        // Extract server key public exponent
        let len = ((data[0] as u16) << 8) | data[1] as u16;
        data = &data[2..];
        let exponent_bytes = Self::extract_bytes(&mut data, ((len + 7) / 8) as usize);
        let server_key_public_exponent = Some(BigInt::from_bytes_be(num_bigint::Sign::Plus, &exponent_bytes));
        
        // Extract server key public modulus
        let len = ((data[0] as u16) << 8) | data[1] as u16;
        data = &data[2..];
        let modulus_bytes = Self::extract_bytes(&mut data, ((len + 7) / 8) as usize);
        let server_key_public_modulus = Some(BigInt::from_bytes_be(num_bigint::Sign::Plus, &modulus_bytes));
        
        // Skip 4 bytes
        Self::extract_bytes(&mut data, 4);
        
        // Extract host key public exponent
        let len = ((data[0] as u16) << 8) | data[1] as u16;
        data = &data[2..];
        let exponent_bytes = Self::extract_bytes(&mut data, ((len + 7) / 8) as usize);
        let host_key_public_exponent = Some(BigInt::from_bytes_be(num_bigint::Sign::Plus, &exponent_bytes));
        
        // Extract host key public modulus
        let len = ((data[0] as u16) << 8) | data[1] as u16;
        data = &data[2..];
        let modulus_bytes = Self::extract_bytes(&mut data, ((len + 7) / 8) as usize);
        let host_key_public_modulus = Some(BigInt::from_bytes_be(num_bigint::Sign::Plus, &modulus_bytes));
        
        // Skip 4 bytes
        Self::extract_bytes(&mut data, 4);
        
        // Get supported ciphers
        let supported_ciphers_mask = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        data = &data[4..];
        
        let mut supported_ciphers = HashMap::new();
        for (mask, name) in &ssh.supported_ciphers {
            if (supported_ciphers_mask & (1 << mask)) != 0 {
                supported_ciphers.insert(*mask, name.clone());
            }
        }
        ssh.supported_ciphers = supported_ciphers;
        
        // Get supported authentications
        let supported_authentications_mask = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        data = &data[4..];
        
        let mut supported_authentications = HashMap::new();
        for (mask, name) in &ssh.supported_authentications {
            if (supported_authentications_mask & (1 << mask)) != 0 {
                supported_authentications.insert(*mask, name.clone());
            }
        }
        ssh.supported_authentications = supported_authentications;
        
        // Generate session ID
        let server_mod_bytes = server_key_public_modulus.as_ref().unwrap().to_bytes_be().1;
        let host_mod_bytes = host_key_public_modulus.as_ref().unwrap().to_bytes_be().1;
        
        let mut hasher = Md5::new();
        hasher.update(&host_mod_bytes);
        hasher.update(&server_mod_bytes);
        hasher.update(&anti_spoofing_cookie);
        let session_id = hasher.finalize();
        
        // Generate random session key
        let mut session_key = vec![0u8; 32];
        OsRng.fill_bytes(&mut session_key);
        
        // XOR session key with session ID
        let mut double_encrypted_session_key = session_key.clone();
        for i in 0..session_id.len() {
            double_encrypted_session_key[i] ^= session_id[i];
        }
        
        // RSA encrypt session key
        if server_key_public_modulus.as_ref().unwrap() < host_key_public_modulus.as_ref().unwrap() {
            double_encrypted_session_key = ssh.rsa_crypt(
                &double_encrypted_session_key,
                server_key_public_exponent.as_ref().unwrap(),
                server_key_public_modulus.as_ref().unwrap()
            );
            
            double_encrypted_session_key = ssh.rsa_crypt(
                &double_encrypted_session_key,
                host_key_public_exponent.as_ref().unwrap(),
                host_key_public_modulus.as_ref().unwrap()
            );
        } else {
            double_encrypted_session_key = ssh.rsa_crypt(
                &double_encrypted_session_key,
                host_key_public_exponent.as_ref().unwrap(),
                host_key_public_modulus.as_ref().unwrap()
            );
            
            double_encrypted_session_key = ssh.rsa_crypt(
                &double_encrypted_session_key,
                server_key_public_exponent.as_ref().unwrap(),
                server_key_public_modulus.as_ref().unwrap()
            );
        }
        
        // Determine cipher to use
        let cipher = if ssh.supported_ciphers.contains_key(&cipher) { cipher } else { CIPHER_3DES };
        
        // Send session key
        let mut data = Vec::new();
        data.push(CMSG_SESSION_KEY);
        data.push(cipher as u8);
        data.extend_from_slice(&anti_spoofing_cookie);
        data.extend_from_slice(&((8 * double_encrypted_session_key.len() as u32).to_be_bytes()));
        data.extend_from_slice(&double_encrypted_session_key);
        data.extend_from_slice(&[0, 0, 0, 0]); // Protocol flags (none)
        
        if !ssh.send_binary_packet(&data)? {
            return Err(Error::ProtocolError("Error sending SSH_CMSG_SESSION_KEY".to_string()));
        }
        
        // Initialize encryption
        match cipher {
            CIPHER_DES => {
                let crypto = Box::new(DesCrypto::new(&session_key[0..8]));
                ssh.crypto = Some(crypto);
            },
            CIPHER_3DES => {
                let crypto = Box::new(TripleDesCrypto::new(&session_key[0..24]));
                ssh.crypto = Some(crypto);
            },
            _ => {
                return Err(Error::CryptoError(format!("Unsupported cipher: {}", cipher)));
            }
        }
        
        // Get server response
        let response = ssh.get_binary_packet()?;
        
        if response[RESPONSE_TYPE] != SMSG_SUCCESS as u8 {
            return Err(Error::ProtocolError("Expected SSH_SMSG_SUCCESS".to_string()));
        }
        
        ssh.bitmap = MASK_CONSTRUCTOR;
        ssh.server_key_public_exponent = server_key_public_exponent;
        ssh.server_key_public_modulus = server_key_public_modulus;
        ssh.host_key_public_exponent = host_key_public_exponent;
        ssh.host_key_public_modulus = host_key_public_modulus;
        
        Ok(ssh)
    }
    
    /// Extract a fixed number of bytes from a data slice
    fn extract_bytes(data: &mut &[u8], n: usize) -> Vec<u8> {
        let bytes = data[..n].to_vec();
        *data = &data[n..];
        bytes
    }
    
    /// RSA encrypt using the given public key
    fn rsa_crypt(&self, data: &[u8], exponent: &BigInt, modulus: &BigInt) -> Vec<u8> {
        // PKCS#1 v1.5 padding
        let modulus_bytes = modulus.to_bytes_be().1;
        let modulus_len = modulus_bytes.len();
        
        let mut padded = vec![0];
        padded.push(2);
        
        // Generate non-zero random padding
        let padding_len = modulus_len - data.len() - 3;
        let mut padding = vec![0; padding_len];
        OsRng.fill_bytes(&mut padding);
        for b in padding.iter_mut() {
            while *b == 0 {
                OsRng.fill_bytes(std::slice::from_mut(b));
            }
        }
        padded.extend_from_slice(&padding);
        
        padded.push(0);
        padded.extend_from_slice(data);
        
        // RSA encryption: m^e mod n
        let m = BigInt::from_bytes_be(num_bigint::Sign::Plus, &padded);
        let c = m.modpow(exponent, modulus);
        
        c.to_bytes_be().1
    }
    
    /// Login to the SSH server with the given username and password
    pub fn login(&mut self, username: &str, password: &str) -> Result<bool> {
        if self.bitmap & MASK_CONSTRUCTOR == 0 {
            return Err(Error::ProtocolError("SSH connection not established".to_string()));
        }
        
        // Send username
        let mut data = Vec::new();
        data.push(CMSG_USER);
        data.extend_from_slice(&(username.len() as u32).to_be_bytes());
        data.extend_from_slice(username.as_bytes());
        
        if !self.send_binary_packet(&data)? {
            return Err(Error::ProtocolError("Error sending SSH_CMSG_USER".to_string()));
        }
        
        // Get server response
        let response = self.get_binary_packet()?;
        
        if response.is_empty() {
            return Ok(false);
        }
        
        if response[RESPONSE_TYPE] == SMSG_SUCCESS as u8 {
            self.bitmap |= MASK_LOGIN;
            return Ok(true);
        } else if response[RESPONSE_TYPE] != SMSG_FAILURE as u8 {
            return Err(Error::ProtocolError("Expected SSH_SMSG_SUCCESS or SSH_SMSG_FAILURE".to_string()));
        }
        
        // Send password
        let mut data = Vec::new();
        data.push(CMSG_AUTH_PASSWORD);
        data.extend_from_slice(&(password.len() as u32).to_be_bytes());
        data.extend_from_slice(password.as_bytes());
        
        if !self.send_binary_packet(&data)? {
            return Err(Error::ProtocolError("Error sending SSH_CMSG_AUTH_PASSWORD".to_string()));
        }
        
        // Get server response
        let response = self.get_binary_packet()?;
        
        if response.is_empty() {
            return Ok(false);
        }
        
        if response[RESPONSE_TYPE] == SMSG_SUCCESS as u8 {
            self.bitmap |= MASK_LOGIN;
            return Ok(true);
        } else if response[RESPONSE_TYPE] == SMSG_FAILURE as u8 {
            return Ok(false);
        } else {
            return Err(Error::ProtocolError("Expected SSH_SMSG_SUCCESS or SSH_SMSG_FAILURE".to_string()));
        }
    }
    
    /// Set timeout for operations
    pub fn set_timeout(&mut self, timeout_secs: u64) {
        if timeout_secs == 0 {
            self.timeout = None;
            self.cur_timeout = None;
        } else {
            