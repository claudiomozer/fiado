#[derive(Clone, Debug)]
pub struct Error {
    kind: Kind,
    code: u8,
    message: String,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Kind {
    Business,
    NotFound,
    AlreadyExists,
    Internal
}

impl Error {
    pub fn get_code(&self) -> u8 {
        self.code
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn get_kind(&self) -> Kind {
        self.kind.clone()
    }

    pub fn new() -> Error{
        Error { kind: Kind::Internal, code: 0, message: String::from("unexpected error") }
    }

    pub fn new_internal(message: &str) -> Error {
        Error { kind: Kind::Internal, code: 0, message: String::from(message) }
    }

    pub fn new_business(code: u8) -> Error {
        Error { kind: Kind::Business, code, message: String::from("invalid request") }
    }

    pub fn new_business_with_message(code: u8, message: &str) -> Error {
        Error { kind: Kind::Business, code, message: String::from(message)  }
    }

    pub fn new_not_found(code: u8, entity: &str) -> Error {
        Error { kind: Kind::NotFound, code, message: format!("{} not found", entity)}
    }
    
    pub fn new_already_exists(code: u8, entity: &str) -> Error {
        Error { kind: Kind::Business, code, message: format!("{} already exists", entity) }
    }
}

impl Default for Error {
    fn default() -> Self {
       Error::new() 
    }
}