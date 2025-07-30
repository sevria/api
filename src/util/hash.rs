use argon2::{
    Argon2,
    password_hash::{
        Error as HashError, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        rand_core::OsRng,
    },
};

use crate::util::error::{self, Error};

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(err) => {
            log::error!("failed to hash password: {}", err);
            Err(error::internal())
        }
    }
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, Error> {
    let argon2 = Argon2::default();

    let parsed_hash = match PasswordHash::new(hash) {
        Ok(hash) => hash,
        Err(err) => {
            log::error!("failed to parse hash: {}", err);
            return Err(error::internal());
        }
    };

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(HashError::Password) => Ok(false),
        Err(err) => {
            log::error!("failed to verify password: {}", err);
            Err(error::internal())
        }
    }
}
