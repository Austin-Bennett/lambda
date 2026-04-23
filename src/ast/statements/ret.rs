use std::collections::VecDeque;
use std::ops::{Deref, DerefMut, DerefPure};
use crate::ast::Syntax;
use crate::ast::statements::expressions::ExprSyntax;
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::token::{StatementToken, Token, TokenType};
use crate::unpack_opt_tk;

pub struct Return(pub ExprSyntax);

impl Deref for Return {
    type Target = ExprSyntax;

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




impl Syntax for Return {
    fn parse(tokens: &mut VecDeque<Token>, context: &mut Compiler) -> Option<Self>
    where
        Self: Sized
    {
        let unpack_opt_tk!( TokenType::Statement(StatementToken::ReturnKW), _) = tokens.get(0) else {
            return None;
        };
        let unpack_opt_tk!( TokenType::Statement(StatementToken::ReturnKW), mut smap) = tokens.pop_front() else { unreachable!() };

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

        smap.extend(&expr.smap);

        Some(Return(
            expr
        ))
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}