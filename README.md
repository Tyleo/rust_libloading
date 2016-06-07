# sharedlib [![Travis CI][tcii]][tci] [![Appveyor CI][acii]][aci]

[tcii]: https://travis-ci.org/tyleo/sharedlib.svg?branch=master
[tci]: https://travis-ci.org/tyleo/sharedlib
[acii]: https://ci.appveyor.com/api/projects/status/95wp614fd08o8rus?svg=true
[aci]: https://ci.appveyor.com/project/tyleo/sharedlib

A cross-platform shared library loader.

([crates.io][crate]) ([docs][docs])

[crate]: https://crates.io/crates/sharedlib
[docs]: https://tyleo.github.io/sharedlib/

Based on [libloading](https://crates.io/crates/libloading) by Simonas Kazlauskas.

sharedlib is a crate for loading shared libraries at runtime. This is a useful primitive for implementing other things like plugins. The advantage of this crate over other shared library crates is that it provides both lifetime-bound and ref-counted libraries, and it allows both functions and data to be loaded.

# Quickstart
To load a library you can use any of the `Lib`, `LibTracked`, or `LibUnsafe` `structs`. Each of these `struct`s provides different guarantees. For more information about the guarantees they provide see the [choosing your guarantees][choosing_your_guarantees] section in the docs. We use `Lib` for the examples below:

[choosing_your_guarantees]: https://tyleo.github.io/sharedlib/index.html#choosing-your-guarantees

### Calling a function in another library
```rust
unsafe {
    let path_to_lib = "examplelib.dll";
    let lib = try!(Lib::new(path_to_lib));
    let hello_world_symbol: Func<extern "C" fn()> = try!(lib.find_func("hello_world"));
    let hello_world = hello_world_symbol.get();
    hello_world();
}
```

### Accessing data in another library
```rust
unsafe {
    let path_to_lib = "examplelib.dll";
    let lib = try!(Lib::new(path_to_lib));
    let my_usize_symbol: Data<usize> = try!(lib.find_data("my_usize"));
    let my_usize = my_usize_symbol.get();
    assert_eq!(*my_usize, 0);
}
```

# Additional information
Plenty of additional information can be found in the [docs].
