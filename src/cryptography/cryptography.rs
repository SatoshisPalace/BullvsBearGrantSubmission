// Reference: https://github.com/public-awesome/badges/blob/master/crates/hub/src/helpers.rs

use super::error::CryptographyError;
use cosmwasm_std::Api;
use sha2::{Digest, Sha256};

/// The hash function to be used to sign a message before signing it. Here we use SHA256.
/// https://docs.rs/sha2/latest/sha2/#usage
/// * `message` - Message to hash.
fn hash(message: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    hasher.finalize().to_vec()
}

/// This is basically a wrapper of `api.secp256k1_verify`, but instead of taking raw bytes in the
/// form of `&[u8]`, it takes the pubkey and signature as hex-encoded strings, and the original
/// message before hashing.
/// * `api` - CosmWasm API from dependencies.
/// * `public_key` - Public key of the signer.
/// * `message` - Message to be verified
/// * `signature` - Signature to be verified
pub fn is_valid_signature(
    api: &dyn Api,
    public_key: &str,
    message: &str,
    signature: &str,
) -> Result<(), CryptographyError> {
    let msg_hash_bytes = hash(message);
    let key_bytes = hex::decode(public_key)?;
    let sig_bytes = hex::decode(signature)?;

    if api.secp256k1_verify(&msg_hash_bytes, &sig_bytes, &key_bytes)? {
        Ok(())
    } else {
        Err(CryptographyError::InvalidSignature)
    }
}
