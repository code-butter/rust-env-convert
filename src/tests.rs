use std::env;
use crate::{EnvVar, EnvVarConversionError, get_env_var};

#[test]
fn default_string() {
    let str: Result<String,_> = get_env_var("UNSET_ENV_VAR", "default var".to_owned()).into();
    assert_eq!(str.unwrap(), "default var".to_owned());
}

#[test]
fn default_int() {
    let int: Result<u32,_> = get_env_var("UNSET_ENV_VAR", "3".to_owned()).into();
    assert_eq!(int.unwrap(), 3);
}

#[test]
fn set_string() {
    env::set_var("STR_VAR", "Trogdor");
    let str: Result<String,_> = get_env_var("STR_VAR", "Burninator".to_owned()).into();
    assert_eq!(str.unwrap(), "Trogdor".to_owned());
}

#[test]
fn set_int() {
    env::set_var("INT_VAR", "5");
    let int: Result<u32,_> = get_env_var("INT_VAR", "3".to_owned()).into();
    assert_eq!(int.unwrap(), 5);
}

#[test]
fn invalid_int() {
    env::set_var("NON_INT_VAR", "WAS A MAN");
    let not_int: Result<u32, _> = get_env_var("NOT_INT_VAR", "".to_owned()).into();
    assert!(not_int.is_err());
}

struct TestStruct;
struct InvalidTestStruct;

impl From<EnvVar> for Result<TestStruct, EnvVarConversionError> {
    fn from(_: EnvVar) -> Self {
        Ok(TestStruct)
    }
}

impl From<EnvVar> for Result<InvalidTestStruct, EnvVarConversionError> {
    fn from(value: EnvVar) -> Self {
        Err(EnvVarConversionError {
            value: value.env_value,
            env_name: value.name,
            conversion_type: "InvalidTestStruct",
        })
    }
}

#[test]
fn valid_custom_conversion() {
    let inst: Result<TestStruct,_> = get_env_var("DONT_THROW_BBY", "".to_owned()).into();
    assert!(inst.is_ok());
}

#[test]
fn invalid_custom_conversion() {
    let inst: Result<InvalidTestStruct, _> = get_env_var("THROW_BBY", "".to_owned()).into();
    assert!(inst.is_err());
}