use std::collections::HashMap;
use std::io::BufWriter;
use lme::desc::{LMEFunctionLocation, LME};
use crate::codegen::encoder::LMEByteEncoder;
use crate::codegen::instructions::Instruction;

pub struct ExecBuilder {
    functions: HashMap<String, u64>,
    code: Vec<Instruction>,
    entry: u64,
}

impl ExecBuilder {

    pub fn new() -> Self {
        Self{
            functions: HashMap::new(),
            code: Vec::new(),
            entry: 0
        }
    }

    pub fn decl_entry(&mut self) -> &mut Self {
        self.entry = self.code.len() as u64;
        
        self
    }

    pub fn decl_func(&mut self, name: String) -> &mut Self {
        self.functions.insert(name, self.code.len() as u64);
        
        self
    }
    
    
    //todo: pseudo instructions
    // pub fn call(func: impl AsRef<str>) -> &mut Self {
    //     
    // }

    pub fn emit(&mut self, i: Instruction) -> &mut Self {
        self.code.push(i);
        
        self
    }
    
    pub fn get_bytes(&mut self) -> Vec<u8> {
        let mut res = BufWriter::new(Vec::new());
        
        for i in &self.code {
            i.encode(&mut res).unwrap();
        }
        
        res.into_inner().unwrap()
    }
    
    pub fn build(mut self) -> LME {
        let code = self.get_bytes();
        LME{
            code_entry: self.entry,
            functions: self.functions.into_iter().map(
                |(name, loc)|  LMEFunctionLocation{ name, loc }
            ).collect(),
            code
        }
    }
}