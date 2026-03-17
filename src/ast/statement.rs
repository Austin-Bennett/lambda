use std::collections::VecDeque;
use std::iter::Peekable;
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::expressions::Expr;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::compiler::CompileMessage;
use crate::lexer::token::{Token, TokenType};

pub enum Statement {
    VariableDeclaration{
        typeid: ModulePath,
        name: ModulePath,
        val: Option<Expr>
    },
    Expression(Expr),
}



pub type StatementSyntax = GenericSyntax<Statement>;


impl Syntax for StatementSyntax {
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