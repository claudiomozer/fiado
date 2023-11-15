#[cfg(test)]
#[test]
fn it_should_return_an_error_when_repo_fails() {
    use chrono::NaiveDate;
    use crate::{data::usecases::user::{UseCase, UserRequestDTO}, domain::usecases::user::UserUseCase};
    use super::protocols::{repository::MockRepository, uuid::MockUuid};
    use crate::domain::error::Error;

    let expected_error =  Error::new_internal(String::from("internal"));

    let mut uuid_mock = MockUuid::new();
    uuid_mock.expect_generate().return_const("uuid");

    let mut repository_mock = MockRepository::new();
    repository_mock.expect_create().return_const( Err(expected_error));
    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock));

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

#[test]
fn it_should_call_uuid_fgenerator() {
    use chrono::NaiveDate;
    use crate::{data::usecases::user::{UseCase, UserRequestDTO}, domain::usecases::user::UserUseCase};
    use super::protocols::{repository::MockRepository, uuid::MockUuid};

    let dto = UserRequestDTO{
        name: String::from("Claudion du fret"),
        document: String::from("11133322292"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap()
    };

    let mut repository_mock = MockRepository::new();
    repository_mock.expect_create().return_const(Ok(()));

    let mut uuid_mock = MockUuid::new();
    uuid_mock.expect_generate().return_const("uuid");

    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock));
    _ = sut.create(dto);
}

#[test]
fn it_should_not_return_error_on_success() {
    use chrono::NaiveDate;
    use crate::{data::usecases::user::{UseCase, UserRequestDTO}, domain::usecases::user::UserUseCase};
    use super::protocols::{repository::MockRepository, uuid::MockUuid};
    use mockall::predicate::eq;

    let dto = UserRequestDTO{
        name: String::from("Claudion du fret"),
        document: String::from("11133322292"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap()
    };

    let mut uuid_mock = MockUuid::new();
    uuid_mock.expect_generate().return_const("uuid");

    let mut repository_mock = MockRepository::new();
    let mut user = dto.clone().to_user();
    user.set_uuid(String::from("uuid"));

    repository_mock.expect_create().with(eq(user)).return_const(Ok(()));

    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock));
    let result = sut.create(dto);

    assert!(match result {
        Ok(()) => true,
        Err(_) => false
    })
}
