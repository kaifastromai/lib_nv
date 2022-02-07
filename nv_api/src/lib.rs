use mir::Mir;

pub mod mir;

#[repr(C)]
//Context holds the current state of the kernel. It must be exposed to the C API.
pub struct Context {
    //Raw ptr to the Mir struct.
    pub mir: *mut Mir,
}

//implement context
impl Context {
    #[no_mangle]
    pub extern "C" fn create() -> Self {
        Context {
            mir: Box::into_raw(Box::new(Mir::new())),
        }
    }
    #[no_mangle]
    pub extern "C" fn destroy(ctx: &mut Self) {
        unsafe {
            Box::from_raw(ctx.mir);
        }
    }
    #[no_mangle]
    pub extern "C" fn say_hello(&self) {
        unsafe {
            (*self.mir).say_hello();
        }
    }
}
#[repr(C)]
pub struct Event {
    pub id: u128,
}