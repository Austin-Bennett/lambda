use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::lvm::lbc::Instruction;
use crate::lvm::lenv::LPTR;

pub struct LExecutable {
    pub instructions: Vec<Instruction>,
    pub entry: LPTR,
}

pub struct LPseudoExecutable {
    pub labels: HashMap<String, usize>,
    pub instructions: Vec<PseudoInstruction>,
    pub entry: LPTR,
}


#[derive(Clone, Debug)]
pub enum PseudoInstruction {
    Instruction(Instruction),
    Call(String),
    Jump(String), //jump to a label
    JumpLess(String),
    JumpGreater(String),
    JumpLessEq(String),
    JumpGreaterEq(String),
    JumpEq(String),
    JumpNEq(String),
}


pub struct LExecutableBuilder {
    entry: LPTR,
    instructions: Vec<PseudoInstruction>,
    labels: HashMap<String, usize>,
}

impl LExecutableBuilder {
    pub fn new() -> Self {
        Self{
            entry: 0,
            instructions: Vec::new(),
            labels: HashMap::new(),
        }
    }
    
    pub fn set_entry(&mut self, entry: LPTR) -> &mut Self {
        self.entry = entry;
        
        self
    }

    pub fn label(&mut self, id: String) -> &mut Self {
        self.labels.insert(id, self.instructions.len());

        self
    }

    pub fn entry_label(&mut self, id: String) -> &mut Self {
        self.labels.insert(id, self.instructions.len());
        self.entry = self.instructions.len() as u64;

        self
    }
    
    pub fn add(&mut self, i: Instruction) -> &mut Self {
        self.instructions.push(PseudoInstruction::Instruction(i));
        
        self
    }

    pub fn pseudo(&mut self, i: PseudoInstruction) -> &mut Self {
        self.instructions.push(i);

        self
    }

    pub fn build_pseudo(&mut self) -> LPseudoExecutable {
        let mut res = LPseudoExecutable{
            instructions: std::mem::take(&mut self.instructions),
            entry: self.entry,
            labels: std::mem::take(&mut self.labels)
        };

        self.entry = 0;
        self.labels.clear();

        res
    }
    
    pub fn build(&mut self) -> LExecutable {
        let instru = std::mem::take(&mut self.instructions);
        let mut res = LExecutable{
            instructions: Vec::new(),
            entry: self.entry,
        };
        for i in instru {
            match i {
                PseudoInstruction::Instruction(i) => {
                    res.instructions.push(i)
                }
                PseudoInstruction::Call(s) => {
                    res.instructions.push(Instruction::Call(*self.labels.get(&s).unwrap() as u64))
                }
                PseudoInstruction::Jump(l) => {
                    res.instructions.push(Instruction::Jump(*self.labels.get(&l).unwrap() as u64))
                }
                PseudoInstruction::JumpLess(l) => {
                    res.instructions.push(Instruction::JumpLess(*self.labels.get(&l).unwrap() as u64))

                }
                PseudoInstruction::JumpGreater(l) => {
                    res.instructions.push(Instruction::JumpGreater(*self.labels.get(&l).unwrap() as u64))

                }
                PseudoInstruction::JumpLessEq(l) => {
                    res.instructions.push(Instruction::JumpLessEq(*self.labels.get(&l).unwrap() as u64))

                }
                PseudoInstruction::JumpGreaterEq(l) => {
                    res.instructions.push(Instruction::JumpGreaterEq(*self.labels.get(&l).unwrap() as u64))

                }
                PseudoInstruction::JumpEq(l) => {
                    res.instructions.push(Instruction::JumpEq(*self.labels.get(&l).unwrap() as u64))

                }
                PseudoInstruction::JumpNEq(l) => {
                    res.instructions.push(Instruction::JumpNEq(*self.labels.get(&l).unwrap() as u64))

                }
            }
        }

        self.labels.clear();
        self.entry = 0;


        res
    }
}

pub fn link(execs: &[LPseudoExecutable]) -> LExecutable {
    let mut builder = LExecutableBuilder::new();
    
    

    builder.build()
}
