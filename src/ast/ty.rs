use std::collections::VecDeque;
use std::fmt::{write, Debug, Formatter};
use std::process::abort;
use std::ptr;
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::statements::expressions::{Expr, ExprSyntax};
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::compiler::{CompileMessage, Compiler};
use crate::lexer::token::{ExpressionToken, Token, TokenType};
use crate::{unpack_opt_tk, unpack_tk};


pub enum Type {
    Typename(ModulePath),
    Reference(Box<Type>),
    Pointer(Box<Type>),
    Slice(Box<Type>),
    Array{ty: Box<Type>, size: Expr},
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Typename(mp) => write!(f, "{}", mp),
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
                let unpack_opt_tk!(_, smap) = tokens.pop_front() else { abort() };

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
                let unpack_opt_tk!(_, smap) = tokens.pop_front() else { abort() };

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
                let unpack_opt_tk!(_, smap) = tokens.pop_front() else { abort() };

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
                        self.smap.extend(expr.smap);
                        unsafe {
                            ptr::write(&raw mut self.data, Type::Array {
                                ty: Box::new(ptr::read(&raw mut self.data) ),
                                size: expr.data
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
            .pop_front_if(|tk| if let unpack_tk!(TokenType::Expression(ExpressionToken::Identifier(s)), _) = tk {
                true
            } else {
                false
            })
        else { return None; };

        let mut res = TypeSyntax::new(
            Type::Typename(ModulePath::from_module_path(s)),
            smap
        );

        loop {
            if !res.parse_next_modifier(tokens, compiler) { break; }
        }

        Some(res)
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}