// Módulos generados automáticamente

pub mod DefaultKey;

// Contenido fusionado desde src/apps/files_encryption/3rdparty/Crypt_Blowfish/Blowfish.rs
//! Crypt_Blowfish allows for encryption and decryption on the fly using
//! the Blowfish algorithm without requiring external dependencies.
//! It supports encryption/decryption with a secret key.

use std::error::Error;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlowfishError {
    #[error("Plain text must be a string")]
    InvalidPlainText,
    #[error("Cipher text must be a string")]
    InvalidCipherText,
    #[error("Key must be a string")]
    InvalidKeyType,
    #[error("Key must be less than 56 characters and non-zero. Supplied key length: {0}")]
    InvalidKeyLength(usize),
}

pub struct CryptBlowfish {
    /// P-Array contains 18 32-bit subkeys
    p: Vec<u32>,
    
    /// Array of four S-Blocks each containing 256 32-bit entries
    s: Vec<Vec<u32>>,
    
    /// Optional mcrypt integration (not implemented in Rust version)
    #[allow(dead_code)]
    mcrypt_available: bool,
}

pub struct DefaultKey {
    pub p: Vec<u32>,
    pub s: Vec<Vec<u32>>,
}

impl DefaultKey {
    pub fn new() -> Self {
        // A simplified version - in a real implementation you'd include
        // all the default values from the original PHP DefaultKey class
        let p = vec![
            // Add the 18 P-array values here
            0x243f6a88, 0x85a308d3, 0x13198a2e, 0x03707344,
            0xa4093822, 0x299f31d0, 0x082efa98, 0xec4e6c89,
            0x452821e6, 0x38d01377, 0xbe5466cf, 0x34e90c6c,
            0xc0ac29b7, 0xc97c50dd, 0x3f84d5b5, 0xb5470917,
            0x9216d5d9, 0x8979fb1b
        ];

        // Initialize S-boxes (this is simplified, real implementation would include all values)
        let mut s = vec![Vec::with_capacity(256); 4];
        
        // Populate S-boxes here with the correct values
        // This is a placeholder - you would need the full set of values from DefaultKey.php
        for i in 0..4 {
            s[i] = vec![0; 256];
        }
        
        // Here you would fill in all the S-box values
        
        Self { p, s }
    }
}

impl CryptBlowfish {
    /// Creates a new CryptBlowfish instance with the provided key
    pub fn new(key: &str) -> Result<Self, BlowfishError> {
        let mut bf = CryptBlowfish {
            p: Vec::new(),
            s: Vec::new(),
            mcrypt_available: false,
        };
        
        bf.init();
        bf.set_key(key)?;
        
        Ok(bf)
    }
    
    /// Initializes the CryptBlowfish object with default values
    fn init(&mut self) {
        let defaults = DefaultKey::new();
        self.p = defaults.p;
        self.s = defaults.s;
    }
    
    /// Enciphers a single 64 bit block
    fn encipher(&self, xl: &mut u32, xr: &mut u32) {
        let mut temp;
        
        for i in 0..16 {
            temp = *xl ^ self.p[i];
            *xl = ((self.s[0][((temp >> 24) & 0xff) as usize] +
                   self.s[1][((temp >> 16) & 0xff) as usize]) ^
                   self.s[2][((temp >> 8) & 0xff) as usize]) +
                   self.s[3][(temp & 0xff) as usize] ^ *xr;
            *xr = temp;
        }
        
        temp = *xl;
        *xl = *xr ^ self.p[16];
        *xr = temp ^ self.p[17];
    }
    
    /// Deciphers a single 64 bit block
    fn decipher(&self, xl: &mut u32, xr: &mut u32) {
        let mut temp;
        
        for i in (2..=17).rev() {
            temp = *xl ^ self.p[i];
            *xl = ((self.s[0][((temp >> 24) & 0xff) as usize] +
                   self.s[1][((temp >> 16) & 0xff) as usize]) ^
                   self.s[2][((temp >> 8) & 0xff) as usize]) +
                   self.s[3][(temp & 0xff) as usize] ^ *xr;
            *xr = temp;
        }
        
        temp = *xl;
        *xl = *xr ^ self.p[0];
        *xr = temp ^ self.p[1];
    }
    
    /// Encrypts a string using Blowfish algorithm
    pub fn encrypt(&self, plain_text: &[u8]) -> Result<Vec<u8>, BlowfishError> {
        if plain_text.is_empty() {
            return Err(BlowfishError::InvalidPlainText);
        }
        
        let mut cipher_text = Vec::new();
        let len = plain_text.len();
        let padding = (8 - (len % 8)) % 8;
        
        // Create padded version of plain text
        let mut padded_text = plain_text.to_vec();
        padded_text.extend(vec![0; padding]);
        
        for i in (0..padded_text.len()).step_by(8) {
            let mut xl = u32::from_be_bytes([
                padded_text[i], 
                padded_text[i+1], 
                padded_text[i+2], 
                padded_text[i+3]
            ]);
            
            let mut xr = u32::from_be_bytes([
                padded_text[i+4], 
                padded_text[i+5], 
                padded_text[i+6], 
                padded_text[i+7]
            ]);
            
            self.encipher(&mut xl, &mut xr);
            
            cipher_text.extend_from_slice(&xl.to_be_bytes());
            cipher_text.extend_from_slice(&xr.to_be_bytes());
        }
        
        Ok(cipher_text)
    }
    
    /// Decrypts an encrypted string using Blowfish algorithm
    pub fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>, BlowfishError> {
        if cipher_text.is_empty() {
            return Err(BlowfishError::InvalidCipherText);
        }
        
        let mut plain_text = Vec::new();
        let len = cipher_text.len();
        let padding = (8 - (len % 8)) % 8;
        
        // Create padded version of cipher text
        let mut padded_text = cipher_text.to_vec();
        padded_text.extend(vec![0; padding]);
        
        for i in (0..len).step_by(8) {
            let mut xl = u32::from_be_bytes([
                padded_text[i], 
                padded_text[i+1], 
                padded_text[i+2], 
                padded_text[i+3]
            ]);
            
            let mut xr = u32::from_be_bytes([
                padded_text[i+4], 
                padded_text[i+5], 
                padded_text[i+6], 
                padded_text[i+7]
            ]);
            
            self.decipher(&mut xl, &mut xr);
            
            plain_text.extend_from_slice(&xl.to_be_bytes());
            plain_text.extend_from_slice(&xr.to_be_bytes());
        }
        
        Ok(plain_text)
    }
    
    /// Sets the secret key for encryption/decryption
    /// The key must be non-zero, and less than or equal to 56 characters in length.
    pub fn set_key(&mut self, key: &str) -> Result<(), BlowfishError> {
        let len = key.len();
        
        if len > 56 || len == 0 {
            return Err(BlowfishError::InvalidKeyLength(len));
        }
        
        self.init();
        
        let key_bytes = key.as_bytes();
        let mut k = 0;
        let mut data;
        let mut datal = 0u32;
        let mut datar = 0u32;
        
        // XOR P-array with the key
        for i in 0..18 {
            data = 0u32;
            for _ in 0..4 {
                data = (data << 8) | (key_bytes[k] as u32);
                k = (k + 1) % len;
            }
            self.p[i] ^= data;
        }
        
        // Update P-array
        for i in (0..=16).step_by(2) {
            self.encipher(&mut datal, &mut datar);
            self.p[i] = datal;
            self.p[i+1] = datar;
        }
        
        // Update S-boxes
        for box_idx in 0..4 {
            for i in (0..256).step_by(2) {
                self.encipher(&mut datal, &mut datar);
                self.s[box_idx][i] = datal;
                self.s[box_idx][i+1] = datar;
            }
        }
        
        Ok(())
    }
}