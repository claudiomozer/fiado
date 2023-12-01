use std::sync::Arc;

use postgres;
use postgres::error::SqlState;
use crate::domain::entities::User;
use crate::domain::error;
use crate::domain::usecases::user::USER_ALREADY_EXISTS;

pub struct Repository{
    client: postgres::Client
}

impl Repository {
    pub fn new(client: postgres::Client) -> Repository {
        Repository { client }
    }

    pub fn create(&mut self, user: User) -> Result<(), error::Error> {
        let query = "INSERT INTO user (
            id,
            name,
            document,
            status,
            password,
            birth_date,
            created_at,
            updated_at
        ) VALUE ($1, $2, $3, $3, $4, $5, $6, $7, $8)";
        let result = self.client.execute(query, &[
            &user.get_id(),
            &user.get_name(),
            &user.get_document().to_string(),
            &user.get_status().to_sring(),
            &user.get_password(),
            &user.get_birth_date().to_naive_date(),
            &user.get_created_at(),
            &user.get_updated_at()
        ]);

        if let Err(error) = result {
            return Err(Self::handle_pg_error(error));
        }

        Ok(())
    }

    fn handle_pg_error(error: postgres::Error) -> error::Error {
        if let Some(code) = error.code(){
            if *code == SqlState::UNIQUE_VIOLATION {
                return error::Error::new_already_exists(USER_ALREADY_EXISTS, "user");
            } 
        }
        error::Error::new_internal(error.to_string().as_str())
    } 
}
