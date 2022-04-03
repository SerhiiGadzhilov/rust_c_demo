extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/lib/src/demo.cpp")
        .compile("libdemo.a");
}