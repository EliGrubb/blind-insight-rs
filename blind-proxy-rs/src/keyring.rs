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

pub fn create_keyring() -> Result<(), Error> {
    let seed_phrase = generate_seed_phrase();
    if seed_phrase.is_empty() {
        return Err(Error::new(std::io::ErrorKind::Other, "Failed to generate seed phrase"));
    }
    
    // Here you would typically save the seed phrase securely, e.g., in a file or database
    println!("Generated seed phrase: {}", seed_phrase);
    
    Ok(())
}