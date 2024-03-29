use chrono::{DateTime, Utc};
use serde::Serialize;
use super::types::{cpf::CPF, birth_date::BirthDate};

#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub enum UserStatus {
    Active,
    Blocked,
    Deleted,
    Unknown,
}

#[derive(Serialize, Debug, Clone)]
pub struct User {
    id: String,
    name: String,
    document: CPF,
    status: UserStatus,
    password: String,
    birth_date: BirthDate,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}

impl User {
    pub fn new(name: String, document: CPF, birth_date: BirthDate) -> User {
        User {
            id: String::new(), 
            name,
            document,
            birth_date,
            status: UserStatus::Active,
            password: String::new(),
            created_at: Utc::now(),
            updated_at: Utc::now()
        }
    }

    pub fn set_uuid(&mut self, uuid: String) {
        self.id = uuid;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_document(&self) -> &CPF {
        &self.document
    }

    pub fn set_document(&mut self, document: CPF) {
        self.document = document;
    }

    pub fn get_birth_date(&self) -> BirthDate {
        self.birth_date
    }

    pub fn set_birth_date(&mut self, birth_date: BirthDate) {
        self.birth_date = birth_date;
    }

    pub fn get_status(&self) -> UserStatus {
        self.status
    }

    pub fn set_status(&mut self, status: UserStatus) {
        self.status = status;
    }

    pub fn get_password(&self) -> &str {
        self.password.as_str()
    }

    pub fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn set_created_at(&mut self, created_at: DateTime<Utc>) {
        self.created_at = created_at;
    }

    pub fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn set_updated_at(&mut self, updated_at: DateTime<Utc>) {
        self.updated_at = updated_at;
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


impl UserStatus {
    pub fn to_sring(&self) -> &'static str {
        match self {
            Self::Active => "ACTIVE",
            Self::Blocked => "BLOCKED",
            Self::Deleted => "DELETED",
            Self::Unknown => "UNKNOWN"
        }
    }

    pub fn from_string(s: &str) -> UserStatus {
        match s {
            "ACTIVE" => UserStatus::Active,
            "BLOCKED" => UserStatus::Blocked,
            "DELETED" => UserStatus::Deleted,
            _ => UserStatus::Unknown,
        }
    }
}