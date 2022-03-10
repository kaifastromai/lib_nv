use nvcore::mir::Mir;
use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn create_mir() -> *mut Mir {
    let mir = Mir::new();
    Box::into_raw(Box::new(mir))
}
/// # Safety
/// There should be no other references to the mir at this point!
#[no_mangle]
pub unsafe extern "C" fn free_mir(mir: *mut Mir) {
    Box::from_raw(mir);
}
