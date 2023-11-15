use chrono::{NaiveDate, DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub enum UserStatus {
    Active,
    Blocked,
    Deleted
}

#[derive(Serialize, Debug)]
pub struct User {
    id: String,
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
            id: String::new(), 
            name,
            document,
            birth_date,
            status: UserStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now()
        }
    }

    pub fn set_uuid(&mut self, uuid: String) {
        self.id = uuid;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_document(&self) -> &String {
        &self.document
    }

    pub fn get_birth_date(&self) -> NaiveDate {
        self.birth_date
    }

    pub fn get_status(&self) -> UserStatus {
        self.status
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.document == other.document &&
        self.birth_date == other.birth_date &&
        self.name == other.name &&
        self.status == other.status
    }
}