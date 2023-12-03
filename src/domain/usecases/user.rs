use chrono::NaiveDate;
use serde::Deserialize;
use async_trait::async_trait;

use crate::domain::{
    entities::User,
    error::Error, types::{cpf::CPF, birth_date::BirthDate}
};

pub const INVALID_DOCUMENT_ERROR: u8 = 1;
pub const UNDERAGE_ERROR: u8 = 2;
pub const USER_ALREADY_EXISTS: u8 = 3;

#[async_trait]
pub trait UserUseCase {
    async fn create(&self, dto: UserRequestDTO) -> Result<(), Error>;
}
    
#[derive(Deserialize, Clone)]
pub struct UserRequestDTO {
    pub name: String,
    pub document: String,
    pub birth_date: NaiveDate,
    pub password: String,
}

impl UserRequestDTO {
    pub fn to_user(self) -> Result<User, Error>{
        if let Ok(document) = CPF::from_string(self.document) {
            return Ok(User::new(self.name, document, BirthDate::from_naive(self.birth_date)));
        }
        return Err(Error::new_business(INVALID_DOCUMENT_ERROR));
    }
}