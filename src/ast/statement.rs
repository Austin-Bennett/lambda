use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::process::abort;
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::expressions::{Expr, ExprSyntax};
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::token::{ExpressionToken, FeatureToken, StatementToken, Token, TokenType};
use crate::{token_match, unpack_opt_tk, unpack_tk};
use crate::ast::common::{VarDecl, VarDeclSyntax};

pub enum Statement {
    VariableDeclaration(VarDecl),
    Expression(Expr),
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