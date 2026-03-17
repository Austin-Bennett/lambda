use std::collections::VecDeque;
use std::iter::Peekable;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::outcome::Outcome;
use crate::compiler::CompileMessage;
use crate::lexer::token::Token;

pub trait Syntax {
    fn parse(tokens: &mut VecDeque<Token>) -> Outcome<Self, CompileMessage> where Self: Sized;

    fn get_sourcemap(&self) -> &SourceMap;
}

pub mod expressions;
pub mod block;
pub mod statement;


pub struct GenericSyntax<T> {
    pub data: T,
    pub smap: SourceMap,
}