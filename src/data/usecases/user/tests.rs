#[cfg(test)]
#[tokio::test]
async fn it_should_return_an_error_when_repo_fails() {
    use chrono::NaiveDate;
    use crate::{data::usecases::user::{UseCase, UserCreateRequestDTO}, domain::usecases::user::UserUseCase};
    use super::protocols::{repository::MockRepository, hash::MockHash};
    use crate::domain::error::Error;
    use crate::data::protocols::uuid::MockUuid;

    let expected_error =  Error::new_internal("internal");

    let mut uuid_mock = MockUuid::new();
    uuid_mock.expect_generate().return_const("uuid");

    let mut hash_mock = MockHash::new();
    hash_mock.expect_run().return_const(Ok(String::from("hash_password")));

    let mut repository_mock = MockRepository::new();
    repository_mock.expect_create().return_const( Err(expected_error));
    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));

    let dto = UserCreateRequestDTO{
        name: String::from("Claudion du fret"),
        document: String::from("11133322292"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap(),
        password: String::from("password")
    };
    let result = sut.create(dto).await;

    assert!(match result {
        Ok(()) => false,
        Err(_) => true
    })
}

#[tokio::test]
async fn it_should_call_uuid_generator() {
    use chrono::NaiveDate;
    use crate::{data::usecases::user::{UseCase, UserCreateRequestDTO}, domain::usecases::user::UserUseCase};
    use super::protocols::{repository::MockRepository, hash::MockHash};
    use crate::data::protocols::uuid::MockUuid;

    let dto = UserCreateRequestDTO{
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

#[tokio::test]
async fn it_should_return_error_if_password_hash_fails() {
    use chrono::NaiveDate;
    use crate::data::usecases::user::{UseCase, UserCreateRequestDTO};
    use crate::domain::{usecases::user::UserUseCase, error::Kind};
    use super::protocols::{repository::MockRepository, hash::MockHash};
    use crate::data::protocols::uuid::MockUuid;
    use mockall::predicate::eq;

    let dto = UserCreateRequestDTO {
        name: String::from("Claudion du fret"),
        document: String::from("52976776024"),
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
    let result = sut.create(dto).await;

    assert!(match result {
        Ok(())=> false,
        Err(domain_error) => domain_error.get_kind() == Kind::Internal
    })
}

#[tokio::test]
async fn it_should_return_error_when_invalid_document_string_is_given() {
    use chrono::NaiveDate;
    use crate::{data::usecases::user::{UseCase, UserCreateRequestDTO}, domain::{usecases::user::UserUseCase, error::Error}};
    use super::protocols::{repository::MockRepository, hash::MockHash};
    use crate::domain::{error::Kind, usecases::user::INVALID_DOCUMENT_ERROR};
    use crate::data::protocols::uuid::MockUuid;

    let dto = UserCreateRequestDTO{
        name: String::from("Claudion du fret"),
        document: String::from("invalid123"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap(),
        password: String::from("password"),
    };

    let mut hash_mock = MockHash::new();
    hash_mock.expect_run().return_const(Ok(String::from("hash_password")));

    let mut uuid_mock = MockUuid::new();
    uuid_mock.expect_generate().return_const("uuid");

    let mut repository_mock = MockRepository::new();
    repository_mock.expect_create().return_const(Ok(()));

    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));
    let result = sut.create(dto).await;

    let mut error: Error = Error::new();
    assert!(match result {
        Ok(()) => false,
        Err(e) => {
            error = e;
            true
        }
    });

    assert_eq!(error.get_kind(), Kind::Business);
    assert_eq!(error.get_code(), INVALID_DOCUMENT_ERROR);
}

#[tokio::test]
async fn it_should_return_error_when_invalid_cpf_is_given() {
    use chrono::NaiveDate;
    use crate::{data::usecases::user::{UseCase, UserCreateRequestDTO}, domain::{usecases::user::UserUseCase, error::Error}};
    use super::protocols::{repository::MockRepository, hash::MockHash};
    use crate::domain::{error::Kind, usecases::user::INVALID_DOCUMENT_ERROR};
    use crate::data::protocols::uuid::MockUuid;

    let dto = UserCreateRequestDTO{
        name: String::from("Claudion du fret"),
        document: String::from("40735626066"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap(),
        password: String::from("password"),
    };

    let mut hash_mock = MockHash::new();
    hash_mock.expect_run().return_const(Ok(String::from("hash_password")));

    let mut uuid_mock = MockUuid::new();
    uuid_mock.expect_generate().return_const("uuid");

    let mut repository_mock = MockRepository::new();
    repository_mock.expect_create().return_const(Ok(()));

    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));
    let result = sut.create(dto).await;

    let mut error: Error = Error::new();
    assert!(match result {
        Ok(()) => false,
        Err(e) => {
            error = e;
            true
        }
    });

    assert_eq!(error.get_kind(), Kind::Business);
    assert_eq!(error.get_code(), INVALID_DOCUMENT_ERROR);
}

#[tokio::test]
async fn it_should_return_error_when_user_is_underage_given() {
    use chrono::NaiveDate;
    use crate::{data::usecases::user::{UseCase, UserCreateRequestDTO}, domain::{usecases::user::UserUseCase, error::Error}};
    use super::protocols::{repository::MockRepository, hash::MockHash};
    use crate::domain::{error::Kind, usecases::user::UNDERAGE_ERROR};
    use crate::data::protocols::uuid::MockUuid;

    let dto = UserCreateRequestDTO{
        name: String::from("Claudion du fret"),
        document: String::from("55168718086"),
        birth_date: NaiveDate::parse_from_str("2007-09-05", "%Y-%m-%d").unwrap(),
        password: String::from("password"),
    };

    let mut hash_mock = MockHash::new();
    hash_mock.expect_run().return_const(Ok(String::from("hash_password")));

    let mut uuid_mock = MockUuid::new();
    uuid_mock.expect_generate().return_const("uuid");

    let mut repository_mock = MockRepository::new();
    repository_mock.expect_create().return_const(Ok(()));

    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));
    let result = sut.create(dto).await;

    let mut error: Error = Error::new();
    assert!(match result {
        Ok(()) => false,
        Err(e) => {
            error = e;
            true
        }
    });

    assert_eq!(error.get_kind(), Kind::Business);
    assert_eq!(error.get_code(), UNDERAGE_ERROR);
}


#[tokio::test]
async fn it_should_not_return_error_on_success() {
    use chrono::NaiveDate;
    use crate::{
        data::usecases::user::{UseCase, UserCreateRequestDTO}, 
        domain::usecases::user::UserUseCase
    };
    use super::protocols::{repository::MockRepository, hash::MockHash};
    use mockall::predicate::eq;
    use crate::data::protocols::uuid::MockUuid;

    let dto = UserCreateRequestDTO{
        name: String::from("Claudion du fret"),
        document: String::from("40735626065"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap(),
        password: String::from("password")
    };

    let mut hash_mock = MockHash::new();
    hash_mock.expect_run().return_const(Ok(String::from("hash_password")));

    let mut uuid_mock = MockUuid::new();
    uuid_mock.expect_generate().return_const("uuid");

    let mut repository_mock = MockRepository::new();
    let mut user = dto.clone().to_user().unwrap();
    user.set_uuid(String::from("uuid"));
    user.set_password(String::from("hash_password"));

    repository_mock.expect_create().with(eq(user)).return_const(Ok(()));

    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));
    let result = sut.create(dto).await;

    assert!(match result {
        Ok(()) => true,
        Err(_) => false
    });
}

#[tokio::test]
async fn it_should_return_error_if_dto_map_fails() {
    use crate::domain::{
        usecases::user::{UserUseCase, UserUpdateRequestDTO, INVALID_DOCUMENT_ERROR},
        error::{Error, Kind}
    };
    use crate::data::usecases::user::UseCase;
    use super::protocols::{repository::MockRepository, hash::MockHash};
    use crate::data::protocols::uuid::MockUuid;
    use uuid::Uuid;
    use chrono::NaiveDate;

    let dto = UserUpdateRequestDTO{
        id: Uuid::new_v4().to_string(),
        name: String::from("Claudion du fret"),
        document: String::from("invalid"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap(),
    };
    let hash_mock = MockHash::new();
    let uuid_mock = MockUuid::new();
    let repository_mock = MockRepository::new();

    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));
    let result = sut.update(dto).await;

    let mut err: Error = Error::new();
    assert!(match result {
        Ok(()) => false,
        Err(e) => {
            err = e;
            true
        } 
    });

    assert_eq!(err.get_kind(), Kind::Business);
    assert_eq!(err.get_code(), INVALID_DOCUMENT_ERROR);
}


#[tokio::test]
async fn it_should_return_error_when_invalid_document_is_provided() {
    use crate::domain::usecases::user::{UserUseCase, UserUpdateRequestDTO};
    use crate::domain::{
        usecases::user::INVALID_DOCUMENT_ERROR,
        error::{Error, Kind}
    };
    use crate::data::usecases::user::UseCase;
    use super::protocols::{repository::MockRepository, hash::MockHash};
    use crate::data::protocols::uuid::MockUuid;
    use uuid::Uuid;
    use chrono::NaiveDate;

    let dto = UserUpdateRequestDTO{
        id: Uuid::new_v4().to_string(),
        name: String::from("Claudion du fret"),
        document: String::from("40735626063"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap(),
    };
    let hash_mock = MockHash::new();
    let uuid_mock = MockUuid::new();
    let repository_mock = MockRepository::new();

    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));
    let result = sut.update(dto).await;

    let mut err: Error = Error::new();
    assert!(match result {
        Ok(()) => false,
        Err(e) => {
            err = e;
            true
        } 
    });

    assert_eq!(err.get_kind(), Kind::Business);
    assert_eq!(err.get_code(), INVALID_DOCUMENT_ERROR);
}

#[tokio::test]
async fn it_should_return_error_when_user_is_underage() {
    use crate::domain::usecases::user::{UserUseCase, UserUpdateRequestDTO};
    use crate::domain::{
        usecases::user::UNDERAGE_ERROR,
        error::{Error, Kind}
    };
    use crate::data::usecases::user::UseCase;
    use super::protocols::{repository::MockRepository, hash::MockHash};
    use crate::data::protocols::uuid::MockUuid;
    use uuid::Uuid;
    use chrono::NaiveDate;

    let dto = UserUpdateRequestDTO{
        id: Uuid::new_v4().to_string(),
        name: String::from("Claudion du fret"),
        document: String::from("40735626065"),
        birth_date: NaiveDate::parse_from_str("2020-09-05", "%Y-%m-%d").unwrap(),
    };
    let hash_mock = MockHash::new();
    let uuid_mock = MockUuid::new();
    let repository_mock = MockRepository::new();

    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));
    let result = sut.update(dto).await;

    let mut err: Error = Error::new();
    assert!(match result {
        Ok(()) => false,
        Err(e) => {
            err = e;
            true
        } 
    });

    assert_eq!(err.get_kind(), Kind::Business);
    assert_eq!(err.get_code(), UNDERAGE_ERROR);
}

#[tokio::test]
async fn it_should_return_error_when_repository_fails() {
    use crate::domain::usecases::user::{UserUseCase, UserUpdateRequestDTO};
    use crate::domain::error::{Error, Kind};
    use crate::data::usecases::user::UseCase;
    use super::protocols::{repository::MockRepository, hash::MockHash};
    use crate::data::protocols::uuid::MockUuid;
    use mockall::predicate::eq;
    use uuid::Uuid;
    use chrono::NaiveDate;

    let expected_err = Error::new_internal("update error");
    let dto = UserUpdateRequestDTO{
        id: Uuid::new_v4().to_string(),
        name: String::from("Claudion du fret"),
        document: String::from("40735626065"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap(),
    };
    let hash_mock = MockHash::new();
    let uuid_mock = MockUuid::new();
    let mut repository_mock = MockRepository::new();
    let user = dto.clone().to_user().unwrap();

    repository_mock.expect_update().with(eq(user)).return_const(Err(expected_err));

    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));
    let result = sut.update(dto).await;

    let mut err: Error = Error::new();
    assert!(match result {
        Ok(()) => false,
        Err(e) => {
            err = e;
            true
        } 
    });

    assert_eq!(err.get_kind(), Kind::Internal);
}


#[tokio::test]
async fn it_should_not_return_error_on_update_success() {
    use crate::domain::usecases::user::{UserUseCase, UserUpdateRequestDTO};
    use crate::data::usecases::user::UseCase;
    use super::protocols::{repository::MockRepository, hash::MockHash};
    use crate::data::protocols::uuid::MockUuid;
    use mockall::predicate::eq;
    use uuid::Uuid;
    use chrono::NaiveDate;

    let dto = UserUpdateRequestDTO{
        id: Uuid::new_v4().to_string(),
        name: String::from("Claudion du fret"),
        document: String::from("40735626065"),
        birth_date: NaiveDate::parse_from_str("1999-09-05", "%Y-%m-%d").unwrap(),
    };
    let hash_mock = MockHash::new();
    let uuid_mock = MockUuid::new();
    let mut repository_mock = MockRepository::new();
    let user = dto.clone().to_user().unwrap();

    repository_mock.expect_update().with(eq(user)).return_const(Ok(()));
    let sut = UseCase::new(Box::new(repository_mock), Box::new(uuid_mock), Box::new(hash_mock));
    let result = sut.update(dto).await;

    assert!(match result {
        Ok(()) => true,
        Err(_) => false, 
    });
}
