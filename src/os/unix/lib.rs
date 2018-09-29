use error::*;
use os::unix::external;
use os::unix::OkOrDlerror;
use os::unix::RTLD_LAZY;
use util;
use std::ffi::CString;
use std::mem;
use std::path::Path;
use std::os::raw::c_char;
use std::os::raw::c_void;

#[derive(Debug)]
pub struct Lib {
    handle: *mut c_void
}

impl Lib {
    pub unsafe fn new<TPath>(path_to_lib: TPath) -> Result<Lib>
        where TPath: AsRef<Path> {
        let path_to_lib_c_str = CString::new(path_to_lib.as_ref().to_string_lossy().as_ref())
            .chain_err( || ErrorKind::LibraryOpen(path_to_lib.as_ref().to_path_buf()))?;
        let path_to_lib_c_ptr = path_to_lib_c_str.as_ptr();

        util::error_guard(
            || {
                let result = external::dlopen(path_to_lib_c_ptr, RTLD_LAZY);

                if result.is_null() {
                    None
                } else {
                    let lib =
                        Lib {
                            handle: result,
                        };
                    Some(lib)
                }
            }
        ).ok_or_dlerror("dlopen")
        .chain_err(
            || ErrorKind::LibraryOpen(path_to_lib.as_ref().to_path_buf())
        )
    }

    pub unsafe fn find<T, TStr>(&self, symbol_str: TStr) -> Result<*const T>
        where TStr: AsRef<str> {
        let symbol = symbol_str.as_ref();
        let symbol = symbol.as_ptr();
        let symbol = symbol as *const c_char;

        util::error_guard(
            || {
                let symbol = external::dlsym(self.handle, symbol);
                if symbol.is_null() {
                    None
                } else {
                    Some(mem::transmute(symbol))
                }
            }
        ).ok_or_dlerror("dlsym")
        .chain_err(
            || ErrorKind::LibraryFindSymbol(symbol_str.as_ref().to_string())
        )
    }
}

unsafe impl Send for Lib { }

unsafe impl Sync for Lib { }

impl Drop for Lib {
    fn drop(&mut self) {
        util::error_guard(
            || {
                if unsafe { external::dlclose(self.handle) } == 0 {
                    Some(())
                } else {
                    None
                }
            }
        ).ok_or_dlerror("dlclose")
        .chain_err(
            || ErrorKind::LibraryClose
        ).unwrap();
    }
}
