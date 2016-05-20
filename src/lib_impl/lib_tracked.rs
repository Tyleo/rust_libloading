use DataTracked;
use FuncTracked;
use LibUnsafe;
use SharedlibResult as R;
use std::path::Path;

/// A shared library which which allows a user-provided ref-counting implementation to track its [Symbols](trait.Symbol.html).
#[derive(Clone, Debug)]
pub struct LibTracked<TLib> {
    inner: TLib,
}

impl <TLib> LibTracked<TLib>
    where TLib: AsRef<LibUnsafe> + Clone + From<LibUnsafe> {
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
    /// # use sharedlib::LibTracked;
    /// # use sharedlib::LibUnsafe;
    /// # use sharedlib::SharedlibResult as R;
    /// # use std::rc::Rc;
    /// type LibRc = LibTracked<Rc<LibUnsafe>>;
    /// # fn test() -> R<()> {
    /// let lib = try!(unsafe { LibRc::new("examplelib.dll") });
    /// # Ok(())
    /// # }
    /// ```
    pub unsafe fn new<TPath>(path_to_lib: TPath) -> R<Self>
        where TPath: AsRef<Path> {
        let lib_unsafe = try!(LibUnsafe::new(path_to_lib));
        let inner = TLib::from(lib_unsafe);
        let result =
            LibTracked {
                inner: inner
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
    /// # use sharedlib::DataTracked;
    /// # use sharedlib::LibTracked;
    /// # use sharedlib::LibUnsafe;
    /// # use sharedlib::SharedlibResult as R;
    /// # use sharedlib::Symbol;
    /// # use std::rc::Rc;
    /// type DataRc<T> = DataTracked<T, Rc<LibUnsafe>>;
    /// # type LibRc = LibTracked<Rc<LibUnsafe>>;
    /// # fn test() -> R<()> {
    /// # let lib = try!(unsafe { LibRc::new("examplelib.dll") });
    /// let some_usize: DataRc<usize> = try!(unsafe { lib.find_data("some_usize") });
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Finding data with maximum performance:
    ///
    /// ``` no_run
    /// # use sharedlib::DataTracked;
    /// # use sharedlib::LibTracked;
    /// # use sharedlib::LibUnsafe;
    /// # use sharedlib::SharedlibResult as R;
    /// # use sharedlib::Symbol;
    /// # use std::rc::Rc;
    /// type DataRc<T> = DataTracked<T, Rc<LibUnsafe>>;
    /// # type LibRc = LibTracked<Rc<LibUnsafe>>;
    /// # fn test() -> R<()> {
    /// # let lib = try!(unsafe { LibRc::new("examplelib.dll") });
    /// let some_usize: DataRc<usize> = try!(unsafe { lib.find_data("some_usize\0") });
    /// # Ok(())
    /// # }
    /// ```
    pub unsafe fn find_data<T, TStr>(&self, symbol: TStr) -> R<DataTracked<T, TLib>>
        where TStr: AsRef<str> {
        let lib = self.inner.as_ref();
        let symbol_ptr = try!(lib.find_data::<T, TStr>(symbol));
        let result = DataTracked::new(symbol_ptr, self.inner.clone());
        Ok(result)
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
    /// # use sharedlib::FuncTracked;
    /// # use sharedlib::LibTracked;
    /// # use sharedlib::LibUnsafe;
    /// # use sharedlib::SharedlibResult as R;
    /// # use sharedlib::Symbol;
    /// # use std::rc::Rc;
    /// type FuncRc<T> = FuncTracked<T, Rc<LibUnsafe>>;
    /// # type LibRc = LibTracked<Rc<LibUnsafe>>;
    /// # fn test() -> R<()> {
    /// # let lib = try!(unsafe { LibRc::new("examplelib.dll") });
    /// let some_func: FuncRc<fn()> = try!(unsafe { lib.find_func("some_func") });
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Finding a function with maximum performance:
    ///
    /// ``` no_run
    /// # use sharedlib::FuncTracked;
    /// # use sharedlib::LibTracked;
    /// # use sharedlib::LibUnsafe;
    /// # use sharedlib::SharedlibResult as R;
    /// # use sharedlib::Symbol;
    /// # use std::rc::Rc;
    /// type FuncRc<T> = FuncTracked<T, Rc<LibUnsafe>>;
    /// # type LibRc = LibTracked<Rc<LibUnsafe>>;
    /// # fn test() -> R<()> {
    /// # let lib = try!(unsafe { LibRc::new("examplelib.dll") });
    /// let some_func: FuncRc<fn()> = try!(unsafe { lib.find_func("some_func\0") });
    /// # Ok(())
    /// # }
    /// ```
    pub unsafe fn find_func<T, TStr>(&self, symbol: TStr) -> R<FuncTracked<T, TLib>>
        where T: Copy,
              TStr: AsRef<str> {
        let lib = self.inner.as_ref();
        let func = try!(lib.find_func::<T, TStr>(symbol));
        let result = FuncTracked::new(func, self.inner.clone());
        Ok(result)
    }
}
