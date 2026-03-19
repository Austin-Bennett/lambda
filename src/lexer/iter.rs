use std::collections::VecDeque;
use std::process::abort;
use crate::common::sourcemap::SourceMap;
use crate::lexer::token::{ExpressionToken, Token, TokenType};



pub trait TokenIterator {
    fn next_expression(&mut self) -> Option<(ExpressionToken, SourceMap)>;
    fn peek_expression(&mut self) -> Option<(&ExpressionToken, &SourceMap)>;
}

impl TokenIterator for VecDeque<Token> {
    fn next_expression(&mut self) -> Option<(ExpressionToken, SourceMap)> {

        if let Some(Token{ typ: TokenType::Expression(_e), smap: _ }) = self.get(0) {

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

#[macro_export]
macro_rules! token_match {
    ($vd: expr, $($pat: pat),* $(,)?) => {
        token_match!(@private $vd, 0, $($pat),*)
    };

    (@private $vd: expr, $i: expr, $first: pat, $($rest: pat),*) => {
        if let Some(Token{ typ: $first, .. }) = $vd.get($i) {
            token_match!(@private $vd, $i+1, $($rest),*)
        } else {
            false
        }
    };

    (@private $vd: expr, $i: expr, $first: pat) => {
        if let Some(Token{ typ: $first, .. }) = $vd.get($i) {
            true
        } else {
            false
        }
    };
}
