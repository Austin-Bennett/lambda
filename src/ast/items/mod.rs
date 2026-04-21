use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::items::function::FunctionSyntax;
use crate::ast::items::structure::StructureSyntax;
use crate::common::sourcemap::SourceMap;
use crate::compiler::Compiler;
use crate::lexer::token::Token;

pub mod structure;
pub mod function;



pub enum Item {
    Func(FunctionSyntax),
    Struct(StructureSyntax),
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Func(func) => write!(f, "{:?}", func.data),
            Item::Struct(s) => write!(f, "{:?}", s.data)
        }
    }
}

pub type ItemSyntax = GenericSyntax<Item>;

impl ItemSyntax {
    
}

impl Syntax for ItemSyntax {
    fn parse(tokens: &mut VecDeque<Token>, compiler: &mut Compiler) -> Option<Self>
    where
        Self: Sized
    {

        if let Some(func) = FunctionSyntax::parse(tokens, compiler) {
            Some(
                Self{
                    smap: func.smap.clone(),
                    data: Item::Func(func),
                }
            )
        } else if let Some(structure) = StructureSyntax::parse(tokens, compiler) {
            Some(
                Self{
                    smap: structure.smap.clone(),
                    data: Item::Struct(structure),
                }
            )
        } else {
            None
        }


    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}