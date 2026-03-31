use std::ffi::{c_char, CString};
use std::ptr;
use std::ptr::slice_from_raw_parts;
use libc::malloc;
use crate::desc::LME;
use crate::ffi::CLME;

unsafe extern "C" {
    pub fn get_last_err() -> *const c_char;
    pub fn set_err(msg: *const c_char);
}

#[unsafe(no_mangle)]
pub extern "C" fn lme_from_memory(data: *const u8, len: usize) -> *mut CLME {
    let slice = slice_from_raw_parts(data, len);
    let lme = LME::from_memory(unsafe{ &*slice });
    let lme = match lme {
        Ok(v) => v,
        Err(e) => {
            let msg = e.to_string();
            let cstr = CString::new(msg).map(|v| v.into_raw()).unwrap_or(ptr::null_mut());

            unsafe { set_err(cstr) }

            let cstr = unsafe{ CString::from_raw(cstr) };

            return ptr::null_mut();
        }
    };

    let res = unsafe{ malloc(size_of::<CLME>()) as *mut CLME };

    unsafe{ ptr::write(res, CLME::from_lme(lme)) };


    res
}