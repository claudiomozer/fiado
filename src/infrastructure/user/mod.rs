use std::str::FromStr;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::Row;
use sqlx::postgres::{PgQueryResult, PgRow};
use sqlx::{Pool, Postgres};
use sqlx::types::Uuid;
use async_trait::async_trait;
use crate::domain::entities::{User, UserStatus};
use crate::data::usecases::user::protocols::repository::Repository;
use crate::domain::error::{self, Error};
use crate::domain::types::birth_date::BirthDate;
use crate::domain::types::cpf::CPF;
use crate::domain::usecases::user::{USER_ALREADY_EXISTS, USER_NOT_FOUND };

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
            return Err(error::Error::new_not_found(USER_NOT_FOUND, "user"));
        }
        Ok(()) 
    }

    fn handle_delete_result(res: PgQueryResult) -> Result<(), error::Error> {
        if res.rows_affected() == 0 {
            return Err(error::Error::new_not_found(USER_NOT_FOUND, "user"));
        }
        Ok(()) 
    }

    fn get_user_from_pg_row(row: PgRow) -> Result<User, sqlx::Error> {
        let id: Uuid = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let document: &str = row.try_get("document")?;
        let status: &str = row.try_get("status")?;
        let birth_date: NaiveDate = row.try_get("birth_date")?;
        let db_created_at: NaiveDateTime = row.try_get("created_at")?;
        let db_updated_at: NaiveDateTime  = row.try_get("updated_at")?;


        let cpf = match CPF::from_string(String::from(document)) {
            Ok(c) => c,
            Err(()) => return Err(sqlx::Error::TypeNotFound { type_name: String::from("CPF") })
        };

        let mut user = User::new(name, cpf, BirthDate::from_naive(birth_date));
        user.set_uuid(id.to_string());
        user.set_status(UserStatus::from_string(status));
        user.set_created_at(db_created_at.and_utc());
        user.set_updated_at(db_updated_at.and_utc());
        return Ok(user);
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

    async fn get_by_cpf(&self, cpf: &str) -> Result<User, error::Error> {
        let result = sqlx::query(
            r#"
                SELECT 
                    id,
                    name,
                    document,
                    status,
                    "password",
                    birth_date,
                    created_at,
                    updated_at
                FROM "user"
                WHERE document = $1
            "#
        ).bind(cpf).fetch_optional(&self.pool).await;

        let row = match result {
            Err(e) => return Err(Error::new_internal(e.to_string().as_str())),
            Ok(None) => return Err(Error::new_not_found(USER_NOT_FOUND, "user")),
            Ok(Some(r)) => r
        };

        match PostgresRepository::get_user_from_pg_row(row) {
            Ok(u) => return Ok(u),
            Err(e) => return Err(Error::new_internal(e.to_string().as_str())),
        };
    }

    async fn delete_by_cpf(&self, cpf: &str) -> Result<(), Error> {
        let query = r#"
            DELETE FROM "user"
            WHERE document = $1 
        "#;
        let result = sqlx::query(query)
            .bind(cpf).execute(&self.pool).await;

        if let Err(e) = result {
            return Err(Error::new_internal(e.to_string().as_str()));
        } 

        match result {
            Err(e) => Err(Error::new_internal(e.to_string().as_str())),
            Ok(r) => Self::handle_delete_result(r)
        }
    }
}
