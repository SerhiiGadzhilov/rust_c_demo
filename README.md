# rust_c_demo
This is a demo project to demonstrate how to call C code from RUST

Rust doesn't support calling C++ functions because  symbol mangling in C++ is implementation defined. Rust supports calling C functions hence to call C++ from Rust it should be passed as C using extern "C".

Calling C code from Rust consists of two major parts:
1. Wrapping the C API
2. Building C or C++ code
3. Call C code from Rust

## Wrapping the C API
It is necessary to define in Rust what data types and function signatures exist in the linked code. In Rust, it is necessary to either manually translate these definitions to Rust, or use a tool to generate these definitions. https://github.com/rust-lang/rust-bindgen is used for automatic generation of the definitions. For simple code like in this demo it could be done manually. 

```
extern "C" {
   pub fn demo_method();
}
```
This statement defines the signature of a function that uses the C ABI, called demo_method. By defining the signature without defining the body of the function, the definition of this function will need to be provided elsewhere, or linked into the final library or binary from a static library.

## Building C or C++ code

The file build.rs is used to compile C/C++ code into a library using the cc crate.
The build.rs script is executed after dependencies of your project have been built, but before your project is built. The cc crate provides Rust interface to the C compiler provided by the host.

## Call C code from Rust

Calling C code from Rust falls under FFI (Foreign Function Interface) and is unsafe by nature, as it's crossing Rust boundary and all the checks that come with it.
Block unsafe should be used to call C code from Rust:
```
unsafe {
    demo_method();
}
```

## For more information
https://docs.rust-embedded.org/book/interoperability/c-with-rust.html
https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
