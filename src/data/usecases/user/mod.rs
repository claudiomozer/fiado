mod protocols;

use crate::domain::{
    error::Error,
    usecases::user::{self, UserUseCase, UserRequestDTO}
};
use protocols::{
    repository::Repository,
    uuid::Uuid,
    hash::Hash,
};

pub struct UseCase {
    repository: Box<dyn Repository>,
    uuid_generator: Box<dyn Uuid>,
    hash: Box<dyn Hash>
}

impl UseCase {
    pub fn new(repository: Box<dyn Repository>, uuid_generator: Box<dyn Uuid>, hash: Box<dyn Hash>) -> UseCase {
        UseCase { repository, uuid_generator, hash}
    }
}

impl UserUseCase for UseCase {
    fn create(&self, dto: UserRequestDTO) -> Result<(), Error>{
        let password = dto.password.clone();
        let mut user = match dto.to_user() {
            Ok(u) => u,
            Err(e) => return Err(e)
        };

        if !user.get_document().is_valid() {
            return Err(Error::new_business(user::INVALID_DOCUMENT_ERROR))
        }

        user.set_uuid(self.uuid_generator.generate());
        match self.hash.run(password) {
            Ok(hashed_password) => user.set_password(hashed_password),
            Err(message) => return Err(Error::new_internal(message))
        }

        return self.repository.create(user);
    }
}

mod tests;