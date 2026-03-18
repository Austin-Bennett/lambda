use std::collections::VecDeque;
use std::iter::Fuse;
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::items::function::{Function, FunctionSyntax};
use crate::ast::items::structure::{Structure, StructureSyntax};
use crate::common::sourcemap::SourceMap;
use crate::common::utils::outcome::Outcome;
use crate::compiler::CompileMessage;
use crate::lexer::token::Token;

pub mod structure;
pub mod function;


#[derive(Debug)]
pub enum Item {
    Func(Function),
    Struct(Structure),
}

pub type ItemSyntax = GenericSyntax<Item>;

impl Syntax for ItemSyntax {
    fn parse(tokens: &mut VecDeque<Token>) -> Outcome<Self, CompileMessage>
    where
        Self: Sized
    {

        if let Some(func) = FunctionSyntax::parse(tokens)? {
            Outcome::Ok(
                Self{
                    data: Item::Func(func.data),
                    smap: func.smap,
                }
            )
        } else if let Some(structure) = StructureSyntax::parse(tokens)? {
            Outcome::Ok(
                Self{
                    data: Item::Struct(structure.data),
                    smap: structure.smap,
                }
            )
        } else {
            Outcome::None
        }


    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}