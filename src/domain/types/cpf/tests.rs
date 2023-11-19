#[cfg(test)]
#[test]
fn it_should_check_if_cpfs_are_valids() {
    use super::CPF;

    let cpfs = vec![
        CPF::from_string(String::from("64020483051")).unwrap(),
        CPF::from_string(String::from("33556796074")).unwrap(),
        CPF::from_string(String::from("20963764080")).unwrap(),
        CPF::from_string(String::from("34782037082")).unwrap(),
        CPF::from_string(String::from("75932477083")).unwrap()
    ];

    for cpf in cpfs {
        assert!(cpf.is_valid())
    }
}

#[test]
fn it_should_check_if_cpfs_are_invalids() {
    use super::CPF;

    let cpfs = vec![
        CPF::from_string(String::from("64020483052")).unwrap(),
        CPF::from_string(String::from("33556796073")).unwrap(),
        CPF::from_string(String::from("11111111111")).unwrap(),
        CPF::from_string(String::from("00000000000")).unwrap(),
        CPF::from_string(String::from("75932477053")).unwrap()
    ];

    for cpf in cpfs {
        assert!(!cpf.is_valid())
    }
}

#[test]
fn it_should_return_error_when_string_given_is_shorter_then_expected() {
    use super::CPF;

    assert!(match CPF::from_string(String::from("123456")) {
        Err(()) => true,
        Ok(_) => false
    })
}

#[test]
fn it_should_return_error_when_string_given_is_bigger_then_expected() {
    use super::CPF;

    assert!(match CPF::from_string(String::from("123456789123")) {
        Err(()) => true,
        Ok(_) => false
    })
}

#[test]
fn it_should_return_error_when_string_has_non_numeric_characters() {
    use super::CPF;

    assert!(match CPF::from_string(String::from("7593f47702 3")) {
        Err(()) => true,
        Ok(_) => false
    })
}