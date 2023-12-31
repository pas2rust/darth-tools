use super::darth_tools::DarthTools;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub trait Argon2Trait {
    fn argon2_default_encode(password: &str) -> Result<String, String>;
    fn argon2_default_verify_password(original_password: &str, encoded_hash: &str) -> Result<bool, String>;
}

impl Argon2Trait for DarthTools {
    fn argon2_default_encode(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt);
        match password_hash {
            Ok(hash) => Ok(hash.to_string()),
            Err(err) => Err(err.to_string()),
        }
    }
    fn argon2_default_verify_password(original_password: &str, encoded_hash: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(encoded_hash).map_err(|e| format!("Invalid hash format: {}", e))?;
        let argon2 = Argon2::default();
        argon2
            .verify_password(original_password.as_bytes(), &parsed_hash)
            .map_err(|_| "Password verification failed".to_string())
            .map(|_| true)
    }
}
