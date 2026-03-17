use std::collections::VecDeque;
use std::iter::Peekable;
use crate::ast::statement::Statement;
use crate::ast::{GenericSyntax, Syntax};
use crate::common::sourcemap::SourceMap;
use crate::common::utils::outcome::Outcome;
use crate::compiler::CompileMessage;
use crate::lexer::token::Token;

pub type Block = Vec<Statement>;

pub type BlockSyntax = GenericSyntax<Block>;

impl Syntax for BlockSyntax {
    fn parse(tokens: &mut VecDeque<Token>) -> Outcome<Self, CompileMessage>
    where
        Self: Sized
    {
        todo!()
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}