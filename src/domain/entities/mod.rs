use uuid::Uuid;
use chrono::{NaiveDate, DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub enum UserStatus {
    Active,
    Blocked,
    Deleted
}


#[derive(Serialize)]
pub struct User {
    id: Uuid,
    name: String,
    document: String,
    status: UserStatus,
    birth_date: NaiveDate,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}

impl User {
    pub fn new(name: String, document: String, birth_date: NaiveDate) -> User {
        User {
            id: uuid::Builder::nil().into_uuid(),
            name,
            document,
            birth_date,
            status: UserStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now()
        }
    }
}