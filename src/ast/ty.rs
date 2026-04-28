use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::ptr;
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::statements::expressions::ExprSyntax;
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, Compiler};
use crate::lexer::token::{ExpressionToken, FeatureToken, Token, TokenType};
use crate::{unpack_opt_tk, unpack_tk};
use crate::consteval::eval_array_len_expr_uint;
use crate::typed_ast::typing::tcontext::TypeContext;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Type {
    Typename(String),
    Generic { name: String, params: Vec<Type> },
    Reference(Box<Type>),
    Pointer(Box<Type>),
    Slice(Box<Type>),
    Array{ty: Box<Type>, size: usize},
    FnPtr { ret: Box<Type>, params: Vec<Type> },
}

impl Type {
    //some types have a known size at compile time without
    //having to resolve their typenames
    /// Returns a stable string suitable for use as a mangled name prefix.
    pub fn mangle_name(&self) -> String {
        match self {
            Type::Typename(n)            => n.clone(),
            Type::Reference(t)           => format!("{}_ref",         t.mangle_name()),
            Type::Pointer(t)             => format!("{}_ptr",         t.mangle_name()),
            Type::Slice(t)               => format!("{}_slice",       t.mangle_name()),
            Type::Array { ty, size }     => format!("{}_array_{}", ty.mangle_name(), size),
            Type::Generic { name, params } => {
                let mut s = name.clone();
                for p in params { s.push('_'); s.push_str(&p.mangle_name()); }
                s
            }
            Type::FnPtr { ret, params } => {
                let mut s = format!("fn_{}", ret.mangle_name());
                for p in params { s.push('_'); s.push_str(&p.mangle_name()); }
                s
            }
        }
    }

    pub fn try_get_size(&self) -> Option<usize> {
        match self {
            Type::Typename(_) => None,
            Type::Generic { .. } => None,
            Type::Reference(_) => Some(TypeContext::SIZE_POINTER),
            Type::Pointer(_) => Some(TypeContext::SIZE_POINTER),
            Type::Slice(_) => Some(TypeContext::SIZE_POINTER * 2),
            Type::Array { .. } => None,
            Type::FnPtr { .. } => Some(TypeContext::SIZE_POINTER),
        }
    }
    
    
}



impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Typename(mp) => write!(f, "{}", mp),
            Type::Generic { name, params } => {
                write!(f, "{}<", name)?;
                let mut first = true;
                for p in params {
                    if !first { write!(f, ", ")?; }
                    first = false;
                    write!(f, "{:?}", p)?;
                }
                write!(f, ">")
            }
            Type::Reference(r) => {
                write!(f, "{:?}&", r)
            }
            Type::Pointer(p) => {
                write!(f, "{:?}*", p)
            }
            Type::Slice(s) => {
                write!(f, "{:?}[]", s)
            }
            Type::Array { ty, size } => {
                write!(f, "{:?}[{:?}]", ty, size)
            }
            Type::FnPtr { ret, params } => {
                write!(f, "{:?}(", ret)?;
                let mut first = true;
                for p in params {
                    if !first { write!(f, ", ")?; }
                    first = false;
                    write!(f, "{:?}", p)?;
                }
                write!(f, ")")
            }
        }
    }
}

pub type TypeSyntax = GenericSyntax<Type>;


impl TypeSyntax {
    pub fn parse_next_modifier(&mut self, tokens: &mut VecDeque<Token>, compiler: &mut Compiler) -> bool {

        let first = match tokens.get(0) {
            Some(v) => v,
            None => return false,
        };

        match first {
            unpack_tk!(TokenType::Expression(
                ExpressionToken::Operator( Operator{ tk: "*", .. } )
            ), _) => {
                let unpack_opt_tk!(_, smap) = tokens.pop_front() else { unreachable!() };

                self.smap.extend(smap);

                unsafe {
                    ptr::write(&raw mut self.data, Type::Pointer(
                        Box::new(ptr::read(&raw mut self.data))
                    ))
                }

                true
            },
            unpack_tk!(TokenType::Expression(
                ExpressionToken::Operator( Operator{ tk: "&", .. } )
            ), _) => {
                let unpack_opt_tk!(_, smap) = tokens.pop_front() else { unreachable!() };

                self.smap.extend(smap);

                unsafe {
                    ptr::write(&raw mut self.data, Type::Reference(
                        Box::new(ptr::read(&raw mut self.data))
                    ))
                }

                true
            }
            unpack_tk!(TokenType::Expression(
                ExpressionToken::OpenBracket
            ), _) => {
                let unpack_opt_tk!(_, smap) = tokens.pop_front() else { unreachable!() };

                let open_brack_smap = smap.clone();

                self.smap.extend(smap);

                //check for a bracket, if no bracket, try to parse an expression, otherwise produce
                //an error and return false

                if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::CloseBracket), _) = tokens.get(0) {
                    unsafe {
                        ptr::write(&raw mut self.data, Type::Slice(
                            Box::new(ptr::read(&raw mut self.data))
                        ))
                    }
                } else {
                    if let Some(expr) = ExprSyntax::parse(tokens, compiler) {
                        self.smap.extend(expr.smap.clone());
                        unsafe {
                            ptr::write(&raw mut self.data, Type::Array {
                                ty: Box::new(ptr::read(&raw mut self.data) ),
                                size: match eval_array_len_expr_uint(&expr, compiler) {
                                    Some(v) => v,
                                    None => return false,
                                } as usize
                            })
                        }
                    }
                }

                let next = tokens.pop_front();
                if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::CloseBracket), smap) = next {
                    self.smap.extend(smap);
                } else {
                    compiler.emit_compile_message(

                        CompileMessage::expected_token_error(
                            self.smap.clone(),
                            "']'",
                            "'['",
                            next
                        )
                            .add_note(CompileMessage::note(
                                open_brack_smap,
                                "after this open bracket".to_string()
                            ))
                    );

                    return false;
                };

                true
            }


            unpack_tk!(TokenType::Expression(ExpressionToken::OpenParentheses), _) => {
                let unpack_opt_tk!(_, smap) = tokens.pop_front() else { unreachable!() };
                self.smap.extend(smap);

                let mut params = Vec::new();
                loop {
                    if let Some(unpack_tk!(TokenType::Expression(ExpressionToken::CloseParentheses), _)) = tokens.get(0) {
                        break;
                    }
                    match TypeSyntax::parse(tokens, compiler) {
                        Some(p) => params.push(p.data),
                        None => break,
                    }
                    if let Some(unpack_tk!(TokenType::Feature(FeatureToken::Comma), _)) = tokens.get(0) {
                        tokens.pop_front();
                    } else {
                        break;
                    }
                }

                let close = tokens.pop_front();
                if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::CloseParentheses), smap) = close {
                    self.smap.extend(smap);
                } else {
                    compiler.emit_compile_message(CompileMessage::expected_token_error(
                        self.smap.clone(), "')'", "'('", close,
                    ));
                    return false;
                }

                unsafe {
                    ptr::write(&raw mut self.data, Type::FnPtr {
                        ret: Box::new(ptr::read(&raw mut self.data)),
                        params,
                    })
                }

                true
            }

            _ => false
        }
    }
}

impl Syntax for TypeSyntax {
    fn parse(tokens: &mut VecDeque<Token>, compiler: &mut Compiler) -> Option<Self>
    where
        Self: Sized
    {

        //always starts with the type identifier
        let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Identifier(s)), smap) = tokens
            .pop_front_if(|tk| if let unpack_tk!(TokenType::Expression(ExpressionToken::Identifier(_)), _) = tk {
                true
            } else {
                false
            })
        else { return None; };

        // Check for generic type params: Name<T, U, ...>
        let base = if let Some(unpack_tk!(
            TokenType::Expression(ExpressionToken::Operator(Operator { tk: "<", .. })), _
        )) = tokens.get(0) {
            tokens.pop_front(); // consume '<'
            let mut params = Vec::new();
            loop {
                // allow trailing '>' with no params for now
                if let Some(unpack_tk!(
                    TokenType::Expression(ExpressionToken::Operator(Operator { tk: ">", .. })), _
                )) = tokens.get(0) {
                    tokens.pop_front();
                    break;
                }
                match TypeSyntax::parse(tokens, compiler) {
                    Some(p) => params.push(p.data),
                    None => break,
                }
                // comma or '>'
                if let Some(unpack_tk!(
                    TokenType::Expression(ExpressionToken::Operator(Operator { tk: ">", .. })), _
                )) = tokens.get(0) {
                    tokens.pop_front();
                    break;
                }
                if let Some(unpack_tk!(TokenType::Feature(FeatureToken::Comma), _)) = tokens.get(0) {
                    tokens.pop_front();
                } else {
                    break;
                }
            }
            Type::Generic { name: s, params }
        } else {
            Type::Typename(s)
        };

        let mut res = TypeSyntax::new(base, smap);

        loop {
            if !res.parse_next_modifier(tokens, compiler) { break; }
        }

        Some(res)
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}