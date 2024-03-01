use bcrypt;
use rand::{Rng, distributions::Alphanumeric};
use crate::data::usecases::user::protocols::hash::Hash;

pub struct Hasher {
    pepper: String,
    cost: u32
}

impl Hash for Hasher {
    fn run(&self, plain_text: String) -> Result<String, String> {
        let salt_pepper = self.get_salt_with_pepper(); 
        match bcrypt::hash_with_salt(plain_text, self.cost, salt_pepper) {
            Ok(hash) =>  Ok(hash.to_string()),
            Err(error) => Err(error.to_string())
        }
    }

    fn verify(&self, plain_text: String, hash: String) -> Result<bool, String>{
        match bcrypt::verify(plain_text, &hash) {
            Ok(result) => Ok(result),
            Err(error) => Err(error.to_string())
        }
    }
}


impl Hasher {
    pub fn new(pepper: String, cost: u32) -> Hasher{
        if pepper.len() < 8 {
            panic!("Pepper must have at least 8 characters")
        }

        if !(4..=31).contains(&cost) {
            panic!("Cost needs to be between 4 and 31")
        }

        Hasher { pepper, cost }
    }

    fn get_salt_with_pepper(&self) -> [u8;16] {
        let mut salt: String =  rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        salt.push_str(&self.pepper);
        let mut count: usize = 0;
        let mut salt_with_pepper: [u8; 16] = [0; 16];
        for c in salt.as_bytes() {
            salt_with_pepper[count] = *c;
            count += 1;
            if count == 16 {
                break;
            }
        }
        salt_with_pepper
    }
} 

mod tests;
