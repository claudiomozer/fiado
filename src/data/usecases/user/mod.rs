pub mod protocols;

use async_trait::async_trait;
use crate::domain::{
    error::Error,
    usecases::user::{self, UserUseCase, UserCreateRequestDTO}
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
}

mod tests;