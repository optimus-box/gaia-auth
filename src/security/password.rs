use argon2::{self, Config};

pub fn hash(password: &str) -> Result<Vec<u8>, argon2::Error> {
    let config = Config::default();
    let hex = std::env::var("PASSWORD_SALT").expect("PASSWORD_SALT must be set");
    let salt = hex_to_bytes(&hex);
    argon2::hash_raw(password.as_bytes(), &salt, &config)
}

pub fn check(password_hash: &[u8], password: &str) -> Result<bool, argon2::Error> {
    let config = Config::default();
    let hex = std::env::var("PASSWORD_SALT").expect("PASSWORD_SALT must be set");
    let salt = hex_to_bytes(&hex);
    argon2::verify_raw(password.as_bytes(), &salt, password_hash, &config)
}

pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex::decode(hex).expect("failed to decode salt hex")
}
