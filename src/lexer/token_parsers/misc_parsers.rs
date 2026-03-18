use crate::lexer::token::TokenType;
use crate::lexer::token_parsers::Parser;

pub struct SingleCharParser<const C: char>(TokenType);

impl<const C: char> SingleCharParser<C> {
    pub const fn new(token_type: TokenType) -> Self {
        Self(token_type)
    }
}

impl<const C: char> Parser for SingleCharParser<C> {
    fn parse(&self, s: &str) -> Option<(TokenType, usize)> {
        if let Some(c) = s.chars().next() {
            if c == C {
                Some((self.0.clone(), C.len_utf8()))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub type OpenParenthesesParser = SingleCharParser<'('>;
pub type CloseParenthesesParser = SingleCharParser<')'>;
pub type OpenBracketParser = SingleCharParser<'['>;
pub type CloseBracketParser = SingleCharParser<']'>;
pub type OpenBraceParser = SingleCharParser<'{'>;
pub type CloseBraceParser = SingleCharParser<'}'>;

pub type CommaParser = SingleCharParser<','>;
pub type ColonParser = SingleCharParser<':'>;

pub type SemicolonParser = SingleCharParser<';'>;