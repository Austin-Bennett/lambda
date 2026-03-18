use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::process::abort;
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::expressions::{Expr, ExprSyntax};
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::compiler::{CompileMessage, CompileMessageType};
use crate::lexer::token::{ExpressionToken, FeatureToken, StatementToken, Token, TokenType};
use crate::{token_match, unpack_opt_tk, unpack_tk};

pub enum Statement {
    VariableDeclaration{
        type_id: ModulePath,
        name: ModulePath,
        val: Option<Expr>
    },
    Expression(Expr),
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::VariableDeclaration { type_id, name, val } => {
                write!(f, "{} : {}", name, type_id)?;
                if let Some(e) = val {
                    write!(f, " = {:?}", e)?;
                }

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
    fn parse(tokens: &mut VecDeque<Token>) -> Outcome<Self, CompileMessage>
    where
        Self: Sized
    {
        if token_match!(
            tokens,
            TokenType::Expression(ExpressionToken::Identifier(_)),
            TokenType::Feature(FeatureToken::Colon),
            TokenType::Expression(ExpressionToken::Identifier(_)),
        ) {
            //ident: ident (identifier, colon, identifier) = variable declaration
            let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Identifier(ident)), mut smap) = tokens.pop_front() else { abort() };
            tokens.pop_front(); //pop the semicolon
            let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Identifier(typeid)), typmap) = tokens.pop_front() else { abort() };

            smap.extend(typmap);

            //check for an expression
            let expr = if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Operator(
                Operator{ tk: "=", .. }
            )), opmap) = tokens.get(0) {
                let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Operator(Operator{ tk: "=", .. })), opmap) = tokens.pop_front() else { abort() };


                let expr = match ExprSyntax::parse(tokens)? {
                    Some(v) => v,
                    None => {
                        smap.extend(opmap);
                        return Outcome::Err(CompileMessage::new(
                            smap,
                            "expected expression after '='".to_string(),
                            CompileMessageType::Error
                        ));
                    }
                };
                smap.extend(expr.smap);
                Some(expr.data)
            } else {
                None
            };

            Outcome::Ok(
                StatementSyntax::new(
                    Statement::VariableDeclaration {
                        type_id: ModulePath::from_module_path(typeid),
                        name: ModulePath::from_module_path(ident),
                        val: expr
                    }
                , smap))
        } else {
            match ExprSyntax::parse(tokens)? {
                Some(v) => {
                    Outcome::Ok(
                        StatementSyntax::new(
                            Statement::Expression(v.data),
                            v.smap
                        )
                    )
                }
                None => Outcome::None
            }
        }
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}