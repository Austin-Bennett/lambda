use crate::lexer::token::{TokenType, UserToken};
use crate::lexer::token_parsers::Parser;

pub struct CommentParser;
pub struct WhitespaceParser;


impl Parser for CommentParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        //if s starts with //, go to the next newline character or end
        //if s starts with /* go to the next */ or end

        if s.starts_with("//") {
            let n = s.find('\n').unwrap_or(s.len());

            Some((TokenType::User(UserToken::Comment), n))
        } else if s.starts_with("/*") {
            let n = s.find("*/").map(|v| v + 1).unwrap_or(s.len());

            Some((TokenType::User(UserToken::Comment), n))
        } else {
            None
        }
    }
}



impl Parser for WhitespaceParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        let mut n = 0;
        let mut chars = s.chars();
        while let Some(c) = chars.next() && c.is_whitespace() {
            n += 1;
        }

        if n == 0 {
            None
        } else {
            Some((TokenType::User(UserToken::Whitespace), n))
        }
    }
}