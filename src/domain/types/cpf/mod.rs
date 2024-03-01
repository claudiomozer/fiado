use serde::{Serialize, Serializer};
use std::fmt::{self, Write, Display};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CPF ([u32; 11]);

impl CPF {

    pub fn is_valid(&self) -> bool {
        self.numbers_are_not_repeated() && self.are_verifier_digits_valid()
    }

    fn numbers_are_not_repeated(&self) -> bool {
        let number = self.0[0];
        for i in 0..11 {
            if self.0[i] != number {
                return true;
            }
        } 
        false
    }

    fn are_verifier_digits_valid(&self) -> bool {
        let mut first_digit_sum: u32 = 0;
        for i in 0..9 {
            first_digit_sum += self.0[i] * (10 - (i as u32));
        }
        let mut first_digit_mod = (first_digit_sum * 10) % 11;
        if first_digit_mod >= 10 {
            first_digit_mod = 0;
        }

        let mut second_digit_sum: u32 = 0;
        for i in 0..10 {
            second_digit_sum += self.0[i] * (11 - (i as u32));
        }
        let mut second_digit_mod = (second_digit_sum * 10) % 11;
        if second_digit_mod >= 10 {
            second_digit_mod = 0;
        }

        if first_digit_mod == self.0[9] && second_digit_mod == self.0[10] {
            return true;   
        }
        false
    }

    pub fn from_string(cpf: String) -> Result<CPF, String> {
        let mut document_numbers: [u32; 11] = [0;11];
        let mut count: usize = 0;

        if cpf.len() > 11 {
            return Err(String::from("CPF len bigger than 11 chars"));
        }

        for ch in cpf.chars() {
            if let Some(number) = ch.to_digit(10) {
                document_numbers[count] = number;
            } else {
                return Err(String::from("CPF with invalid digits"));
            }
            count += 1;
        }
        
        if count < 11 {
            return Err(String::from("CPF shorter than 11 chars"));
        }

        Ok(CPF(document_numbers))
    }
}

impl Serialize for CPF {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut val: String = String::new();
        for n in self.0 {
            let _ = write!(&mut val, "{}", n);
        }
        serializer.serialize_str(&val)
    }
}

impl Display for CPF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.map(|digit| digit.to_string()).concat())
    }
}


mod tests;
