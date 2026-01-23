use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use log::debug;
use crate::lambda_jit::il_env::EnvError::BadRegisterMutation;
use crate::lambda_jit::lambda_il::*;
use crate::lambda_jit::lambda_il::Instruction::Return;
use crate::lambda_jit::lambda_il::Value::Void;

pub struct Env {
    pub reg_ret: Value,
    pub reg_stack: usize,
    pub reg_bottom: usize,
    pub reg_pc: usize,

    pub stack: Vec<Value>, //increases when necessary
}

pub enum EnvError {
    OutOfBoundsAccess(DataLocation),
    EmptyStack,
    BadRegisterMutation(Register)
}

impl Debug for EnvError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvError::OutOfBoundsAccess(dl) => match dl {
                DataLocation::Register(r) => write!(f, "Attempted to access register: {:?}", r),
                DataLocation::StackOffset(o) => write!(f, "Attempted to access memory outside of stack: {}", o),
                DataLocation::StackRegOffset(reg, o) =>
                    write!(f, "Attempted to access memory outside of stack: {:?}+{}", reg, o)
            },
            EnvError::EmptyStack => f.write_str("Attempted to access data on empty stack"),
            EnvError::BadRegisterMutation(r) => write!(f, "Cannot mutate register: {:?}", r)
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
    pub fn new() -> Self {
        Self{
            reg_stack: 0,
            reg_ret: Value::Void,
            reg_bottom: 0,
            reg_pc: 0,
            stack: Vec::with_capacity(10),
        }
    }



    pub fn get_register(&self, r: &Register) -> Value {
        match r {
            Register::Bottom => Value::Pointer(self.reg_bottom),
            Register::Stack => Value::Pointer(self.reg_stack),
            Register::Ret => self.reg_ret,
            Register::Pc => Value::Pointer(self.reg_pc),
        }
    }

    pub fn set_register(&mut self, r: &Register, d: Value) -> Result<(), EnvError> {
        match r {
            Register::Bottom => {
                match d {
                    Value::Num(n) => self.reg_bottom = n as usize,
                    Value::Offset(o) => self.reg_bottom = o as usize,
                    Value::Pointer(p) => self.reg_bottom = p,
                    Void => self.reg_bottom = 0,
                }
            }
            Register::Stack => {
                return Err(BadRegisterMutation(Register::Stack))
            }
            Register::Ret => {
                self.reg_ret = d;
            }
            Register::Pc => {
                match d {
                    Value::Num(n) => self.reg_pc = n as usize,
                    Value::Offset(o) => self.reg_pc = o as usize,
                    Value::Pointer(p) => self.reg_pc = p,
                    Void => self.reg_pc = 0,
                }
            }
        }
        Ok(())
    }


    pub fn get_stack(&self, offset: usize) -> Result<Value, EnvError> {
        if offset > self.stack.len() {
            Err(EnvError::OutOfBoundsAccess(DataLocation::StackOffset(offset)))
        } else {
            Ok(self.stack[offset])
        }
    }

    pub fn set_stack(&mut self, offset: usize, v: Value) -> Result<(), EnvError> {
        if offset > self.stack.len() {
            Err(EnvError::OutOfBoundsAccess(DataLocation::StackOffset(offset)))
        } else {

            self.stack[offset] = v;
            Ok(())
        }
    }

    pub fn get_data(&self, loc: &DataLocation) -> Result<Value, EnvError> {
        match loc {
            DataLocation::Register(r) => Ok(self.get_register(r)),
            DataLocation::StackOffset(o) => self.get_stack(*o),
            DataLocation::StackRegOffset(r, o) => {
                match self.get_register(r) {
                    Value::Pointer(ro) => self.get_stack(unsafe{ (ro as isize).unchecked_add(*o) } as usize),
                    _ => Err(EnvError::OutOfBoundsAccess(DataLocation::StackRegOffset(*r, *o))),
                }
            }
        }
    }

    pub fn set_data(&mut self, dest: &DataLocation, src: Value) -> Result<(), EnvError> {
        match dest {
            DataLocation::Register(r) => Ok(self.set_register(r, src)?),
            DataLocation::StackOffset(i) => {
                self.set_stack(*i, src)
            }
            DataLocation::StackRegOffset(r, i) => {
                let Value::Pointer(o) = self.get_register(r) else {
                    return Err(EnvError::OutOfBoundsAccess(DataLocation::StackRegOffset(*r, *i)));
                };
                self.set_stack(unsafe{ (o as isize).unchecked_add(*i) } as usize, src)
            }
        }
    }

    pub fn arg_to_val(&self, iarg: &IArg) -> Result<Value, EnvError> {
        match iarg {
            IArg::Value(v) => Ok(*v),
            IArg::Data(dl) => self.get_data(dl)
        }
    }

    pub fn push_stack(&mut self, v: Value) {
        if self.reg_stack >= self.stack.len() {
            self.reg_stack = self.stack.len(); //clamp
            self.stack.push(Void);
        }

        self.stack[self.reg_stack] = v;
        self.reg_stack += 1;
    }

    pub fn pop_stack(&mut self) -> Result<Value, EnvError> {
        if self.reg_stack == 0 {
            Err(EnvError::EmptyStack)
        } else {
            self.reg_stack-=1;
            Ok(self.stack[self.reg_stack])
        }
    }

    pub fn execute(&mut self, code: &Bytecode, debug: bool) -> Result<Value, EnvError> {
        self.reg_pc = code.entry;
        self.reg_stack = 0;
        self.reg_bottom = 0;

        if debug {
            println!("PROGRAM:");
            for (i, j) in code.code.iter().enumerate() {
                println!("{}: {:?}", i, j);
            }
        }

        while self.reg_pc < code.code.len() {
            let i = &code.code[self.reg_pc];
            if debug {
                println!("DEBUG {}: {:?}", self.reg_pc, i);
            }
            let mut dont_inc_pc = false;
            match i {
                Instruction::Push(v) => {
                    self.push_stack(self.arg_to_val(v)?);
                }
                Instruction::Pop(loc) => {
                    let v = self.pop_stack()?;
                    self.set_data(loc, v)?;
                }
                Instruction::PopN(i) => {
                    if *i > self.reg_stack { self.reg_stack = 0 } else { self.reg_stack -= i }
                }
                Instruction::Store(dest, src) => {
                    let v = self.arg_to_val(src)?;
                    self.set_data(dest, v)?;
                }
                Return => {
                    //in order to return, we need to get the call return address
                    //off the stack, should be the last value on the return address stack
                    //if not, well that's the programmers fault
                    if self.reg_stack == 0 {
                        //no stack left to read
                        break;
                    }
                    let ret = self.pop_stack()?.to_pointer();
                    self.reg_pc = ret;
                }
                Instruction::Jump(loc) => {
                    self.reg_pc = *loc;
                    dont_inc_pc = true;
                }
                Instruction::Call(loc) => {
                    //push the current addr
                    self.push_stack(Value::Pointer(self.reg_pc));

                    self.reg_pc = *loc;
                    dont_inc_pc = true;
                }
                Instruction::Add(loc, v) => {
                    let v1 = self.get_data(loc)?;
                    let v2 = self.arg_to_val(v)?;
                    self.set_data(loc, v1+v2)?;
                }
                Instruction::Sub(loc, v) => {
                    let v1 = self.get_data(loc)?;
                    let v2 = self.arg_to_val(v)?;
                    self.set_data(loc, v1-v2)?;
                }
                Instruction::Mul(loc, v) => {
                    let v1 = self.get_data(loc)?;
                    let v2 = self.arg_to_val(v)?;
                    self.set_data(loc, v1*v2)?;
                }
                Instruction::Div(loc, v) => {
                    let v1 = self.get_data(loc)?;
                    let v2 = self.arg_to_val(v)?;
                    self.set_data(loc, v1/v2)?;
                }
            }
            if !dont_inc_pc {
                self.reg_pc += 1;
            }
        }

        Ok(self.reg_ret)
    }
}