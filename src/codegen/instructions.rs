use std::io::{BufWriter, Write};
use std::{io, mem};
use crate::codegen::encoder::LMEByteEncoder;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum EffectiveType {
    Signed = 0,
    Unsigned,
    F32,
    F64,
}

impl LMEByteEncoder for EffectiveType {
    fn encode<T: Write>(&self, writer: &mut BufWriter<T>) -> io::Result<()> {
        writer.write(&[(*self) as u8]).map(|v| {})
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Size {
    Byte = 1,
    Word = 2,
    DWord = 4,
    QWord = 8,
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

impl Data {
    pub fn byte_length(&self) -> u64 {
        match self {
            Data::Value(_) => 9,
            Data::Register(_) => 2,
        }
    }
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

    Push(Data) = 0xD,
    Pop(Register) = 0xE,

    //moves data into a pointer
    MvPtr(Data, Size, Pointer) = 0x6,

    //loads the value pointed by the pointer into RET
    Lea(Pointer, Size) = 0x8,

    //loads from the bottom address
    Leab(Data, Size) = 0xF,


    //arithmetic
    Add(EffectiveType) = 0x1,
    Sub(EffectiveType) = 0x2,
    Mul(EffectiveType) = 0x3,
    Div(EffectiveType) = 0x4,

    Call(Data) = 0x5,
    Return = 0xB,
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
            Instruction::Push(d) => {
                writer.write(&[0xD])?;
                d.encode(writer)?;
            }
            Instruction::Pop(r) => {
                writer.write(&[0xE])?;
                r.encode(writer)?;
            }
            Instruction::MvPtr(d, s, p) => {
                writer.write(&[0x6])?;
                d.encode(writer)?;
                writer.write(&[*s as u8])?;
                p.encode(writer)?;
            }
            Instruction::Lea(p, s) => {
                writer.write(&[0x8])?;
                p.encode(writer)?;
                writer.write(&[*s as u8])?;
            }
            Instruction::Leab(d, s) => {
                writer.write(&[0x8])?;
                d.encode(writer)?;
                writer.write(&[*s as u8])?;
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
            Instruction::Return => {
                writer.write(&[0xB])?;
            }
        }

        Ok(())
    }
}

impl Instruction {
    pub fn byte_length(&self) -> u64 {
        match self {
            Instruction::Move(d, _) => {
                1 + d.byte_length() + 1
            }
            Instruction::MvRet(d) => {
                1 + d.byte_length()
            }
            Instruction::MvAux(d) => {
                1 + d.byte_length()
            }
            Instruction::MvPtr(src, size, dest) => {
                1 + src.byte_length() + 1 + dest.byte_length()
            }
            Instruction::Push(d) => {
                1 + d.byte_length()
            },
            Instruction::Pop(_) => {
                2
            },
            Instruction::Lea(d, s) => {
                1 + d.byte_length() + 1
            }
            Instruction::Leab(d, s) => {
                1 + d.byte_length() + 1
            }
            Instruction::Add(_) => { 2 }
            Instruction::Sub(_) => { 2 }
            Instruction::Mul(_) => { 2 }
            Instruction::Div(_) => { 2 }
            Instruction::Call(d) => { 1 + d.byte_length() }
            Instruction::Return => { 1 }
        }
    }
}