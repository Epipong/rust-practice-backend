use once_cell::sync::Lazy;

use crate::errors::ServiceError;

pub static SECRET_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(16)));

const SALT: &[u8] = b"supersecuresalt";

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let config = argon2::Config {
        secret: SECRET_KEY.as_bytes(),
        ..argon2::Config::rfc9106_low_mem()
    };
    argon2::hash_encoded(password.as_bytes(), SALT, &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    argon2::verify_encoded_ext(hash, password.as_bytes(), SECRET_KEY.as_bytes(), &[]).map_err(
        |err| {
            dbg!(err);
            ServiceError::Unauthorized
        },
    )
}

#[cfg(test)]
mod tests {
    use std::env;

    use actix_web::cookie::Key;

    use super::SECRET_KEY;

    #[test]
    fn secret_key_default() {
        env::remove_var("SECRET_KEY");

        assert!(Key::try_from(SECRET_KEY.as_bytes()).is_ok());
    }
}