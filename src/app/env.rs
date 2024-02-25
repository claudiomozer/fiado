use std::env;

pub struct Vars {
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: String,
    pub admin_jwt_secret: String,
    pub admin_role_name: String,
    pub admin_token_duration: u64,
}

impl Vars {
    pub fn load() -> Vars {
        let db_name: String = match env::var("DB_NAME") {
            Ok(v) => v,
            Err(e) => panic!("DB_NAME is not set: {}", e)
        };

        let db_user: String = match env::var("DB_USER") {
            Ok(v) => v,
            Err(e) => panic!("DB_USER is not set: {}", e)
        };

        let db_password: String = match env::var("DB_PASSWORD") {
            Ok(v) => v,
            Err(e) => panic!("DB_PASSWORD is not set: {}", e)
        };

        let db_host: String = match env::var("DB_HOST") {
            Ok(v) => v,
            Err(e) => panic!("DB_HOST is not set: {}", e)
        };

        let db_port: String = match env::var("DB_PORT") {
            Ok(v) => v,
            Err(_) => String::from("5432")
        };

        let admin_jwt_secret: String = match env::var("ADMIN_JWT_SECRET") {
            Ok(v) => v,
            Err(e) => panic!("ADMIN_JWT_SECRET is not set: {}", e)
        };

        let admin_role_name: String = match env::var("ADMIN_ROLE_NAME") {
            Ok(v) => v,
            Err(_) => String::from("ADMIN")
        };

        let admin_token_duration: u64 = match env::var("ADMIN_TOKEN_DURATION_IN_DAYS") {
            Ok(v) => match v.parse::<u64>() {
                Ok(v) => v,
                Err(_) => panic!("Invalid type for ADMIN_TOKEN_DURATION_IN_DAYS")
            },
            Err(_) => 1 
        };

        Vars {
            db_name,
            db_user,
            db_password,
            db_host,
            db_port,
            admin_jwt_secret,
            admin_role_name,
            admin_token_duration
        }
    }
}