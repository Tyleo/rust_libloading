use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::os_error_failure as string;

/// An error which occurs when an operating system function fails and no information is available.
#[derive(Debug)]
pub struct OsErrorFailure {
    function_called: String,
}

impl OsErrorFailure {
    /// Creates a new [OsErrorFailure](struct.OsErrorFailure.html).
    pub fn new(function_called: String) -> Self {
        OsErrorFailure {
            function_called: function_called,
        }
    }
}

impl Display for OsErrorFailure {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            string::display_1(),
            self.function_called,
            string::display_2(),
        )
    }
}

impl Error for OsErrorFailure {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
