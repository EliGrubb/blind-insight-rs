/// methods for generating BIP39 mnemonics

use bip39::Mnemonic;

pub fn generate_mnemonic_from_entropy(entropy: &[u8]) -> Result<String, bip39::Error> {
    // Create a BIP39 mnemonic from the provided entropy
    match Mnemonic::from_entropy(entropy) {
        Ok(mnemonic) => Ok(mnemonic.to_string()),
        Err(e) => Err(e),
    }
}

// given 256 bits of entropy, generate a BIP39 mnemonic phrase
// or maybe given a CSPRNG object?

