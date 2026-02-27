use anyhow::{Context, Result};
use argon2::{Argon2, Params, Version, Algorithm};
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
    XChaCha20Poly1305, XNonce,
};
use std::path::PathBuf;
use zeroize::Zeroize;

pub fn config_dir() -> Result<PathBuf> {
    let base = dirs::config_dir().context("Cannot find config directory")?;
    let path = base.join("hazeveil");
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn derive_key(password: &[u8], salt: &[u8; 16]) -> Result<Vec<u8>> {
    let params = Params::new(65536, 3, 4, Some(32))
        .map_err(|e| anyhow::anyhow!("Argon2 params error: {}", e))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = vec![0u8; 32];
    argon2.hash_password_into(password, salt, &mut key)
        .map_err(|e| anyhow::anyhow!("Argon2 hash error: {}", e))?;
    Ok(key)
}

pub fn encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = XChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| anyhow::anyhow!("Cipher init error: {}", e))?;
    let mut nonce_bytes = [0u8; 24];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = XNonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, plaintext)
        .map_err(|e| anyhow::anyhow!("Encryption error: {}", e))?;
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

pub fn decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < 24 {
        anyhow::bail!("Data too short");
    }
    let cipher = XChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| anyhow::anyhow!("Cipher init error: {}", e))?;
    let nonce = XNonce::from_slice(&data[..24]);
    let plaintext = cipher.decrypt(nonce, &data[24..])
        .map_err(|e| anyhow::anyhow!("Decryption error: {}", e))?;
    Ok(plaintext)
}

pub fn secure_zero(data: &mut Vec<u8>) {
    data.zeroize();
}

pub struct PerfTimer {
    start: std::time::Instant,
}

impl PerfTimer {
    pub fn new() -> Self { Self { start: std::time::Instant::now() } }
    pub fn elapsed_us(&self) -> u128 { self.start.elapsed().as_micros() }
}