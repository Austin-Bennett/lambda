use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::process::abort;
use crate::ast::{GenericSyntax, Syntax};
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::compiler::CompileMessage;
use crate::lexer::token::{ExpressionToken, Token, TokenType};
use crate::unpack_opt_tk;

//currently all we have are typename's, but soon we might have pointers, references, etc
pub enum Type {
    Typename(ModulePath),
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { 
            Type::Typename(mp) => write!(f, "{}", mp),
        }
    }
}

pub type TypeSyntax = GenericSyntax<Type>;


impl Syntax for TypeSyntax {
    fn parse(tokens: &mut VecDeque<Token>) -> Outcome<Self, CompileMessage>
    where
        Self: Sized
    {

        if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Identifier(_)), _) = tokens.get(0) {
            let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Identifier(id)), smap) = tokens.pop_front()
            else { abort() };

            Outcome::Ok(
                Self{
                    data: Type::Typename(ModulePath::from_module_path(id)),
                    smap
                }
            )
        } else {
            Outcome::None
        }
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}