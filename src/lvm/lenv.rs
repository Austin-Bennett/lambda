use crate::lvm::lheap::LHeap;

pub type LPTR = u64;
pub const L_NULL: LPTR = 0;

pub const LENV_STACK_SIZE: usize = 1_000_000;

pub struct LEnv {
    stack: [u8; LENV_STACK_SIZE],
    heap_manager: LHeap,
}



impl LEnv {
    fn _init(&mut self) {

    }
    pub fn new() -> Box<Self> {
        let mut res = Box::<Self>::new_uninit();

        unsafe{ (*res.as_mut_ptr())._init() }

        unsafe{ res.assume_init() }
    }
}