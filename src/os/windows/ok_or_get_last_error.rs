use error::*;
use kernel32;
use std::io;

pub trait OkOrGetLastError<T> {
    fn ok_or_get_last_error<TStr>(self, function: TStr) -> Result<T>
        where TStr: AsRef<str>;
}

impl <T> OkOrGetLastError<T> for Option<T> {
    fn ok_or_get_last_error<TStr>(self, function: TStr) -> Result<T>
        where TStr: AsRef<str> {
        match self {
            Some(some) => Ok(some),
            None => {
                match unsafe { kernel32::GetLastError() } {
                    0 => {
                        Err(ErrorKind::OsErrorFailure(function.as_ref().to_string()).into())
                    },
                    error_code => {
                        let cause = io::Error::from_raw_os_error(error_code as i32);
                        Err(ErrorKind::OsError(cause.to_string(), function.as_ref().to_string()).into())
                    },
                }
            },
        }
    }
}
