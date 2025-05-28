//! Custom error types
use rand::rand_core::OsError;
use thiserror::Error;
use bip39::Error as Bip39Error;
use keyring::Error as KeyringError;
use std::io::Error as StdError;


#[derive(Debug, Error)]
pub enum ProxyError {
    /// The operation was cancelled.
    #[error("CSPRNG failed: {0}")]
    CSPRNGFailure(#[from] OsError),
    /// The operation was cancelled.
    #[error("BIP39 mnemonic generation failed: {0}")]
    Bip39Failure(#[from] Bip39Error),
    #[error("Keyring operation failed: {0}")]
    KeyringFailure(#[from] KeyringError),
    #[error("Async operation failed: {0}")]
    AsyncFailure(#[from] StdError),
    #[error("{0:?}")]
    LoginFailure(String),
    #[error("Unknown error occurred: {0}")]
    Unknown(String),
}