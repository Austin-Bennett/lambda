use std::cmp::Ordering;
use std::ops::Deref;
use lazy_static::lazy_static;
use crate::lexer::lang_parser::Parser;
use crate::lexer::tokenizer::Token;

#[derive(Debug, Clone, Copy)]
pub enum OperatorType {
    Feature,
    Unary,
    Binary(u8, u8),
    BinaryOrUnary(u8, u8)
}

#[derive(Debug, Copy, Clone)]
pub struct Operator {
    pub token: &'static str,
    pub typ: OperatorType,
}

impl Operator {
    pub fn new(token: &'static str, typ: OperatorType) -> Operator {
        Operator{ token, typ }
    }
}

impl Eq for Operator {}

impl PartialEq<Self> for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Operator {
    fn cmp(&self, other: &Self) -> Ordering {
        other.token.len().cmp(&other.token.len())
    }


}

lazy_static!{

    pub static ref operators: Vec<Operator> = {
        let mut res = vec![
            Operator::new("(", OperatorType::Feature),
            Operator::new(")", OperatorType::Feature),
            Operator::new("{", OperatorType::Feature),
            Operator::new("}", OperatorType::Feature),
            Operator::new("=", OperatorType::Binary(0, 1)),
            Operator::new("+", OperatorType::BinaryOrUnary(2, 3)),
            Operator::new("-", OperatorType::BinaryOrUnary(2, 3)),
            Operator::new("*", OperatorType::BinaryOrUnary(4, 5)),
            Operator::new("/", OperatorType::BinaryOrUnary(4, 5)),
        ];

        res.sort();
        res
    };

}



pub struct OpParser;

impl Parser for OpParser {
    fn parse(&self, s: &str) -> Option<(Token, usize)> {
        for o in operators.deref() {
            if s.starts_with(o.token) {
                return Some((Token::Op(*o), o.token.len()));
            }
        }

        None
    }
}