extern "C" {
    pub fn demo_method();
}

fn main() {
    unsafe{
        demo_method();
    }
}
