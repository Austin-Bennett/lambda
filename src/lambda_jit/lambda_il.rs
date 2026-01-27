use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use crate::lambda_jit::il_env::Env;



#[allow(unused)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum Register {
    Bottom,
    Stack,
    Ret,
    Pc,
    Cmp,
    R1,
    R2,
    R3,
    R4,
    R5,
}

impl Debug for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::Bottom => f.write_str("BOTTOM"),
            Register::Stack => f.write_str("STACK"),
            Register::Ret => f.write_str("RET"),
            Register::Pc => f.write_str("PC"),
            Register::Cmp => f.write_str("CMP"),
            Register::R1 => f.write_str("R1"),
            Register::R2 => f.write_str("R2"),
            Register::R3 => f.write_str("R3"),
            Register::R4 => f.write_str("R4"),
            Register::R5 => f.write_str("R5"),
        }
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub enum DataLocation {
    Register(Register), //REG
    StackOffset(usize), //[isize]
    StackRegOffset(Register, isize), //[REG + isize]
    StackBottomOffset(isize),
    StackStackOffset(isize),
    Dynamic(String),
}

#[derive(Copy, Clone)]
pub enum Value {
    Num(Decimal),
    Offset(isize),
    Pointer(usize),
    Void
}



impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Num(n) => {
                //let l = n.clone().fract().to_string().len();
                write!(f, "{}", n)
            }
            Value::Offset(n) => {
                f.write_str(n.to_string().as_str())
            }
            Value::Pointer(n) => {
                f.write_str(n.to_string().as_str())
            }
            Value::Void => {

                f.write_str("void")
            }
        }
    }
}

impl Value {

    #[inline(always)]
    pub fn num<T>(val: T) -> Self
    where Decimal: From<T>{
        Self::Num(Decimal::from(val))
    }

    #[inline(always)]
    pub fn numf64(val: f64) -> Self {
        Self::Num(Decimal::from_f64_retain(val).unwrap())
    }
    
    #[inline(always)]
    pub fn to_pointer(self) -> usize {
        match self {
            Value::Num(n) => n.to_usize().unwrap(),
            Value::Offset(o) => o as usize,
            Value::Pointer(p) => p,
            Value::Void => 0
        }
    }

    #[inline(always)]
    pub fn to_number(self) -> Decimal {
        match self {
            Value::Num(n) => n,
            Value::Offset(o) => Decimal::from(o),
            Value::Pointer(p) => Decimal::from(p),
            Value::Void => Decimal::new(0, 0)
        }
    }

    pub fn val(&self) -> Decimal {
        match self {
            Value::Num(n) => n.clone(),
            Value::Offset(i) => Decimal::from(*i),
            Value::Pointer(u) => Decimal::from(*u),
            Value::Void => Decimal::new(0, 0)
        }
    }
}


impl Add for Value {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        if let Value::Num(n1) = self && let Value::Num(n2) = rhs {
            Self::Num(n1 + n2)
        } else {
            Self::Void
        }
    }
}

impl Sub for Value {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        if let Value::Num(n1) = self && let Value::Num(n2) = rhs {
            Self::Num(n1 - n2)
        } else {
            Self::Void
        }
    }
}

impl Mul for Value {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        if let Value::Num(n1) = self && let Value::Num(n2) = rhs {
            Self::Num(n1 * n2)
        } else {
            Self::Void
        }
    }
}

impl Div for Value {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        if let Value::Num(n1) = self && let Value::Num(n2) = rhs {
            Self::Num(n1 / n2)
        } else {
            Self::Void
        }
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum IArg {
    Value(Value), //1.1, 2.5, etc
    Data(DataLocation)
}


#[allow(unused)]
#[derive(Clone, Debug)]
pub enum Instruction {
    Nop,
    Push(IArg),
    Pop(DataLocation),
    PopN(usize),
    Store(DataLocation, IArg),
    Return,
    Cmp(IArg, IArg),
    Jump(usize),
    JumpZ(usize),
    JumpNZ(usize),
    JumpL(usize),
    JumpG(usize),
    JumpLE(usize),
    JumpGE(usize),
    Call(usize),
    CallDynamic(String),
    CallNative(fn(&mut Env) -> ()),
    Add(DataLocation, IArg),
    Sub(DataLocation, IArg),
    Mul(DataLocation, IArg),
    Div(DataLocation, IArg)
}

#[derive(Clone)]
pub struct Bytecode {
    pub code: Vec<Instruction>,
    pub entry: usize,
}

impl Debug for Bytecode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, j) in self.code.iter().enumerate() {
            write!(f, "{}: {:?}\n", i, j)?;
        }

        Ok(())
    }
}

enum BytecodePrecomp {
    Instruction(Instruction),
    Call(String),
    Jump(String),
    JumpZero(String),
    JumpNotZero(String),
    JumpLess(String),
    JumpGreater(String),
    JumpLessEq(String),
    JumpGreatEq(String),
}

pub struct BytecodeBuilder {
    code: Vec<BytecodePrecomp>,
    entry: usize,
    labels: HashMap<String, usize> //points to functions, entry is MAIN
}

impl BytecodeBuilder {
    pub fn new() -> Self {
        Self{  code: Vec::new(), entry: 0, labels: HashMap::new() }
    }

    pub fn add_code(&mut self, mut code: Bytecode, replace_dynamics: bool) -> &mut Self {
        let label_offsets = self.code.len();

        for mut i in code.code {
            match &mut i {
                Instruction::Call(u) |
                Instruction::Jump(u) |
                Instruction::JumpL(u) |
                Instruction::JumpNZ(u) |
                Instruction::JumpZ(u) => {
                    *u += label_offsets;
                }
                _ => {}
            }
            if let Instruction::CallDynamic(s) = i {
                self.call(s);
            } else {
                self.code.push(BytecodePrecomp::Instruction(i))
            }
        }

        self
    }

    pub fn emit(&mut self, i: Instruction) -> &mut Self {

        self.code.push(BytecodePrecomp::Instruction(i));
        self
    }



    //inserts the call assuming the function exists
    pub fn call(&mut self, label: impl AsRef<str>) -> &mut Self {


        self.code.push(BytecodePrecomp::Call(label.as_ref().to_string()));

        self
    }
    
    pub fn jump(&mut self, label: impl AsRef<str>) -> &mut Self {
        self.code.push(BytecodePrecomp::Jump(label.as_ref().to_string()));

        self
    }

    pub fn jump_zero(&mut self, label: impl AsRef<str>) -> &mut Self {
        self.code.push(BytecodePrecomp::JumpZero(label.as_ref().to_string()));

        self
    }

    pub fn jump_not_zero(&mut self, label: impl AsRef<str>) -> &mut Self {
        self.code.push(BytecodePrecomp::JumpNotZero(label.as_ref().to_string()));

        self
    }

    pub fn jump_less(&mut self, label: impl AsRef<str>) -> &mut Self {
        self.code.push(BytecodePrecomp::JumpLess(label.as_ref().to_string()));

        self
    }

    pub fn jump_greater(&mut self, label: impl AsRef<str>) -> &mut Self {
        self.code.push(BytecodePrecomp::JumpGreater(label.as_ref().to_string()));

        self
    }

    pub fn jump_less_eq(&mut self, label: impl AsRef<str>) -> &mut Self {
        self.code.push(BytecodePrecomp::JumpLessEq(label.as_ref().to_string()));

        self
    }

    pub fn jump_greater_eq(&mut self, label: impl AsRef<str>) -> &mut Self {
        self.code.push(BytecodePrecomp::JumpGreatEq(label.as_ref().to_string()));

        self
    }

    pub fn decl_label(&mut self, name: impl AsRef<str>) -> &mut Self {
        let loc = self.code.len();
        if name.as_ref() == "main" {
            self.entry = loc;
        }
        self.labels.insert(name.as_ref().to_string(), loc);

        self
    }


    pub fn pop_instruction(&mut self) -> Option<Instruction> {

        if let Some(BytecodePrecomp::Instruction(i)) = self.code.pop() {
            Some(i)
        } else {
            None
        }
    }

    pub fn peek_last_instruct(&mut self) -> Option<&Instruction> {

        if let Some(BytecodePrecomp::Instruction(i)) = self.code.last() {
            Some(i)
        } else {
            None
        }
    }

    pub fn build(self) -> Result<Bytecode, String> {
        let mut result = Bytecode{ code: Vec::new(), entry: self.entry };

        for i in self.code {
            match i {
                BytecodePrecomp::Instruction(i) => {
                    result.code.push(i)
                }
                BytecodePrecomp::Call(s) => {
                    if let Some(n) = self.labels.get(&s) {
                        result.code.push(Instruction::Call(*n))
                    } else {
                        return Err(format!("Couldnt find symbol: {}", s))
                    }
                }
                BytecodePrecomp::Jump(s) => {
                    if let Some(n) = self.labels.get(&s) {
                        result.code.push(Instruction::Jump(*n))
                    } else {
                        return Err(format!("Couldnt find symbol: {}", s))
                    }
                }
                BytecodePrecomp::JumpZero(s) => {
                    if let Some(n) = self.labels.get(&s) {
                        result.code.push(Instruction::JumpZ(*n))
                    } else {
                        return Err(format!("Couldnt find symbol: {}", s))
                    }
                }

                BytecodePrecomp::JumpNotZero(s) => {
                    if let Some(n) = self.labels.get(&s) {
                        result.code.push(Instruction::JumpNZ(*n))
                    } else {
                        return Err(format!("Couldnt find symbol: {}", s))
                    }
                }
                BytecodePrecomp::JumpLess(s) => {
                    if let Some(n) = self.labels.get(&s) {
                        result.code.push(Instruction::JumpL(*n))
                    } else {
                        return Err(format!("Couldnt find symbol: {}", s))
                    }
                }
                BytecodePrecomp::JumpGreater(s) => {
                    if let Some(n) = self.labels.get(&s) {
                        result.code.push(Instruction::JumpG(*n))
                    } else {
                        return Err(format!("Couldnt find symbol: {}", s))
                    }
                }
                BytecodePrecomp::JumpLessEq(s) => {
                    if let Some(n) = self.labels.get(&s) {
                        result.code.push(Instruction::JumpLE(*n))
                    } else {
                        return Err(format!("Couldnt find symbol: {}", s))
                    }
                }
                BytecodePrecomp::JumpGreatEq(s) => {
                    if let Some(n) = self.labels.get(&s) {
                        result.code.push(Instruction::JumpGE(*n))
                    } else {
                        return Err(format!("Couldnt find symbol: {}", s))
                    }
                }
            }
        }
        Ok(result)
    }
}