/// methods for generating BIP39 mnemonics
use crate::error::ProxyError;
use bip39::Mnemonic;

pub fn generate_mnemonic_from_entropy(entropy: &[u8]) -> Result<String, ProxyError> {
    // Create a BIP39 mnemonic from the provided entropy
    let mnemonic = Mnemonic::from_entropy(entropy)?;
    let mnemonic = mnemonic.to_string();
    if mnemonic.is_empty() {
        return Err(ProxyError::Bip39Failure(
            bip39::Error::BadWordCount(0)),
        );
    }
    Ok(mnemonic)
}

// given 256 bits of entropy, generate a BIP39 mnemonic phrase
// or maybe given a CSPRNG object?

