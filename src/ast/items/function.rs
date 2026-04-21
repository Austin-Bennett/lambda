use crate::ast::block::BlockSyntax;
use crate::ast::statements::vardecl::VarDeclSyntax;
use crate::ast::ty::{Type, TypeSyntax};
use crate::ast::{GenericSyntax, Syntax};
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::token::{ExpressionToken, FeatureToken, StatementToken, Token, TokenType};
use crate::unpack_opt_tk;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};


//todo: return value
pub struct Function {
    pub name: String,
    pub parameters: Vec<VarDeclSyntax>,
    pub body: Option<BlockSyntax>,
    pub ty: Option<Type>, //None for void

    //i.e C compliant (if it has a body) or declared in an externally linked file
    pub is_extern: bool,
}



impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_extern {
            write!(f, "extern ")?;
        }

        write!(f, "{}(", self.name)?;

        let mut first = true;
        for p in &self.parameters {
            if !first {
                f.write_str(", ")?;
            }
            first = false;
            write!(f, "{:?}", p.data)?;
        }

        f.write_str(")")?;

        if let Some(ty) = &self.ty {
            write!(f, " = {:?}", ty)?;
        }

        if let Some(data) = &self.body {
            write!(f, " {{{}", if data.data.is_empty() { "" } else { "\n" })?;

            for s in &data.data {
                write!(f, "\t{:?}\n", s)?;
            }

            write!(f, "}}")?;
        }


        Ok(())
    }
}

pub type FunctionSyntax = GenericSyntax<Function>;

impl Syntax for FunctionSyntax {
    fn parse(tokens: &mut VecDeque<Token>, compiler: &mut Compiler) -> Option<Self>
    where
        Self: Sized
    {
        let is_extern = if let unpack_opt_tk!( TokenType::Statement(StatementToken::ExternKW), _ ) = tokens.get(0) {
            true
        } else {
            false
        };
        let ind = if is_extern { 1 } else { 0 };

        let unpack_opt_tk!( TokenType::Statement(StatementToken::FnKW), _ ) = tokens.get(ind) else { return None };

        let smap = if is_extern {
            let unpack_opt_tk!( TokenType::Statement(StatementToken::ExternKW), smap ) = tokens.pop_front()
            else { unreachable!() };
            Some(smap)
        } else {
            None
        };
        let unpack_opt_tk!( TokenType::Statement(StatementToken::FnKW), fnsmap ) = tokens.pop_front() else { unreachable!() };

        let mut smap = if let Some(mut smap) = smap {
            smap.extend(fnsmap);
            smap
        } else {
            fnsmap
        };


        //get the name next
        let next = tokens.pop_front();
        let name = if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Identifier(s)), imap) = next {
            smap.extend(imap);
            s
        } else {
            compiler.emit_compile_message(
                CompileMessage::expected_token_error(
                    smap,
                    "identifier",
                    "'fn' keyword",
                    next
                )
            );
            return None;
        };

        //expect an open parentheses
        let next = tokens.pop_front();
        let mut last_smap = if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::OpenParentheses), imap) = next {
            smap.extend(imap.clone());
            imap
        }
        else {
            compiler.emit_compile_message(
                CompileMessage::expected_token_error(
                    smap,
                    "'('",
                    format!("function declaration: {}", name),
                    next
                )
            );
            return None;
        };

        let mut args = Vec::new();


        //parse in arguments
        let mut ended = false;
        while let Some(var_decl) = VarDeclSyntax::parse(tokens, compiler) {
            
            if var_decl.data.value.is_some() {
                compiler.emit_compile_message(CompileMessage::new(
                    var_decl.smap.clone(),
                    "default values for function arguments are not allowed!".to_string(),
                    CompileMessageType::Error
                ));
            }
            
            last_smap = var_decl.smap.clone();
            smap.extend(&var_decl.smap);
            args.push(var_decl);

            let next = tokens.pop_front();
            if let unpack_opt_tk!(TokenType::Feature(FeatureToken::Comma), imap) = next {
                smap.extend(imap);

            } else if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::CloseParentheses), imap) = next {
                smap.extend(imap);
                ended = true;

                break;
            } else {
                compiler.emit_compile_message(CompileMessage::expected_token_error(
                    last_smap.clone(),
                    "')'",
                    "function parameter",
                    next
                ));
            }
        }

        if !ended {
            let next = tokens.pop_front();
            if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::CloseParentheses), imap) = next {
                smap.extend(imap);
            } else {
                compiler.emit_compile_message(
                    CompileMessage::expected_token_error(
                        last_smap,
                        "')'",
                        "'('",
                        next
                    )
                );
                return None;
            }
        }

        //check for a return type
        let ty = if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Operator( Operator{ tk: "=", .. } )), _) = tokens.get(0) {
            tokens.pop_front();
            match TypeSyntax::parse(tokens, compiler) {
                Some(t) => {
                    smap.extend(t.smap);
                    Some(t.data)
                }
                None => None
            }
        } else {
            None
        };

        //get the block
        let block = BlockSyntax::parse(tokens, compiler);


        block.as_ref().map(|b| smap.extend(&b.smap));


        Some(Self{
            smap,
            data: Function{
                name,
                parameters: args,
                body: block,
                ty,
                is_extern
            }
        })
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}