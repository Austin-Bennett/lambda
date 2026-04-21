use std::ops::Deref;
use lazy_static::lazy_static;
use crate::lexer::token::{ExpressionToken, StatementToken};
use crate::lexer::token::TokenType;
use crate::lexer::token_parsers::Parser;
use crate::lexer::token_parsers::expression_parsers::IdentifierParser;

pub struct KeywordParser;

lazy_static!{

    pub static ref keywords: Vec<(&'static str, TokenType)> = {
        let mut res = vec![
            ("struct", TokenType::Statement(StatementToken::StructKW)),
            ("fn", TokenType::Statement(StatementToken::FnKW)),
            ("return", TokenType::Statement(StatementToken::ReturnKW)),
            ("extern", TokenType::Statement(StatementToken::ExternKW)),
            ("as", TokenType::Expression(ExpressionToken::AsKW)),
            ("if", TokenType::Statement(StatementToken::IfKW)),
            ("else", TokenType::Statement(StatementToken::ElseKW)),
            ("while", TokenType::Statement(StatementToken::WhileKW))
        ];

        res.sort_by(|(s1, _), (s2, _)| s2.len().cmp(&s1.len()));

        res
    };
}

impl Parser for KeywordParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        for (k, kw) in keywords.deref() {
            if s.starts_with(k) {
                let after = s[k.len()..].chars().next();
                if after.map_or(true, |c| !IdentifierParser::is_identifier_character(c)) {
                    return Some((kw.clone(), k.len()));
                }
            }
        }

        None
    }
}