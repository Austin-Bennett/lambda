use crate::lvm::lvm_exception::LVMException;




/*
Pointers:
0x0-0x7 -> nullzone
0x8-0xf4247 -> stack
>= 0x1ffffffffffff -> heap

between stack and heap, constants?
*/
pub type LPTR = usize;
pub const L_NULL: LPTR = 0;

pub const LENV_STACK_SIZE: usize = 1_000_000;


#[derive(Clone)]
pub struct LEnvState {
    r_ret: u64,
    r_stack: LPTR,
    r_bottom: LPTR,
    stack: [u8; LENV_STACK_SIZE],
}



pub struct LEnv {
    state: LEnvState,
}



impl LEnv {
    fn _init(&mut self) {
        self.state.r_stack = LENV_STACK_SIZE as LPTR;
        self.state.r_bottom = self.state.r_stack;
    }



    pub fn new() -> Box<Self> {
        let mut res = Box::<Self>::new_uninit();

        unsafe{ (*res.as_mut_ptr())._init() }

        unsafe{ res.assume_init() }
    }
}