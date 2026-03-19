use std::collections::VecDeque;
use std::ops::{Deref, DerefMut, DerefPure};
use std::process::abort;
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::statements::expressions::{Expr, ExprSyntax};
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::token::{StatementToken, Token, TokenType};
use crate::unpack_opt_tk;

pub struct Return(Expr);

impl Deref for Return {
    type Target = Expr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Return {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl DerefPure for Return {}

pub type ReturnSyntax = GenericSyntax<Return>;



impl Syntax for ReturnSyntax {
    fn parse(tokens: &mut VecDeque<Token>, context: &mut Compiler) -> Option<Self>
    where
        Self: Sized
    {
        let unpack_opt_tk!( TokenType::Statement(StatementToken::ReturnKW), _) = tokens.get(0) else {
            return None;
        };
        let unpack_opt_tk!( TokenType::Statement(StatementToken::ReturnKW), mut smap) = tokens.pop_front() else { abort(); };

        let expr = match ExprSyntax::parse(tokens, context) {
            Some(e) => e,
            None => {
                context.emit_compile_message(
                    CompileMessage::new(
                        smap,
                        "expected expression after return keyword!".to_string(),
                        CompileMessageType::Error,
                    )
                );

                return None;
            }
        };

        smap.extend(expr.smap);

        Some(ReturnSyntax{
            data: Return(expr.data),
            smap
        })
    }

    fn get_sourcemap(&self) -> &SourceMap {
        todo!()
    }
}