#[cfg(test)]
#[test]
fn it_should_return_error_when_birth_date_is_under_age() {
    use chrono::NaiveDate;
    use super::BirthDate;

    let date = NaiveDate::parse_from_str("2010-09-05", "%Y-%m-%d").unwrap();
    assert!(BirthDate::from_naive(date).is_under_age());
}

#[test]
fn it_should_return_error_when_birth_date_is_not_under_age() {
    use chrono::NaiveDate;
    use super::BirthDate;

    let date = NaiveDate::parse_from_str("1999-03-01", "%Y-%m-%d").unwrap();
    assert!(!BirthDate::from_naive(date).is_under_age());
}
