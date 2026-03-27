use std::collections::VecDeque;
use std::fmt::{Debug, Formatter, Write};
use std::process::abort;
use crate::ast::block::{Block, BlockSyntax};
use crate::ast::statements::vardecl::{VarDecl, VarDeclSyntax};
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::ty::{Type, TypeSyntax};
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::token::{ExpressionToken, FeatureToken, StatementToken, Token, TokenType};
use crate::{compiler, unpack_opt_tk};


//todo: return value
pub struct Function {
    pub name: String,
    pub parameters: Vec<VarDeclSyntax>,
    pub body: BlockSyntax,
    pub ty: Option<Type>, //None for void
}

impl Function {
    
    //converts this function into its
    //typename, basically its signature
    pub fn to_typename(&self) -> String {
        
        format!("{}({})", self.name, self.parameters
            .iter()
            .map(|v| format!("{:?}", v.data.ty))
            .collect::<Vec<String>>()
            .join(", "))
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

        write!(f, " {{{}", if self.body.data.is_empty() { "" } else { "\n" })?;

        for s in &self.body.data {
            write!(f, "\t{:?}\n", s)?;
        }

        write!(f, "}}")?;

        Ok(())
    }
}

pub type FunctionSyntax = GenericSyntax<Function>;

impl Syntax for FunctionSyntax {
    fn parse(tokens: &mut VecDeque<Token>, compiler: &mut Compiler) -> Option<Self>
    where
        Self: Sized
    {
        let unpack_opt_tk!( TokenType::Statement(StatementToken::FnKW), smap ) = tokens.get(0) else { return None };
        let unpack_opt_tk!( TokenType::Statement(StatementToken::FnKW), mut smap ) = tokens.pop_front() else { abort(); };

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
                smap.extend(imap.clone());
                ended = true;
                last_smap = imap;
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
        let block = match BlockSyntax::parse(tokens, compiler) {
            Some(b) => b,
            None => {
                compiler.emit_compile_message(
                    CompileMessage::expected_token_error(
                        last_smap,
                        "function body",
                        "')'",
                        tokens.pop_front(),
                    )
                );
                return None;
            }
        };

        smap.extend(&block.smap);


        Some(Self{
            smap,
            data: Function{
                name,
                parameters: args,
                body: block,
                ty
            }
        })
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}