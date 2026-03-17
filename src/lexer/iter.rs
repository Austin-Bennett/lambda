use std::collections::VecDeque;
use std::iter::Peekable;
use std::process::abort;
use crate::common::sourcemap::SourceMap;
use crate::lexer::token::{ExpressionToken, Token, TokenType};



pub trait TokenIterator {
    fn next_expression(&mut self) -> Option<(ExpressionToken, SourceMap)>;
    fn peek_expression(&mut self) -> Option<(&ExpressionToken, &SourceMap)>;
}

impl TokenIterator for VecDeque<Token> {
    fn next_expression(&mut self) -> Option<(ExpressionToken, SourceMap)> {
        
        if let Some(Token{ typ: TokenType::Expression(e), smap }) = self.get(0) {
            
            let Some(Token{ typ: TokenType::Expression(e), smap }) = self.pop_front() 
            else { 
                abort()
            };


            Some((e, smap))
        } else {
            None
        }
    }

    fn peek_expression(&mut self) -> Option<(&ExpressionToken, &SourceMap)> {

        if let Some(Token{ typ: TokenType::Expression(e), smap }) = self.get(0) {
            
            Some((e, smap))
        } else {
            None
        }
    }
}