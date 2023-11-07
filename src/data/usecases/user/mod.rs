mod protocols;

use crate::domain::{
    error::Error,
    usecases::user::UserUseCase,
    usecases::user::UserRequestDTO
};
use protocols::repository::Repository;

pub struct UseCase {
    repository: Box<dyn Repository>
}

impl UseCase {
    pub fn new(repository: Box<dyn Repository>) -> UseCase {
        UseCase { repository }
    }
}

impl UserUseCase for UseCase {
    fn create(&self, dto: UserRequestDTO) -> Result<(), Error>{
        return self.repository.create(dto.to_user());
    }
}
