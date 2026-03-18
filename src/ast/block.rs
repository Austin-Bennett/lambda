use std::collections::VecDeque;
use std::fmt::{Debug, Formatter, Pointer, Write};
use std::process::abort;
use crate::ast::statement::{Statement, StatementSyntax};
use crate::ast::{GenericSyntax, Syntax};
use crate::common::sourcemap::SourceMap;
use crate::common::utils::outcome::Outcome;
use crate::compiler::{CompileMessage, CompileMessageType};
use crate::lexer::token::{FeatureToken, Token, TokenType};
use crate::{token_match, unpack_opt_tk};

pub type Block = Vec<Statement>;

pub type BlockSyntax = GenericSyntax<Block>;

impl Debug for BlockSyntax {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("{\n")?;
        for s in &self.data {
            write!(f, "\t{:?}\n", s)?;
        }


        f.write_str("}")
    }
}

impl Syntax for BlockSyntax {
    fn parse(tokens: &mut VecDeque<Token>) -> Outcome<Self, CompileMessage>
    where
        Self: Sized
    {
        if token_match!(tokens, TokenType::Feature(FeatureToken::OpenBrace)) {
            let unpack_opt_tk!(TokenType::Feature(FeatureToken::OpenBrace), smap) = tokens.pop_front() else { abort(); };
            let og_smap = smap.clone();

            let mut result = BlockSyntax::new(Block::new(), smap);
            loop {
                if token_match!(tokens, TokenType::Feature(FeatureToken::CloseBrace)) {
                    tokens.pop_front();
                    break;
                }else if token_match!(tokens, TokenType::Feature(FeatureToken::StatementEnd)) {
                    tokens.pop_front();
                } else {
                    let stmt = match StatementSyntax::parse(tokens)? {
                        Some(v) => v,
                        None => {
                            //one of two things has gone wrong, 1. a bad token, 2. a EOF

                            match tokens.pop_front() {
                                Some(tk) => return Outcome::Err(
                                    CompileMessage::new(
                                        tk.smap.clone(),
                                        format!("didnt expect token: {:?}", tk.typ),
                                        CompileMessageType::Error
                                    )
                                        .add_note(CompileMessage::note(tk.smap, "expected statement".to_string()))
                                ),
                                None => return Outcome::Err(
                                    //todo: replace with error at the end of the block
                                    CompileMessage::new(
                                        og_smap,
                                        "unclosed '}'".to_string(),
                                        CompileMessageType::Error
                                    )
                                )
                            }
                        }
                    };


                    result.smap.extend(stmt.smap);
                    result.data.push(stmt.data);
                }
            }

            Outcome::Ok(result)
        } else {
            Outcome::None
        }
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}