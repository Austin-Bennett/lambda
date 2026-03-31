use std::ptr;
use libc::malloc;
use crate::desc::LME;

mod readwrite;

#[repr(C)]
pub struct CUstring {
    pub len: usize,
    pub str: *mut u8,
}


#[repr(C)]
pub struct CSlice<T> {
    pub len: usize,
    pub data: *mut T,
}

impl<T> CSlice<T> {

    //using this function is safe but using the result is not
    pub unsafe fn new_slice(len: usize) -> Self {
        Self{
            len,
            data: unsafe{ malloc(len * size_of::<T>()) as *mut T }
        }
    }

    pub unsafe fn from_vec(data: &Vec<T>) -> Self where T: Clone {
        let mut res = unsafe{ Self::new_slice(data.len()) };
        for (i, t) in data.iter().enumerate() {
            unsafe {
                ptr::write(res.data.add(i), t.clone())
            }
        }

        res
    }

    pub unsafe fn at_mut(&mut self, i: usize) -> *mut T {
        unsafe { self.data.add(i) }
    }
}


#[repr(C)]
pub struct CLMEFunctionLocation {
    pub name: CUstring,
    pub loc: u64,
}


#[repr(C)]
pub struct CLME {
    pub code_entry: u64,
    pub functions: CSlice<CLMEFunctionLocation>,
    pub code: CSlice<u8>,
}

impl CLME {
    pub fn from_lme(l: LME) -> Self {
        let mut res = Self{
            code_entry: l.code_entry,
            functions: unsafe{ CSlice::new_slice(l.functions.len()) },
            code: unsafe{ CSlice::from_vec(&l.code) }
        };

        for (i, f) in l.functions.iter().enumerate() {
            unsafe{
                ptr::write(res.functions.at_mut(i),
                    CLMEFunctionLocation{
                        name: us_from_bytes(f.name.as_bytes().as_ptr(), f.name.len()),
                        loc: f.loc
                    }
                )
            }
        }

        res
    }
}


unsafe extern "C" {
    pub fn us_from_bytes(bytes: *const u8, len: usize) -> CUstring;
}

