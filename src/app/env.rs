use std::env;

pub struct Vars {
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: String
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

        Vars {
            db_name,
            db_user,
            db_password,
            db_host,
            db_port
        }
    }
}