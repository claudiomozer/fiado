pub mod protocols;

use async_trait::async_trait;
use crate::domain::{
    error::Error, 
    types::cpf::CPF,
    usecases::user::{self, UserUseCase, UserCreateRequestDTO, UserUpdateRequestDTO, PublicUserResponseDTO}
};
use protocols::{
    repository::Repository,
    hash::Hash,
};
use crate::data::protocols::uuid::Uuid;

pub struct UseCase {
    repository: Box<dyn Repository + Send + Sync>,
    uuid_generator: Box<dyn Uuid + Send + Sync>,
    hash: Box<dyn Hash + Send + Sync>
}

impl UseCase {
    pub fn new(repository: Box<dyn Repository + Send + Sync>, uuid_generator: Box<dyn Uuid + Sync + Send>, hash: Box<dyn Hash + Sync + Send>) -> UseCase {
        UseCase { repository, uuid_generator, hash}
    }
}

#[async_trait]
impl UserUseCase for UseCase {
    async fn create(&self, dto: UserCreateRequestDTO) -> Result<(), Error>{
        let password = dto.password.clone();
        let mut user = match dto.to_user() {
            Ok(u) => u,
            Err(e) => return Err(e)
        };

        if !user.get_document().is_valid() {
            return Err(Error::new_business(user::INVALID_DOCUMENT_ERROR));
        }

        if user.get_birth_date().is_under_age() {
            return Err(Error::new_business(user::UNDERAGE_ERROR));
        }

        user.set_uuid(self.uuid_generator.generate());
        match self.hash.run(password) {
            Ok(hashed_password) => user.set_password(hashed_password),
            Err(message) => return Err(Error::new_internal(&message))
        }

        return self.repository.create(user).await;
    }

    async fn update(&self, dto: UserUpdateRequestDTO) -> Result<(), Error> {
        let user = match dto.to_user() {
            Ok(u) => u,
            Err(e) => return Err(e)
        };

        if !user.get_document().is_valid() {
            return Err(Error::new_business(user::INVALID_DOCUMENT_ERROR))
        }

        if user.get_birth_date().is_under_age() {
            return Err(Error::new_business(user::UNDERAGE_ERROR))
        }

        return self.repository.update(user).await;
    }

    async fn get(&self, document: &str) -> Result<PublicUserResponseDTO, Error> {
        let cpf = match CPF::from_string(String::from(document)) {
            Ok(c) => c,
            Err(_) => return Err(Error::new_business(user::INVALID_DOCUMENT_ERROR))
        };

        if !cpf.is_valid() {
            return Err(Error::new_business(user::INVALID_DOCUMENT_ERROR))
        }

        match self.repository.get_by_cpf(document).await {
            Ok(u) => Ok(PublicUserResponseDTO::from_user(u)),
            Err(e) => Err(e)
        }
    }
}

mod tests;