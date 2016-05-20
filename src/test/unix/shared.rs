use Func;
use Lib;
use Symbol;
use test::unix::LIBM;

#[test]
fn new_libm() {
    unsafe { Lib::new(LIBM).unwrap() };
}

#[test]
fn new_m() {
    unsafe { Lib::new("m").err().unwrap() };
}

#[test]
fn libm_ceil() {
    unsafe {
        let lib = Lib::new(LIBM).unwrap();
        let ceil: Func<extern fn(f64) -> f64> = lib.find_func("ceil").unwrap();
        assert_eq!(ceil.get()(0.45), 1.0);
    }
}

#[test]
fn libm_ceil0() {
    unsafe {
        let lib = Lib::new(LIBM).unwrap();
        let ceil: Func<extern fn(f64) -> f64> = lib.find_func("ceil\0").unwrap();
        assert_eq!(ceil.get()(0.45), 1.0);
    }
}
