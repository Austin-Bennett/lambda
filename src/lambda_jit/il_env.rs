use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use log::debug;
use crate::lambda_jit::lambda_il::*;
use crate::lambda_jit::lambda_il::Instruction::Return;
use crate::lambda_jit::lambda_il::Value::Void;

pub struct Env {
    reg_ret: Value,
    reg_stack: usize,
    reg_bottom: usize,

    stack: Vec<Value>, //increases when necessary
    functions: HashMap<String, Arc<Bytecode>>,
}

pub enum EnvError {
    OutOfBoundsAccess(DataLocation),
    EmptyStack,
    #[allow(unused)]
    UnknownFunction(String),
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
            EnvError::UnknownFunction(s) => write!(f, "Unknown function name: {}", s)
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
            stack: Vec::with_capacity(10),
            functions: HashMap::new()
        }
    }

    #[allow(unused)]
    pub fn add_function(&mut self, name: impl AsRef<str>, bc: Bytecode) {
        self.functions.insert(name.as_ref().to_string(), Arc::new(bc));
    }

    pub fn get_register(&self, r: &Register) -> Value {
        match r {
            Register::Bottom => Value::Pointer(self.reg_bottom),
            Register::Stack => Value::Pointer(self.reg_stack),
            Register::Ret => self.reg_ret
        }
    }

    pub fn set_register(&mut self, r: &Register, d: Value) {
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
                match d {
                    Value::Num(n) => self.reg_stack = n as usize,
                    Value::Offset(o) => self.reg_stack = o as usize,
                    Value::Pointer(p) => self.reg_stack = p,
                    Void => self.reg_stack = 0,
                }
            }
            Register::Ret => {
                self.reg_ret = d;
            }
        }
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
            DataLocation::Register(r) => Ok(self.set_register(r, src)),
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
    }

    pub fn pop_stack(&mut self) -> Result<Value, EnvError> {
        if self.reg_stack == 0 {
            Err(EnvError::EmptyStack)
        } else {
            self.reg_stack-=1;
            Ok(self.stack[self.reg_stack])
        }
    }

    pub fn execute(&mut self, code: &Bytecode) -> Result<Value, EnvError> {
        for i in code {
            match i {
                Instruction::Push(v) => {
                    self.push_stack(self.arg_to_val(v)?);
                }
                Instruction::Pop(dl) => {
                    let val = self.pop_stack()?;
                    self.set_data(dl, val)?;
                }
                Instruction::Store(dest, src) => {
                    self.set_data(dest, self.arg_to_val(src)?)?
                }
                Instruction::Return => {
                    return Ok(self.get_register(&Register::Ret));
                }
                Instruction::Call(func) => {
                    if let Some(bc) = self.functions.get(func).cloned() {
                        self.execute(bc.as_ref())?; //we can ignore the return value
                    }
                }
                Instruction::ADD(dl, v) => {
                    let v1 = self.get_data(dl)?;
                    let v2 = self.arg_to_val(v)?;

                    self.set_data(dl, v1 + v2)?;
                }
                Instruction::SUB(dl, v) => {
                    let v1 = self.get_data(dl)?;
                    let v2 = self.arg_to_val(v)?;

                    self.set_data(dl, v1 - v2)?;
                }
                Instruction::MUL(dl, v) => {
                    let v1 = self.get_data(dl)?;
                    let v2 = self.arg_to_val(v)?;

                    self.set_data(dl, v1 * v2)?;
                }
                Instruction::DIV(dl, v) => {
                    let v1 = self.get_data(dl)?;
                    let v2 = self.arg_to_val(v)?;

                    self.set_data(dl, v1 / v2)?;
                }
            }
        }

        Ok(self.reg_ret)
    }
}