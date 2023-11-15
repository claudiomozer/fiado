use chrono::NaiveDate;
use serde::Deserialize;

use crate::domain::{
    entities::User,
    error::Error
};

pub trait UserUseCase {
    fn create(&self, dto: UserRequestDTO) -> Result<(), Error>;
}

#[derive(Deserialize, Clone)]
pub struct UserRequestDTO {
    pub name: String,
    pub document: String,
    pub birth_date: NaiveDate,
    pub password: String,
}

impl UserRequestDTO {
    pub fn to_user(self) -> User{
        User::new(self.name, self.document, self.birth_date)
    }
} 