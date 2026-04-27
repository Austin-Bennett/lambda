use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use crate::ast::block::BlockSyntax;
use crate::ast::statements::vardecl::VarDeclSyntax;
use crate::ast::ty::{Type, TypeSyntax};
use crate::ast::{GenericSyntax, Syntax};
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::token::{ExpressionToken, FeatureToken, StatementToken, Token, TokenType};
use crate::unpack_opt_tk;

/// How a method's `self` parameter is passed.
#[derive(Clone, PartialEq, Debug)]
pub enum SelfMode {
    None,   // no self parameter (static method)
    Value,  // fn method(self)  — receiver passed by value
    ByRef,  // fn method(self&) — receiver passed by reference (lvalue required)
}

impl SelfMode {
    pub fn has_self(&self) -> bool {
        !matches!(self, SelfMode::None)
    }
}

#[derive(Clone)]
pub struct MethodDecl {
    pub name: String,
    pub self_mode: SelfMode,
    pub params: Vec<VarDeclSyntax>,
    pub ret: Option<Type>,
    pub body: Option<BlockSyntax>,
    pub public: bool,
}

impl Debug for MethodDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.public { write!(f, "public ")?; }
        write!(f, "{}(", self.name)?;
        match self.self_mode {
            SelfMode::None => {}
            SelfMode::Value => { write!(f, "self")?; }
            SelfMode::ByRef => { write!(f, "self&")?; }
        }
        for p in &self.params {
            write!(f, ", {:?}", p.data)?;
        }
        write!(f, ")")?;
        if let Some(ret) = &self.ret { write!(f, " = {:?}", ret)?; }
        Ok(())
    }
}

#[derive(Clone)]
pub struct OperatorDecl {
    pub op_name: String, // "add", "sub", "mul", "div", "assign", "cmp", "drop"
    pub self_mode: SelfMode,
    pub params: Vec<VarDeclSyntax>,
    pub ret: Option<Type>,
    pub body: Option<BlockSyntax>,
}

impl Debug for OperatorDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "operator {}(", self.op_name)?;
        match self.self_mode {
            SelfMode::None => {}
            SelfMode::Value => { write!(f, "self")?; }
            SelfMode::ByRef => { write!(f, "self&")?; }
        }
        for p in &self.params {
            write!(f, ", {:?}", p.data)?;
        }
        write!(f, ")")?;
        if let Some(ret) = &self.ret { write!(f, " = {:?}", ret)?; }
        Ok(())
    }
}

#[derive(Clone)]
pub struct ModifyBlock {
    pub type_parameters: Vec<String>,
    pub ty: Type,
    pub methods: Vec<MethodDecl>,
    pub operators: Vec<OperatorDecl>,
}

impl Debug for ModifyBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "modify {:?} {{ {} methods, {} operators }}", self.ty, self.methods.len(), self.operators.len())
    }
}

pub type ModifySyntax = GenericSyntax<ModifyBlock>;

impl Syntax for ModifySyntax {
    fn parse(tokens: &mut VecDeque<Token>, compiler: &mut Compiler) -> Option<Self>
    where
        Self: Sized
    {
        let unpack_opt_tk!(TokenType::Statement(StatementToken::ModifyKW), _) = tokens.get(0) else {
            return None;
        };
        let unpack_opt_tk!(TokenType::Statement(StatementToken::ModifyKW), mut smap) = tokens.pop_front() else {
            unreachable!()
        };

        // optional type parameters: modify<T, U> Type { ... }
        let type_parameters = if let Some(Token {
            typ: TokenType::Expression(ExpressionToken::Operator(Operator { tk: "<", .. })), ..
        }) = tokens.get(0) {
            tokens.pop_front(); // consume '<'
            let mut tps = Vec::new();
            loop {
                if let Some(Token {
                    typ: TokenType::Expression(ExpressionToken::Operator(Operator { tk: ">", .. })), ..
                }) = tokens.get(0) {
                    tokens.pop_front();
                    break;
                }
                let next = tokens.pop_front();
                if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Identifier(tp)), _) = next {
                    tps.push(tp);
                } else {
                    compiler.emit_compile_message(CompileMessage::expected_token_error(
                        smap.clone(), "type parameter name", "generic modify declaration", next,
                    ));
                    break;
                }
                if let Some(Token {
                    typ: TokenType::Expression(ExpressionToken::Operator(Operator { tk: ">", .. })), ..
                }) = tokens.get(0) {
                    tokens.pop_front();
                    break;
                }
                if let Some(Token { typ: TokenType::Feature(FeatureToken::Comma), .. }) = tokens.get(0) {
                    tokens.pop_front();
                } else {
                    break;
                }
            }
            tps
        } else {
            Vec::new()
        };

        // Parse the type being modified
        let Some(ty_syntax) = TypeSyntax::parse(tokens, compiler) else {
            compiler.emit_compile_message(CompileMessage::new(
                smap,
                "expected type after 'modify'".into(),
                CompileMessageType::Error,
            ));
            return None;
        };
        smap.extend(&ty_syntax.smap);
        let ty = ty_syntax.data;

        // Consume '{'
        let next = tokens.pop_front();
        let unpack_opt_tk!(TokenType::Feature(FeatureToken::OpenBrace), bsmap) = next else {
            compiler.emit_compile_message(CompileMessage::expected_token_error(
                smap,
                "{",
                format!("modify {:?}", ty),
                next,
            ));
            return None;
        };
        smap.extend(bsmap);

        let mut methods = Vec::new();
        let mut operators = Vec::new();

        loop {
            // Skip semicolons
            while let Some(Token { typ: TokenType::Feature(FeatureToken::StatementEnd), .. }) = tokens.get(0) {
                tokens.pop_front();
            }

            // Check for closing brace
            if let unpack_opt_tk!(TokenType::Feature(FeatureToken::CloseBrace), cbsmap) = tokens.get(0) {
                smap.extend(cbsmap);
                tokens.pop_front();
                break;
            }

            // Check for EOF
            if tokens.is_empty() {
                compiler.emit_compile_message(CompileMessage::new(
                    smap.clone(),
                    "expected '}' to close modify block".into(),
                    CompileMessageType::Error,
                ));
                break;
            }

            // Optional 'public' keyword — methods are private by default
            let (public, pub_smap) = if let Some(Token {
                typ: TokenType::Statement(StatementToken::PublicKW), ..
            }) = tokens.get(0) {
                let tok = tokens.pop_front().unwrap();
                (true, Some(tok.smap))
            } else {
                (false, None)
            };
            
            //fn kw
            let next = tokens.pop_front();
            if let unpack_opt_tk!(TokenType::Statement(StatementToken::FnKW), fmap) = next {
                smap.extend(fmap);
            } else {
                compiler.emit_compile_message(
                    CompileMessage::expected_token_in_error(
                        smap,
                        "fn keyword",
                        "modify block declaration",
                        next
                    )
                );
                return None;
            }

            // Check for 'operator' keyword
            if let Some(Token { typ: TokenType::Statement(StatementToken::OperatorKW), .. }) = tokens.get(0) {
                if public {
                    compiler.emit_compile_message(CompileMessage::new(
                        smap.clone(),
                        "operators are always public; 'public' keyword is redundant here".into(),
                        CompileMessageType::Warning,
                    ));
                }
                tokens.pop_front(); // consume 'operator'

                // Operator name
                let next = tokens.pop_front();
                let (op_name, opsmap) = if let unpack_opt_tk!(
                    TokenType::Expression(ExpressionToken::Identifier(name)), imap
                ) = next {
                    (name, imap)
                } else {
                    compiler.emit_compile_message(CompileMessage::expected_token_error(
                        smap.clone(), "operator name", "operator declaration", next,
                    ));
                    continue;
                };

                // Consume '('
                let next = tokens.pop_front();
                if !matches!(next, unpack_opt_tk!(TokenType::Expression(ExpressionToken::OpenParentheses), _)) {
                    compiler.emit_compile_message(CompileMessage::expected_token_error(
                        opsmap.clone(), "'('", format!("operator '{}'", op_name), next,
                    ));
                    continue;
                }

                // Optional 'self' or 'self&'
                let self_mode = if let Some(Token {
                    typ: TokenType::Expression(ExpressionToken::Identifier(s)), ..
                }) = tokens.get(0) && s == "self" {
                    tokens.pop_front();
                    let mode = if let Some(Token {
                        typ: TokenType::Expression(ExpressionToken::Operator(Operator { tk: "&", .. })), ..
                    }) = tokens.get(0) {
                        tokens.pop_front();
                        SelfMode::ByRef
                    } else {
                        SelfMode::Value
                    };
                    if let Some(Token { typ: TokenType::Feature(FeatureToken::Comma), .. }) = tokens.get(0) {
                        tokens.pop_front();
                    }
                    mode
                } else {
                    SelfMode::None
                };

                // Parse remaining params
                let mut op_params = Vec::new();
                let mut ended = false;
                while let Some(vdecl) = VarDeclSyntax::parse(tokens, compiler) {
                    op_params.push(vdecl);
                    let next = tokens.pop_front();
                    if let unpack_opt_tk!(TokenType::Feature(FeatureToken::Comma), _) = next {
                        // continue
                    } else if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::CloseParentheses), _) = next {
                        ended = true;
                        break;
                    } else {
                        compiler.emit_compile_message(CompileMessage::expected_token_error(
                            opsmap.clone(), "')'", format!("operator '{}'", op_name), next,
                        ));
                        break;
                    }
                }
                if !ended {
                    let next = tokens.pop_front();
                    if !matches!(next, unpack_opt_tk!(TokenType::Expression(ExpressionToken::CloseParentheses), _)) {
                        compiler.emit_compile_message(CompileMessage::expected_token_error(
                            opsmap.clone(), "')'", format!("operator '{}'", op_name), next,
                        ));
                    }
                }

                // Optional return type: '= Type'
                let op_ret = if let Some(Token {
                    typ: TokenType::Expression(ExpressionToken::Operator(Operator { tk: "=", .. })), ..
                }) = tokens.get(0) {
                    tokens.pop_front();
                    TypeSyntax::parse(tokens, compiler).map(|t| t.data)
                } else {
                    None
                };

                // Parse body
                let body = BlockSyntax::parse(tokens, compiler);
                if body.is_none() {
                    compiler.emit_compile_message(CompileMessage::new(
                        opsmap.clone(),
                        format!("expected '{{' to begin body of operator '{}'", op_name),
                        CompileMessageType::Error,
                    ));
                }
                smap.extend(opsmap);
                operators.push(OperatorDecl {
                    op_name,
                    self_mode,
                    params: op_params,
                    ret: op_ret,
                    body,
                });
                continue;
            }

            // Method name
            let next = tokens.pop_front();
            let (method_name, mut msmap) = if let unpack_opt_tk!(
                TokenType::Expression(ExpressionToken::Identifier(name)), imap
            ) = next {
                (name, imap)
            } else {
                compiler.emit_compile_message(CompileMessage::expected_token_error(
                    smap.clone(),
                    "method name",
                    "modify block",
                    next,
                ));
                continue;
            };
            if let Some(ps) = pub_smap { let mut ps2 = ps; ps2.extend(msmap.clone()); msmap = ps2; }

            // Consume '('
            let next = tokens.pop_front();
            let unpack_opt_tk!(TokenType::Expression(ExpressionToken::OpenParentheses), _) = next else {
                compiler.emit_compile_message(CompileMessage::expected_token_error(
                    msmap,
                    "('",
                    format!("method '{}'", method_name),
                    next,
                ));
                continue;
            };

            // Check for bare 'self' or 'self&' as first parameter
            let self_mode = if let Some(Token {
                typ: TokenType::Expression(ExpressionToken::Identifier(s)), ..
            }) = tokens.get(0)
                && s == "self"
            {
                tokens.pop_front(); // consume 'self'
                let mode = if let Some(Token {
                    typ: TokenType::Expression(ExpressionToken::Operator(Operator { tk: "&", .. })), ..
                }) = tokens.get(0) {
                    tokens.pop_front(); // consume '&'
                    SelfMode::ByRef
                } else {
                    SelfMode::Value
                };
                if let Some(Token {
                    typ: TokenType::Feature(FeatureToken::Comma), ..
                }) = tokens.get(0)
                {
                    tokens.pop_front();
                }
                mode
            } else {
                SelfMode::None
            };

            // Parse remaining parameters
            let mut params = Vec::new();
            let mut ended = false;
            while let Some(vdecl) = VarDeclSyntax::parse(tokens, compiler) {
                if vdecl.data.value.is_some() {
                    compiler.emit_compile_message(CompileMessage::new(
                        vdecl.smap.clone(),
                        "default parameter values are not allowed in methods".into(),
                        CompileMessageType::Error,
                    ));
                }
                params.push(vdecl);

                let next = tokens.pop_front();
                if let unpack_opt_tk!(TokenType::Feature(FeatureToken::Comma), _) = next {
                    // continue
                } else if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::CloseParentheses), _) = next {
                    ended = true;
                    break;
                } else {
                    compiler.emit_compile_message(CompileMessage::expected_token_error(
                        msmap.clone(),
                        "')'",
                        format!("method '{}' parameters", method_name),
                        next,
                    ));
                    break;
                }
            }

            if !ended {
                let next = tokens.pop_front();
                if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::CloseParentheses), _) = next {
                    // ok
                } else {
                    compiler.emit_compile_message(CompileMessage::expected_token_error(
                        msmap.clone(),
                        "')'",
                        format!("method '{}' parameters", method_name),
                        next,
                    ));
                }
            }

            // Optional return type: '= Type'
            let ret = if let Some(Token {
                typ: TokenType::Expression(ExpressionToken::Operator(Operator { tk: "=", .. })), ..
            }) = tokens.get(0)
            {
                tokens.pop_front();
                TypeSyntax::parse(tokens, compiler).map(|t| t.data)
            } else {
                None
            };

            // Parse body
            let body = BlockSyntax::parse(tokens, compiler);
            if body.is_none() {
                compiler.emit_compile_message(CompileMessage::new(
                    msmap.clone(),
                    format!("expected '{{' to begin body of method '{}'", method_name),
                    CompileMessageType::Error,
                ));
            }

            if let Some(ref body_syntax) = body {
                smap.extend(&body_syntax.smap);
            }
            smap.extend(msmap);
            methods.push(MethodDecl {
                name: method_name,
                self_mode,
                params,
                ret,
                body,
                public,
            });
        }

        Some(ModifySyntax {
            smap,
            data: ModifyBlock { type_parameters, ty, methods, operators },
        })
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}
