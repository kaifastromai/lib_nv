use nvcore::mir::Mir;

#[repr(C)]
//Context holds the current state of the kernel. It must be exposed to the C API.
pub struct Context<'a> {
    //Raw ptr to the Mir struct.
    pub mir: *mut Mir<'a>,
}

//implement context
impl Context<'_> {
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
}
#[repr(C)]
pub struct Event {
    pub id: u128,
}
