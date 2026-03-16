use std::iter::Peekable;
use std::process::abort;
use crate::common::sourcemap::SourceMap;
use crate::lexer::token::{ExpressionToken, Token, TokenType};



pub trait TokenIterator {
    fn next_expression(&mut self) -> Option<(ExpressionToken, SourceMap)>;
    fn peek_expression(&mut self) -> Option<(&ExpressionToken, &SourceMap)>;
}

impl<'a, I: Iterator<Item=Token>> TokenIterator for Peekable<I> {
    fn next_expression(&mut self) -> Option<(ExpressionToken, SourceMap)> {
        
        if let Some(Token{ typ: TokenType::Expression(e), smap }) = self.peek() {
            
            let Some(Token{ typ: TokenType::Expression(e), smap }) = self.next()
                else { abort() };


            Some((e, smap))
        } else {
            None
        }
    }

    fn peek_expression(&mut self) -> Option<(&ExpressionToken, &SourceMap)> {

        if let Some(Token{ typ: TokenType::Expression(e), smap }) = self.peek() {

            

            Some((e, smap))
        } else {
            None
        }
    }
}