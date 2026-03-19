use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::statements::vardecl::*;
use crate::ast::statements::expressions::*;
use crate::ast::statements::ret::{Return, ReturnSyntax};
use crate::common::sourcemap::SourceMap;
use crate::compiler::Compiler;
use crate::lexer::token::Token;
pub mod vardecl;
pub mod expressions;
pub mod ret;

pub enum Statement {
    VariableDeclaration(VarDecl),
    Expression(Expr),
    Return(Return)
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::VariableDeclaration(vd) => {
                write!(f, "{:?}", vd)?;
            }
            Statement::Expression(e) => {
                write!(f, "{:?}", e)?;
            }
            Statement::Return(r) => {
                write!(f, "return {:?}", r.deref())?;
            }
        }
        Ok(())
    }
}

pub type StatementSyntax = GenericSyntax<Statement>;


impl Syntax for StatementSyntax {
    fn parse(tokens: &mut VecDeque<Token>, compiler: &mut Compiler) -> Option<Self>
    where
        Self: Sized
    {
        if let Some(vardecl) = VarDeclSyntax::parse(tokens, compiler) {
            Some(
                StatementSyntax::new(
                    Statement::VariableDeclaration(vardecl.data),
                    vardecl.smap
                )
            )
        } else if let Some(ret) = ReturnSyntax::parse(tokens, compiler) {
            Some(
                StatementSyntax::new(
                    Statement::Return(ret.data),
                    ret.smap
                )
            )
        } else {
            match ExprSyntax::parse(tokens, compiler) {
                Some(v) => {
                    Some(
                        StatementSyntax::new(
                            Statement::Expression(v.data),
                            v.smap
                        )
                    )
                }
                None => None
            }

        }
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}