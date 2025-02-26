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

use openssl::x509::X509;
use openssl::sign::Verifier;
use openssl::hash::MessageDigest;
use std::error::Error;
use std::fmt;

/// Error types for Google authentication
#[derive(Debug)]
pub enum GoogleError {
    AuthError(String),
    OpenSslError(openssl::error::ErrorStack),
    MissingDependency(String),
}

impl fmt::Display for GoogleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GoogleError::AuthError(msg) => write!(f, "Authentication error: {}", msg),
            GoogleError::OpenSslError(err) => write!(f, "OpenSSL error: {}", err),
            GoogleError::MissingDependency(msg) => write!(f, "Missing dependency: {}", msg),
        }
    }
}

impl Error for GoogleError {}

impl From<openssl::error::ErrorStack> for GoogleError {
    fn from(err: openssl::error::ErrorStack) -> Self {
        GoogleError::OpenSslError(err)
    }
}

/// Trait for signature verification
pub trait Verifier {
    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, GoogleError>;
}

/// Verifies signatures using PEM encoded certificates.
pub struct PemVerifier {
    public_key: X509,
}

impl PemVerifier {
    /// Constructs a verifier from the supplied PEM-encoded certificate.
    ///
    /// # Arguments
    /// * `pem` - a PEM encoded certificate (not a file).
    ///
    /// # Returns
    /// A new PemVerifier instance
    ///
    /// # Errors
    /// If the OpenSSL library is not available or the PEM cannot be parsed
    pub fn new(pem: &str) -> Result<Self, GoogleError> {
        let public_key = X509::from_pem(pem.as_bytes())
            .map_err(|_| GoogleError::AuthError(format!("Unable to parse PEM: {}", pem)))?;
        
        Ok(PemVerifier { public_key })
    }
}

impl Verifier for PemVerifier {
    /// Verifies the signature on data.
    ///
    /// # Arguments
    /// * `data` - The data that was signed
    /// * `signature` - The signature to verify
    ///
    /// # Returns
    /// true if the signature is valid, false otherwise.
    ///
    /// # Errors
    /// If there is an error during signature verification
    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, GoogleError> {
        let public_key = self.public_key.public_key()?;
        let mut verifier = Verifier::new(MessageDigest::sha256(), &public_key)?;
        verifier.update(data)?;
        
        match verifier.verify(signature) {
            Ok(result) => Ok(result),
            Err(err) => Err(GoogleError::AuthError(format!("Signature verification error: {}", err))),
        }
    }
}

// No need for explicit destructor as Rust's Drop trait handles resource cleanup automatically