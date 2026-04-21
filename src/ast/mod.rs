use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use crate::common::sourcemap::SourceMap;
use crate::compiler::Compiler;
use crate::lexer::token::Token;


pub mod block;
pub mod items;
pub mod ty;
pub mod statements;

pub use items::*;
pub trait Syntax {
    fn parse(tokens: &mut VecDeque<Token>, context: &mut Compiler) -> Option<Self> where Self: Sized;

    fn get_sourcemap(&self) -> &SourceMap;
}

#[derive(Clone)]
pub struct GenericSyntax<T> {
    pub data: T,
    pub smap: SourceMap,
}

impl<T: Hash> Hash for GenericSyntax<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
        self.smap.hash(state);
    }
}

impl<T> GenericSyntax<T> {
    pub fn new(data: T, smap: SourceMap) -> Self {
        Self{
            data,
            smap
        }
    }
}