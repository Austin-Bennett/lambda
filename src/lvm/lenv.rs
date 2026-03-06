use std::alloc::{alloc, Layout};
use std::io::{stdin, stdout, Write};
use std::ops::{Add, Neg};
use crate::lvm::lbc::*;
use crate::lvm::lexecutable::LExecutable;
use crate::lvm::lheap::LHeap;
use std::{mem, ptr};


/*
Pointers:
0x0-0x7 -> nullzone
0x8-0xf4247 -> stack
>= 0x1ffffffffffff -> heap

between stack and heap, constants?
*/
pub type LPTR = u64;
pub const L_NULL: LPTR = 0;

pub const LENV_STACK_SIZE: usize = 8_000_000;


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
#[derive(Clone, Default)]
pub struct LProgramState {
    pub r_ret: u64,
    pub r_aux: u64,
    pub r_cmp: i64,
    pub r_stack: LPTR,
    pub r_bottom: LPTR,
    pub r_pc: LPTR,
    pub stack: *mut u8,
}



pub struct LEnv {
    pub state: Box<LProgramState>,
    pub heap: LHeap,
    pub flags: u8,
}


impl LEnv {

    pub const FLAG_EXIT: u8 = 0b1;
    pub const FLAG_DEBUG: u8 = 0b10;



    pub fn new() -> Self {
        Self{
            state: Box::new({
                LProgramState{
                    stack: unsafe{ alloc(Layout::array::<u8>(LENV_STACK_SIZE).unwrap()) },
                    r_stack: LENV_STACK_SIZE as u64 + 0x8,
                    r_bottom: LENV_STACK_SIZE as u64 + 0x8,
                    ..Default::default()
                }
            }),
            heap: LHeap::new(),
            flags: 0
        }
    }

    pub fn get_ptr(&mut self, ptr: LPTR) -> *mut u8 {
        if ptr < 0x8 { return ptr::null_mut(); }

        if ptr >= LHeap::MIN_HEAP_ADDR {
            self.heap.get_ptr(ptr).unwrap_or(ptr::null_mut())
        } else {
            unsafe{
                let ptr = ptr - 0x8;
                if ptr as usize >= LENV_STACK_SIZE {
                    ptr::null_mut()
                } else {
                    self.state.stack.add(ptr as usize)
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
                self.state.stack.add(ptr as usize)
            }
        }
    }


    
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

    pub fn exec(&mut self, exec: &LExecutable, flags: u8) {
        self.flags = flags;

        self.state.r_stack = 0x8 + LENV_STACK_SIZE as u64;
        self.state.r_bottom = 0x8 + LENV_STACK_SIZE as u64;
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


    fn push64(&mut self, v: u64) {
        unsafe {
            self.state.r_stack -= 8;
            * (self.state.stack.add((self.state.r_stack - 0x8) as usize) as *mut u64) = v;
        }
    }


    fn pop64(&mut self) -> u64 {
        unsafe {
            let v = *(self.state.stack.add((self.state.r_stack - 0x8) as usize) as *mut u64);
            self.state.r_stack += 8;
            v
        }
    }

    fn exec_instruction(&mut self, i: &Instruction) -> bool {

        if self.flags & Self::FLAG_DEBUG != 0 {

            //runt the debug console
            loop {
                println!("PC {}: {:?}", self.state.r_pc, i);
                print!("> ");
                stdout().flush().unwrap();
                let mut msg = String::new();
                stdin().read_line(&mut msg).unwrap();
                let msg = msg.trim();

                let parts: Vec<&str> = msg.split(' ').collect();



                if msg.is_empty() || parts[0] == "step" {
                    break;
                }

                if parts[0] == "inspect" {
                    if parts.len() == 1 {
                        //print all registers
                        println!("RET: {}, AUX: {}, STACK: {}, BOTTOM: {}, CMP: {}",
                             self.state.r_ret, self.state.r_aux,
                             self.state.r_stack,
                             self.state.r_bottom, self.state.r_cmp
                        )
                    } else if parts[1] == "stack" {
                        println!("FIRST 32 BYTES AFTER STACK POINTER");
                        let i = self.state.r_stack - 0x8;
                        for j in i as usize..LENV_STACK_SIZE {
                            println!("[{}]: {}", j, unsafe{ *self.state.stack.add(j) });
                        }
                    } else {
                        let ptr: LPTR = parts[1].parse().unwrap();
                        let p = self.get_ptr(ptr) as *mut u64;
                        if p.is_null() {
                            println!("NULL");
                        } else {
                            println!("0x{:x}: {}", ptr, unsafe{ *p })
                        }
                    }
                }
            }

        }


        match i {
            Instruction::StackAlloc(n) => {
                self.state.r_stack -= *n;
            }
            Instruction::StackFree(n) => {
                self.state.r_stack += *n;
            }

            Instruction::Push(r) => {
                let v = self.get_register(r);
                self.push64(v);
            }
            Instruction::Pop(r) => {
                let v = self.pop64();
                self.set_register(r, v);
            }
            Instruction::PopN(n) => {
                self.state.r_stack += n * 8;
            }
            Instruction::Exit => {
                self.flags |= Self::FLAG_EXIT;
            }

            Instruction::Mov(r, v) => {
                self.set_register(r, *v)
            }
            Instruction::MovBottom(r, offset) => {
                let offset = ((self.state.r_bottom - 0x8).cast_signed() + offset).cast_unsigned();
                unsafe {
                    let stack_ptr = self.state.stack.add(offset as usize);

                    let v = * ( stack_ptr as *mut u64 );
                    self.set_register(r, v);
                }
            }
            Instruction::MovR(r1, r2) => {
                let v = self.get_register(r2);
                self.set_register(r1, v);
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


            Instruction::Cmp => {

                self.state.r_cmp = self.state.r_ret.cast_signed() - self.state.r_aux.cast_signed();
            }
            Instruction::CmpLT => {
                self.state.r_cmp = self.state.r_ret.cast_signed() - self.state.r_aux.cast_signed();
                if self.state.r_cmp < 0 {
                    self.state.r_cmp = 0;
                }
            }
            Instruction::CmpLTE => {
                self.state.r_cmp = self.state.r_ret.cast_signed() - self.state.r_aux.cast_signed();
                if self.state.r_cmp <= 0 {
                    self.state.r_cmp = 0;
                }
            }
            Instruction::CmpGT => {
                self.state.r_cmp = self.state.r_ret.cast_signed() - self.state.r_aux.cast_signed();
                if self.state.r_cmp > 0 {
                    self.state.r_cmp = 0;
                }
            }
            Instruction::CmpGTE => {
                self.state.r_cmp = self.state.r_ret.cast_signed() - self.state.r_aux.cast_signed();
                if self.state.r_cmp >= 0 {
                    self.state.r_cmp = 0;
                }
            }

            
            Instruction::Call(pc) => {
                self.push64(self.state.r_pc);
                self.state.r_pc = *pc;
                return false;
            }
            Instruction::Return => {
                self.state.r_pc = self.pop64();
            }


            Instruction::CastInt => {
                self.state.r_ret = (f64::from_bits(self.state.r_ret) as i64).cast_unsigned()
            }
            Instruction::CastFloat => {
                self.state.r_ret = ((self.state.r_ret.cast_signed()) as f64).to_bits()
            }

            Instruction::Jump(loc) => {
                self.state.r_pc = *loc;
                return false;
            }
            Instruction::JumpLess(loc) => {
                //quick negative check
                if self.state.r_cmp < 0 {
                    self.state.r_pc = *loc;
                    return false;
                }
            }
            Instruction::JumpGreater(loc) => {
                //greater than 0
                if self.state.r_cmp > 0 {
                    self.state.r_pc = *loc;
                    return false;
                }
            }
            Instruction::JumpLessEq(loc) => {
                if self.state.r_cmp <= 0 {
                    self.state.r_pc = *loc;
                    return false;
                }
            }
            Instruction::JumpGreaterEq(loc) => {
                if self.state.r_cmp >= 0 {
                    self.state.r_pc = *loc;
                    return false;
                }
            }
            Instruction::JumpEq(loc) => {
                if self.state.r_cmp == 0 {
                    self.state.r_pc = *loc;
                    return false;
                }
            }
            Instruction::JumpNEq(loc) => {
                if self.state.r_cmp != 0 {
                    self.state.r_pc = *loc;
                    return false;
                }
            }

            Instruction::Negate => {
                //2's complement
                //0001 (+1) -> 1111 (-1)
                //1111 (-1) -> 0001 (+1)
                //we actually want the overflow to work this time
                self.state.r_ret = unsafe{ (!self.state.r_ret).unchecked_add(1) }
            }

            Instruction::Add => {
                self.state.r_ret = (self.state.r_ret.cast_signed() + self.state.r_aux.cast_signed()).cast_unsigned();
            }
            Instruction::Sub => {
                self.state.r_ret = (self.state.r_ret.cast_signed() - self.state.r_aux.cast_signed()).cast_unsigned();
            }
            Instruction::Mul=> {
                self.state.r_ret = (self.state.r_ret.cast_signed() * self.state.r_aux.cast_signed()).cast_unsigned();
            }
            Instruction::Div => {
                self.state.r_ret = (self.state.r_ret.cast_signed() / self.state.r_aux.cast_signed()).cast_unsigned();
            }
            

            Instruction::FCmp => {

                //heres how this works:
                /*
                FLOATING BIT REPRESENTATION:
                SIGN  EXPONENT MANTISSA
                1 bit 11 bits  53 bits

                so if both numbers are exactly equal, cmp = 0,
                because the bit representation will all be 0's
                but if less than, then the sign bit will be on, which means the integer will be
                negative too, and the other way around if positive
                */
                self.state.r_cmp = (f64::from_bits( self.state.r_ret ) - f64::from_bits( self.state.r_aux )).to_bits().cast_signed();

            }
            Instruction::FCmpLT => {

                let res = (f64::from_bits( self.state.r_ret ) - f64::from_bits( self.state.r_aux )).to_bits().cast_signed();
                self.state.r_cmp = if res < 0 { 0 } else { 1 }
            }
            Instruction::FCmpLTE => {
                let res = (f64::from_bits( self.state.r_ret ) - f64::from_bits( self.state.r_aux )).to_bits().cast_signed();
                self.state.r_cmp = if res <= 0 { 0 } else { 1 }
            }
            Instruction::FCmpGT => {
                let res = (f64::from_bits( self.state.r_ret ) - f64::from_bits( self.state.r_aux )).to_bits().cast_signed();
                self.state.r_cmp = if res > 0 { 0 } else { 1 }
            }
            Instruction::FCmpGTE => {

                let res = (f64::from_bits( self.state.r_ret ) - f64::from_bits( self.state.r_aux )).to_bits().cast_signed();
                self.state.r_cmp = if res >= 0 { 0 } else { 1 }
            }

            Instruction::FAdd => {
                self.state.r_ret = (f64::from_bits(self.state.r_ret) + f64::from_bits(self.state.r_aux)).to_bits();
            }
            Instruction::FSub => {
                self.state.r_ret = (f64::from_bits(self.state.r_ret) - f64::from_bits(self.state.r_aux)).to_bits();

            }
            Instruction::FMul => {
                self.state.r_ret = (f64::from_bits(self.state.r_ret) * f64::from_bits(self.state.r_aux)).to_bits();

            }
            Instruction::FDiv => {
                self.state.r_ret = (f64::from_bits(self.state.r_ret) / f64::from_bits(self.state.r_aux)).to_bits();

            }

            Instruction::FNegate => {
                self.state.r_ret = f64::from_bits(self.state.r_ret).neg().to_bits()
            }
        }

        true
    }
}