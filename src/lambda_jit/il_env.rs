

use crate::lambda_jit::il_env::EnvError::BadRegisterMutation;
use crate::lambda_jit::lambda_il::Instruction::{Nop, Return};
use crate::lambda_jit::lambda_il::Value::Void;
use crate::lambda_jit::lambda_il::*;
use crate::lambda_parser::FloatHelpers;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Write};
use std::hint::{likely, unlikely};
use std::io::Write as w;
use std::ptr;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use crate::time;

const STACK_SIZE: usize = 1_000_000;

//MUST BE HEAP-ALLOCATED
pub struct Env {
    pub reg_ret: Value,
    pub reg_stack: usize,
    pub reg_bottom: usize,
    pub reg_pc: usize,
    pub reg_cmp: Decimal,

    pub reg_r1: Value,
    pub reg_r2: Value,
    pub reg_r3: Value,
    pub reg_r4: Value,
    pub reg_r5: Value,

    pub stack: [Value; STACK_SIZE],

    pub dynamic_functions: HashMap<String, Rc<Bytecode>>, //has to be run via recursion
    pub dynamic_values: HashMap<String, Value>, //todo
    pub last_error: EnvError,

    pub run: bool,
}

#[derive(Clone)]
#[derive(PartialEq)]
pub enum EnvError {
    None,
    OutOfBoundsAccess(DataLocation),
    EmptyStack,
    BadRegisterMutation(Register),
    Stackoverflow,
    NoDynamicFunction(String),
    Other(String),
}

impl Debug for EnvError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvError::None => f.write_str("No error lol"),
            EnvError::OutOfBoundsAccess(dl) => match dl {
                DataLocation::Register(r) => write!(f, "Attempted to access register: {:?}", r),
                DataLocation::StackOffset(o) => write!(f, "Attempted to access memory outside of stack: {}", o),
                DataLocation::StackRegOffset(reg, o) =>
                    write!(f, "Attempted to access memory outside of stack: {:?}+{}", reg, o),
                DataLocation::StackBottomOffset( o) =>
                    write!(f, "Attempted to access memory outside of stack: BOTTOM+{}", o),
                DataLocation::StackStackOffset(o) =>
                    write!(f, "Attempted to access memory outside of stack STACK+{}", o),
                DataLocation::Dynamic(s) => write!(f, "Attempted to access non-existent variable: {}", s)
            },
            EnvError::EmptyStack => f.write_str("Attempted to access data on empty stack"),
            EnvError::BadRegisterMutation(r) => write!(f, "Cannot mutate register: {:?}", r),
            EnvError::Stackoverflow => f.write_str("Stack overflowed"),
            EnvError::NoDynamicFunction(s) => write!(f, "Could not find dynamic function: {}", s),
            EnvError::Other( s ) => write!(f, "{}", s)
        }
    }
}

impl Display for EnvError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for EnvError {}

impl Env {

    unsafe fn init(ptr: *mut Self) {

        unsafe {
            (*ptr).stack.fill(Void);

            ptr::write(&mut (*ptr).last_error, EnvError::None);

            ptr::write(&mut (*ptr).dynamic_functions, HashMap::new());
            ptr::write(&mut (*ptr).dynamic_values, HashMap::new());

            ptr::write(&mut (*ptr).reg_r1, Void);
            ptr::write(&mut (*ptr).reg_r2, Void);
            ptr::write(&mut (*ptr).reg_r3, Void);
            ptr::write(&mut (*ptr).reg_r4, Void);
            ptr::write(&mut (*ptr).reg_r5, Void);

            ptr::write(&mut (*ptr).reg_stack, 0);
            ptr::write(&mut (*ptr).reg_ret, Void);
            ptr::write(&mut (*ptr).reg_bottom, 0);
            ptr::write(&mut (*ptr).reg_pc, 0);
            ptr::write(&mut (*ptr).reg_cmp, Decimal::new(0, 0));
        }
    }

    pub fn new() -> Box<Self> {

        let mut boxed = Box::<Self>::new_uninit();

        unsafe {
            let ptr = boxed.as_mut_ptr();
            Self::init(ptr);
            boxed.assume_init()
        }
    }



    pub fn add_dynamic_mod(&mut self, name: String, code: Bytecode) {
        self.dynamic_functions.insert(name, Rc::new(code));
    }

    pub fn add_native_function(&mut self, name: impl AsRef<str>, func: fn(&mut Env) -> ()) {
        self.add_dynamic_mod(
            name.as_ref().to_string(),

            {
                let mut b = BytecodeBuilder::new();
                b.emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Bottom))))
                .emit(Instruction::Store(DataLocation::Register(Register::Bottom), IArg::Data(DataLocation::Register(Register::Stack))))
                .emit(Instruction::CallNative(func))
                .emit(Instruction::Pop(DataLocation::Register(Register::Bottom)))
                .emit(Return);

                b.build().unwrap()
            }
        )
    }



    #[inline(always)]
    pub fn get_register(&self, r: &Register) -> Value {
        match r {
            Register::Bottom => Value::Pointer(self.reg_bottom),
            Register::Stack => Value::Pointer(self.reg_stack),
            Register::Ret => self.reg_ret,
            Register::Pc => Value::Pointer(self.reg_pc),
            Register::Cmp => Value::Num(self.reg_cmp),
            Register::R1 => self.reg_r1,
            Register::R2 => self.reg_r2,
            Register::R3 => self.reg_r3,
            Register::R4 => self.reg_r4,
            Register::R5 => self.reg_r5,
        }
    }

    #[inline(always)]
    pub fn set_register(&mut self, r: &Register, d: Value) {
        match r {
            Register::Bottom => {
                match d {
                    Value::Num(n) => self.reg_bottom = n.to_usize().unwrap(),
                    Value::Offset(o) => self.reg_bottom = o as usize,
                    Value::Pointer(p) => self.reg_bottom = p,
                    Void => self.reg_bottom = 0,
                }
            }
            Register::Stack => {
                self.last_error = BadRegisterMutation(Register::Stack)
            }
            Register::Ret => {
                self.reg_ret = d;
            }
            Register::Pc => {
                match d {
                    Value::Num(n) => self.reg_pc = n.to_usize().unwrap(),
                    Value::Offset(o) => self.reg_pc = o as usize,
                    Value::Pointer(p) => self.reg_pc = p,
                    Void => self.reg_pc = 0,
                }
            }
            Register::Cmp => {
                self.reg_cmp = d.val();
            }
            Register::R1 => { self.reg_r1 = d; }
            Register::R2 => { self.reg_r2 = d; }
            Register::R3 => { self.reg_r3 = d; }
            Register::R4 => { self.reg_r4 = d; }
            Register::R5 => { self.reg_r5 = d; }
        }
    }

    #[inline(always)]
    pub fn get_stack(&mut self, offset: usize) -> Value {
        if offset > self.stack.len() {
            self.last_error = EnvError::OutOfBoundsAccess(DataLocation::StackOffset(offset));
            return Void
        }
        unsafe { *self.stack.get_unchecked(offset) }
    }

    #[inline(always)]
    pub fn set_stack(&mut self, offset: usize, v: Value) {
        if offset > self.stack.len() {
            self.last_error = EnvError::OutOfBoundsAccess(DataLocation::StackOffset(offset));
            return;
        }

        unsafe { *self.stack.get_unchecked_mut(offset) = v };

    }

    #[inline(always)]
    pub fn get_data(&mut self, loc: &DataLocation) -> Value {
        match loc {
            DataLocation::Register(r) => self.get_register(r),
            DataLocation::StackOffset(o) => self.get_stack(*o),
            DataLocation::StackRegOffset(r, o) => {
                match self.get_register(r) {
                    Value::Pointer(ro) => self.get_stack(unsafe{ (ro as isize).unchecked_add(*o) } as usize),
                    _ => { self.last_error = EnvError::OutOfBoundsAccess(DataLocation::StackRegOffset(*r, *o)); Void },
                }
            }
            DataLocation::StackBottomOffset(i) => {
                self.get_stack((self.reg_bottom as isize + i) as usize)
            }
            DataLocation::StackStackOffset(i) => {
                self.get_stack((self.reg_stack as isize + i) as usize)
            }
            DataLocation::Dynamic(s) => {
                match self.dynamic_values.get(s) {
                    Some(v) => *v,
                    None => { self.last_error = EnvError::OutOfBoundsAccess(DataLocation::Dynamic(s.clone())); Void }
                }
            }
        }
    }

    #[inline(always)]
    pub fn set_data(&mut self, dest: &DataLocation, src: Value) {
        match dest {
            DataLocation::Register(r) => self.set_register(r, src),
            DataLocation::StackOffset(i) => {
                self.set_stack(*i, src)
            }
            DataLocation::StackRegOffset(r, i) => {
                let Value::Pointer(o) = self.get_register(r) else {
                    self.last_error = EnvError::OutOfBoundsAccess(DataLocation::StackRegOffset(*r, *i));
                    return;
                };
                self.set_stack(unsafe{ (o as isize).unchecked_add(*i) } as usize, src)
            }
            DataLocation::StackBottomOffset(i) => {
                self.set_stack(unsafe{ (self.reg_bottom as isize).unchecked_add(*i) } as usize, src)
            }
            DataLocation::StackStackOffset(i) => {
                self.set_stack(unsafe{ (self.reg_stack as isize).unchecked_add(*i) } as usize, src)
            }
            DataLocation::Dynamic(s) => {
                self.dynamic_values.insert(s.clone(), src);
            }
        }
    }

    #[inline(always)]
    pub fn arg_to_val(&mut self, iarg: &IArg) -> Value {
        match iarg {
            IArg::Value(v) => *v,
            IArg::Data(dl) => self.get_data(dl)
        }
    }

    #[inline(always)]
    pub fn push_stack(&mut self, v: Value) {
        if unlikely( self.reg_stack >= self.stack.len() ) {
            self.last_error = EnvError::Stackoverflow;
            return;
        }

        //since we are sure we are not going to access out-of-bounds data, we can use
        //unchecked index
        unsafe{ *self.stack.get_unchecked_mut(self.reg_stack) = v; }
        self.reg_stack += 1;
    }

    #[inline(always)]
    pub fn pop_stack(&mut self) -> Value {
        if unlikely( self.reg_stack == 0 ) {
            self.last_error = EnvError::EmptyStack;
            return Void;
        }

        self.reg_stack-=1;
        unsafe { *self.stack.get_unchecked(self.reg_stack) }
    }

    pub fn execute(&mut self, code: &Bytecode) -> Result<Value, EnvError> {

        self.last_error = EnvError::None;
        let v =self.__execute(code);

        if let EnvError::None = self.last_error {
            Ok(v)
        } else { Err(self.last_error.clone()) }
    }

    fn __execute(&mut self, code: &Bytecode) -> Value {
        self.reg_pc = code.entry;
        let len = code.code.len();


        self.run = true;

        while self.reg_pc < len && self.run {
            let i = unsafe{ code.code.get_unchecked(self.reg_pc) };
            let inc;
            if !cfg!(feature = "profiling-mode") {

                inc = self.execute_instruction(&i);
            } else {

                let time = time! {
                    inc = self.execute_instruction(&i)
                };
                println!("Instruction: {:?} [time: {:?}]", &i, time);
            }

            if !inc {
                self.reg_pc += 1;
            }
        }

        self.reg_ret
    }




    #[inline(always)]
    fn execute_instruction(&mut self, i: &Instruction) -> bool {

        let mut dont_inc_pc = false;


        match i {
            Instruction::Nop => {},
            Instruction::Push(v) => {
                let v = self.arg_to_val(v);
                self.push_stack(v);
            }
            Instruction::Pop(loc) => {
                let v = self.pop_stack();
                self.set_data(loc, v);
            }
            Instruction::PopN(i) => {
                if *i > self.reg_stack { self.reg_stack = 0 } else { self.reg_stack -= i }
            }
            Instruction::Store(dest, src) => {
                let v = self.arg_to_val(src);
                self.set_data(dest, v);
            }
            Return => {
                //exit if nothing is on the stack
                if self.reg_stack == 0 {
                    self.run = false;
                    return true;
                }

                let ret = self.pop_stack();
                if let Void = ret {
                    //exit if the return address is VOID
                    self.run = false;
                    return true;
                }
                self.reg_pc = ret.to_pointer();
            }
            Instruction::Cmp(i1, i2) => {
                let v1 = self.arg_to_val(i1);
                let v2 = self.arg_to_val(i2);

                self.reg_cmp = v1.val() - v2.val();
            }
            Instruction::Jump(loc) => {
                self.reg_pc = *loc;
                dont_inc_pc = true;
            }
            Instruction::JumpNZ(loc) => {
                if !self.reg_cmp.is_zero() {
                    self.reg_pc = *loc;
                    dont_inc_pc = true;
                }
            }
            Instruction::JumpZ(loc) => {
                if self.reg_cmp.is_zero() {
                    self.reg_pc = *loc;
                    dont_inc_pc = true;
                }
            }
            Instruction::JumpL(loc) => {
                if self.reg_cmp < Decimal::new(0, 0) && !self.reg_cmp.is_zero() {
                    self.reg_pc = *loc;
                    dont_inc_pc = true;
                }
            }
            Instruction::JumpG(loc) => {
                if self.reg_cmp > Decimal::new(0, 0) && !self.reg_cmp.is_zero() {
                    self.reg_pc = *loc;
                    dont_inc_pc = true;
                }
            }
            Instruction::JumpLE(loc) => {
                if self.reg_cmp < Decimal::new(0, 0) || self.reg_cmp.is_zero() {
                    self.reg_pc = *loc;
                    dont_inc_pc = true;
                }
            }
            Instruction::JumpGE(loc) => {
                if self.reg_cmp > Decimal::new(0, 0) || self.reg_cmp.is_zero() {
                    self.reg_pc = *loc;
                    dont_inc_pc = true;
                }
            }
            Instruction::Call(loc) => {
                //push the current addr
                self.push_stack(Value::Pointer(self.reg_pc));

                self.reg_pc = *loc;
                dont_inc_pc = true;
            }
            Instruction::CallNative(fnptr) => {
                fnptr(self);
            }
            Instruction::CallDynamic(s) => {
                let pc = self.reg_pc;

                if let Some(func) = self.dynamic_functions.get(s) {
                    let func = func.clone();
                    self.push_stack(Void);
                    self.__execute(&func);
                    self.reg_pc = pc;
                } else {
                    self.last_error = EnvError::NoDynamicFunction(s.clone());
                }
            }
            Instruction::Add(loc, v) => {
                let v1 = self.get_data(loc);
                let v2 = self.arg_to_val(v);
                self.set_data(loc, v1+v2);
            }
            Instruction::Sub(loc, v) => {
                let v1 = self.get_data(loc);
                let v2 = self.arg_to_val(v);
                self.set_data(loc, v1-v2);
            }
            Instruction::Mul(loc, v) => {
                let v1 = self.get_data(loc);
                let v2 = self.arg_to_val(v);
                self.set_data(loc, v1*v2);
            }
            Instruction::Div(loc, v) => {
                let v1 = self.get_data(loc);
                let v2 = self.arg_to_val(v);
                self.set_data(loc, v1/v2);
            }
        }



        dont_inc_pc
    }
}

