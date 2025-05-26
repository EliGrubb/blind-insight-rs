/// keyring-related functions and structures
use rand::rngs::OsRng;
use crate::bip39::generate_mnemonic_from_entropy;

/// generate a new keyring
pub fn generate_seed_phrase() -> String {
    let mut rng = OsRng;
    
    let entropy = [0u8; 32]; // 256 bits of entropy
    rng.fill_bytes(&mut entropy);
    match generate_mnemonic_from_entropy(&entropy) {
        Ok(mnemonic) => mnemonic.to_string(),
        Err(e) => {
            eprintln!("Error generating mnemonic: {}", e);
            String::new()
        }
    }
}