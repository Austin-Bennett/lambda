use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::process::abort;
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::statements::expressions::ExprSyntax;
use crate::ast::ty::{Type, TypeSyntax};
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::token::{ExpressionToken, FeatureToken, Token, TokenType};
use crate::token_match;
use crate::unpack_opt_tk;

#[derive(Hash, Clone)]
pub struct VarDecl {
    pub name: String,
    pub ty: Type,
    pub value: Option<ExprSyntax>
}

impl Debug for VarDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {:?}", self.name, self.ty)?;
        
        if let Some(e) = &self.value {
            write!(f, " = {:?}", e.data)?;
        }
        
        Ok(())
    }
}

pub type VarDeclSyntax = GenericSyntax<VarDecl>;

impl Syntax for VarDeclSyntax {
    fn parse(tokens: &mut VecDeque<Token>, compiler: &mut Compiler) -> Option<Self>
    where
        Self: Sized
    {
        //name : type

        if token_match!( tokens,
            TokenType::Expression(ExpressionToken::Identifier(_)),
            TokenType::Feature(FeatureToken::Colon),
        ) {
            let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Identifier(name)), mut smap) = tokens.pop_front() else { abort() };
            tokens.pop_front();
            let ty = if let Some(ty) = TypeSyntax::parse(tokens, compiler) {
                smap.extend(ty.smap);
                ty.data
            } else {
                compiler.emit_compile_message(CompileMessage::new(
                    smap,
                    format!("expected type after variable declaration: {:?}", name),
                    CompileMessageType::Error
                ));
                return None;
            };


            //check for an expression
            let expr = if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Operator(
                Operator{ tk: "=", .. }
            )), _) = tokens.get(0) {
                let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Operator(Operator{ tk: "=", .. })), opmap) = tokens.pop_front() else { abort() };


                let expr = match ExprSyntax::parse(tokens, compiler) {
                    Some(v) => v,
                    None => {
                        smap.extend(opmap);
                        compiler.emit_compile_message(CompileMessage::new(
                            smap,
                            "expected expression after '='".to_string(),
                            CompileMessageType::Error
                        ));
                        return None;
                    }
                };
                smap.extend(&expr.smap);
                Some(expr)
            } else {
                None
            };


            Some(
                VarDeclSyntax{
                    smap,
                    data: VarDecl{
                        name,
                        ty,
                        value: expr
                    }
                }
            )
        } else {
            None
        }
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}