use crate::data::protocols::uuid::Uuid;

pub struct Generator{}

impl Generator {
    pub fn new() -> Self {
        Generator {}
    }
}

impl Default for Generator {
    fn default() -> Self {
        Generator::new()
    }
}

impl Uuid for Generator {
    fn generate(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }
}