//! Random Number Generator
//!
//! Here's a short example of how to use this library:
//! ```rust
//! use crypt::random::crypt_random_string;
//! 
//! let random_bytes = crypt_random_string(8).unwrap();
//! println!("{}", hex::encode(&random_bytes));
//!

use lazy_static::lazy_static;
use rand::{rngs::OsRng, RngCore};
use sha1::{Sha1, Digest};
use std::{
    fs::File,
    io::{Read, Error as IoError},
    sync::Mutex,
    env,
    collections::HashMap,
};

#[cfg(feature = "encryption")]
use crate::crypt::{Aes, TripleDes, Des, Rc4, CipherMode, CryptoTrait};

lazy_static! {
    static ref DEV_RANDOM_FP: Mutex<Option<File>> = Mutex::new(None);
    static ref CRYPTO_STATE: Mutex<Option<CryptoState>> = Mutex::new(None);
}

/// Cryptographic state for the PRNG
enum CryptoState {
    #[cfg(feature = "encryption")]
    Cipher(Box<dyn CryptoTrait + Send>),
    Seed(Vec<u8>),
    Vector(Vec<u8>),
}

/// Generate a random string of specified length.
///
/// This function prioritizes OS-provided random sources before falling back
/// to pure-Rust implementations.
///
/// # Arguments
///
/// * `length` - The length of the random string to generate
///
/// # Returns
///
/// A Result containing a Vec<u8> with the random bytes or an error
pub fn crypt_random_string(length: usize) -> Result<Vec<u8>, String> {
    #[cfg(target_os = "windows")]
    {
        // On Windows, we try secure OS methods first
        if cfg!(feature = "openssl") {
            return openssl_random_bytes(length);
        }
    } 
    #[cfg(not(target_os = "windows"))]
    {
        // On non-Windows platforms, try OpenSSL first
        if cfg!(feature = "openssl") {
            return openssl_random_bytes(length);
        }

        // Then try reading from /dev/urandom
        if let Some(bytes) = read_dev_urandom(length) {
            return Ok(bytes);
        }
    }

    // Last resort: pure Rust CSPRNG
    pure_rust_csprng(length)
}

#[cfg(feature = "openssl")]
fn openssl_random_bytes(length: usize) -> Result<Vec<u8>, String> {
    let mut buf = vec![0u8; length];
    openssl::rand::rand_bytes(&mut buf)
        .map_err(|e| format!("OpenSSL error: {}", e))?;
    Ok(buf)
}

fn read_dev_urandom(length: usize) -> Option<Vec<u8>> {
    let mut dev_random_guard = DEV_RANDOM_FP.lock().ok()?;
    
    // Try to open /dev/urandom if we haven't already
    if dev_random_guard.is_none() {
        *dev_random_guard = File::open("/dev/urandom").ok();
    }
    
    // If we have a valid file handle, read from it
    if let Some(ref mut file) = *dev_random_guard {
        let mut buffer = vec![0u8; length];
        match file.read_exact(&mut buffer) {
            Ok(_) => return Some(buffer),
            Err(_) => return None,
        }
    }
    
    None
}

fn pure_rust_csprng(length: usize) -> Result<Vec<u8>, String> {
    let mut crypto_guard = CRYPTO_STATE.lock().map_err(|e| format!("Mutex lock failed: {}", e))?;
    
    if crypto_guard.is_none() {
        // Initialize the crypto state
        let seed = generate_entropy_seed()?;
        
        #[cfg(feature = "encryption")]
        {
            // Generate key and IV for cipher
            let key = sha1_hash(&[&seed, b"A"].concat());
            let iv = sha1_hash(&[&seed, b"C"].concat());
            
            let crypto = initialize_cipher(key, iv)?;
            *crypto_guard = Some(crypto);
        }
        
        #[cfg(not(feature = "encryption"))]
        {
            *crypto_guard = Some(CryptoState::Seed(seed));
        }
    }
    
    match crypto_guard.as_mut().unwrap() {
        #[cfg(feature = "encryption")]
        CryptoState::Cipher(cipher) => {
            // Initialize v as a shared state if not already set
            let mut result = Vec::with_capacity(length);
            
            // ANSI X9.31 inspired algorithm
            while result.len() < length {
                let mut rng = OsRng;
                let mut i = vec![0u8; 20];
                rng.fill_bytes(&mut i);
                
                let v = match cipher.encrypt(&i) {
                    Ok(v) => v,
                    Err(e) => return Err(format!("Encryption error: {}", e)),
                };
                
                let r = match cipher.encrypt(&v) {
                    Ok(r) => r,
                    Err(e) => return Err(format!("Encryption error: {}", e)),
                };
                
                result.extend_from_slice(&r);
            }
            
            Ok(result[..length].to_vec())
        },
        
        #[cfg(not(feature = "encryption"))]
        CryptoState::Seed(seed) => {
            let mut v = sha1_hash(seed);
            let mut result = Vec::with_capacity(length);
            
            while result.len() < length {
                let timestamp = format!("{:?}", std::time::SystemTime::now());
                let i = sha1_hash(timestamp.as_bytes());
                
                let mut r = sha1_hash(&xor_bytes(&i, &v));
                v = sha1_hash(&xor_bytes(&r, &i));
                
                result.extend_from_slice(&r);
            }
            
            Ok(result[..length].to_vec())
        },
        
        CryptoState::Vector(v) => {
            let mut current_v = v.clone();
            let mut result = Vec::with_capacity(length);
            
            while result.len() < length {
                let timestamp = format!("{:?}", std::time::SystemTime::now());
                let i = sha1_hash(timestamp.as_bytes());
                
                let r = sha1_hash(&xor_bytes(&i, &current_v));
                current_v = sha1_hash(&xor_bytes(&r, &i));
                
                result.extend_from_slice(&r);
            }
            
            // Update the stored vector
            *v = current_v;
            
            Ok(result[..length].to_vec())
        }
    }
}

#[cfg(feature = "encryption")]
fn initialize_cipher(key: Vec<u8>, iv: Vec<u8>) -> Result<CryptoState, String> {
    // Try to initialize the strongest available cipher
    if cfg!(feature = "aes") {
        let mut cipher = Aes::new(CipherMode::Ctr);
        cipher.set_key(&key).map_err(|e| format!("AES key setting error: {}", e))?;
        cipher.set_iv(&iv).map_err(|e| format!("AES IV setting error: {}", e))?;
        cipher.enable_continuous_buffer();
        return Ok(CryptoState::Cipher(Box::new(cipher)));
    } else if cfg!(feature = "tripledes") {
        let mut cipher = TripleDes::new(CipherMode::Ctr);
        cipher.set_key(&key).map_err(|e| format!("3DES key setting error: {}", e))?;
        cipher.set_iv(&iv).map_err(|e| format!("3DES IV setting error: {}", e))?;
        cipher.enable_continuous_buffer();
        return Ok(CryptoState::Cipher(Box::new(cipher)));
    } else if cfg!(feature = "des") {
        let mut cipher = Des::new(CipherMode::Ctr);
        cipher.set_key(&key).map_err(|e| format!("DES key setting error: {}", e))?;
        cipher.set_iv(&iv).map_err(|e| format!("DES IV setting error: {}", e))?;
        cipher.enable_continuous_buffer();
        return Ok(CryptoState::Cipher(Box::new(cipher)));
    } else if cfg!(feature = "rc4") {
        let mut cipher = Rc4::new();
        cipher.set_key(&key).map_err(|e| format!("RC4 key setting error: {}", e))?;
        return Ok(CryptoState::Cipher(Box::new(cipher)));
    } 
    
    // If no ciphers are available, fall back to the seed method
    Ok(CryptoState::Seed(key))
}

fn generate_entropy_seed() -> Result<Vec<u8>, String> {
    // Collect system entropy from various sources
    let mut entropy_sources = Vec::new();
    
    // Add environment variables
    let env_vars: HashMap<String, String> = env::vars().collect();
    entropy_sources.push(format!("{:?}", env_vars));
    
    // Add current time with high precision
    entropy_sources.push(format!("{:?}", std::time::SystemTime::now()));
    
    // Add process ID
    entropy_sources.push(format!("{}", std::process::id()));
    
    // Add thread ID
    entropy_sources.push(format!("{:?}", std::thread::current().id()));
    
    // Combine all sources
    let combined = entropy_sources.join("");
    
    Ok(sha1_hash(combined.as_bytes()))
}

fn sha1_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x ^ y)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_string_generation() {
        let length = 16;
        let random_bytes = crypt_random_string(length).unwrap();
        assert_eq!(random_bytes.len(), length);
        
        // Verify that two calls produce different results
        let random_bytes2 = crypt_random_string(length).unwrap();
        assert_ne!(random_bytes, random_bytes2);
    }
}