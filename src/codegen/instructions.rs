use std::io::{BufWriter, Write};
use std::{io, mem};
use crate::codegen::encoder::LMEByteEncoder;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Type {
    I8 = 0,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
}

impl LMEByteEncoder for Type {
    fn encode<T: Write>(&self, writer: &mut BufWriter<T>) -> io::Result<()> {
        writer.write(&[(*self) as u8]).map(|v| {})
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Register {
    Ret = 0,
    Aux,
    Stack,
    Bottom,
    R1,
    R2,
    R3,
    R4,
    R5,
    Pc,
}

impl LMEByteEncoder for Register {
    fn encode<T: Write>(&self, writer: &mut BufWriter<T>) -> io::Result<()> {
        writer.write(&[*self as u8]).map(|v| {})
    }
}

#[repr(C, u8)]
#[derive(Copy, Clone)]
pub enum Data {
    Value(u64) = 0,
    Register(Register) = 1,
}

impl LMEByteEncoder for Data {
    fn encode<T: Write>(&self, writer: &mut BufWriter<T>) -> io::Result<()> {
        writer.write(&[match self {
            Data::Value(_) => 0,
            Data::Register(_) => 1,
        }])?;

        match self {
            Data::Value(v) => { writer.write(&v.to_le_bytes())?; }
            Data::Register(r) => { r.encode(writer)?; }
        }

        Ok(())
    }
}

pub type Pointer = Data;

#[repr(C, u8)]
pub enum Instruction {
    Move(Data, Register) = 0x0,
    MvRet(Data) = 0x9,
    MvAux(Data) = 0xA,
    MvPtr(Data, Pointer) = 0x6,

    //loads the value pointed by the pointer into RET
    Lea(Pointer) = 0x8,


    //arithmetic
    Add(Type) = 0x1,
    Sub(Type) = 0x2,
    Mul(Type) = 0x3,
    Div(Type) = 0x4,

    Call(Data) = 0x5,
}

impl LMEByteEncoder for Instruction {
    fn encode<T: Write>(&self, writer: &mut BufWriter<T>) -> io::Result<()> {
        match self {
            Instruction::Move(d, r) => {
                writer.write(&[0x0])?;
                d.encode(writer)?;
                r.encode(writer)?;
            }
            Instruction::MvRet(d) => {
                writer.write(&[0x9])?;
                d.encode(writer)?;
            }
            Instruction::MvAux(d) => {
                writer.write(&[0xA])?;
                d.encode(writer)?;
            }
            Instruction::MvPtr(d, p) => {
                writer.write(&[0x6])?;
                d.encode(writer)?;
                p.encode(writer)?;
            }
            Instruction::Lea(p) => {
                writer.write(&[0x8])?;
                p.encode(writer)?;
            }
            Instruction::Add(t) => {
                writer.write(&[0x1])?;
                t.encode(writer)?;

            }
            Instruction::Sub(t) => {
                writer.write(&[0x2])?;
                t.encode(writer)?;
            }
            Instruction::Mul(t) => {
                writer.write(&[0x3])?;
                t.encode(writer)?;
            }
            Instruction::Div(t) => {
                writer.write(&[0x4])?;
                t.encode(writer)?;
            }
            Instruction::Call(d) => {
                writer.write(&[0x5])?;
                d.encode(writer)?;
            }
        }

        Ok(())
    }
}