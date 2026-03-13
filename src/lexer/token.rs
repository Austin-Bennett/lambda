use crate::common::sourcemap::SourceMap;
use crate::lexer::literal::IntegerLiteral;
use crate::lexer::token_parsers::Parser;

#[derive(Clone, Debug)]
pub enum ExpressionToken {
    Identifier(String), //variable names
    
    //arithmetic operators
    Assign, //=

    Add, // +
    Sub, // -
    Mul, // *
    Div, // /

    //todo: bitwise operators, boolean operators, etc

    //groups
    //parentheses ()
    OpenParentheses,
    CloseParentheses,

    //brackets []
    OpenBracket,
    CloseBracket,

    IntegerLiteral(IntegerLiteral)
}

#[derive(Clone, Debug)]
pub enum FeatureToken {
    //code braces {}
    OpenBrace,
    CloseBrace,
    StatementEnd,
    Comma,
}

#[derive(Clone, Debug)]
pub enum UserToken {
    Whitespace,
    Comment,
}



#[derive(Clone, Debug)]
pub enum TokenType {
    User(UserToken),
    Expression(ExpressionToken),
    Feature(FeatureToken),

    CompileWarning(String),
    CompileError(String),
}

#[derive(Debug)]
pub struct Token {
    pub typ: TokenType,
    pub smap: SourceMap
}

