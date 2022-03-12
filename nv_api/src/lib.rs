use nvcore::mir::Mir;

pub struct ContextInternal {
    pub mir: Mir,
}
pub fn new_ctx() -> *mut ContextInternal {
    Box::into_raw(Box::new(ContextInternal { mir: Mir::new() }))
}
pub fn say_hello() {
    println!("Hello rust. Ready to really get started?");
}
#[cxx::bridge]
mod ffi {
    #[namespace = "nv"]
    extern "Rust" {
        type ContextInternal;
        pub fn new_ctx() -> *mut ContextInternal;
        pub fn say_hello();

    }
}
