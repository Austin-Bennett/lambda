use std::ops::Deref;
use lazy_static::lazy_static;
use crate::lexer::token::{ExpressionToken, TokenType};
use crate::lexer::token_parsers::Parser;

lazy_static!{

    pub static ref escaped_char_sequences: Vec<(&'static str, char)> = {
        let mut res = vec![
            ("\"", '\"'),
            ("\'", '\''),
            ("\\", '\\'),
        ];

        res.sort_by(|(s1, _), (s2, _)| s2.len().cmp(&s1.len()));

        res
    };
}


//starts after the '\'
pub fn parse_escaped_char(s: &str) -> Option<(char, usize)> {
    for (seq, c) in escaped_char_sequences.deref() {
        if s.starts_with(seq) {
            return Some((*c, seq.len()));
        }
    }
    None
}

pub struct CharLiteralParser;

impl Parser for CharLiteralParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        if s.starts_with("\'") {
            let Some(len) = s[1..].find('\'') else {
                return Some((TokenType::CompileError(format!("Unclosed char literal: {}", s)), s.len()));
            };

            if s[1..].starts_with("\\") {
                let Some((escape, len)) = parse_escaped_char(&s[1..(len-1)]) else {
                    return Some((TokenType::CompileError(format!("Unknown escaped char: {}", s)), s.len()));
                };

                return Some((TokenType::Expression(ExpressionToken::CharLiteral(escape)), len));
            } else {

            }

        } else {
            None
        }
    }
}