use std::collections::VecDeque;
use crate::common::sourcemap::SourceMap;
use crate::compiler::Compiler;
use crate::lexer::token::Token;


pub mod block;
pub mod items;
pub mod ty;
pub mod statements;

pub use {items::*, items::*};
pub trait Syntax {
    fn parse(tokens: &mut VecDeque<Token>, context: &mut Compiler) -> Option<Self> where Self: Sized;

    fn get_sourcemap(&self) -> &SourceMap;
}

pub struct GenericSyntax<T> {
    pub data: T,
    pub smap: SourceMap,
}

impl<T> GenericSyntax<T> {
    pub fn new(data: T, smap: SourceMap) -> Self {
        Self{
            data,
            smap
        }
    }
}