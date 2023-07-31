use std::env;
use std::env::VarError;
use crate::{EnvVar, EnvVarConversionError, get_default_env_var, get_env_var};

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
fn default_string() {
    let str: Result<String,_> = get_default_env_var("UNSET_ENV_VAR", "default var".to_owned()).into();
    assert_eq!(str.unwrap(), "default var".to_owned());
}

#[test]
fn default_int() {
    let int: Result<u32,_> = get_default_env_var("UNSET_ENV_VAR", "3".to_owned()).into();
    assert_eq!(int.unwrap(), 3);
}

#[test]
fn set_string() {
    let key = "STR_VAR";
    let value = "Trogdor";
    let default_value = "Burninator";
    env::set_var(key, value);
    let result: Result<String,_>= get_default_env_var(key, default_value).into();
    let str: String = get_default_env_var(key, default_value).into();
    assert_eq!(result.unwrap(), value.to_owned());
    assert_eq!(str, value.clone().to_owned());

}

#[test]
fn set_int() {
    env::set_var("INT_VAR", "5");
    let int: Result<u32,_> = get_default_env_var("INT_VAR", "3".to_owned()).into();
    assert_eq!(int.unwrap(), 5);
}

#[test]
fn invalid_int() {
    env::set_var("NON_INT_VAR", "WAS A MAN");
    let not_int: Result<u32, _> = get_default_env_var("NOT_INT_VAR", "".to_owned()).into();
    assert!(not_int.is_err());
}

#[test]
fn valid_default_custom_conversion() {
    let inst: Result<TestStruct,_> = get_default_env_var("DONT_THROW_BBY", "".to_owned()).into();
    assert!(inst.is_ok());
}

#[test]
fn invalid_default_custom_conversion() {
    let inst: Result<InvalidTestStruct, _> = get_default_env_var("THROW_BBY", "".to_owned()).into();
    assert!(inst.is_err());
}

#[test]
fn not_exist_returns_error() {
    let result = get_env_var("NONEXISTENT_VAR");
    assert!(result.is_err())
}

#[test]
fn exist_returns_value() -> Result<(), VarError> {
    let key = "EXISTENT_VAR";
    env::set_var(key, "3");
    let var: Result<usize,_> = get_env_var(key)?.into();
    assert_eq!(var.unwrap(), 3);
    Ok(())
}