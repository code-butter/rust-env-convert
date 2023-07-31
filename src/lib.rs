#[cfg(test)]
mod tests;

use std::any::type_name;
use std::convert::Infallible;
use std::env;
use std::env::VarError;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{Debug, Display, Formatter};

pub struct EnvVar {
    name: String,
    env_value: String
}

macro_rules! impl_from_envvar {
    ($($t:ty),+ $(,)?) => {
        $(
            impl From<EnvVar> for Result<$t, EnvVarConversionError> {
                fn from(value: EnvVar) -> Self {
                     match value.env_value.parse() {
                         Ok(v) => Ok(v),
                         Err(_) => Err(EnvVarConversionError {
                             value: value.env_value,
                             env_name: value.name,
                             conversion_type: type_name::<$t>(),
                         })
                     }
                }
            }
        )+
    };
}

impl From<EnvVar> for Result<String, Infallible> {
    fn from(value: EnvVar) -> Self {
        Ok(value.env_value)
    }
}

impl From<EnvVar> for String {
    fn from(value: EnvVar) -> Self { value.env_value }
}

impl_from_envvar!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, usize, isize);
impl_from_envvar!(f32, f64);

#[derive(Debug)]
pub struct EnvVarConversionError {
    pub value: String,
    pub env_name: String,
    pub conversion_type: &'static str
}

impl Display for EnvVarConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Environment conversion error. Environment variable `{}` with value '{}' count not be converted to `{}`", self.env_name, self.value, self.conversion_type)
    }
}

impl Error for EnvVarConversionError {}

pub fn get_env_var<N: AsRef<OsStr>>(name: N) -> Result<EnvVar, VarError> {
    Ok(EnvVar {
        name: name.as_ref().to_string_lossy().into_owned(),
        env_value: env::var(name)?
    })
}

pub fn get_default_env_var<N: AsRef<OsStr>, S: AsRef<str>>(name: N, default: S) -> EnvVar {
    let name_str = name.as_ref().to_string_lossy().into_owned();
    let value = env::var(name).unwrap_or_else(|_| default.as_ref().to_owned());
    EnvVar {
        name: name_str,
        env_value: value,
    }
}
