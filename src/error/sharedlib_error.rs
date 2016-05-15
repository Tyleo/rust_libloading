use error::*;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

define_error!(
    #[derive(Debug)]
    #[doc="The error type returned when a function in [sharedlib](index.html) fails."]
    pub error SharedlibError {
        #[doc="Wraps the [LibraryClose](struct.LibraryClose.html) error type into a [SharedlibError](enum.SharedlibError.html)."]
        suberror LibraryClose,
        #[doc="Wraps the [LibraryFindSymbol](struct.LibraryFindSymbol.html) error type into a [SharedlibError](enum.SharedlibError.html)."]
        suberror LibraryFindSymbol,
        #[doc="Wraps the [LibraryOpen](struct.LibraryOpen.html) error type into a [SharedlibError](enum.SharedlibError.html)."]
        suberror LibraryOpen,
        #[doc="Wraps the [OsError](struct.OsError.html) error type into a [SharedlibError](enum.SharedlibError.html)."]
        suberror OsError,
        #[doc="Wraps the [OsErrorFailure](struct.OsErrorFailure.html) error type into a [SharedlibError](enum.SharedlibError.html)."]
        suberror OsErrorFailure
    }
);
