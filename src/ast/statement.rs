use std::collections::VecDeque;
use std::intrinsics::abort;
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::expressions::Expr;
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::compiler::CompileMessage;
use crate::lexer::token::{ExpressionToken, StatementToken, Token, TokenType};
use crate::{match_tks, token_match};

pub enum Statement {
    VariableDeclaration{
        type_id: ModulePath,
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
        if token_match!(
            tokens, 
        ) {
            
        }
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}