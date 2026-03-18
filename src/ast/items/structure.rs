use std::collections::VecDeque;
use std::process::abort;
use crate::ast::{GenericSyntax, Syntax};
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::compiler::{CompileMessage, CompileMessageType};
use crate::lexer::token::{ExpressionToken, FeatureToken, StatementToken, Token, TokenType};
use crate::unpack_opt_tk;

pub struct StructMember {
    name: ModulePath,
    ty: ModulePath,
}

pub struct Structure {
    name: ModulePath,
    members: Vec<StructMember>

    //todo: methods?
}

pub type StructureSyntax = GenericSyntax<Structure>;

impl Syntax for StructureSyntax {
    fn parse(tokens: &mut VecDeque<Token>) -> Outcome<Self, CompileMessage>
    where
        Self: Sized
    {
        let unpack_opt_tk!( TokenType::Statement(StatementToken::StructKW), smap ) = tokens.get(0) else { return Outcome::None };
        let unpack_opt_tk!( TokenType::Statement(StatementToken::StructKW), mut smap ) = tokens.pop_front() else { abort(); };



        //next token should be an identifier
        let next = tokens.pop_front();
        let unpack_opt_tk!( TokenType::Expression(ExpressionToken::Identifier(s)), imap )
            = next else {
            return match next {
                Some(tk) => Outcome::Err(
                    CompileMessage::new(
                        tk.smap,
                        format!("expected identifier after struct keyword, got token: {:?}", tk.typ),
                        CompileMessageType::Error
                    )
                        .add_note(
                            CompileMessage::note(
                                smap,
                                "struct keyword here".to_string()
                            )
                        )
                ),
                None => Outcome::Err(
                    CompileMessage::new(
                        smap,
                        "expected identifier after struct keyword".to_string(),
                        CompileMessageType::Error
                    )
                ),
            }
        };
        smap.extend(imap);
        
        let mut res = StructureSyntax{
            data: Structure{
                name: ModulePath::from_module_path(s),
                members: Vec::new()
            },
            smap
        };
        
        //check for the '{'
        
        let next = tokens.pop_front();
        let unpack_opt_tk!( TokenType::Feature(FeatureToken::OpenBrace), mut smap ) = next else {
            return match next {
                Some(tk) => Outcome::Err(
                    CompileMessage::new(
                        tk.smap,
                        format!("expected '{{' after struct, got token: {:?}", tk.typ),
                        CompileMessageType::Error
                    )
                        .add_note(
                            CompileMessage::note(
                                res.smap,
                                format!("Expected after declaration of struct: {}", res.data.name)
                            )
                        )
                ),
                None => Outcome::Err(
                    CompileMessage::new(
                        res.smap,
                        "expected '{' after struct declaration".to_string(),
                        CompileMessageType::Error
                    )
                ),
            }
        };
        
        //time to parse members
        
        
        todo!()
    }

    fn get_sourcemap(&self) -> &SourceMap {
        & self.smap
    }
}