use std::ops::Deref;
use lazy_static::lazy_static;
use crate::common::operator::Operator;
lazy_static!{
    pub static ref OPERATORS: Vec<Operator> = {
        let mut res = vec![
            Operator::ASSIGN,
            Operator::ADD,
            Operator::SUB,
            Operator::MUL,
            Operator::DIV,
            Operator::BITWISE_AND,
        ];

        res.sort_by(|f, s| s.tk.len().cmp(&f.tk.len()));

        res
    };
}
use crate::lexer::literal::IntegerLiteral;
use crate::lexer::token::{ExpressionToken, TokenType};

use crate::lexer::token_parsers::Parser;

pub struct OperatorParser;
pub struct IdentifierParser;
pub struct IntLiteralParser;
pub struct BoolLiteralParser;
pub struct FloatLiteralParser;

impl IdentifierParser {
    pub fn is_identifier_character(c: char) -> bool {
        c.is_numeric() || c.is_alphabetic() || c == '_'
    }

    pub fn can_start_with(c: char) -> bool {
        c.is_alphabetic() || c == '_'
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

        let mut chars = s.char_indices();

        let Some((_, first)) = chars.next() else { return None; };

        if Self::can_start_with(first) {
            let mut len = first.len_utf8();
            while let Some((i, c)) = chars.next() {
                if Self::is_identifier_character(c) {
                    len += c.len_utf8();
                } else if s[i..].starts_with("::") {
                    len += "::".len();
                    chars.next();
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

impl Parser for BoolLiteralParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        if s.starts_with("true") && !IdentifierParser::is_identifier_character(s[4..].chars().next().unwrap_or('\0')) {
            Some((TokenType::Expression(ExpressionToken::BoolLiteral(true)), 4))
        } else if s.starts_with("false") && !IdentifierParser::is_identifier_character(s[5..].chars().next().unwrap_or('\0')) {
            Some((TokenType::Expression(ExpressionToken::BoolLiteral(false)), 5))
        } else {
            None
        }
    }
}

impl Parser for FloatLiteralParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        //parse in all numbers and exactly 1 period, if the next character is an e, parse in that aswell
        let mut num = String::new();
        let mut found_period = false;
        
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c.is_numeric() {
                num.push(c)
            } else if c == '.' && !found_period {
                found_period = true;
                num.push('.')
            } else {
                return Some((TokenType::CompileError("Extraneous '.'".to_string()), num.len()))
            }
        }
        
        if let Some('e') = chars.next() {
            
        }
        
        todo!()
    }
}