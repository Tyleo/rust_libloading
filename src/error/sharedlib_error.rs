use error::*;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

define_error!(
    #[derive(Debug)]
    #[doc="An error returned when a function in [sharedlib](index.html) fails. Other errors are wrapped in this enum before being returned."]
    pub error SharedlibError {
        #[doc="Wraps a `LibraryClose` error in a [SharedlibError](enum.SharedlibError.html)."]
        suberror LibraryClose,
        #[doc="Wraps a `LibraryFindSymbol` error in a [SharedlibError](enum.SharedlibError.html)."]
        suberror LibraryFindSymbol,
        #[doc="Wraps a `LibraryOpen` error in a [SharedlibError](enum.SharedlibError.html)."]
        suberror LibraryOpen,
        #[doc="Wraps a `OsError` error in a [SharedlibError](enum.SharedlibError.html)."]
        suberror OsError,
        #[doc="Wraps a `OsErrorFailure` error in a [SharedlibError](enum.SharedlibError.html)."]
        suberror OsErrorFailure
    }
);
