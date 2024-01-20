use mockall::automock;
use async_trait::async_trait;
use crate::domain::{
    entities::User,
    error::Error
};

#[automock]
#[async_trait]
pub trait Repository {
    async fn create(&self, user: User) -> Result<(), Error>;
    async fn update(&self, user: User) -> Result<(), Error>;
}