// Pure-Rust implementation of DES.
//
// This module provides a pure-Rust implementation of the Data Encryption Standard (DES).
// It supports various encryption modes: ECB, CBC, CFB, OFB, and CTR.
//
// # Example
//
// use des::Des;
//
// let mut des = Des::new(des::Mode::Cbc);
// des.set_key("abcdefgh");
//
// let size = 10 * 1024;
// let plaintext = "a".repeat(size);
//
// let encrypted = des.encrypt(&plaintext);
// let decrypted = des.decrypt(&encrypted);
//
// assert_eq!(plaintext, decrypted);
//

use std::convert::TryInto;

/// DES encryption mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    /// Counter mode
    Ctr = -1,
    /// Electronic Code Book mode
    Ecb = 1,
    /// Cipher Block Chaining mode
    Cbc = 2,
    /// Cipher Feedback mode
    Cfb = 3,
    /// Output Feedback mode
    Ofb = 4,
}

/// DES encryption direction
#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Encrypt = 0,
    Decrypt = 1,
}

/// Implementation of DES (Data Encryption Standard) encryption algorithm
pub struct Des {
    /// The Key Schedule
    keys: Vec<Vec<[u32; 8]>>,
    
    /// The Encryption Mode
    mode: Mode,
    
    /// Continuous Buffer status
    continuous_buffer: bool,
    
    /// Padding status
    padding: bool,
    
    /// The Initialization Vector
    iv: [u8; 8],
    
    /// A "sliding" Initialization Vector for encryption
    encrypt_iv: [u8; 8],
    
    /// A "sliding" Initialization Vector for decryption
    decrypt_iv: [u8; 8],
    
    /// Is the mode one that is paddable?
    paddable: bool,
    
    /// Encryption buffer for CTR, OFB and CFB modes
    enbuffer: Buffer,
    
    /// Decryption buffer for CTR, OFB and CFB modes
    debuffer: Buffer,
}

/// Buffer structure for encryption/decryption operations
struct Buffer {
    /// Encrypted/decrypted data
    data: Vec<u8>,
    /// XOR data
    xor: Vec<u8>,
    /// Position in buffer
    pos: usize,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            xor: Vec::new(),
            pos: 0,
        }
    }
}

impl Des {
    /// Creates a new Des instance with the specified mode
    pub fn new(mode: Mode) -> Self {
        let paddable = match mode {
            Mode::Ecb | Mode::Cbc => true,
            _ => false,
        };
        
        let mut des = Self {
            keys: vec![Vec::new(); 2],
            mode,
            continuous_buffer: false,
            padding: true,
            iv: [0; 8],
            encrypt_iv: [0; 8],
            decrypt_iv: [0; 8],
            paddable,
            enbuffer: Buffer::default(),
            debuffer: Buffer::default(),
        };
        
        // Default key is all zeros
        des.set_key(vec![0; 8]);
        
        des
    }

    /// Sets the encryption/decryption key.
    ///
    /// Keys can be of any length. DES uses 64-bit keys (8 bytes), but only the first
    /// 56 bits are actually used (the remaining 8 bits are parity bits). If the key is
    /// shorter than 8 bytes, it will be padded with zeros.
    pub fn set_key<T: AsRef<[u8]>>(&mut self, key: T) {
        let key_bytes = key.as_ref();
        let mut key_padded = [0u8; 8];
        
        // Copy up to 8 bytes from the key
        for (i, &byte) in key_bytes.iter().take(8).enumerate() {
            key_padded[i] = byte;
        }
        
        self.keys = self.prepare_key(&key_padded);
    }

    /// Sets the password using the specified method
    pub fn set_password(&mut self, password: &str, method: &str, hash: Option<&str>, salt: Option<&str>, count: Option<usize>) {
        let hash = hash.unwrap_or("sha1");
        let salt = salt.unwrap_or("phpseclib/salt");
        let count = count.unwrap_or(1000);
        
        let mut key = Vec::new();
        
        match method {
            "pbkdf2" => {
                // This is a simplified PBKDF2 implementation
                // In a real implementation, you would use the ring crate or similar
                let mut i = 1;
                while key.len() < 8 {
                    let mut hmac_input = salt.as_bytes().to_vec();
                    hmac_input.extend_from_slice(&i.to_be_bytes());
                    
                    let mut f = self.hmac(hash, password.as_bytes(), &hmac_input);
                    let mut u = f.clone();
                    
                    for _ in 2..=count {
                        u = self.hmac(hash, password.as_bytes(), &u);
                        for j in 0..f.len() {
                            f[j] ^= u[j];
                        }
                    }
                    
                    key.extend_from_slice(&f);
                    i += 1;
                }
            },
            _ => panic!("Unsupported password method"),
        }
        
        self.set_key(&key[0..8]);
    }

    /// Sets the initialization vector
    ///
    /// IV is not required when ECB mode is being used. If not explicitly set,
    /// it will be assumed to be all zeros.
    pub fn set_iv<T: AsRef<[u8]>>(&mut self, iv: T) {
        let iv_bytes = iv.as_ref();
        
        // Pad or truncate the IV to exactly 8 bytes
        let mut iv_padded = [0u8; 8];
        for (i, &byte) in iv_bytes.iter().take(8).enumerate() {
            iv_padded[i] = byte;
        }
        
        self.iv = iv_padded;
        self.encrypt_iv = iv_padded;
        self.decrypt_iv = iv_padded;
    }

    /// Enable continuous buffer mode
    ///
    /// When enabled, encryption/decryption operations maintain state between calls.
    /// This allows encrypting/decrypting data in chunks while maintaining proper
    /// operation of the selected cipher mode.
    pub fn enable_continuous_buffer(&mut self) {
        self.continuous_buffer = true;
    }

    /// Disable continuous buffer mode
    ///
    /// This is the default behavior. When disabled, each encryption/decryption
    /// operation is independent.
    pub fn disable_continuous_buffer(&mut self) {
        self.continuous_buffer = false;
        self.encrypt_iv = self.iv;
        self.decrypt_iv = self.iv;
        self.enbuffer = Buffer::default();
        self.debuffer = Buffer::default();
    }

    /// Enable padding
    ///
    /// DES works by encrypting eight bytes at a time. If input is not a multiple of eight,
    /// it becomes necessary to pad the input so that its length is a multiple of eight.
    /// Padding is enabled by default.
    pub fn enable_padding(&mut self) {
        self.padding = true;
    }

    /// Disable padding
    pub fn disable_padding(&mut self) {
        self.padding = false;
    }

    /// Encrypts a message
    ///
    /// The plaintext will be padded with up to 8 additional bytes if padding is enabled.
    pub fn encrypt(&mut self, plaintext: &str) -> Vec<u8> {
        let plaintext = plaintext.as_bytes();
        let mut plaintext_vec = plaintext.to_vec();
        
        if self.paddable && self.padding {
            plaintext_vec = self.pad(&plaintext_vec);
        }
        
        match self.mode {
            Mode::Ecb => self.encrypt_ecb(&plaintext_vec),
            Mode::Cbc => self.encrypt_cbc(&plaintext_vec),
            Mode::Cfb => self.encrypt_cfb(&plaintext_vec),
            Mode::Ofb => self.encrypt_ofb(&plaintext_vec),
            Mode::Ctr => self.encrypt_ctr(&plaintext_vec),
        }
    }

    /// Decrypts a message
    ///
    /// The ciphertext must be a multiple of 8 bytes long.
    pub fn decrypt(&mut self, ciphertext: &[u8]) -> String {
        let mut ciphertext_vec = ciphertext.to_vec();
        
        // We pad with zeros to make sure the length is a multiple of 8
        if self.paddable {
            let padding_length = (8 - (ciphertext_vec.len() % 8)) % 8;
            ciphertext_vec.extend(vec![0; padding_length]);
        }
        
        let plaintext = match self.mode {
            Mode::Ecb => self.decrypt_ecb(&ciphertext_vec),
            Mode::Cbc => self.decrypt_cbc(&ciphertext_vec),
            Mode::Cfb => self.decrypt_cfb(&ciphertext_vec),
            Mode::Ofb => self.decrypt_ofb(&ciphertext_vec),
            Mode::Ctr => self.decrypt_ctr(&ciphertext_vec),
        };
        
        let unpadded = if self.paddable && self.padding {
            self.unpad(&plaintext)
        } else {
            plaintext
        };
        
        // Convert to string, replacing invalid UTF-8 with replacement character
        String::from_utf8_lossy(&unpadded).into_owned()
    }

    // Private implementation methods

    fn encrypt_ecb(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        
        for chunk in plaintext.chunks(8) {
            if chunk.len() == 8 {
                let processed = self.process_block(chunk, Direction::Encrypt);
                ciphertext.extend_from_slice(&processed);
            }
        }
        
        ciphertext
    }
    
    fn decrypt_ecb(&mut self, ciphertext: &[u8]) -> Vec<u8> {
        let mut plaintext = Vec::with_capacity(ciphertext.len());
        
        for chunk in ciphertext.chunks(8) {
            if chunk.len() == 8 {
                let processed = self.process_block(chunk, Direction::Decrypt);
                plaintext.extend_from_slice(&processed);
            }
        }
        
        plaintext
    }
    
    fn encrypt_cbc(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        let mut xor = self.encrypt_iv;
        
        for chunk in plaintext.chunks(8) {
            if chunk.len() == 8 {
                // XOR the plaintext with the previous ciphertext block
                let mut block = [0u8; 8];
                for i in 0..8 {
                    block[i] = chunk[i] ^ xor[i];
                }
                
                // Encrypt the XORed block
                let encrypted = self.process_block(&block, Direction::Encrypt);
                ciphertext.extend_from_slice(&encrypted);
                
                // Save this ciphertext block for the next iteration
                xor.copy_from_slice(&encrypted);
            }
        }
        
        if self.continuous_buffer {
            self.encrypt_iv = xor;
        }
        
        ciphertext
    }
    
    fn decrypt_cbc(&mut self, ciphertext: &[u8]) -> Vec<u8> {
        let mut plaintext = Vec::with_capacity(ciphertext.len());
        let mut xor = self.decrypt_iv;
        
        for chunk in ciphertext.chunks(8) {
            if chunk.len() == 8 {
                // Decrypt the block
                let decrypted = self.process_block(chunk, Direction::Decrypt);
                
                // XOR with the previous ciphertext block
                let mut block = [0u8; 8];
                for i in 0..8 {
                    block[i] = decrypted[i] ^ xor[i];
                }
                
                plaintext.extend_from_slice(&block);
                
                // Save this ciphertext block for the next iteration
                xor.copy_from_slice(chunk);
            }
        }
        
        if self.continuous_buffer {
            self.decrypt_iv = xor;
        }
        
        plaintext
    }
    
    fn encrypt_cfb(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::new();
        
        if self.continuous_buffer {
            let mut iv = &mut self.encrypt_iv;
            let pos = &mut self.enbuffer.pos;
            
            let mut i = 0;
            let len = plaintext.len();
            
            if *pos > 0 {
                let orig_pos = *pos;
                let max = 8 - orig_pos;
                
                if len >= max {
                    i = max;
                    *pos = 0;
                } else {
                    i = len;
                    *pos += len;
                }
                
                let mut tmp = Vec::new();
                for j in 0..i {
                    tmp.push(iv[orig_pos + j] ^ plaintext[j]);
                }
                
                for j in 0..i {
                    iv[orig_pos + j] = tmp[j];
                }
                
                ciphertext.extend_from_slice(&tmp);
            }
            
            while i + 8 <= len {
                let encrypted = self.process_block(&iv, Direction::Encrypt);
                let mut block = [0u8; 8];
                
                for j in 0..8 {
                    block[j] = encrypted[j] ^ plaintext[i + j];
                }
                
                ciphertext.extend_from_slice(&block);
                iv.copy_from_slice(&block);
                
                i += 8;
            }
            
            if i < len {
                let remaining = len - i;
                let encrypted = self.process_block(&iv, Direction::Encrypt);
                
                for j in 0..remaining {
                    let byte = encrypted[j] ^ plaintext[i + j];
                    ciphertext.push(byte);
                    iv[j] = byte;
                }
                
                *pos = remaining;
            }
        } else {
            let mut iv = self.encrypt_iv;
            let mut i = 0;
            let len = plaintext.len();
            
            while i + 8 <= len {
                let encrypted = self.process_block(&iv, Direction::Encrypt);
                let mut block = [0u8; 8];
                
                for j in 0..8 {
                    block[j] = encrypted[j] ^ plaintext[i + j];
                }
                
                ciphertext.extend_from_slice(&block);
                iv.copy_from_slice(&block);
                
                i += 8;
            }
            
            if i < len {
                let remaining = len - i;
                let encrypted = self.process_block(&iv, Direction::Encrypt);
                
                for j in 0..remaining {
                    ciphertext.push(encrypted[j] ^ plaintext[i + j]);
                }
            }
        }
        
        ciphertext
    }
    
    fn decrypt_cfb(&mut self, ciphertext: &[u8]) -> Vec<u8> {
        let mut plaintext = Vec::new();
        
        if self.continuous_buffer {
            let mut iv = &mut self.decrypt_iv;
            let pos = &mut self.debuffer.pos;
            
            let mut i = 0;
            let len = ciphertext.len();
            
            if *pos > 0 {
                let orig_pos = *pos;
                let max = 8 - orig_pos;
                
                if len >= max {
                    i = max;
                    *pos = 0;
                } else {
                    i = len;
                    *pos += len;
                }
                
                let encrypted = self.process_block(&iv, Direction::Encrypt);
                
                for j in 0..i {
                    plaintext.push(encrypted[orig_pos + j] ^ ciphertext[j]);
                    iv[orig_pos + j] = ciphertext[j];
                }
            }
            
            while i + 8 <= len {
                let encrypted = self.process_block(&iv, Direction::Encrypt);
                
                for j in 0..8 {
                    plaintext.push(encrypted[j] ^ ciphertext[i + j]);
                }
                
                iv.copy_from_slice(&ciphertext[i..i+8]);
                
                i += 8;
            }
            
            if i < len {
                let remaining = len - i;
                let encrypted = self.process_block(&iv, Direction::Encrypt);
                
                for j in 0..remaining {
                    plaintext.push(encrypted[j] ^ ciphertext[i + j]);
                    iv[j] = ciphertext[i + j];
                }
                
                *pos = remaining;
            }
        } else {
            let mut iv = self.decrypt_iv;
            let mut i = 0;
            let len = ciphertext.len();
            
            while i + 8 <= len {
                let encrypted = self.process_block(&iv, Direction::Encrypt);
                
                for j in 0..8 {
                    plaintext.push(encrypted[j] ^ ciphertext[i + j]);
                }
                
                iv.copy_from_slice(&ciphertext[i..i+8]);
                
                i += 8;
            }
            
            if i < len {
                let remaining = len - i;
                let encrypted = self.process_block(&iv, Direction::Encrypt);
                
                for j in 0..remaining {
                    plaintext.push(encrypted[j] ^ ciphertext[i + j]);
                }
            }
        }
        
        plaintext
    }
    
    fn encrypt_ofb(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        let mut xor = self.encrypt_iv;
        let buffer = &mut self.enbuffer;
        
        let mut i = 0;
        
        if !buffer.xor.is_empty() {
            while i < plaintext.len() && !buffer.xor.is_empty() {
                let xor_byte = buffer.xor.remove(0);
                ciphertext.push(plaintext[i] ^ xor_byte);
                i += 1;
            }
        }
        
        while i + 8 <= plaintext.len() {
            xor = self.process_block(&xor, Direction::Encrypt);
            
            for j in 0..8 {
                ciphertext.push(plaintext[i + j] ^ xor[j]);
            }
            
            i += 8;
        }
        
        if i < plaintext.len() {
            xor = self.process_block(&xor, Direction::Encrypt);
            let mut key = xor;
            
            for j in 0..(plaintext.len() - i) {
                ciphertext.push(plaintext[i + j] ^ xor[j]);
            }
            
            if self.continuous_buffer {
                let start = plaintext.len() % 8;
                if start != 0 {
                    buffer.xor = key[start..].to_vec();
                }
            }
        }
        
        if self.continuous_buffer {
            self.encrypt_iv = xor;
        }
        
        ciphertext
    }
    
    fn decrypt_ofb(&mut self, ciphertext: &[u8]) -> Vec<u8> {
        // OFB mode decryption is identical to encryption
        self.encrypt_ofb(ciphertext)
    }
    
    fn encrypt_ctr(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        let mut xor = self.encrypt_iv;
        let buffer = &mut self.enbuffer;
        
        let mut i = 0;
        
        if !buffer.data.is_empty() {
            while i < plaintext.len() && !buffer.data.is_empty() {
                let key_byte = buffer.data.remove(0);
                ciphertext.push(plaintext[i] ^ key_byte);
                i += 1;
            }
        }
        
        while i + 8 <= plaintext.len() {
            let key = self.process_block(&self.generate_xor(&mut xor), Direction::Encrypt);
            
            for j in 0..8 {
                ciphertext.push(plaintext[i + j] ^ key[j]);
            }
            
            i += 8;
        }
        
        if i < plaintext.len() {
            let key = self.process_block(&self.generate_xor(&mut xor), Direction::Encrypt);
            
            for j in 0..(plaintext.len() - i) {
                ciphertext.push(plaintext[i + j] ^ key[j]);
            }
            
            if self.continuous_buffer {
                let start = plaintext.len() % 8;
                if start != 0 {
                    buffer.data = key[start..].to_vec();
                }
            }
        }
        
        if self.continuous_buffer {
            self.encrypt_iv = xor;
        }
        
        ciphertext
    }
    
    fn decrypt_ctr(&mut self, ciphertext: &[u8]) -> Vec<u8> {
        // CTR mode decryption is identical to encryption
        self.encrypt_ctr(ciphertext)
    }

    /// Generates the next counter value for CTR mode
    fn generate_xor(&self, iv: &mut [u8; 8]) -> [u8; 8] {
        let xor = *iv;
        
        // Increment the counter
        for j in (0..8).rev() {
            iv[j] = iv[j].wrapping_add(1);
            if iv[j] != 0 {
                break;
            }
        }
        
        xor
    }

    /// Processes a single 8-byte block
    fn process_block(&self, block: &[u8], mode: Direction) -> [u8; 8] {
        // s-boxes
        static SBOX: [[u32; 64]; 8] = [
            [
                14,  0,  4, 15, 13,  7,  1,  4,  2, 14, 15,  2, 11, 13,  8,  1,
                 3, 10, 10,  6,  6, 12, 12, 11,  5,  9,  9,  5,  0,  3,  7,  8,
                 4, 15,  1, 12, 14,  8,  8,  2, 13,  4,  6,  9,  2,  1, 11,  7,
                15,  5, 12, 11,  9,  3,  7, 14,  3, 10, 10,  0,  5,  6,  0, 13
            ],
            [
                15,  3,  1, 13,  8,  4, 14,  7,  6, 15, 11,  2,  3,  8,  4, 14,
                 9, 12,  7,  0,  2,  1, 13, 10, 12,  6,  0,  9,  5, 11, 10,  5,
                 0, 13, 14,  8,  7, 10, 11,  1, 10,  3,  4, 15, 13,  4,  1,  2,
                 5, 11,  8,  6, 12,  7,  6, 12,  9,  0,  3,  5,  2, 14, 15,  9
            ],
            [
                10, 13,  0,  7,  9,  0, 14,  9,  6,  3,  3,  4, 15,  6,  5, 10,
                 1,  2, 13,  8, 12,  5,  7, 14, 11, 12,  4, 11,  2, 15,  8,  1,
                13,  1,  6, 10,  4, 13,  9,  0,  8,  6, 15,  9,  3,  8,  0,  7,
                11,  4,  1, 15,  2, 14, 12,  3,  5, 11, 10,  5, 14,  2,  7, 12
            ],
            [
                 7, 13, 13,  8, 14, 11,  3,  5,  0,  6,  6, 15,  9,  0, 10,  3,
                 1,  4,  2,  7,  8,  2,  5, 12, 11,  1, 12, 10,  4, 14, 15,  9,
                10,  3,  6, 15,  9,  0,  0,  6, 12, 10, 11,  1,  7, 13, 13,  8,
                15,  9,  1,  4,  3,  5, 14, 11,  5, 12,  2,  7,  8,  2,  4, 14
            ],
            [
                 2, 14, 12, 11,  4,  2,  1, 12,  7,  4, 10,  7, 11, 13,  6,  1,
                 8,  5,  5,  0,  3, 15, 15, 10, 13,  3,  0,  9, 14,  8,  9,  6,
                 4, 11,  2,  8,  1, 12, 11,  7, 10,  1, 13, 14,  7,  2,  8, 13,
                15,  6,  9, 15, 12,  0,  5,  9,  6, 10,  3,  4,  0,  5, 14,  3
            ],
            [
                12, 10,  1, 15, 10,  4, 15,  2,  9,  7,  2, 12,  6,  9,  8,  5,
                 0,  6, 13,  1,  3, 13,  4, 14, 14,  0,  7, 11,  5,  3, 11,  8,
                 9,  4, 14,  3, 15,  2,  5, 12,  2,  9,  8,  5, 12, 15,  3, 10,
                 7, 11,  0, 14,  4,  1, 10,  7,  1,  6, 13,  0, 11,  8,  6, 13
            ],
            [
                 4, 13, 11,  0,  2, 11, 14,  7, 15,  4,  0,  9,  8,  1, 13, 10,
                 3, 14, 12,  3,  9,  5,  7, 12,  5,  2, 10, 15,  6,  8,  1,  6,
                 1,  6,  4, 11, 11, 13, 13,  8, 12,  1,  3,  4,  7, 10, 14,  7,
                10,  9, 15,  5,  6,  0,  8, 15,  0, 14,  5,  2,  9,  3,  2, 12
            ],
            [
                13,  1,  2, 15,  8, 13,  4,  8,  6, 10, 15,  3, 11,  7,  1,  4,
                10, 12,  9,  5,  3,  6, 14, 11,  5,  0,  0, 14, 12,  9,  7,  2,
                 7,  2, 11,  1,  4, 14,  1,  7,  9,  4, 12, 10, 14,  8,  2, 13,
                 0, 15,  6, 12, 10,  9, 13,  0, 15,  3,  3,  5,  5,  6,  8, 11
            ]
        ];
        
        let mut result = [0u8; 8];
        
        // Unpack block into two 32-bit integers
        let b = block.as_ref();
        let mut block_ints = [
            ((b[0] as u32) << 24) | ((b[1] as u32) << 16) | ((b[2] as u32) << 8) | (b[3] as u32),
            ((b[4] as u32) << 24) | ((b[5] as u32) << 16) | ((b[6] as u32) << 8) | (b[7] as u32)
        ];
        
        // Extract MSBs
        let msb = [
            (block_ints[0] >> 31) & 1,
            (block_ints[1] >> 31) & 1
        ];
        
        // Mask out MSBs
        block_ints[0] &= 0x7FFFFFFF;
        block_ints[1] &= 0x7FFFFFFF;
        
        // Initial permutation and bit manipulation
        let mut block_permuted = [
            (((block_ints[1] & 0x00000040) << 25) | ((block_ints[1] & 0x00004000) << 16) |
            ((block_ints[1] & 0x00400001

}} // Añadido por reparador automático