use crate::domain::{
    usecases::admin::{AdminUseCase, EXPIRED_TOKEN_ERROR, INVALID_TOKEN_ERROR},
    error::Error
};
use std::time::{Duration, SystemTime};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{Header, Algorithm, encode, EncodingKey, DecodingKey, decode, Validation, errors::{self, ErrorKind}};
use async_trait::async_trait;

const SECONDS_IN_A_DAY: u64 = 86400;

pub struct UseCase {
    api_secret: String,
    role_name: String,
    token_duration_in_days: u64 
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

impl UseCase {
    pub fn new(api_secret: String, role_name: String, token_duration_in_days: u64) -> UseCase {
        UseCase { api_secret, role_name, token_duration_in_days}
    }
}

#[async_trait]
impl AdminUseCase for UseCase {
    async fn validate_token(&self, token: String) -> Result<(), Error> {
        let mut validation = Validation::new(Algorithm::HS512);
        validation.sub = Some(self.role_name.clone());

        let result =decode::<Claims>(
            token.as_str(),
            &DecodingKey::from_secret(self.api_secret.as_ref()),
            &validation
        );

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(to_domain_error(e))
        }
    }   
    
    async fn generate_token(&self) -> Result<String, Error> {
        let header = Header::new(Algorithm::HS512);
        let expires_at = match self.get_expiration_timestamp() {
            Ok(timestamp) => timestamp,
            Err(e) => return Err(e)
        };

        let claims = Claims {
            sub: self.role_name.clone(),
            exp: expires_at
        };

        match encode(&header, &claims, &EncodingKey::from_secret(self.api_secret.as_ref())) {
            Ok(token) => Ok(token),
            Err(e) => {
                Err(Error::new_internal(e.to_string().as_str()))
            }
        }
    }
}

impl UseCase {
    fn get_expiration_timestamp(&self) -> Result<u64, Error> {
        let now = SystemTime::now();
        if let Some(expires_at) = now.checked_add(Duration::from_secs(SECONDS_IN_A_DAY * self.token_duration_in_days)) {

            match expires_at.duration_since(std::time::UNIX_EPOCH) {
                Ok(expires_at_time_since) => return Ok(expires_at_time_since.as_secs()),
                Err(e) => return Err(Error::new_internal(&e.to_string()))
            };
        }
        Err(Error::new_internal("error generating expiration date"))
    }
}

fn to_domain_error(e: errors::Error) -> Error {
    match e.kind() {
        ErrorKind::ExpiredSignature => Error::new_business(EXPIRED_TOKEN_ERROR),
        ErrorKind::InvalidSubject | ErrorKind::InvalidSignature => Error::new_business(INVALID_TOKEN_ERROR),
        _ => Error::new_internal(e.to_string().as_str())
    }
}

mod tests;
