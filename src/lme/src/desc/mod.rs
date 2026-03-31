pub mod reading;
pub mod writing;

pub use reading::*;

#[repr(C)]
pub struct LMEFunctionLocation {
    pub name: String,
    pub loc: u64,
}

#[repr(C)]
pub struct LME {
    pub code_entry: u64,
    pub functions: Vec<LMEFunctionLocation>,
    pub code: Vec<u8>,
}

