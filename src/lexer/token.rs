use crate::common::sourcemap::SourceMap;

pub enum ExpressionToken {
    Identifier, //variable names
    
    //arithmetic operators
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


    IntegerLiteral()
}

pub enum TokenType {
    Expression(ExpressionToken),
}

pub struct Token {
    pub typ: TokenType,
    pub smap: SourceMap
}

