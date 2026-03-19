use std::collections::VecDeque;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::outcome::Outcome;
use crate::compiler::{CompileMessage, Compiler};
use crate::lexer::token::Token;



pub mod expressions;
pub mod block;
pub mod statement;
pub mod items;
pub mod common;
pub mod ty;

pub use {items::*, block::*, statement::*, items::*, common::*, ty::*};

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