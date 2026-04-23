use std::collections::VecDeque;
use crate::common::sourcemap::SourceMap;
use crate::lexer::token::{ExpressionToken, FeatureToken, Token, TokenType};



pub trait TokenIterator {
    fn next_expression(&mut self) -> Option<(ExpressionToken, SourceMap)>;
    fn peek_expression(&mut self) -> Option<(&ExpressionToken, &SourceMap)>;
    fn next_feature(&mut self) -> Option<(FeatureToken, SourceMap)>;
    fn peek_feature(&mut self) -> Option<(&FeatureToken, &SourceMap)>;
}

impl TokenIterator for VecDeque<Token> {
    fn next_expression(&mut self) -> Option<(ExpressionToken, SourceMap)> {
        if let Some(Token { typ: TokenType::Expression(_), .. }) = self.get(0) {
            let Some(Token { typ: TokenType::Expression(e), smap }) = self.pop_front()
            else { unreachable!() };
            Some((e, smap))
        } else {
            None
        }
    }

    fn peek_expression(&mut self) -> Option<(&ExpressionToken, &SourceMap)> {
        if let Some(Token { typ: TokenType::Expression(e), smap }) = self.get(0) {
            Some((e, smap))
        } else {
            None
        }
    }

    fn next_feature(&mut self) -> Option<(FeatureToken, SourceMap)> {
        if let Some(Token { typ: TokenType::Feature(_), .. }) = self.get(0) {
            let Some(Token { typ: TokenType::Feature(f), smap }) = self.pop_front()
            else { unreachable!() };
            Some((f, smap))
        } else {
            None
        }
    }

    fn peek_feature(&mut self) -> Option<(&FeatureToken, &SourceMap)> {
        if let Some(Token { typ: TokenType::Feature(f), smap }) = self.get(0) {
            Some((f, smap))
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
