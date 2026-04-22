use std::fs;
use std::path::Path;
use crate::lexer::token::{ExpressionToken, FeatureToken, Token, TokenType};
use crate::lexer::token_parsers::expression_parsers::{BoolLiteralParser, FloatLiteralParser, IdentifierParser, IntLiteralParser, OperatorParser};
use crate::lexer::token_parsers::misc_parsers::*;
use crate::lexer::token_parsers::Parser;
use anyhow::Result;
use crate::common::source_owner::{SourceDescriptor, SourceOwner};
use crate::common::sourcemap::SourceMap;
use crate::lexer::token_parsers::keyword_parser::KeywordParser;
use crate::lexer::token_parsers::use_parser::UseParser;
use crate::lexer::token_parsers::user_parsers::{CommentParser, WhitespaceParser};

pub struct Tokens {
    owner: SourceOwner,
    raw: String,
    line: usize,
    char: usize,
    pos: usize,
}

impl Tokens {
    pub const PARSERS: &[&dyn Parser] = &[
        &SemicolonParser::new(TokenType::Feature(FeatureToken::StatementEnd)),
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
        &ColonParser::new(TokenType::Feature(FeatureToken::Colon)),
        &UseParser,
        &KeywordParser,
        &BoolLiteralParser,   // before IdentifierParser — "true"/"false" are also valid identifiers
        &FloatLiteralParser,  // before IntLiteralParser — "3.14" must not be tokenized as "3" then ".14"
        &DotParser::new(TokenType::Expression(ExpressionToken::Dot)),
        &IntLiteralParser,
        &IdentifierParser,
    ];

    pub fn tokenize_string(owner: SourceOwner, contents: String) -> Self {
        Self{
            owner,
            raw: contents,
            pos: 0,
            line: 0,
            char: 0,
        }
    }

    pub fn tokenize(file: impl AsRef<Path>) -> Result<Self> {
        let file_contents = fs::read_to_string(&file)?;

        Ok(Self::tokenize_string(SourceOwner::new(
            SourceDescriptor::File,
            file.as_ref().to_string_lossy().to_string()
        ), file_contents))
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
    
    pub fn get_owner(&self) -> &SourceOwner {
        &self.owner
    }
}

impl Iterator for Tokens {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        //get the next token
        let og_pos = self.pos;
        let og_line = self.line;
        let og_char = self.char;
        let mut bad_buf = String::new();
        let mut res = None;
        
        while self.pos < self.raw.len() && res.is_none() {
            if let Some((tk, len)) = Self::parse_next_token(&self.raw[self.pos..]) {

                if bad_buf.is_empty() {
                    let smap = SourceMap{
                        owner: self.owner.clone(),
                        offset: self.pos,
                        line: self.line,
                        char: self.char,
                        len,
                    };

                    res = Some(Token{
                        typ: tk,
                        smap
                    });

                    for (i, c) in (&self.raw[self.pos..]).char_indices() {
                        if i >= len {
                            break;
                        }
                        self.char += 1;
                        if c == '\n' {
                            self.line += 1;
                            self.char = 0;
                        }
                    }
                    self.pos += len;

                    
                    
                } else {
                    res = Some(Token{
                        typ: TokenType::CompileWarning(format!("Could not make token out of string: \"{}\"", bad_buf)),
                        smap: SourceMap{
                            owner: self.owner.clone(),
                            offset: og_pos,
                            line: og_line,
                            char: og_char,
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
                    line: og_line,
                    char: og_char,
                    len: bad_buf.len(),
                }
            });
        }

        res
    }
}