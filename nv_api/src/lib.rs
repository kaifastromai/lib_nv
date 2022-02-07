use mir::Mir;

pub mod mir;

#[repr(C)]
pub struct Event {
    pub id: u128,
}

#[no_mangle]
pub extern "C" fn mir_new() -> *mut Mir {
    let mir = Box::new(mir::Mir::new());
    Box::into_raw(mir)
}
