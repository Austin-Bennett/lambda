use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use crate::ast::Syntax;
use crate::ast::statements::vardecl::*;
use crate::ast::statements::expressions::*;
use crate::ast::statements::ret::{Return};
use crate::common::sourcemap::SourceMap;
use crate::compiler::Compiler;
use crate::lexer::token::Token;
pub mod vardecl;
pub mod expressions;
pub mod ret;

pub enum Statement {
    VariableDeclaration(VarDeclSyntax),
    Expression(ExprSyntax),
    Return(Return)
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::VariableDeclaration(vd) => {
                write!(f, "{:?}", vd.data)?;
            }
            Statement::Expression(e) => {
                write!(f, "{:?}", e.data)?;
            }
            Statement::Return(r) => {
                write!(f, "return {:?}", r.deref().data)?;
            }
        }
        Ok(())
    }
}



impl Syntax for Statement {
    fn parse(tokens: &mut VecDeque<Token>, compiler: &mut Compiler) -> Option<Self>
    where
        Self: Sized
    {
        if let Some(vardecl) = VarDeclSyntax::parse(tokens, compiler) {
            Some(
                Statement::VariableDeclaration(
                    vardecl,
                )
            )
        } else if let Some(ret) = Return::parse(tokens, compiler) {
            Some(
                    Statement::Return(ret),
            )
        } else {
            match ExprSyntax::parse(tokens, compiler) {
                Some(v) => {
                    Some(
                        Statement::Expression(v),
                    )
                }
                None => None
            }

        }
    }

    fn get_sourcemap(&self) -> &SourceMap {
        
        match self {
            Statement::VariableDeclaration(vd) => { vd.get_sourcemap() }
            Statement::Expression(e) => { e.get_sourcemap() }
            Statement::Return(r) => { r.get_sourcemap() }
        }
    }
}