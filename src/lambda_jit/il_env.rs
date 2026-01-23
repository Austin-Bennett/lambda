use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{write, Debug, Display, Formatter, Write};
use std::io::{stdin, stdout, Write as w};
use std::rc::Rc;
use std::sync::Arc;
use log::debug;
use crate::lambda_jit::il_env::EnvError::BadRegisterMutation;
use crate::lambda_jit::lambda_il::*;
use crate::lambda_jit::lambda_il::Instruction::Return;
use crate::lambda_jit::lambda_il::Value::Void;
use crate::lambda_parser::FloatHelpers;

pub struct Env {
    pub reg_ret: Value,
    pub reg_stack: usize,
    pub reg_bottom: usize,
    pub reg_pc: usize,
    pub reg_cmp: f64,

    pub stack: Vec<Value>, //increases when necessary

    pub dynamic_modules: HashMap<String, Rc<Bytecode>>, //have to be run via recursion
    pub dynamic_values: HashMap<String, Value>,
}

pub enum EnvError {
    OutOfBoundsAccess(DataLocation),
    EmptyStack,
    BadRegisterMutation(Register),
    Stackoverflow,
    NoDynamicFunction(String),
}

impl Debug for EnvError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvError::OutOfBoundsAccess(dl) => match dl {
                DataLocation::Register(r) => write!(f, "Attempted to access register: {:?}", r),
                DataLocation::StackOffset(o) => write!(f, "Attempted to access memory outside of stack: {}", o),
                DataLocation::StackRegOffset(reg, o) =>
                    write!(f, "Attempted to access memory outside of stack: {:?}+{}", reg, o),
                DataLocation::Dynamic(s) => write!(f, "Attempted to access non-existent variable: {}", s)
            },
            EnvError::EmptyStack => f.write_str("Attempted to access data on empty stack"),
            EnvError::BadRegisterMutation(r) => write!(f, "Cannot mutate register: {:?}", r),
            EnvError::Stackoverflow => f.write_str("Stack overflowed"),
            EnvError::NoDynamicFunction(s) => write!(f, "Could not find dynamic function: {}", s),
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
            reg_cmp: 0.0,
            stack: Vec::with_capacity(10),
            dynamic_modules: HashMap::new(),
            dynamic_values: HashMap::new(),
        }
    }

    pub fn load_dynamic_mod(&mut self, name: String, code: Bytecode) {
        self.dynamic_modules.insert(name, Rc::new(code));
    }

    pub fn get_register(&self, r: &Register) -> Value {
        match r {
            Register::Bottom => Value::Pointer(self.reg_bottom),
            Register::Stack => Value::Pointer(self.reg_stack),
            Register::Ret => self.reg_ret,
            Register::Pc => Value::Pointer(self.reg_pc),
            Register::Cmp => Value::Num(self.reg_cmp),
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
            Register::Cmp => {
                self.reg_cmp = d.val();
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
            DataLocation::Dynamic(s) => {
                match self.dynamic_values.get(s) {
                    Some(v) => Ok(*v),
                    None => Err(EnvError::OutOfBoundsAccess(DataLocation::Dynamic(s.clone())))
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
            DataLocation::Dynamic(s) => {
                self.dynamic_values.insert(s.clone(), src);
                Ok(())
            }
        }
    }

    pub fn arg_to_val(&self, iarg: &IArg) -> Result<Value, EnvError> {
        match iarg {
            IArg::Value(v) => Ok(*v),
            IArg::Data(dl) => self.get_data(dl)
        }
    }

    pub fn push_stack(&mut self, v: Value) -> Result<(), EnvError> {
        if self.reg_stack >= self.stack.len() {
            self.reg_stack = self.stack.len(); //clamp
            self.stack.push(Void);
        }

        self.stack[self.reg_stack] = v;
        self.reg_stack += 1;

        if self.reg_stack >= 1_000_000 {
            Err(EnvError::Stackoverflow)
        } else {
            Ok(())
        }
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

        if debug {
            println!("PROGRAM:");
            for (i, j) in code.code.iter().enumerate() {
                println!("{}: {:?}", i, j);
            }
            println!()
        }

        let mut run = true;

        while self.reg_pc < code.code.len() && run {
            let i = &code.code[self.reg_pc];
            if debug {
                let mut buf = "___".to_string();
                while !buf.is_empty() {
                    buf.clear();
                    println!("DEBUG {}: {:?}", self.reg_pc, i);
                    print!("> ");
                    let _ = stdout().flush();
                    let _ = stdin().read_line(&mut buf);
                    buf = buf.trim().to_string();

                    if buf.trim() == "data" {
                        println!("STACK: {}, PC: {}, BOTTOM: {}, RET: {:?}, CMP: {}", self.reg_stack, self.reg_pc, self.reg_bottom, self.reg_ret, self.reg_cmp);
                        let mut i = 0;
                        while i < self.reg_stack && i < self.stack.len() {
                            print!("STACK[{}]: {:?}", i, self.stack[i]);
                            if i == self.reg_stack && i == self.reg_bottom  {
                                println!(" <- STACK, BOTTOM");
                            } else if i == self.reg_stack {
                                println!(" <- STACK");
                            } else if i == self.reg_bottom {
                                println!(" <- BOTTOM");
                            } else {
                                println!();
                            }

                            i+=1;
                        }
                    }
                    println!()
                }
            }

            if !self.execute_instruction(i, &mut run)? {
                self.reg_pc += 1;
            }
        }

        Ok(self.reg_ret)
    }

    pub fn execute_instruction(&mut self, i: &Instruction, run: &mut bool) -> Result<bool, EnvError> {

        let mut dont_inc_pc = false;
        match i {
            Instruction::Push(v) => {
                self.push_stack(self.arg_to_val(v)?)?;
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
                    *run = false;
                    return Ok(true);
                }
                let ret = self.pop_stack()?.to_pointer();
                self.reg_pc = ret;
            }
            Instruction::Cmp(i1, i2) => {
                let v1 = self.arg_to_val(i1)?;
                let v2 = self.arg_to_val(i2)?;

                self.reg_cmp = v1.val() - v2.val();
            }
            Instruction::Jump(loc) => {
                self.reg_pc = *loc;
                dont_inc_pc = true;
            }
            Instruction::JumpZ(loc) => {
                if self.reg_cmp.equates(0.0) {
                    self.reg_pc = *loc;
                    dont_inc_pc = true;
                }
            }
            Instruction::JumpL(loc) => {
                if self.reg_cmp < 0.0 && !self.reg_cmp.equates(0.0) {
                    self.reg_pc = *loc;
                    dont_inc_pc = true;
                }
            }
            Instruction::Call(loc) => {
                //push the current addr
                self.push_stack(Value::Pointer(self.reg_pc))?;

                self.reg_pc = *loc;
                dont_inc_pc = true;
            }
            Instruction::CallDynamic(s) => {
                let pc = self.reg_pc;

                if let Some(func) = self.dynamic_modules.get(s) {
                    let func = func.clone();
                    self.execute(&func, false)?;
                    self.reg_pc = pc;
                } else {
                    return Err(EnvError::NoDynamicFunction(s.clone()))
                }
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

        Ok(dont_inc_pc)
    }
}