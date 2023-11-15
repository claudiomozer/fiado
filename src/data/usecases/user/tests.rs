use mockall::predicate::eq;

#[cfg(test)]
#[test]
fn it_should_return_an_error_when_repo_fails() {
    use chrono::NaiveDate;
    use crate::{data::usecases::user::{UseCase, UserRequestDTO}, domain::usecases::user::UserUseCase};
    use super::protocols::{repository::MockRepository, uuid::MockUuid, hash::MockHash};
    use crate::domain::error::Error;

    let expected_error =  Error::new_internal(String::from("internal"));

    let mut uuid_mock = MockUuid::new();
    uuid_mock.expect_generate().return_const("uuid");

    let mut hash_mock = MockHash::new();
    hash_mock.expect_run().return_const(Ok(String::from("hash_password")));

    let mut repository_mock = MockRepository::new();
    repository_mock.expect_create().return_const( Err(expected_error));
    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));

    let dto = UserRequestDTO{
        name: String::from("Claudion du fret"),
        document: String::from("11133322292"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap(),
        password: String::from("password")
    };
    let result = sut.create(dto);

    assert!(match result {
        Ok(()) => false,
        Err(_) => true
    })
}

#[test]
fn it_should_call_uuid_generator() {
    use chrono::NaiveDate;
    use crate::{data::usecases::user::{UseCase, UserRequestDTO}, domain::usecases::user::UserUseCase};
    use super::protocols::{repository::MockRepository, uuid::MockUuid, hash::MockHash};

    let dto = UserRequestDTO{
        name: String::from("Claudion du fret"),
        document: String::from("11133322292"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap(),
        password: String::from("password")
    };

    let mut hash_mock = MockHash::new();
    hash_mock.expect_run().return_const(Ok(String::from("hash_password")));

    let mut repository_mock = MockRepository::new();
    repository_mock.expect_create().return_const(Ok(()));

    let mut uuid_mock = MockUuid::new();
    uuid_mock.expect_generate().return_const("uuid");

    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));
    _ = sut.create(dto);
}

#[test]
fn it_should_return_error_if_password_hash_fails() {
    use chrono::NaiveDate;
    use crate::data::usecases::user::{UseCase, UserRequestDTO};
    use crate::domain::{usecases::user::UserUseCase, error::Kind};
    use super::protocols::{repository::MockRepository, uuid::MockUuid, hash::MockHash};

    let dto = UserRequestDTO {
        name: String::from("Claudion du fret"),
        document: String::from("11133322292"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap(),
        password: String::from("password")
    };

    let mut hash_mock = MockHash::new();
    hash_mock.expect_run().with(eq(String::from("password"))).return_const(Err(String::from("hash_error")));

    let mut repository_mock = MockRepository::new();
    repository_mock.expect_create().return_const(Ok(()));

    let mut uuid_mock = MockUuid::new();
    uuid_mock.expect_generate().return_const("uuid");

    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));
    let result = sut.create(dto);

    assert!(match result {
        Ok(())=> false,
        Err(domain_error) => domain_error.get_kind() == Kind::Internal
    })
}

#[test]
fn it_should_not_return_error_on_success() {
    use chrono::NaiveDate;
    use crate::{data::usecases::user::{UseCase, UserRequestDTO}, domain::usecases::user::UserUseCase};
    use super::protocols::{repository::MockRepository, uuid::MockUuid, hash::MockHash};
    use mockall::predicate::eq;

    let dto = UserRequestDTO{
        name: String::from("Claudion du fret"),
        document: String::from("11133322292"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap(),
        password: String::from("password"),
    };

    let mut hash_mock = MockHash::new();
    hash_mock.expect_run().return_const(Ok(String::from("hash_password")));

    let mut uuid_mock = MockUuid::new();
    uuid_mock.expect_generate().return_const("uuid");

    let mut repository_mock = MockRepository::new();
    let mut user = dto.clone().to_user();
    user.set_uuid(String::from("uuid"));
    user.set_password(String::from("hash_password"));

    repository_mock.expect_create().with(eq(user)).return_const(Ok(()));

    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));
    let result = sut.create(dto);

    assert!(match result {
        Ok(()) => true,
        Err(_) => false
    })
}
