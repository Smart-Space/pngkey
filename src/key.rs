use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce
};
use argon2::{
    Argon2, password_hash::{PasswordHasher, SaltString, rand_core::RngCore}
};
use base64::{Engine as _, engine::general_purpose};

use crate::Result;

pub fn encrypt(plaintext: &str, password: &str) -> Result<String> {
    // 生成随机salt
    let salt = SaltString::generate(&mut OsRng);
    
    // 使用Argon2id派生密钥
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    let key = password_hash.hash.unwrap();
    
    // 生成随机nonce（ChaCha20-Poly1305使用12字节nonce）
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    // 创建加密器
    let cipher = ChaCha20Poly1305::new_from_slice(key.as_bytes())?;
    
    // 加密
    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())?;
    
    // 组合：salt + nonce + ciphertext
    let salt_b64 = general_purpose::STANDARD.encode(salt.as_str().as_bytes());
    let nonce_b64 = general_purpose::STANDARD.encode(&nonce_bytes);
    let ciphertext_b64 = general_purpose::STANDARD.encode(&ciphertext);
    let combined = format!("{}::{}::{}", salt_b64, nonce_b64, ciphertext_b64);
    
    Ok(combined)
}

pub fn decrypt(encrypted: &str, password: &str) -> Result<String> {
    let parts: Vec<&str> = encrypted.split("::").collect();
    if parts.len() != 3 {
        return Err("Invalid encrypted data format".into());
    }
    if password.is_empty() {
        return Err("Need password to decrypt".into());
    }
    
    let salt_str = String::from_utf8(general_purpose::STANDARD.decode(parts[0])?)?;
    let nonce_bytes = general_purpose::STANDARD.decode(parts[1])?;
    let ciphertext = general_purpose::STANDARD.decode(parts[2])?;
    
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    // 使用Argon2重新派生密钥
    let argon2 = Argon2::default();
    let salt = SaltString::from_b64(&salt_str)?;
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    let key = password_hash.hash.unwrap();
    
    // 解密
    let cipher = ChaCha20Poly1305::new_from_slice(key.as_bytes())?;
    let plaintext_bytes = cipher.decrypt(nonce, ciphertext.as_slice())?;
    
    Ok(String::from_utf8(plaintext_bytes)?)
}
