use SharedlibError;

/// The result type returned by functions in [sharedlib](index.html) to indicate success or failure.
pub type SharedlibResult<T> = Result<T, SharedlibError>;
