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
            ("n", '\n'),
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
pub struct StringLiteralParser;

impl Parser for CharLiteralParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        if s.starts_with("\'") {
            let Some(len) = s[1..].find('\'') else {
                return Some((TokenType::CompileError(format!("Unclosed char literal: {}", s)), s.len()));
            };

            if s[1..].starts_with("\\") {
                let Some((escape, len)) = parse_escaped_char(&s[1..(len-1)]) else {
                    return Some((TokenType::CompileError(format!("Unknown escaped char: {}", s)), 2 + len));
                };

                return Some((TokenType::Expression(ExpressionToken::CharLiteral(escape)), 2+len));
            } else {
                if len == 0 {
                    return Some((
                        TokenType::CompileError("Empty char literal!".to_string()), 2
                    ));
                }
                
                let char = s.chars().next().unwrap();
                
                if len != char.len_utf8() {
                    Some((
                        TokenType::CompileError("Char literal must contain only 1 char, did you mean to escape it?".to_string()),
                        2 + len
                    ))
                } else {
                    Some((TokenType::Expression(ExpressionToken::CharLiteral(char)), 2+len))
                }
            }

        } else {
            None
        }
    }
}

impl Parser for StringLiteralParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        if s.starts_with('"') {

            let Some(len) = s[1..].find('"') else {

                return Some((TokenType::CompileError(format!("Unclosed string literal: {}", s)), s.len()));
            };

            let mut res = String::new();

            //parse in each char
            let mut i = 1;
            while i <= len {
                let c = s[i..].chars().next().unwrap();
                i += c.len_utf8();

                if c == '\\' {
                    let Some((c, size)) = parse_escaped_char(&s[i..]) else {
                        return Some((TokenType::CompileError(format!("Unknown escaped char: {}", &s[i..])), len + 2));
                    };
                    i += size;

                    res.push(c);
                } else {
                    res.push(c);
                }
            }

            Some((TokenType::Expression(ExpressionToken::StringLiteral(res)), len + 2))

        } else {
            None
        }
    }
}