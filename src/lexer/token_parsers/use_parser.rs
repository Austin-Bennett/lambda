use crate::lexer::token::{StatementToken, TokenType};
use crate::lexer::token_parsers::Parser;

pub struct UseParser;


impl Parser for UseParser {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {

        if s.starts_with("use") {

            //consume all the rest of the tokens until the next newline
            let end = s.find('\n').unwrap_or(s.len());

            let str = &s[3..end].trim();

            Some((TokenType::Statement(StatementToken::UseKW(str.to_string())), end))
        } else {
            None
        }
    }
}