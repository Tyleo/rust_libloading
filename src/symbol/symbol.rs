/// A symbol from a shared library.
pub trait Symbol<T> {
    /// Provides access to the data that this symbol references.
    ///
    /// # Unsafety
    /// If the data that this symbol references contains pointers to other things in the shared
    /// library, and `T: Clone`, we can obtain a clone of the data and use it to outlast the
    /// library. To prevent this, the return of this function should never be cloned.
    unsafe fn get(&self) -> T;
}
