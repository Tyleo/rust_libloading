use Data;
use Func;
use LibUnsafe;
use result::Result as R;
use std::ffi::OsStr;
use std::mem;

#[derive(Debug)]
pub struct Lib {
    inner: LibUnsafe,
}

impl Lib {
    pub fn new<P: AsRef<OsStr>>(filename: P) -> R<Self> {
        let inner = try!(LibUnsafe::new(filename));
        let result =
            Lib {
                inner: inner
            };
        Ok(result)
    }

    pub unsafe fn find_data<'a, T>(&'a self, symbol: &[u8]) -> R<Data<'a, T>> {
        let symbol_ptr = try!(self.inner.find_data::<T>(symbol));
        let symbol_ref = mem::transmute(symbol_ptr);
        let result = Data::new(symbol_ref);
        Ok(result)
    }

    pub unsafe fn find_func<'a, T>(&'a self, symbol: &[u8]) -> R<Func<'a, T>>
        where T: Copy {
        let func = try!(self.inner.find_func::<T>(symbol));
        let result = Func::new(func);
        Ok(result)
    }
}
