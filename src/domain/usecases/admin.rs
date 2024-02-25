use crate::domain::error::Error;
use async_trait::async_trait;

pub const INVALID_TOKEN_ERROR: u8 = 5;
pub const EXPIRED_TOKEN_ERROR: u8 = 6;
pub const MISSING_AUTH_TOKEN: u8 = 7;

#[async_trait]
pub trait AdminUseCase {
    async fn validate_token(&self, token: String) -> Result<(), Error>;
    async fn generate_token(&self) -> Result<String, Error>;
}