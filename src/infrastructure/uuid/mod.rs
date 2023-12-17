use crate::data::protocols::uuid::Uuid;

pub struct Generator{}

impl Generator {
    pub fn new() -> Self {
        Generator {}
    }
}

impl Uuid for Generator {
    fn generate(&self) -> String {
        return uuid::Uuid::new_v4().to_string();
    }
}