use std::fs;
use std::path::{Path, PathBuf};
use crate::lexer::token::{ExpressionToken, FeatureToken, Token, TokenType};
use crate::lexer::token_parsers::expression_parsers::{IdentifierParser, IntLiteralParser, OperatorParser};
use crate::lexer::token_parsers::misc_parsers::{CloseBraceParser, CloseBracketParser, CloseParenthesesParser, CommaParser, NewlineParser, OpenBraceParser, OpenBracketParser, OpenParenthesesParser};
use crate::lexer::token_parsers::Parser;
use anyhow::Result;
use crate::common::sourcemap::SourceMap;
use crate::lexer::token_parsers::keyword_parser::KeywordParser;
use crate::lexer::token_parsers::user_parsers::{CommentParser, WhitespaceParser};

pub struct Tokens {
    owner: String,
    raw: String,
    pos: usize,
}

impl Tokens {
    pub const PARSERS: &[&dyn Parser] = &[
        &NewlineParser::new(TokenType::Feature(FeatureToken::StatementEnd)),
        &WhitespaceParser,
        &CommentParser,
        &OperatorParser,
        &OpenParenthesesParser::new(TokenType::Expression(ExpressionToken::OpenParentheses)),
        &CloseParenthesesParser::new(TokenType::Expression(ExpressionToken::CloseParentheses)),
        &OpenBracketParser::new(TokenType::Expression(ExpressionToken::OpenBracket)),
        &CloseBracketParser::new(TokenType::Expression(ExpressionToken::CloseBracket)),
        &OpenBraceParser::new(TokenType::Feature(FeatureToken::OpenBrace)),
        &CloseBraceParser::new(TokenType::Feature(FeatureToken::CloseBrace)),
        &CommaParser::new(TokenType::Feature(FeatureToken::Comma)),
        &KeywordParser,
        &IdentifierParser,
        &IntLiteralParser,
    ];

    pub fn tokenize_string(owner: String, contents: String) -> Self {
        Self{
            owner,
            raw: contents,
            pos: 0,
        }
    }

    pub fn tokenize(file: impl AsRef<Path>) -> Result<Self> {
        let file_contents = fs::read_to_string(&file)?;

        Ok(Self::tokenize_string(file.as_ref().to_string_lossy().to_string(), file_contents))
    }

    //returns None on a compiler error/warning
    pub fn parse_next_token(tk: &str) -> Option<(TokenType, usize)> {
        for p in Self::PARSERS {
            if let Some((ty, len)) = p.parse(tk) {
                return Some((ty, len));
            }
        }

        None
    }
}

impl Iterator for Tokens {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        //get the next token
        let og_pos = self.pos;
        let mut bad_buf = String::new();
        let mut res = None;
        while self.pos < self.raw.len() && res.is_none() {
            if let Some((tk, len)) = Self::parse_next_token(&self.raw[self.pos..]) {

                if bad_buf.is_empty() {
                    let smap = SourceMap{
                        owner: self.owner.clone(),
                        offset: self.pos,
                        len,
                    };

                    res = Some(Token{
                        typ: tk,
                        smap
                    });

                    self.pos += len;
                } else {
                    res = Some(Token{
                        typ: TokenType::CompileWarning(format!("Could not make token out of string: \"{}\"", bad_buf)),
                        smap: SourceMap{
                            owner: self.owner.clone(),
                            offset: og_pos,
                            len: bad_buf.len(),
                        }
                    });
                }
            } else {
                let c = self.raw[self.pos..].chars().next().unwrap();
                bad_buf.push(c);
                self.pos += c.len_utf8();
            }
        }

        if !bad_buf.is_empty() && res.is_none() {
            res = Some(Token{
                typ: TokenType::CompileWarning(format!("Could not make token out of string: \"{}\"", bad_buf)),
                smap: SourceMap{
                    owner: self.owner.clone(),
                    offset: og_pos,
                    len: bad_buf.len(),
                }
            });
        }

        res
    }
}