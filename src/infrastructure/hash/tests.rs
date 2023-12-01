#[cfg(test)]
#[test]
fn it_should_generate_diferents_passwords_to_the_same_entry() {
    use crate::data::usecases::user::protocols::hash::Hash;

    use super::Hasher;

    let hasher = Hasher::new(String::from("_pepper_"), 4);
    let plaintext = "password";

    match hasher.run(String::from(plaintext)) {
        Ok(encrypted) => print!("{}", encrypted),
        Err(error) => println!("{}", error)
    }

    let hash1 = hasher.run(String::from(plaintext)).unwrap();
    let hash2 = hasher.run(String::from(plaintext)).unwrap();
    let hash3 = hasher.run(String::from(plaintext)).unwrap();

    assert!(
        hash1 != hash2 && hash1 != hash3 && hash2 != hash3
    )
}

#[test]
fn it_should_return_an_error_if_passwords_does_not_match() {
    use crate::data::usecases::user::protocols::hash::Hash;

    use super::Hasher;

    let hasher = Hasher::new(String::from("_pepper_"), 4);
    let plaintext = "password";

    match hasher.run(String::from(plaintext)) {
        Ok(encrypted) => print!("{}", encrypted),
        Err(error) => println!("{}", error)
    }

    let hash = hasher.run(String::from(plaintext)).unwrap();

    assert!(!hasher.verify(String::from("another"), hash).unwrap());
}

#[test]
fn it_should_return_true_if_passwords_match() {
    use crate::data::usecases::user::protocols::hash::Hash;

    use super::Hasher;

    let hasher = Hasher::new(String::from("_pepper_"), 4);
    let plaintext = "password";

    match hasher.run(String::from(plaintext)) {
        Ok(encrypted) => print!("{}", encrypted),
        Err(error) => println!("{}", error)
    }

    let hash = hasher.run(String::from(plaintext)).unwrap();

    assert!(hasher.verify(String::from(plaintext), hash).unwrap());
}