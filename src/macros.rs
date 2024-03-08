/// Unwrap the result and log the error message in the console and in the log file without need to use the `?` operator
#[macro_export]
macro_rules! unwrap_log {
    ($expr:expr, $msg:expr) => {
        {
            use $crate::UnwrapLogError;
            use $crate::log_handle;

            match $expr {
                Some(val) => val,
                None => {
                    log_handle!("{} : Caller: `{}` Line {}", $msg, file!(), line!());
                    return Err(UnwrapLogError { msg: $msg });
                }
            }
        }
    };
    ($expr:expr, $msg:expr) => {
        {
            use $crate::UnwrapLogError;
            use $crate::log_handle;

            match $expr {
                Ok(val) => val,
                Err(why) => {
                    log_handle!("{}: {} : `{}` Line {}", $msg, why, file!(), line!());
                    return Err(UnwrapLogError { msg: $msg });
                }
            }
        }
    };
}

/// Log the error message in the console and in the log file
#[macro_export]
macro_rules! log_handle {
    ($($arg:tt)*) => {
        {
            use std::io::Write;

            // Obtener la hora actual y formatearla
            let current_time = chrono::Local::now();
            let error_msg = format!("[{}] Error: {}\n", current_time.format("%Y-%m-%d %H:%M:%S"), format!($($arg)*));

            // Imprimir el mensaje de error en la consola
            eprintln!("{error_msg}");

            // Guardar el mensaje de error en el archivo de logs
            let log_result = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt");

            // Si no se pudo abrir el archivo de log, imprimir el error en la consola
            if let Err(err) = &log_result {
                eprintln!("Error al abrir el archivo de logs: {err}");
            }

            // Si se pudo abrir el archivo de log, escribir el mensaje de error en el archivo
            if let Ok(mut file) = log_result {
                if let Err(err) = write!(file, "{error_msg}") {
                    eprintln!("Error al escribir en el archivo de logs: {err}");
                }
            }
        }
    };
}

#[cfg(test)]
mod test {
    use crate::UnwrapLogError;

    fn test_function() -> Result<i32, UnwrapLogError> {
        let option: Option<i32> = Some(5);
        let result = unwrap_log!(option, "Error message");
        Ok(result)
    }

    fn test_fail_function() -> Result<i32, UnwrapLogError> {
        let option: Option<i32> = None;
        let result = unwrap_log!(option, "Error message");
        Ok(result)
    }

    #[test]
    fn test_unwrap_log_macro_option() {
        let result = test_function();
        assert!(result.is_ok());
        assert_eq!(result, Ok(5));
    }

    #[test]
    fn test_unwrap_log_macro_option_fail() {
        let result = test_fail_function();
        assert!(result.is_err());
    }
}