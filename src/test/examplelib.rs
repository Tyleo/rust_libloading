#[cfg(all(unix, not(target_os="macos")))]
pub const EXAMPLELIB: &'static str = "test/.build/libexamplelib.so";

#[cfg(all(unix, target_os="macos"))]
pub const EXAMPLELIB: &'static str = "test/.build/libexamplelib.dylib";

#[cfg(windows)]
pub const EXAMPLELIB: &'static str = "test/.build/examplelib.dll";
