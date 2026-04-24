use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::statements::expressions::ExprSyntax;
use crate::ast::ty::{Type, TypeSyntax};
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::token::{ExpressionToken, FeatureToken, StatementToken, Token, TokenType};
use crate::token_match;
use crate::unpack_opt_tk;

#[derive(Hash, Clone)]
pub struct VarDecl {
    pub name: String,
    pub ty: Option<Type>,
    pub value: Option<ExprSyntax>,
    pub public: bool,
}

impl Debug for VarDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.ty {
            Some(ty) => write!(f, "{} : {:?}", self.name, ty)?,
            None => write!(f, "let {}", self.name)?,
        }
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
        // consume optional 'public' keyword
        let (public, public_smap) = if let Some(Token { typ: TokenType::Statement(StatementToken::PublicKW), .. }) = tokens.get(0) {
            let tok = tokens.pop_front().unwrap();
            (true, Some(tok.smap))
        } else {
            (false, None)
        };

        // --- let name = expr  (type-inferred) ---
        if let Some(Token { typ: TokenType::Statement(StatementToken::LetKW), .. }) = tokens.get(0) {
            let let_tok = tokens.pop_front().unwrap();
            let mut smap = if let Some(mut psmap) = public_smap { psmap.extend(let_tok.smap.clone()); psmap } else { let_tok.smap };

            let Some(Token { typ: TokenType::Expression(ExpressionToken::Identifier(_)), .. }) = tokens.get(0) else {
                compiler.emit_compile_message(CompileMessage::new(
                    smap,
                    "expected identifier after 'let'".to_string(),
                    CompileMessageType::Error,
                ));
                return None;
            };
            let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Identifier(name)), ident_smap) = tokens.pop_front() else { unreachable!() };
            smap.extend(ident_smap);

            // require '='
            let Some(Token { typ: TokenType::Expression(ExpressionToken::Operator(Operator { tk: "=", .. })), .. }) = tokens.get(0) else {
                compiler.emit_compile_message(CompileMessage::new(
                    smap,
                    format!("expected '=' after 'let {}'", name),
                    CompileMessageType::Error,
                ));
                return None;
            };
            let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Operator(Operator { tk: "=", .. })), opmap) = tokens.pop_front() else { unreachable!() };

            let expr = match ExprSyntax::parse(tokens, compiler) {
                Some(v) => v,
                None => {
                    smap.extend(opmap);
                    compiler.emit_compile_message(CompileMessage::new(
                        smap,
                        format!("expected expression after 'let {} ='", name),
                        CompileMessageType::Error,
                    ));
                    return None;
                }
            };
            smap.extend(&expr.smap);

            return Some(VarDeclSyntax {
                smap,
                data: VarDecl { name, ty: None, value: Some(expr), public },
            });
        }

        // --- name : type [= expr]  (explicit type) ---
        if token_match!( tokens,
            TokenType::Expression(ExpressionToken::Identifier(_)),
            TokenType::Feature(FeatureToken::Colon),
        ) {
            let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Identifier(name)), ident_smap) = tokens.pop_front() else { unreachable!() };
            tokens.pop_front();
            let mut smap = if let Some(mut psmap) = public_smap { psmap.extend(ident_smap); psmap } else { ident_smap };
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

            let expr = if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Operator(
                Operator{ tk: "=", .. }
            )), _) = tokens.get(0) {
                let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Operator(Operator{ tk: "=", .. })), opmap) = tokens.pop_front() else { unreachable!() };
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

            Some(VarDeclSyntax {
                smap,
                data: VarDecl { name, ty: Some(ty), value: expr, public },
            })
        } else {
            None
        }
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}