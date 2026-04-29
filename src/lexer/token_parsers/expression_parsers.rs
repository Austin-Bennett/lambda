use std::ops::Deref;
use lazy_static::lazy_static;
use crate::common::operator::Operator;
lazy_static!{
    pub static ref OPERATORS: Vec<Operator> = {
        let mut res = vec![
            // 3-char (longest first after sort)
            Operator::SHL_ASSIGN,
            Operator::SHR_ASSIGN,
            Operator::BOOL_AND_ASSIGN,
            Operator::BOOL_OR_ASSIGN,
            // 2-char
            Operator::ADD_ASSIGN,
            Operator::SUB_ASSIGN,
            Operator::MUL_ASSIGN,
            Operator::DIV_ASSIGN,
            Operator::MOD_ASSIGN,
            Operator::BIT_AND_ASSIGN,
            Operator::BIT_OR_ASSIGN,
            Operator::BIT_XOR_ASSIGN,
            Operator::BOOL_AND,
            Operator::BOOL_OR,
            Operator::EQ,
            Operator::NE,
            Operator::LE,
            Operator::GE,
            Operator::SHL,
            Operator::SHR,
            // 1-char
            Operator::ASSIGN,
            Operator::ADD,
            Operator::SUB,
            Operator::MUL,
            Operator::DIV,
            Operator::MOD,
            Operator::BIT_AND,
            Operator::BIT_OR,
            Operator::BIT_XOR,
            Operator::LT,
            Operator::GT,
            Operator::NOT,
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
pub struct NullPtrParser;
pub struct LambdaKWParser;

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

impl Parser for NullPtrParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        if s.starts_with("nullptr") && !IdentifierParser::is_identifier_character(s[7..].chars().next().unwrap_or('\0')) {
            Some((TokenType::Expression(ExpressionToken::NullPtr), 7))
        } else {
            None
        }
    }
}

impl Parser for LambdaKWParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        if s.starts_with("lambda") && !IdentifierParser::is_identifier_character(s[6..].chars().next().unwrap_or('\0')) {
            Some((TokenType::Expression(ExpressionToken::LambdaKW), 6))
        } else {
            None
        }
    }
}

impl Parser for FloatLiteralParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        let mut len = 0;
        let mut found_period = false;
        let mut has_digits_before = false;
        let mut has_digits_after = false;

        for c in s.chars() {
            if c.is_ascii_digit() {
                if found_period { has_digits_after = true; } else { has_digits_before = true; }
                len += 1;
            } else if c == '.' && !found_period && has_digits_before {
                found_period = true;
                len += 1;
            } else {
                break;
            }
        }

        if !found_period || !has_digits_before || !has_digits_after {
            return None;
        }

        let val: f64 = s[..len].parse().ok()?;
        Some((TokenType::Expression(ExpressionToken::FloatLiteral(val)), len))
    }
}