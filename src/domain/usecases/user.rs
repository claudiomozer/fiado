use chrono::{NaiveDate, Utc, DateTime};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::domain::{
    entities::{User, UserStatus},
    error::Error, types::{cpf::CPF, birth_date::BirthDate, }
};

pub const INVALID_DOCUMENT_ERROR: u8 = 1;
pub const UNDERAGE_ERROR: u8 = 2;
pub const USER_ALREADY_EXISTS: u8 = 3;
pub const USER_NOT_FOUND: u8 = 4;

#[async_trait]
pub trait UserUseCase {
    async fn create(&self, dto: UserCreateRequestDTO) -> Result<(), Error>;
    async fn update(&self, dto: UserUpdateRequestDTO) -> Result<(), Error>;
    async fn get(&self, document: &str) -> Result<PublicUserResponseDTO, Error>;
    async fn delete(&self, document: &str) -> Result<(), Error>;
}
    
#[derive(Deserialize, Clone)]
pub struct UserCreateRequestDTO {
    pub name: String,
    pub document: String,
    pub birth_date: NaiveDate,
    pub password: String,
}

impl UserCreateRequestDTO {
    pub fn to_user(self) -> Result<User, Error>{
        if let Ok(document) = CPF::from_string(self.document) {
            return Ok(User::new(self.name, document, BirthDate::from_naive(self.birth_date)));
        }
        Err(Error::new_business(INVALID_DOCUMENT_ERROR))
    }
}

#[derive(Deserialize, Clone)]
pub struct UserUpdateRequestDTO {
    pub id: String,
    pub name: String,
    pub document: String,
    pub birth_date: NaiveDate,
}
impl UserUpdateRequestDTO {
    pub fn to_user(self) -> Result<User, Error>{
        if let Ok(document) = CPF::from_string(self.document) {
            let mut user = User::new(self.name, document, BirthDate::from_naive(self.birth_date));
            user.set_uuid(self.id);
            return Ok(user);
        }
        Err(Error::new_business(INVALID_DOCUMENT_ERROR))
    }
}

#[derive(Serialize)]
pub struct PublicUserResponseDTO {
	pub id: String ,
	pub name: String,
	pub document: CPF,
	pub status: UserStatus,
	pub birth_date: BirthDate,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>
}
impl PublicUserResponseDTO {
    pub fn from_user(user: User) -> Self {
        PublicUserResponseDTO { 
            id: String::from(user.get_id()),
            name: String::from(user.get_name()),
            document: *user.get_document(), 
            status: user.get_status(), 
            birth_date: user.get_birth_date(), 
            created_at: user.get_created_at(),
            updated_at: user.get_updated_at()
        }
    }
}