/// keyring-related functions and structures
use rand::{rngs::OsRng, TryRngCore};
use crate::{
    bip39::generate_mnemonic_from_entropy,
    error::ProxyError,
};
use keyring::Entry;

const KEYRING_SERVICE: &str = "blind-proxy-rs";
const KEYRING_USERNAME: &str = "proxy-user";

/// generate a new keyring
pub fn generate_seed_phrase() -> Result<String, ProxyError> {
    let mut rng = OsRng;
    
    let mut entropy = [0u8; 32]; // 256 bits of entropy
    rng.try_fill_bytes(&mut entropy)?;

    // Generate a BIP39 mnemonic from the entropy
    generate_mnemonic_from_entropy(&entropy)
}

pub fn create_keyring() -> Result<(), ProxyError> {
    let seed_phrase = generate_seed_phrase()?;
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)?;
    entry.set_password(&seed_phrase)?;
    
    Ok(())
}

pub fn inspect_keyring() -> Result<(), ProxyError> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)?;
    match entry.get_password() {
        Ok(password) => println!("Your seed phrase: {}", password),
        Err(_) => println!("No seed phrase set. Call keyring create to create one."),
    }
    
    Ok(())
}