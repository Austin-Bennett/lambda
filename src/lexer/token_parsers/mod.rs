use crate::lexer::token::TokenType;

pub mod expression_parsers;
pub mod misc_parsers;
pub mod user_parsers;
pub mod keyword_parser;
pub mod use_parser;
pub mod string_parser;

pub trait Parser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)>;
}