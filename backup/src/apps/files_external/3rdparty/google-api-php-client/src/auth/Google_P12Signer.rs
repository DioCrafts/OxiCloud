/*
 * Copyright 2011 Google Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use openssl::pkcs12::Pkcs12;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use openssl::hash::MessageDigest;
use std::error::Error;
use std::fmt;

/// Custom error type for Google authentication
#[derive(Debug)]
pub struct GoogleAuthError {
    message: String,
}

impl GoogleAuthError {
    fn new<T: Into<String>>(message: T) -> Self {
        GoogleAuthError {
            message: message.into(),
        }
    }
}

impl fmt::Display for GoogleAuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GoogleAuthError: {}", self.message)
    }
}

impl Error for GoogleAuthError {}

/// Signer trait for signing data
pub trait GoogleSigner {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
}

/// Signs data using a P12 certificate.
///
/// Only used for testing.
pub struct GoogleP12Signer {
    private_key: PKey<openssl::pkey::Private>,
}

impl GoogleP12Signer {
    /// Creates a new signer from a .p12 file.
    pub fn new(p12_data: &[u8], password: &str) -> Result<Self, Box<dyn Error>> {
        // Parse the PKCS#12 file
        let pkcs12 = Pkcs12::from_der(p12_data)
            .map_err(|e| GoogleAuthError::new(format!("Unable to parse the p12 file: {}", e)))?;
            
        let parsed = pkcs12.parse(password)
            .map_err(|e| GoogleAuthError::new(format!(
                "Unable to parse the p12 file. Is this a .p12 file? Is the password correct? OpenSSL error: {}", e
            )))?;

        // Get the private key
        let private_key = parsed.pkey;
        
        if private_key.private_key_to_der().is_err() {
            return Err(Box::new(GoogleAuthError::new("No private key found in p12 file.")));
        }

        Ok(GoogleP12Signer {
            private_key,
        })
    }
}

impl GoogleSigner for GoogleP12Signer {
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.private_key)
            .map_err(|e| GoogleAuthError::new(format!("Unable to create signer: {}", e)))?;
            
        signer.update(data)
            .map_err(|e| GoogleAuthError::new(format!("Error updating data to sign: {}", e)))?;
            
        let signature = signer.sign_to_vec()
            .map_err(|e| GoogleAuthError::new(format!("Unable to sign data: {}", e)))?;
            
        Ok(signature)
    }
}

// No need for explicit destructor in Rust as resources are automatically freed when the value is dropped