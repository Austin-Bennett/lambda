use std::collections::VecDeque;
use std::fmt::{Debug, Formatter, Write};
use std::process::abort;
use crate::ast::block::{Block, BlockSyntax};
use crate::ast::common::{VarDecl, VarDeclSyntax};
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::ty::{Type, TypeSyntax};
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::compiler::CompileMessage;
use crate::lexer::token::{ExpressionToken, FeatureToken, StatementToken, Token, TokenType};
use crate::unpack_opt_tk;


//todo: return value
pub struct Function {
    name: ModulePath,
    parameters: Vec<VarDecl>,
    body: Block,
    ty: Option<Type>, //None for void
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
            write!(f, "{:?}", p)?;
        }

        f.write_str(")")?;

        if let Some(ty) = &self.ty {
            write!(f, " = {:?}", ty)?;
        }

        write!(f, " {{ {} statement{} }}", self.body.len(), if self.body.len() == 1 { "" } else { "s" })?;

        Ok(())
    }
}

pub type FunctionSyntax = GenericSyntax<Function>;

impl Syntax for FunctionSyntax {
    fn parse(tokens: &mut VecDeque<Token>) -> Outcome<Self, CompileMessage>
    where
        Self: Sized
    {
        let unpack_opt_tk!( TokenType::Statement(StatementToken::FnKW), smap ) = tokens.get(0) else { return Outcome::None };
        let unpack_opt_tk!( TokenType::Statement(StatementToken::FnKW), mut smap ) = tokens.pop_front() else { abort(); };

        //get the name next
        let next = tokens.pop_front();
        let name = if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Identifier(s)), imap) = next {
            smap.extend(imap);
            ModulePath::from_module_path(s)
        } else {
            return Outcome::Err(
                CompileMessage::expected_token_error(
                    smap,
                    "identifier",
                    "'fn' keyword",
                    next
                )
            )
        };

        //expect an open parentheses
        let next = tokens.pop_front();
        let mut last_smap = if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::OpenParentheses), imap) = next {
            smap.extend(imap.clone());
            imap
        }
        else {
            return Outcome::Err(
                CompileMessage::expected_token_error(
                    smap,
                    "'('",
                    format!("function declaration: {}", name),
                    next
                )
            )
        };

        let mut args = Vec::new();


        //parse in arguments
        let mut ended = false;
        while let Some(var_decl) = VarDeclSyntax::parse(tokens)? {
            last_smap = var_decl.smap.clone();
            args.push(var_decl.data);
            smap.extend(var_decl.smap);

            let next = tokens.pop_front();
            if let unpack_opt_tk!(TokenType::Feature(FeatureToken::Comma), imap) = next {
                smap.extend(imap);

            } else if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::CloseParentheses), imap) = next {
                smap.extend(imap);
                ended = true;

                break;
            } else {
                return Outcome::Err(CompileMessage::expected_token_error(
                    last_smap,
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
                return Outcome::Err(
                    CompileMessage::expected_token_error(
                        last_smap,
                        "')'",
                        "'('",
                        next
                    )
                )
            }
        }

        //check for a return type
        let ty = if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Operator( Operator{ tk: "=", .. } )), _) = tokens.get(0) {
            tokens.pop_front();
            match TypeSyntax::parse(tokens)? {
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
        let block = match BlockSyntax::parse(tokens)? {
            Some(b) => b,
            None =>
                return Outcome::Err(
                    CompileMessage::expected_token_error(
                        last_smap,
                        "function body",
                        "')'",
                        tokens.pop_front()
                    )
                )
        };

        smap.extend(block.smap);


        Outcome::Ok(Self{
            smap,
            data: Function{
                name,
                parameters: args,
                body: block.data,
                ty
            }
        })
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}