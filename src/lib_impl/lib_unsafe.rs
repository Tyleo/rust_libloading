use DataUnsafe;
use error::*;
use FuncUnsafe;
use os::uses::Lib as InnerLib;
use std::mem;
use std::path::Path;
use util;

/// A shared library which does not track its [Symbols](trait.Symbol.html).
/// The inner library may be dropped at any time, even if it has loose symbols.
#[derive(Debug)]
pub struct LibUnsafe {
    inner: InnerLib,
}

impl LibUnsafe {
    /// Opens a shared library at the specified path.
    /// The path is used in conjunction with platform specific shared library search paths to determine which shared library will be opened. Search paths vary across environments and are not discussed in this documentation. The behavior of this function when it is called on the same path multiple times is platform specific. If you wish to obtain multiple copies of a library within the same code base in a platform generic way, you should load the symbol once in a [LibTracked](struct.LibTracked.html) like [LibArc](struct.LibArc.html), or [LibRc](struct.LibRc.html), and pass around copies of the [LibTracked](struct.LibTracked.html).
    ///
    /// # Errors
    /// A `LibraryOpen` error will be returned as a [SharedlibError](enum.SharedlibError.html) variant if there is a problem opening the shared library. For instance, this may happen if the shared library is not at the path specified.
    ///
    /// # Safety
    /// Opening a shared library may execute code within the shared library. Since it is impossible to guarantee that the code witin the shared library is safe, the call to new is unsafe.
    ///
    /// # Examples
    /// ``` no_run
    /// # use sharedlib::error::*;
    /// # use sharedlib::LibUnsafe;
    /// # fn test() -> Result<()> {
    /// let lib = try!(unsafe { LibUnsafe::new("examplelib.dll") });
    /// # Ok(())
    /// # }
    /// ```
    pub unsafe fn new<TPath>(path_to_lib: TPath) -> Result<Self>
        where TPath: AsRef<Path> {
        let inner = try!(InnerLib::new(path_to_lib));
        let result =
            LibUnsafe {
                inner: inner,
            };
        Ok(result)
    }

    /// Finds and returns a data symbol within the shared library.
    /// By passing in a null terminated string, an extra allocation may be avoided.
    ///
    /// # Errors
    /// A `LibraryFindSymbol` error will be returned as a [SharedlibError](enum.SharedlibError.html) variant if there is a problem finding the symbol. For instance, this may happen if the shared library does not contain the requested symbol.
    ///
    /// # Safety
    /// This function is not type safe so there is no guarntee that `T` is really the type of the symbol. Using a symbol as a `T` when the symbol is not really of type `T` causes undefined behavior.
    ///
    /// # Examples
    /// Finding data convieniently:
    ///
    /// ``` no_run
    /// # use sharedlib::DataUnsafe;
    /// # use sharedlib::error::*;
    /// # use sharedlib::LibUnsafe;
    /// # use sharedlib::Symbol;
    /// # fn test() -> Result<()> {
    /// # let lib = try!(unsafe { LibUnsafe::new("examplelib.dll") });
    /// let some_usize: DataUnsafe<usize> = try!(unsafe { lib.find_data("some_usize") });
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Finding data with maximum performance:
    ///
    /// ``` no_run
    /// # use sharedlib::DataUnsafe;
    /// # use sharedlib::error::*;
    /// # use sharedlib::LibUnsafe;
    /// # use sharedlib::Symbol;
    /// # fn test() -> Result<()> {
    /// # let lib = try!(unsafe { LibUnsafe::new("examplelib.dll") });
    /// let some_usize: DataUnsafe<usize> = try!(unsafe { lib.find_data("some_usize\0") });
    /// # Ok(())
    /// # }
    /// ```
    pub unsafe fn find_data<T, TStr>(&self, symbol: TStr) -> Result<DataUnsafe<T>>
        where TStr: AsRef<str> {
        match util::null_terminate(&symbol) {
            Some(symbol) => self.inner.find(symbol),
            None => self.inner.find(symbol),
        }
    }

    /// Finds and returns a function symbol within the shared library.
    /// By passing in a null terminated string, an extra allocation may be avoided.
    ///
    /// # Errors
    /// A `LibraryFindSymbol` error will be returned as a [SharedlibError](enum.SharedlibError.html) variant if there is a problem finding the symbol. For instance, this may happen if the shared library does not contain the requested symbol.
    ///
    /// # Safety
    /// This function is not type safe so there is no guarntee that `T` is really the type of the symbol. Using a symbol as a `T` when the symbol is not really of type `T` causes undefined behavior.
    ///
    /// # Examples
    /// Finding a function convieniently:
    ///
    /// ``` no_run
    /// # use sharedlib::error::*;
    /// # use sharedlib::FuncUnsafe;
    /// # use sharedlib::LibUnsafe;
    /// # use sharedlib::Symbol;
    /// # fn test() -> Result<()> {
    /// # let lib = try!(unsafe { LibUnsafe::new("examplelib.dll") });
    /// let some_func: FuncUnsafe<fn()> = try!(unsafe { lib.find_func("some_func") });
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Finding a function with maximum performance:
    ///
    /// ``` no_run
    /// # use sharedlib::error::*;
    /// # use sharedlib::FuncUnsafe;
    /// # use sharedlib::LibUnsafe;
    /// # use sharedlib::Symbol;
    /// # fn test() -> Result<()> {
    /// # let lib = try!(unsafe { LibUnsafe::new("examplelib.dll") });
    /// let some_func: FuncUnsafe<fn()> = try!(unsafe { lib.find_func("some_func\0") });
    /// # Ok(())
    /// # }
    /// ```
    pub unsafe fn find_func<T, TStr>(&self, symbol: TStr) -> Result<FuncUnsafe<T>>
        where T: Copy,
              TStr: AsRef<str> {
        let func =
            match util::null_terminate(&symbol) {
                Some(symbol) => try!(self.inner.find::<u8, _>(symbol)),
                None => try!(self.inner.find::<u8, _>(symbol)),
            };
        let func_ref = &func;
        let result: T = mem::transmute_copy(func_ref);
        Ok(result)
    }
}
