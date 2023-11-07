#[cfg(test)]
#[test]
fn it_should_return_an_error_when_repo_fails() {
    use chrono::NaiveDate;

    use crate::{data::usecases::user::{UseCase, UserRequestDTO}, domain::usecases::user::UserUseCase};
    use super::protocols::repository::MockRepository;
    use crate::domain::error::Error;

    let expected_error =  Error::new_internal(String::from("internal"));

    let mut repository_mock = MockRepository::new();
    repository_mock.expect_create().return_const( Err(expected_error));
    let sut = UseCase::new(Box::new(repository_mock));

    let dto = UserRequestDTO{
        name: String::from("Claudion du fret"),
        document: String::from("11133322292"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap()
    };
    let result = sut.create(dto);

    assert!(match result {
        Ok(()) => false,
        Err(_) => true
    })
}
