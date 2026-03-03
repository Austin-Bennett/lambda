use crate::lvm::lbc::*;
use crate::lvm::lexecutable::LExecutable;
use crate::lvm::lheap::LHeap;
use std::ptr;


/*
Pointers:
0x0-0x7 -> nullzone
0x8-0xf4247 -> stack
>= 0x1ffffffffffff -> heap

between stack and heap, constants?
*/
pub type LPTR = u64;
pub const L_NULL: LPTR = 0;

pub const LENV_STACK_SIZE: usize = 1_000_000;


/*
Registers:
 name  | size
=============
 ret   |  8
       |
 aux   |  8
       |
stack  |  8
bottom |  8
*/
#[derive(Clone)]
pub struct LProgramState {
    pub r_ret: u64,
    pub r_aux: u64,
    pub r_cmp: i64,
    pub r_stack: LPTR,
    pub r_bottom: LPTR,
    pub r_pc: LPTR,
    pub stack: [u8; LENV_STACK_SIZE],
}



pub struct LEnv {
    pub state: LProgramState,
    pub heap: LHeap,
    pub flags: u8,
}


impl LEnv {

    pub const FLAG_EXIT: u8 = 0b1;

    fn _init(&mut self) {
        self.state.r_stack = LENV_STACK_SIZE as u64;
        self.state.r_bottom = self.state.r_stack;
        self.flags = 0;
        self.heap = LHeap::new()
    }

    pub fn new() -> Box<Self> {
        let mut res = Box::<Self>::new_uninit();

        unsafe{ (*res.as_mut_ptr())._init() }

        unsafe{ res.assume_init() }
    }

    pub fn get_ptr(&mut self, ptr: LPTR) -> *mut u8 {
        if ptr < 0x8 { return ptr::null_mut(); }

        if ptr >= LHeap::MIN_HEAP_ADDR {
            self.heap.get_ptr(ptr).unwrap_or(ptr::null_mut())
        } else {
            unsafe{
                let ptr = ptr - 0x8;
                if ptr as usize >= self.state.stack.len() {
                    ptr::null_mut()
                } else {
                    self.state.stack.as_mut_ptr().add(ptr as usize)
                }
            }
        }
    }

    pub unsafe fn get_ptr_unchecked(&mut self, ptr: LPTR) -> *mut u8 {
        if ptr >= LHeap::MIN_HEAP_ADDR {
            self.heap.get_ptr(ptr).unwrap_or(ptr::null_mut())
        } else {
            unsafe{
                let ptr = ptr - 0x8;
                self.state.stack.as_mut_ptr().add(ptr as usize)
            }
        }
    }


    #[inline]
    pub fn set_register(&mut self, r: &Register, v: u64) {
        match r {
            Register::Ret => {
                self.state.r_ret = v;
            }
            Register::Cmp => {
                self.state.r_cmp = (v).cast_signed();
            }
            Register::Aux => {
                self.state.r_aux = v;
            }
            Register::Stack => {
                self.state.r_stack = v;
            }
            Register::Bottom => {
                self.state.r_bottom = v;
            }
            Register::Pc => {
                self.state.r_pc = v;
            }
        }
    }

    #[inline]
    pub fn get_register(&mut self, r: &Register) -> u64 {
        match r {
            Register::Ret => {
                self.state.r_ret
            }
            Register::Cmp => {
                self.state.r_cmp.cast_unsigned()
            }
            Register::Aux => {
                self.state.r_aux
            }
            Register::Stack => {
                self.state.r_stack
            }
            Register::Bottom => {
                self.state.r_bottom
            }
            Register::Pc => {
                self.state.r_pc
            }
        }
    }

    pub fn exec(&mut self, exec: &LExecutable) {
        self.flags = 0;
        self.state.r_stack = LENV_STACK_SIZE as u64;
        self.state.r_bottom = LENV_STACK_SIZE as u64;
        self.state.r_pc = exec.entry;
        loop {
            let i = &exec.instructions[self.state.r_pc as usize];

            if self.exec_instruction(i) {
                 unsafe{ self.state.r_pc = self.state.r_pc.unchecked_add(1) };
            }

            if self.flags & Self::FLAG_EXIT > 0 {
                return;
            }
        }

    }

    #[inline]
    fn push64(&mut self, v: u64) {
        unsafe {
            self.state.r_stack = self.state.r_stack.unchecked_sub(size_of::<u64>() as u64);
            * (self.state.stack.as_mut_ptr().add(self.state.r_stack as usize) as *mut u64) = v;
        }
    }

    #[inline]
    fn pop64(&mut self) -> u64 {
        unsafe {
            let v = *(self.state.stack.as_mut_ptr().add(self.state.r_stack as usize) as *mut u64);
            v
        }
    }

    fn exec_instruction(&mut self, i: &Instruction) -> bool {

        match i {
            Instruction::Push(r) => {
                let v = self.get_register(r);
                self.push64(v);
            }
            Instruction::Pop(r) => {
                let v = self.pop64();
                self.set_register(r, v);
            }
            Instruction::Exit => {
                self.flags |= Self::FLAG_EXIT;
            }

            Instruction::Mov(r, v) => {
                self.set_register(r, *v)
            }
            Instruction::MovPtrReg(p, r) => {
                let ptr = self.get_ptr(*p);
                unsafe {
                    match r {
                        Register::Ret => {
                            *(ptr as *mut u64) = self.state.r_ret;
                        }
                        Register::Cmp => {
                            *(ptr as *mut u64) = self.state.r_cmp.cast_unsigned();
                        }
                        Register::Aux => {
                            *(ptr as *mut u64) = self.state.r_aux;
                        }
                        Register::Stack => {
                            *(ptr as *mut u64) = self.state.r_stack;
                        }
                        Register::Bottom => {
                            *(ptr as *mut u64) = self.state.r_bottom;
                        }
                        Register::Pc => {
                            *(ptr as *mut u64) = self.state.r_pc;
                        }
                    }
                }
            }
            Instruction::MovRegPtr(r, p) => {
                let ptr = self.get_ptr(*p);
                unsafe {
                    match r {
                        Register::Ret => {
                            self.state.r_ret = *(ptr as *mut u64);
                        }
                        Register::Cmp => {
                            self.state.r_cmp = *(ptr as *mut i64);
                        }
                        Register::Aux => {
                            self.state.r_aux = *(ptr as *mut u64);
                        }
                        Register::Stack => {
                            self.state.r_stack = *(ptr as *mut u64);
                        }
                        Register::Bottom => {
                            self.state.r_bottom = *(ptr as *mut u64);
                        }
                        Register::Pc => {
                            self.state.r_pc = *(ptr as *mut u64);
                        }
                    }
                }
            }


            Instruction::Cmp(a, b) => {
                let v1 = self.get_register(a).cast_signed();
                let v2 = self.get_register(b).cast_signed();

                self.state.r_cmp = v1 - v2;
            }

            
            Instruction::Call(pc) => {
                self.push64(self.state.r_pc);
                self.state.r_pc = *pc;
                return false;
            }
            Instruction::Return => {
                self.state.r_pc = self.pop64()
            }
            

            Instruction::Jump(loc) => {
                self.state.r_pc = *loc;
            }
            Instruction::JumpLess(loc) => {
                //quick negative check
                if self.state.r_cmp < 0 {
                    self.state.r_pc = *loc
                }
            }
            Instruction::JumpGreater(loc) => {
                //greater than 0
                if self.state.r_cmp > 0 {
                    self.state.r_pc = *loc
                }
            }
            Instruction::JumpLessEq(loc) => {
                if self.state.r_cmp <= 0 {
                    self.state.r_pc = *loc
                }
            }
            Instruction::JumpGreaterEq(loc) => {
                if self.state.r_cmp >= 0 {
                    self.state.r_pc = *loc
                }
            }
            Instruction::JumpEq(loc) => {
                if self.state.r_cmp == 0 {
                    self.state.r_pc = *loc
                }
            }
            Instruction::JumpNEq(loc) => {
                if self.state.r_cmp != 0 {
                    self.state.r_pc = *loc
                }
            }

            Instruction::Add(a, b) => {
                let b = self.get_register(b).cast_signed();
                match a {
                    Register::Ret => {
                        self.state.r_ret = (self.state.r_ret.cast_signed() + b).cast_unsigned();
                    }
                    Register::Cmp => {
                        self.state.r_cmp += b;
                    }
                    Register::Aux => {
                        self.state.r_aux = (self.state.r_aux.cast_signed() + b).cast_unsigned();
                    }
                    Register::Stack => {
                        self.state.r_stack = (self.state.r_stack.cast_signed() + b).cast_unsigned();
                    }
                    Register::Bottom => {
                        self.state.r_bottom = (self.state.r_bottom.cast_signed() + b).cast_unsigned();
                    }
                    Register::Pc => {
                        self.state.r_pc = (self.state.r_pc.cast_signed() + b).cast_unsigned();
                    }
                }
            }
            Instruction::Sub(a, b) => {
                let b = self.get_register(b).cast_signed();
                match a {
                    Register::Ret => {
                        self.state.r_ret = (self.state.r_ret.cast_signed() - b).cast_unsigned();
                    }
                    Register::Cmp => {
                        self.state.r_cmp -= b;
                    }
                    Register::Aux => {
                        self.state.r_aux = (self.state.r_aux.cast_signed() - b).cast_unsigned();
                    }
                    Register::Stack => {
                        self.state.r_stack = (self.state.r_stack.cast_signed() - b).cast_unsigned();
                    }
                    Register::Bottom => {
                        self.state.r_bottom = (self.state.r_bottom.cast_signed() - b).cast_unsigned();
                    }
                    Register::Pc => {
                        self.state.r_pc = (self.state.r_pc.cast_signed() - b).cast_unsigned();
                    }
                }
            }
            Instruction::Mul(a, b) => {
                let b = self.get_register(b).cast_signed();
                match a {
                    Register::Ret => {
                        self.state.r_ret = (self.state.r_ret.cast_signed() * b).cast_unsigned();
                    }
                    Register::Cmp => {
                        self.state.r_cmp *= b;
                    }
                    Register::Aux => {
                        self.state.r_aux = (self.state.r_aux.cast_signed() * b).cast_unsigned();
                    }
                    Register::Stack => {
                        self.state.r_stack = (self.state.r_stack.cast_signed() * b).cast_unsigned();
                    }
                    Register::Bottom => {
                        self.state.r_bottom = (self.state.r_bottom.cast_signed() * b).cast_unsigned();
                    }
                    Register::Pc => {
                        self.state.r_pc = (self.state.r_pc.cast_signed() * b).cast_unsigned();
                    }
                }
            }
            Instruction::Div(a, b) => {
                let b = self.get_register(b).cast_signed();
                match a {
                    Register::Ret => {
                        self.state.r_ret = (self.state.r_ret.cast_signed() / b).cast_unsigned();
                    }
                    Register::Cmp => {
                        self.state.r_cmp /= b;
                    }
                    Register::Aux => {
                        self.state.r_aux = (self.state.r_aux.cast_signed() / b).cast_unsigned();
                    }
                    Register::Stack => {
                        self.state.r_stack = (self.state.r_stack.cast_signed() / b).cast_unsigned();
                    }
                    Register::Bottom => {
                        self.state.r_bottom = (self.state.r_bottom.cast_signed() / b).cast_unsigned();
                    }
                    Register::Pc => {
                        self.state.r_pc = (self.state.r_pc.cast_signed() / b).cast_unsigned();
                    }
                }
            }
        }

        true
    }
}