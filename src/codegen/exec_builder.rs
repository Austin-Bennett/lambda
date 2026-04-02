use std::collections::HashMap;
use std::io::{BufWriter, Write};
use lme::desc::{LMEFunctionLocation, LME};
use crate::codegen::encoder::LMEByteEncoder;
use crate::codegen::instructions::Instruction;

pub struct ExecBuilder {
    functions: HashMap<String, u64>,
    code: Vec<PseudoInstruction>,
    entry: u64,
    offset: u64,
}

pub enum PseudoInstruction {
    Instruction(Instruction),
    Call(String),
}

impl ExecBuilder {

    pub fn new() -> Self {
        Self{
            functions: HashMap::new(),
            code: Vec::new(),
            entry: 0,
            offset: 0,
        }
    }

    pub fn decl_entry(&mut self) -> &mut Self {
        self.entry = self.offset;
        
        self
    }

    pub fn decl_label(&mut self, name: String) -> &mut Self {
        self.functions.insert(name, self.offset);
        
        self
    }
    

    pub fn call(&mut self, func: impl AsRef<str>) -> &mut Self {
        self.code.push(PseudoInstruction::Call(func.as_ref().to_string()));

        //1 for the call opcode, 8 for the label address
        self.offset += 9;

        self
    }

    pub fn emit(&mut self, i: Instruction) -> &mut Self {
        self.offset += i.byte_length();

        self.code.push(PseudoInstruction::Instruction(i));
        
        self
    }

    
    pub fn build(self) -> anyhow::Result<LME> {
        let mut byte_code = BufWriter::new(Vec::new());
        for i in self.code {
            match i {
                PseudoInstruction::Instruction(i) => {
                    i.encode(&mut byte_code)?;
                }
                PseudoInstruction::Call(c) => {
                    let offset = *self.functions.get(&c)
                        .ok_or(anyhow::Error::msg(format!("Unknown label: {}", c)))?;

                    byte_code.write(&[0x5])?;
                    byte_code.write(&offset.to_le_bytes())?;

                }
            }
        }


        Ok(LME{
            code_entry: self.entry,
            functions: self.functions.into_iter().map(|(name, loc)| LMEFunctionLocation{ name, loc }).collect(),
            code: byte_code.into_inner()?,
        })
    }
}