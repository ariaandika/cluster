use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::libs::errors::{Result, Error};

pub fn hash(password: &[u8]) -> Result<String> {
    if cfg!(test)  {
        Ok(String::from_utf8(password.to_vec()).unwrap())
    } else {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2.hash_password(password, &salt).map_err(Error::fatal)?.to_string())
    }
}

pub fn verify(hash: &str, password: &[u8]) -> Result<()> {
    if cfg!(test) {
        Ok(())
    } else {
        let parsed_hash = PasswordHash::new(hash).map_err(Error::fatal)?;
        Argon2::default().verify_password(password, &parsed_hash).map_err(Error::fatal)
    }
}


// #[cfg(not(test))]
// pub fn hash(password: &[u8]) -> Result<String> {
//     let salt = SaltString::generate(&mut OsRng);
//     let argon2 = Argon2::default();
//     Ok(argon2.hash_password(password, &salt).map_err(Error::fatal)?.to_string())
// }
//
// #[cfg(not(test))]
// pub fn verify(hash: &str, password: &[u8]) -> Result<()> {
//     let parsed_hash = PasswordHash::new(hash).map_err(Error::fatal)?;
//     Argon2::default().verify_password(password, &parsed_hash).map_err(Error::fatal)
// }

