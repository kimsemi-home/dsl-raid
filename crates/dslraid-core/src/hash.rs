use anyhow::Result;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::canonical_json_bytes;

pub fn sha256_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("sha256:{}", hex::encode(digest))
}

pub fn sha256_json<T: Serialize>(value: &T) -> Result<String> {
    Ok(sha256_bytes(&canonical_json_bytes(value)?))
}
