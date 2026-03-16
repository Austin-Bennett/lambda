use std::iter::Peekable;
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompilerError};
use crate::lexer::token::Token;

pub trait Syntax {
    fn parse(tokens: &mut Peekable<impl Iterator<Item=Token>>) -> CompilerError<Self> where Self: Sized;

    fn get_sourcemap(&self) -> &SourceMap;
}

pub mod expressions;