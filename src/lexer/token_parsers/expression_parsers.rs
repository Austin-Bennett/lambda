use std::ops::Deref;
use lazy_static::lazy_static;
use crate::common::operator::Operator;
use crate::lexer::literal::IntegerLiteral;
use crate::lexer::token::{ExpressionToken, TokenType};
use crate::lexer::token_parsers::Parser;

lazy_static!{
    pub static ref OPERATORS: Vec<Operator> = {
        let mut res = vec![
            Operator{ tk: "+", bp: Operator::ADDITIVE_BP },
            Operator{ tk: "-", bp: Operator::ADDITIVE_BP },
            Operator{ tk: "*", bp: Operator::MULTIPLICATIVE_BP },
            Operator{ tk: "/", bp: Operator::MULTIPLICATIVE_BP },
        ];

        res.sort_by(|f, s| s.tk.len().cmp(&f.tk.len()));

        res
    };
}

pub struct OperatorParser;
pub struct IdentifierParser;
pub struct IntLiteralParser;

impl IdentifierParser {
    pub fn is_identifier_character(c: char) -> bool {
        c.is_numeric() || c.is_alphabetic() || c == '_' || c == ':'
    }

    pub fn can_start_with(c: char) -> bool {
        c.is_alphabetic() || c == '_' || c == ':'
    }

    pub fn is_valid_identifier(s: &str) -> bool {
        let mut chars = s.chars();

        let Some(first) = chars.next() else { return false; };
        if !Self::can_start_with(first) {
            false
        } else {
            !chars.any(|c| !Self::is_identifier_character(c))
        }
    }
}

impl Parser for OperatorParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        for op in OPERATORS.deref() {
            if s.starts_with(op.tk) {
                return Some((
                    TokenType::Expression(ExpressionToken::Operator(op.clone())), 
                    op.tk.len()
                ));
            }
        }

        None
    }
}


impl Parser for IdentifierParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        //identifiers must start with either a _ or an alphabetical character,
        // after that, any letter, number, or '_' is valid

        let mut chars = s.chars();

        let Some(first) = chars.next() else { return None; };

        if Self::can_start_with(first) {
            let mut len = first.len_utf8();
            while let Some(c) = chars.next() {
                if Self::is_identifier_character(c) {
                    len += c.len_utf8();
                } else {
                    break;
                }
            }

            Some((TokenType::Expression(ExpressionToken::Identifier(s[0..len].to_owned())), len))
        } else {
            None
        }
    }
}

impl Parser for IntLiteralParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        let mut chars = s.chars().peekable();

        let mut i = 0;

        if let Some('-') = chars.peek() {
            chars.next();
            i += 1;
        }

        while let Some(c) = chars.next() {
            if c.is_numeric() {
                i += 1;
            } else {
                break;
            }
        }


        if i > 0 {
            Some((TokenType::Expression(ExpressionToken::IntegerLiteral(IntegerLiteral::from_string(&s[0..i])?)), i))
        } else {
            None
        }
    }
}