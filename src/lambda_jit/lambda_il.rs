use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use crate::lambda_jit::il_env::Env;



#[allow(unused)]
#[derive(Copy, Clone)]
pub enum Register {
    Bottom,
    Stack,
    Ret,
    Aux,
    Pc,
    Cmp,
}

impl Debug for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::Bottom => f.write_str("BOTTOM"),
            Register::Stack => f.write_str("STACK"),
            Register::Ret => f.write_str("RET"),
            Register::Aux => f.write_str("AUX"),
            Register::Pc => f.write_str("PC"),
            Register::Cmp => f.write_str("CMP"),
        }
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum DataLocation {
    Register(Register), //REG
    StackOffset(usize), //[isize]
    StackRegOffset(Register, isize), //[REG + isize]
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

    #[inline]
    pub fn num<T>(val: T) -> Self
    where Decimal: From<T>{
        Self::Num(Decimal::from(val))
    }

    #[inline]
    pub fn numf64(val: f64) -> Self {
        Self::Num(Decimal::from_f64_retain(val).unwrap())
    }
    
    #[inline]
    pub fn to_pointer(self) -> usize {
        match self {
            Value::Num(n) => n.to_usize().unwrap(),
            Value::Offset(o) => o as usize,
            Value::Pointer(p) => p,
            Value::Void => 0
        }
    }

    #[inline]
    pub fn to_number(self) -> Decimal {
        match self {
            Value::Num(n) => n,
            Value::Offset(o) => Decimal::from(o),
            Value::Pointer(p) => Decimal::from(p),
            Value::Void => Decimal::new(0, 0)
        }
    }

    #[inline]
    pub fn bop_num<T>(self, n: Decimal, bop: T) -> Self
    where T: Fn(Decimal, Decimal) -> Decimal {
        match self {
            Value::Num(n1) => Value::Num(bop(n1, n)),
            Value::Offset(i) => Value::Num(bop(Decimal::from(i), n)),
            Value::Pointer(u) => Value::Num(bop(Decimal::from(u), n)),
            Value::Void => Value::Num(n),
        }
    }


    #[inline]
    pub fn bop_offset<T>(self, n: isize, bop: T) -> Self
    where T: Fn(isize, isize) -> isize {
        match self {
            Value::Num(n1) => Value::Offset(bop(n1.to_isize().unwrap(), n)),
            Value::Offset(i) => Value::Offset(bop(i, n)),
            Value::Pointer(u) => Value::Offset(bop(u as isize, n)),
            Value::Void => Value::Offset(n),
        }
    }


    #[inline]
    pub fn bop_pointer<T>(self, n: usize, bop: T) -> Self
    where T: Fn(usize, usize) -> usize {
        match self {
            Value::Num(n1) => Value::Pointer(bop(n1.to_usize().unwrap(), n)),
            Value::Offset(i) => Value::Pointer(bop(i as usize, n)),
            Value::Pointer(u) => Value::Pointer(bop(u, n)),
            Value::Void => Value::Pointer(n),
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

#[inline]
fn gadd<T: Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

#[inline]
fn gsub<T: Sub<Output=T>>(a: T, b: T) -> T {
    a - b
}

#[inline]
fn gmul<T: Mul<Output=T>>(a: T, b: T) -> T {
    a * b
}

#[inline]
fn gdiv<T: Div<Output=T>>(a: T, b: T) -> T {
    a / b
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match rhs {
            Value::Num(n) => self.bop_num(n, &gadd),
            Value::Offset(o) => self.bop_offset(o, &gadd),
            Value::Pointer(p) => self.bop_pointer(p, &gadd),
            Value::Void => self
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match rhs {
            Value::Num(n) => self.bop_num(n, &gsub),
            Value::Offset(o) => self.bop_offset(o, &gsub),
            Value::Pointer(p) => self.bop_pointer(p, &gsub),
            Value::Void => self
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match rhs {
            Value::Num(n) => self.bop_num(n, &gmul),
            Value::Offset(o) => self.bop_offset(o, &gmul),
            Value::Pointer(p) => self.bop_pointer(p, &gmul),
            Value::Void => self
        }
    }
}

impl Div for Value {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match rhs {
            Value::Num(n) => self.bop_num(n, &gdiv),
            Value::Offset(o) => self.bop_offset(o, &gdiv),
            Value::Pointer(p) => self.bop_pointer(p, &gdiv),
            Value::Void => self
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

    pub fn add_code(mut self, mut code: Bytecode) -> Self {
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

            self.code.push(BytecodePrecomp::Instruction(i))
        }

        self
    }

    pub fn emit(mut self, i: Instruction) -> Self {

        self.code.push(BytecodePrecomp::Instruction(i));

        self
    }

    //inserts the call assuming the function exists
    pub fn call(mut self, label: impl AsRef<str>) -> Self {


        self.code.push(BytecodePrecomp::Call(label.as_ref().to_string()));

        self
    }
    
    pub fn jump(mut self, label: impl AsRef<str>) -> Self {
        self.code.push(BytecodePrecomp::Jump(label.as_ref().to_string()));

        self
    }

    pub fn jump_zero(mut self, label: impl AsRef<str>) -> Self {
        self.code.push(BytecodePrecomp::JumpZero(label.as_ref().to_string()));

        self
    }

    pub fn jump_not_zero(mut self, label: impl AsRef<str>) -> Self {
        self.code.push(BytecodePrecomp::JumpNotZero(label.as_ref().to_string()));

        self
    }

    pub fn jump_less(mut self, label: impl AsRef<str>) -> Self {
        self.code.push(BytecodePrecomp::JumpLess(label.as_ref().to_string()));

        self
    }

    pub fn decl_label(mut self, name: impl AsRef<str>) -> Self {
        let loc = self.code.len();
        if name.as_ref() == "main" {
            self.entry = loc;
        }
        self.labels.insert(name.as_ref().to_string(), loc);
        self
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
                BytecodePrecomp::JumpLess(s) => {
                    if let Some(n) = self.labels.get(&s) {
                        result.code.push(Instruction::JumpL(*n))
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
            }
        }
        Ok(result)
    }
}