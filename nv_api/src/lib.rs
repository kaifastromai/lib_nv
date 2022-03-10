use nvcore::mir::Mir;

pub struct ContextInternal {
    pub mir: Mir,
}
pub fn new_ctx() -> *mut ContextInternal {
    Box::into_raw(Box::new(ContextInternal { mir: Mir::new() }))
}
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type ContextInternal;
        pub fn new_ctx() -> *mut ContextInternal;
    }
}
