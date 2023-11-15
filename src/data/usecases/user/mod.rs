mod protocols;

use crate::domain::{
    error::Error,
    usecases::user::UserUseCase,
    usecases::user::UserRequestDTO
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
        let mut user = dto.to_user();
        user.set_uuid(self.uuid_generator.generate());
        
        match self.hash.run(password) {
            Ok(hashed_password) => user.set_password(hashed_password),
            Err(message) => return Err(Error::new_internal(message))
        }

        return self.repository.create(user);
    }
}

mod tests;