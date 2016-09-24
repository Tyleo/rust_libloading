use Data;
use Func;
use FuncArc;
use FuncUnsafe;
use Lib;
use LibArc;
use LibUnsafe;
use std::mem;
use std::cell::RefCell;
use std::ops::DerefMut;
use Symbol;
use test::examplelib::EXAMPLELIB;

#[test]
fn load_examplelib() {
    unsafe {
        Lib::new(EXAMPLELIB).unwrap();
    }
}

#[test]
fn add_2_numbers() {
    unsafe {
        let lib = Lib::new(EXAMPLELIB).unwrap();
        let add_u32s: Func<extern "C" fn(u32, u32) -> u32> = lib.find_func("add_u32s").unwrap();
        let result = add_u32s.get()(1, 1);
        assert_eq!(result, 2);
    }
}

#[test]
fn check_test_value() {
    unsafe {
        let lib = Lib::new(EXAMPLELIB).unwrap();
        let test_value: Data<u32> = lib.find_data("TEST_VALUE").unwrap();
        assert_eq!(*test_value.get(), 100);
    }
}

struct SafeDll<'a> {
    _lib: Lib,
    _func: RefCell<Option<Func<'a, extern "C" fn(u32, u32) ->  u32>>>,
}

#[test]
fn create_struct_safe() {
    unsafe {
        let lib = Lib::new(EXAMPLELIB).unwrap();
        let dll = SafeDll { _lib: lib, _func: RefCell::new(None), };
        let add_u32s: Func<extern "C" fn(u32, u32) -> u32> = dll._lib.find_func("add_u32s").unwrap();
        mem::replace(dll._func.borrow_mut().deref_mut(), Some(add_u32s));
    }
}

struct UnsafeDll {
    _lib: LibUnsafe,
    _func: FuncUnsafe<extern "C" fn(u32, u32) -> u32>,
}

#[test]
fn create_struct_unsafe() {
    unsafe {
        let lib = LibUnsafe::new(EXAMPLELIB).unwrap();
        let add_u32s: FuncUnsafe<extern "C" fn(u32, u32) -> u32> = lib.find_func("add_u32s").unwrap();
        let _dll = UnsafeDll { _lib: lib, _func: add_u32s, };
    }
}

struct ArcDll {
    _lib: LibArc,
    _func: FuncArc<extern "C" fn(u32, u32) -> u32>,
}

#[test]
fn create_struct_arc() {
    unsafe {
        let lib = LibArc::new(EXAMPLELIB).unwrap();
        let add_u32s: FuncArc<extern "C" fn(u32, u32) -> u32> = lib.find_func("add_u32s").unwrap();
        let _dll = ArcDll { _lib: lib, _func: add_u32s, };
    }
}
