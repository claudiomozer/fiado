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
        self.code.clone()
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

    pub fn new_internal(message: String) -> Error {
        Error { kind: Kind::Internal, code: 0, message }
    }

    pub fn new_business(code: u8) -> Error {
        Error { kind: Kind::Business, code: code, message: String::from("invalid request") }
    }
}