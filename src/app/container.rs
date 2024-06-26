use crate::domain::usecases::user::UserUseCase;
use crate::domain::usecases::admin::AdminUseCase;
use crate::data::usecases::user;
use crate::data::usecases::admin;
use crate::infrastructure::logger;
use crate::infrastructure::{
    user::PostgresRepository,
    hash::Hasher,
    uuid::Generator,
    tracer
};
use sqlx::{Pool, Postgres};
use opentelemetry_sdk::trace::Tracer;
use opentelemetry::global::shutdown_tracer_provider;

use super::env;

pub struct Container{
    pub pg_pool: Pool<Postgres>,
    pub tracer: Tracer,
    pub admin_use_case: Box<dyn AdminUseCase + Send + Sync + 'static>,
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

        let user_use_case = Box::new(user::UseCase::new(user_repository, uuid_generator, hash_provider));
        let admin_use_case = Box::new(admin::UseCase::new(vars.admin_jwt_secret, vars.admin_role_name, vars.admin_token_duration));

        let tracer = tracer::init_tracer(&vars.otlp_endpoint,&vars.service_name).unwrap();

        logger::init();

        Container{
            tracer,
            user_use_case,
            admin_use_case, 
            pg_pool
        }    
    }

    pub async fn destroy(&mut self) {
        shutdown_tracer_provider();
        self.pg_pool.close().await;
    }
}
