use Data;
use test::examplelib::EXAMPLELIB;
use Func;
use Lib;
use Symbol;

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
