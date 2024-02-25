use tokio;

#[cfg(test)]
#[tokio::test]
async fn it_should_create_token_return_error_when_jwt_fails() {
    use crate::data::usecases::admin::UseCase;
    use crate::domain::usecases::admin::AdminUseCase;

    let sut = UseCase::new(String::from("key"), String::from("ADMIN"),1);

    let result = sut.generate_token().await;

    assert!(match result {
       Ok(t) => t.len() > 0,
       Err(_) => false, 
    });
}

#[tokio::test]
async fn it_should_not_return_error_when_validating_a_valid_token_string() {
    use crate::data::usecases::admin::UseCase;
    use crate::domain::usecases::admin::AdminUseCase;

    let sut = UseCase::new(String::from("secret"), String::from("ADMIN"), 1);
    let result = sut.generate_token().await;

    let mut token = String::new();
    assert!(match result {
       Ok(t) => {
        token = t;
        true
       }, 
       Err(_) => false,
    });

    let validation_result = sut.validate_token(token).await;
    assert!(match validation_result {
        Ok(()) => true,
        Err(_) => false
    });
}

#[tokio::test]
async fn it_should_return_error_when_token_is_expired() {
    use crate::data::usecases::admin::UseCase;
    use crate::domain::usecases::admin::{AdminUseCase, EXPIRED_TOKEN_ERROR};
    use crate::domain::error::Error;

    let expired_token = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJBRE1JTiIsImV4cCI6MTcwODM4NjM2MX0.UuuTuCzLVVx17yzfDrAsqxLonT7dot8-jsGBubt9lW6HBHpJGVWzvBcbkPxoIfE94W9dismxLpJidH2acR8TeA");
    let sut = UseCase::new(String::from("s3cret"), String::from("ADMIN"), 0);

    let validation_result = sut.validate_token(expired_token).await;
    let err: Error;
    assert!(match validation_result {
        Ok(()) => panic!("must fails when token is expired"),
        Err(e) => {
            err = e;
            true
        }
    });

    assert!(
        match err.get_code() {
            EXPIRED_TOKEN_ERROR => true,
            _ => false
        }
    )
}


#[tokio::test]
async fn it_should_return_error_when_invalid_token_subject_is_provided() {
    use crate::data::usecases::admin::UseCase;
    use crate::domain::usecases::admin::{AdminUseCase, INVALID_TOKEN_ERROR};
    use crate::domain::error::Error;

    let mut sut = UseCase::new(String::from("s3cret"), String::from("ORDINARY"), 1);
    let token: String;
    
    if let Ok(t) = sut.generate_token().await {
        token = t;
    } else {
        panic!("should generate a token with invalid subject");
    }

    sut = UseCase::new(String::from("s3cret"), String::from("ADMIN"), 1);   
    let validation_result = sut.validate_token(token).await;
    let err: Error;
    assert!(match validation_result {
        Ok(()) => panic!("must fails when invalid subject is expired"),
        Err(e) => {
            err = e;
            true
        }
    });

    assert!(
        match err.get_code() {
            INVALID_TOKEN_ERROR => true,
            _ => false
        }
    )
}

#[tokio::test]
async fn it_should_return_error_when_signature_is_invalid() {
    use crate::data::usecases::admin::UseCase;
    use crate::domain::usecases::admin::{AdminUseCase, INVALID_TOKEN_ERROR};
    use crate::domain::error::Error;



    let mut sut = UseCase::new(String::from("invalid"), String::from("ADMIN"), 1);
    let token: String;
    
    if let Ok(t) = sut.generate_token().await {
        token = t;
    } else {
        panic!("should generate a token with invalid secret");
    }

    sut = UseCase::new(String::from("s3cret"), String::from("ADMIN"), 1);   
    let validation_result = sut.validate_token(token).await;
    let err: Error;
    assert!(match validation_result {
        Ok(()) => panic!("must fails when invalid subject is expired"),
        Err(e) => {
            err = e;
            true
        }
    });

    assert!(
        match err.get_code() {
            INVALID_TOKEN_ERROR => true,
            _ => false
        }
    )
}

