use crate::lexer::tokenizer::Token;

pub trait Parser {
    //None = cant parse this, Some = parsed this
    fn parse(&self, s: &str) -> Option<(Token, usize)>;


}