use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};


#[allow(unused)]
#[derive(Copy, Clone)]
pub enum Register {
    Bottom,
    Stack,
    Ret
}

impl Debug for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::Bottom => f.write_str("BOTTOM"),
            Register::Stack => f.write_str("STACK"),
            Register::Ret => f.write_str("RET"),
        }
    }
}

#[allow(unused)]
#[derive(Clone, Copy, Debug)]
pub enum DataLocation {
    Register(Register), //REG
    StackOffset(usize), //[isize]
    StackRegOffset(Register, isize) //[REG + isize]
}

#[derive(Copy, Clone)]
pub enum Value {
    Num(f64),
    Offset(isize),
    Pointer(usize),
    Void
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Num(n) => {
                f.write_str(n.to_string().as_str())
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
    pub fn bop_num<T>(self, n: f64, bop: T) -> Self
    where T: Fn(f64, f64) -> f64 {
        match self {
            Value::Num(n1) => Value::Num(bop(n1, n)),
            Value::Offset(i) => Value::Num(bop(i as f64, n)),
            Value::Pointer(u) => Value::Num(bop(u as f64, n)),
            Value::Void => Value::Num(n),
        }
    }


    #[inline]
    pub fn bop_offset<T>(self, n: isize, bop: T) -> Self
    where T: Fn(isize, isize) -> isize {
        match self {
            Value::Num(n1) => Value::Offset(bop(n1 as isize, n)),
            Value::Offset(i) => Value::Offset(bop(i, n)),
            Value::Pointer(u) => Value::Offset(bop(u as isize, n)),
            Value::Void => Value::Offset(n),
        }
    }


    #[inline]
    pub fn bop_pointer<T>(self, n: usize, bop: T) -> Self
    where T: Fn(usize, usize) -> usize {
        match self {
            Value::Num(n1) => Value::Pointer(bop(n1 as usize, n)),
            Value::Offset(i) => Value::Pointer(bop(i as usize, n)),
            Value::Pointer(u) => Value::Pointer(bop(u, n)),
            Value::Void => Value::Pointer(n),
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
#[derive(Copy, Clone)]
pub enum IArg {
    Value(Value), //1.1, 2.5, etc
    Data(DataLocation)
}


#[allow(unused)]
#[derive(Clone)]
pub enum Instruction {
    Push(IArg),
    Pop(DataLocation),
    Store(DataLocation, IArg),
    Return,
    Call(String),
    //CallNative(fnptr) TODO
    ADD(DataLocation, IArg),
    SUB(DataLocation, IArg),
    MUL(DataLocation, IArg),
    DIV(DataLocation, IArg)
}

pub type Bytecode = Vec<Instruction>;

pub struct BytecodeBuilder {
    result: Bytecode
}

impl BytecodeBuilder {
    pub fn new() -> Self {
        Self{ result: Vec::new() }
    }

    pub fn exec(mut self, i: Instruction) -> Self {
        self.result.push(i);

        self
    }

    pub fn build(self) -> Bytecode {
        self.result
    }
}