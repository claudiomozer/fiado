mod protocols;

use crate::domain::{
    error::Error,
    usecases::user::UserUseCase,
    usecases::user::UserRequestDTO
};
use protocols::{
    repository::Repository,
    uuid::Uuid
};

pub struct UseCase {
    repository: Box<dyn Repository>,
    uuid_generator: Box<dyn Uuid> 
}

impl UseCase {
    pub fn new(repository: Box<dyn Repository>, uuid_generator: Box<dyn Uuid>) -> UseCase {
        UseCase { repository, uuid_generator }
    }
}

impl UserUseCase for UseCase {
    fn create(&self, dto: UserRequestDTO) -> Result<(), Error>{
        let mut user = dto.to_user();
        user.set_uuid(self.uuid_generator.generate());

        return self.repository.create(user);
    }
}

mod tests;