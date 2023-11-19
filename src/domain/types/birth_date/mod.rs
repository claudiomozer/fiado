use chrono::{NaiveDate, Utc, Months};
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub struct BirthDate(NaiveDate);

impl BirthDate {
    pub fn from_naive(date: NaiveDate) -> BirthDate {
        return BirthDate(date)
    }

    pub fn is_under_age(&self) -> bool{ 
        let today = Utc::now().date_naive(); 
        if let Some(result) = self.0.checked_add_months(Months::new(12 * 18))  {
            return result > today;
        }      
        false
    }
}

mod tests;
