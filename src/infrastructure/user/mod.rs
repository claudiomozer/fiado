use std::str::FromStr;

use sqlx::postgres::PgQueryResult;
use sqlx::{Pool, Postgres, types::Uuid};
use async_trait::async_trait;
use crate::domain::entities::User;
use crate::data::usecases::user::protocols::repository::Repository;
use crate::domain::error;
use crate::domain::usecases::user::USER_ALREADY_EXISTS;

pub struct PostgresRepository{
    pool: Pool<Postgres>
}

impl PostgresRepository {
    pub fn new(pool: Pool<Postgres>) -> PostgresRepository {
        PostgresRepository { pool }
    }

    fn handle_postgres_error(error: sqlx::Error) -> error::Error {
        let raw_error_message: &str = &error.to_string();
        if let sqlx::Error::Database(dbe) = error{ 
            if dbe.is_unique_violation() {
                return error::Error::new_already_exists(USER_ALREADY_EXISTS, "user");
            } 
        }
        error::Error::new_internal(raw_error_message)
    } 

    fn handle_update_result(res: PgQueryResult) -> Result<(), error::Error> {
        if res.rows_affected() == 0 {
            return Err(error::Error::new_not_found(USER_ALREADY_EXISTS, "user"));
        }
        Ok(()) 
    }

}

#[async_trait]
impl Repository for PostgresRepository {
    async fn create(&self, user: User) -> Result<(), error::Error> {
        
        let user_id = match Uuid::from_str(user.get_id()) {
            Ok(id) => id,
            Err(err) => return Err(error::Error::new_internal(err.to_string().as_str()))
        };
        
        let result: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query(
            // language=PostgreSQL
            r#"
                INSERT INTO "user" (
                    id,
                    name,
                    document,
                    status,
                    "password",
                    birth_date,
                    created_at,
                    updated_at
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#)
        .bind(user_id)
        .bind(user.get_name())
        .bind(user.get_document().to_string())
        .bind(user.get_status().to_sring())
        .bind(user.get_password())
        .bind(user.get_birth_date().to_naive_date())
        .bind(user.get_created_at())
        .bind(user.get_updated_at())
        .execute(&self.pool).await;

        match result{
            Ok(_) => Ok(()),
            Err(err) => return Err(Self::handle_postgres_error(err))
        }
    }

    async fn update(&self, user: User) -> Result<(), error::Error> {
        let id =  match Uuid::from_str(user.get_id()) {
            Ok(uuid) => uuid,
            Err(err) => return Err(error::Error::new_internal(&err.to_string()))
        };

        let result = sqlx::query(
            r#"
                UPDATE "user" SET
                    name = $1,
                    document = $2,
                    status = $3,
                    "password" = $4,
                    birth_date  = $5,
                    updated_at = $6
                WHERE
                    id = $7
            "#
        ).bind(user.get_name())
        .bind(user.get_document().to_string())
        .bind(user.get_status().to_sring())
        .bind(user.get_password())
        .bind(user.get_birth_date().to_naive_date())
        .bind(user.get_created_at())
        .bind(id)
        .execute(&self.pool).await;

        match result {
            Err(e) => return Err(Self::handle_postgres_error(e)),
            Ok(r) => return Self::handle_update_result(r)
        };
    }
}
