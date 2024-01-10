use crate::domain::usecases::user::UserUseCase;
use crate::data::usecases::user::UseCase;
use crate::infrastructure::{
    user::PostgresRepository,
    hash::Hasher,
    uuid::Generator
};
use sqlx::{Pool, Postgres};

use super::env;

pub struct Container{
    pg_pool: Pool<Postgres>,
    pub user_use_case: Box<dyn UserUseCase + Send + Sync + 'static>
}

impl Container {
    pub async fn load_dependencies() -> Container {
        let vars = env::Vars::load();
        let conn_string = format!("postgresql://{}:{}@{}:{}/{}", vars.db_user, vars.db_password, vars.db_host, vars.db_port, vars.db_name);
        let pg_pool: Pool<Postgres> = Pool::<Postgres>::connect(&conn_string).await.unwrap();

        let user_repository = Box::new(PostgresRepository::new(pg_pool.clone()));
        let hash_provider = Box::new(Hasher::new(String::from("12345678"), 5));
        let uuid_generator = Box::new(Generator::new());

        let user_use_case = Box::new(UseCase::new(user_repository, uuid_generator, hash_provider));

        Container{
            user_use_case: user_use_case,
            pg_pool: pg_pool
        }    
    }

    pub async fn destroy(&mut self) {
        self.pg_pool.close().await;
    }
}
