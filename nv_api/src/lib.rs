use nvcore::mir::Mir;

pub struct ContextInternal {
    pub mir: Mir,
}
pub fn new_ctx() -> *mut ContextInternal {
    Box::into_raw(Box::new(ContextInternal { mir: Mir::new() }))
}
pub fn say_hello() {}
#[cxx::bridge]
mod ffi {
    #[namespace = "nv_rust"]
    extern "Rust" {
        type ContextInternal;
        pub fn new_ctx() -> *mut ContextInternal;
        pub fn say_hello();
    }
}
