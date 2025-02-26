//! # Encryption Crypt Module
//! 
//! This module provides common cryptography functionality for the Encryption app.
//! 
//! Originally ported from PHP to Rust.
//! 
//! Original authors: Sam Tuke, Frank Karlitschek, Robin Appelman
//! Original copyright: 2012 Sam Tuke samtuke@owncloud.com,
//! Robin Appelman icewind@owncloud.com, Frank Karlitschek frank@owncloud.org

use openssl::{
    encrypt::{Encrypter, Decrypter},
    pkey::{PKey, Private, Public},
    rand::rand_bytes,
    symm::{Cipher, encrypt, decrypt},
    rsa::Padding,
    error::ErrorStack,
};
use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use thiserror::Error;
use log::{error, warn};
use crate::helper::Helper;
use crate::util::Util;

/// Error types for encryption operations
#[derive(Error, Debug)]
pub enum CryptError {
    #[error("Encryption is not initialized")]
    NotInitialized,
    
    #[error("Private key is not valid")]
    PrivateKeyNotValid,
    
    #[error("No share key found")]
    NoShareKeyFound,
    
    #[error("OpenSSL error: {0}")]
    OpenSSL(#[from] ErrorStack),
    
    #[error("Encryption error: {0}")]
    Other(String),
    
    #[error("Unknown encryption error")]
    Unknown,
}

/// Legacy Blowfish crypto handler (external dependency)
struct CryptBlowfish {
    key: String,
}

impl CryptBlowfish {
    fn new(key: &str) -> Self {
        Self { key: key.to_string() }
    }

    fn decrypt(&self, content: &[u8]) -> Vec<u8> {
        // This would be a wrapper around the actual Blowfish implementation
        // Since the original used an external PHP library, we'd need to find
        // a Rust equivalent or implement it
        unimplemented!("Blowfish decryption not implemented")
    }
}

/// Main struct for cryptography operations
pub struct Crypt;

impl Crypt {
    /// Return encryption mode: client or server side encryption
    pub fn mode(_user: Option<&str>) -> String {
        // In the original, this always returned 'server'
        "server".to_string()
    }

    /// Create a new encryption keypair
    pub fn create_keypair() -> Result<HashMap<String, String>, CryptError> {
        let config = Helper::get_openssl_config();
        
        match Helper::get_openssl_pkey() {
            Ok(pkey) => {
                let private_key = pkey.private_key_to_pem_pkcs8()?;
                let details = pkey.public_key_to_pem()?;
                
                let keypair = HashMap::from([
                    ("publicKey".to_string(), String::from_utf8_lossy(&details).to_string()),
                    ("privateKey".to_string(), String::from_utf8_lossy(&private_key).to_string()),
                ]);
                
                Ok(keypair)
            },
            Err(e) => {
                error!("Couldn't generate users key-pair: {}", e);
                Err(CryptError::Other("Couldn't generate keypair".to_string()))
            }
        }
    }

    /// Add arbitrary padding to encrypted data
    fn add_padding(data: &[u8]) -> Vec<u8> {
        let mut padded = data.to_vec();
        padded.extend_from_slice(b"xx");
        padded
    }

    /// Remove arbitrary padding from encrypted data
    fn remove_padding(padded: &[u8]) -> Option<Vec<u8>> {
        if padded.len() >= 2 && &padded[padded.len() - 2..] == b"xx" {
            Some(padded[0..padded.len() - 2].to_vec())
        } else {
            // TODO: Log that unpadded data was submitted for removal of padding
            None
        }
    }

    /// Check if a file's contents contains an IV and is symmetrically encrypted
    pub fn is_catfile_content(content: &[u8]) -> bool {
        if content.is_empty() {
            return false;
        }

        match Self::remove_padding(content) {
            Some(no_padding) => {
                if no_padding.len() < 22 {
                    return false;
                }
                
                // Fetch encryption metadata from end of file
                let meta = &no_padding[no_padding.len() - 22..];
                
                // Fetch identifier from start of metadata
                let identifier = &meta[0..6];
                
                identifier == b"00iv00"
            },
            None => false,
        }
    }

    /// Check if a file is encrypted according to database file cache
    pub fn is_encrypted_meta(path: &str) -> bool {
        // Note: This would need integration with the actual filesystem API
        // which is not shown here. The implementation would depend on how
        // the Rust version accesses file metadata.
        unimplemented!("is_encrypted_meta not implemented")
    }

    /// Check if a file is encrypted via legacy system
    pub fn is_legacy_encrypted_content(is_cat_file_content: bool, rel_path: &str) -> bool {
        // Note: This would need integration with the actual filesystem API
        // The implementation would depend on how the Rust version accesses
        // file metadata.
        unimplemented!("is_legacy_encrypted_content not implemented")
    }

    /// Symmetrically encrypt a string
    fn encrypt(plain_content: &[u8], iv: &[u8], passphrase: &str) -> Result<Vec<u8>, CryptError> {
        let cipher = Cipher::aes_128_cfb();
        let encrypted = encrypt(
            cipher,
            passphrase.as_bytes(),
            Some(iv),
            plain_content
        )?;
        
        Ok(encrypted)
    }

    /// Symmetrically decrypt a string
    fn decrypt(encrypted_content: &[u8], iv: &[u8], passphrase: &str) -> Result<Vec<u8>, CryptError> {
        let cipher = Cipher::aes_128_cfb();
        let decrypted = decrypt(
            cipher,
            passphrase.as_bytes(),
            Some(iv),
            encrypted_content
        )?;
        
        Ok(decrypted)
    }

    /// Concatenate encrypted data with its IV and padding
    fn concat_iv(content: &[u8], iv: &[u8]) -> Vec<u8> {
        let mut combined = content.to_vec();
        combined.extend_from_slice(b"00iv00");
        combined.extend_from_slice(iv);
        combined
    }

    /// Split concatenated data and IV into respective parts
    fn split_iv(cat_file: &[u8]) -> HashMap<String, Vec<u8>> {
        let meta_start = cat_file.len() - 22;
        
        // Fetch IV from end of file
        let iv = cat_file[cat_file.len() - 16..].to_vec();
        
        // Remove IV and IV identifier text to expose encrypted content
        let encrypted = cat_file[0..meta_start].to_vec();
        
        HashMap::from([
            ("encrypted".to_string(), encrypted),
            ("iv".to_string(), iv),
        ])
    }

    /// Symmetrically encrypts a string and returns keyfile content
    pub fn symmetric_encrypt_file_content(plain_content: &[u8], passphrase: &str) -> Result<Vec<u8>, CryptError> {
        if plain_content.is_empty() {
            error!("Symmetric encryption failed, no content given.");
            return Err(CryptError::Other("No content given".to_string()));
        }

        let iv = Self::generate_iv()?;
        let iv_decoded = BASE64.decode(&iv)?;
        
        let encrypted_content = Self::encrypt(plain_content, &iv_decoded, passphrase)?;
        
        // Combine content to encrypt with IV identifier and actual IV
        let cat_file = Self::concat_iv(&encrypted_content, &iv_decoded);
        let padded = Self::add_padding(&cat_file);
        
        Ok(padded)
    }

    /// Symmetrically decrypts keyfile content
    pub fn symmetric_decrypt_file_content(keyfile_content: &[u8], passphrase: &str) -> Result<Vec<u8>, CryptError> {
        if keyfile_content.is_empty() {
            return Err(CryptError::Other("No data provided for decryption".to_string()));
        }

        // Remove padding
        let no_padding = Self::remove_padding(keyfile_content)
            .ok_or_else(|| CryptError::Other("Invalid padding".to_string()))?;

        // Split into enc data and catfile
        let cat_file = Self::split_iv(&no_padding);
        
        let encrypted = cat_file.get("encrypted")
            .ok_or_else(|| CryptError::Other("Missing encrypted data".to_string()))?;
        let iv = cat_file.get("iv")
            .ok_or_else(|| CryptError::Other("Missing IV".to_string()))?;
        
        let plain_content = Self::decrypt(encrypted, iv, passphrase)?;
        
        Ok(plain_content)
    }

    /// Decrypt private key and check if the result is a valid keyfile
    pub fn decrypt_private_key(encrypted_key: &[u8], passphrase: &str) -> Result<Vec<u8>, CryptError> {
        let plain_key = Self::symmetric_decrypt_file_content(encrypted_key, passphrase)?;
        
        // Check if this is a valid private key
        match PKey::private_key_from_pem(&plain_key) {
            Ok(_) => Ok(plain_key),
            Err(_) => Err(CryptError::PrivateKeyNotValid),
        }
    }

    /// Create asymmetrically encrypted keyfile content using a generated key
    pub fn multi_key_encrypt(plain_content: &[u8], public_keys: HashMap<String, Vec<u8>>) 
        -> Result<HashMap<String, Vec<u8>>, CryptError> {
        
        if plain_content.is_empty() {
            return Err(CryptError::Other("Cannot multiKeyEncrypt empty plain content".to_string()));
        }
        
        let mut mapped_share_keys = HashMap::new();
        let mut sealed_data = Vec::new();
        
        // Generating a random symmetric key for envelope encryption
        let mut sym_key = vec![0; 32];
        rand_bytes(&mut sym_key)?;
        
        // Encrypting content with the symmetric key
        let cipher = Cipher::aes_256_cbc();
        let mut iv = vec![0; 16];
        rand_bytes(&mut iv)?;
        let encrypted_content = encrypt(cipher, &sym_key, Some(&iv), plain_content)?;
        
        // Combining IV and encrypted content
        sealed_data.extend_from_slice(&iv);
        sealed_data.extend_from_slice(&encrypted_content);
        
        // Encrypt the symmetric key with each public key
        for (user_id, public_key_pem) in public_keys {
            let public_key = PKey::public_key_from_pem(&public_key_pem)?;
            let mut encrypter = Encrypter::new(&public_key)?;
            encrypter.set_rsa_padding(Padding::PKCS1)?;
            
            let buffer_len = encrypter.encrypt_len(&sym_key)?;
            let mut encrypted_key = vec![0; buffer_len];
            let encrypted_len = encrypter.encrypt(&sym_key, &mut encrypted_key)?;
            encrypted_key.truncate(encrypted_len);
            
            mapped_share_keys.insert(user_id, encrypted_key);
        }
        
        Ok(HashMap::from([
            ("keys".to_string(), serde_json::to_vec(&mapped_share_keys)
                .map_err(|e| CryptError::Other(format!("JSON serialization error: {}", e)))?),
            ("data".to_string(), sealed_data),
        ]))
    }

    /// Asymmetrically decrypt a file using multiple public keys
    pub fn multi_key_decrypt(encrypted_content: &[u8], share_key: &[u8], private_key_pem: &[u8]) 
        -> Result<Vec<u8>, CryptError> {
        
        if encrypted_content.is_empty() {
            return Err(CryptError::Other("Empty encrypted content".to_string()));
        }
        
        let private_key = PKey::private_key_from_pem(private_key_pem)?;
        
        let mut decrypter = Decrypter::new(&private_key)?;
        decrypter.set_rsa_padding(Padding::PKCS1)?;
        
        let buffer_len = decrypter.decrypt_len(share_key)?;
        let mut decrypted_key = vec![0; buffer_len];
        let decrypted_len = decrypter.decrypt(share_key, &mut decrypted_key)?;
        decrypted_key.truncate(decrypted_len);
        
        // Separate IV from encrypted content
        if encrypted_content.len() <= 16 {
            return Err(CryptError::Other("Invalid encrypted content format".to_string()));
        }
        
        let iv = &encrypted_content[0..16];
        let actual_content = &encrypted_content[16..];
        
        // Decrypt with the symmetric key
        let cipher = Cipher::aes_256_cbc();
        let plain_content = decrypt(cipher, &decrypted_key, Some(iv), actual_content)?;
        
        Ok(plain_content)
    }

    /// Generates a pseudo random initialization vector
    fn generate_iv() -> Result<String, CryptError> {
        let mut random_bytes = vec![0; 12];
        rand_bytes(&mut random_bytes)?;
        
        // We encode the iv purely for string manipulation purposes
        let iv = BASE64.encode(&random_bytes);
        
        Ok(iv)
    }

    /// Generate a pseudo random 1024kb ASCII key, used as file key
    pub fn generate_key() -> Result<String, CryptError> {
        let mut random_bytes = vec![0; 183];
        rand_bytes(&mut random_bytes)?;
        
        let key = BASE64.encode(&random_bytes);
        
        Ok(key)
    }

    /// Get the blowfish encryption handler for a key
    fn get_blowfish(key: &str) -> Option<CryptBlowfish> {
        if !key.is_empty() {
            Some(CryptBlowfish::new(key))
        } else {
            None
        }
    }

    /// Decrypts content using legacy blowfish system
    pub fn legacy_decrypt(content: &[u8], passphrase: &str) -> Vec<u8> {
        if let Some(bf) = Self::get_blowfish(passphrase) {
            bf.decrypt(content)
        } else {
            Vec::new()
        }
    }

    /// Block decrypt for legacy encryption
    pub fn legacy_block_decrypt(data: &[u8], key: &str, max_length: usize) -> Vec<u8> {
        let mut result = Vec::new();
        let mut offset = 0;
        
        while offset < data.len() {
            let chunk_size = std::cmp::min(8192, data.len() - offset);
            let chunk = &data[offset..offset + chunk_size];
            
            let decrypted_chunk = Self::legacy_decrypt(chunk, key);
            result.extend_from_slice(&decrypted_chunk);
            
            offset += chunk_size;
        }
        
        if max_length > 0 && result.len() > max_length {
            result.truncate(max_length);
        } else {
            // Trim trailing null bytes (equivalent to rtrim in PHP)
            while result.last() == Some(&0) {
                result.pop();
            }
        }
        
        result
    }
}