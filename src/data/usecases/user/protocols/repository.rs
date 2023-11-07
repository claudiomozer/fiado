use mockall::automock;
use crate::domain::{
    entities::User,
    error::Error
};

#[automock]
pub trait Repository {
    fn create(&self, user: User) -> Result<(), Error>;
}