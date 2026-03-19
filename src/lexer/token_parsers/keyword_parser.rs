use std::ops::Deref;
use lazy_static::lazy_static;
use crate::lexer::token::StatementToken;
use crate::lexer::token::TokenType;
use crate::lexer::token_parsers::Parser;

pub struct KeywordParser;

lazy_static!{

    pub static ref keywords: Vec<(&'static str, TokenType)> = {
        let mut res = vec![
            ("struct", TokenType::Statement(StatementToken::StructKW)),
            ("fn", TokenType::Statement(StatementToken::FnKW)),
            ("return", TokenType::Statement(StatementToken::ReturnKW)),
        ];

        res.sort_by(|(s1, _), (s2, _)| s2.len().cmp(&s1.len()));

        res
    };
}

impl Parser for KeywordParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        for (k, kw) in keywords.deref() {
            if s.starts_with(k) {
                return Some((kw.clone(), k.len()));
            }
        }

        None
    }
}