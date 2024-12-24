use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PasswordDto {
    pub password: String,
    #[serde(skip)]
    pub password_hash: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}
