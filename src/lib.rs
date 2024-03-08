mod macros;

use std::fmt;
use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum UnwrapErrors {
    #[error(transparent)]
    Unwrap(#[from] UnwrapLogError),

    #[error("Error: {0}")]
    StaticStr(&'static str),
}

#[derive(Error, Debug, PartialEq)]
pub struct UnwrapLogError {
    pub msg: &'static str,
}

impl fmt::Display for UnwrapLogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnwrapLogError: {}", self.msg)
    }
}

///
pub trait UnwrapLog<T, E> {
    fn unwrap_log(self, msg: &'static str, module: &str, line: u32) -> Result<T, E>;
}

impl<T, E> UnwrapLog<T, E> for Option<T>
    where
        T: Default,
        E: Error + From<UnwrapLogError>,
{
    fn unwrap_log(self, msg: &'static str, module: &str, line: u32) -> Result<T, E> {
        self.map_or_else(move || {
            log_handle!("{msg} : Caller: `{module}` Line {line}");
            Err( E::from(UnwrapLogError { msg } ))
        }, move |t| Ok(t))
    }
}

impl<T, E> UnwrapLog<T, E> for Result<T, E>
    where
        T: Default,
        E: Error + From<UnwrapLogError>,
{
    fn unwrap_log(self, msg: &'static str, module: &str, line: u32) -> Result<T, E> {
        match self {
            Ok(t) => Ok(t),
            Err(why) => {
                log_handle!("{msg}: {why} : `{module}` Line {line}");
                Err(E::from(UnwrapLogError { msg} ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Importa todas las funciones del módulo actual

    #[test]
    fn test_unwrap_log_option() {
        let option: Option<i32> = Some(5);
        let result : Result<i32, UnwrapErrors> = option.unwrap_log("Error message", module_path!(), line!());
        assert_eq!(result, Ok(5));
    }

    #[test]
    fn test_unwrap_log_result() {
        let result: Result<i32, UnwrapErrors> = Ok(5);
        let test_result = result.unwrap_log("Error message", module_path!(), line!());
        assert_eq!(test_result, Ok(5));
    }

    #[test]
    fn test_unwrap_log_error() {
        let error: Result<i32, UnwrapErrors> = Err(UnwrapErrors::Unwrap(UnwrapLogError { msg: "Error message" }));
        let test_result = error.unwrap_log("Error message", module_path!(), line!());
        assert_eq!(test_result, Err(UnwrapErrors::Unwrap(UnwrapLogError { msg: "Error message" })));
    }

    #[test]
    fn test_unwrap_log_none() {
        let option: Option<i32> = None;
        let result : Result<i32, UnwrapErrors> = option.unwrap_log("Value is None", module_path!(), line!());
        assert_eq!(result, Err(UnwrapErrors::Unwrap(UnwrapLogError { msg: "Value is None" })));
    }
}