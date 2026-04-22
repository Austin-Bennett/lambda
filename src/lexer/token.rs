use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::lexer::literal::IntegerLiteral;

//anything ending in KW is a keyword

#[derive(Clone, Debug)]
pub enum ExpressionToken {
    Identifier(String), //variable names

    IntegerLiteral(IntegerLiteral),
    BoolLiteral(bool),
    FloatLiteral(f64),

    
    Operator(Operator),
    
    OpenParentheses,

    CloseParentheses,
    //brackets []
    OpenBracket,

    CloseBracket,

    AsKW,

    Dot,
}

impl ExpressionToken {
    
    pub fn is_operator(&self) -> bool {
        match self {
            ExpressionToken::Operator(_) => true,
            _ => false,
        }
    }
    
    
}

#[derive(Clone)]
#[derive(Debug)]
pub enum StatementToken {
    UseKW(String), //containing the module path
    StructKW,
    ExternKW,
    FnKW,
    ReturnKW,
    IfKW,
    ElseKW,
    WhileKW,
    PublicKW,
    ModifyKW,
}

#[derive(Clone, Debug)]
pub enum FeatureToken {
    //code braces {}
    OpenBrace,
    CloseBrace,
    StatementEnd,
    Comma,
    Colon,
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
    Statement(StatementToken),

    CompileWarning(String),
    CompileError(String),
}

#[derive(Debug)]
pub struct Token {
    pub typ: TokenType,
    pub smap: SourceMap
}

#[macro_export]
macro_rules! unpack_tk {
    ($typ: pat, $smap: pat) => {
        Token{ typ: $typ, smap: $smap }
    };
}

#[macro_export]
macro_rules! unpack_opt_tk {
    ($typ: pat, $smap: pat) => {
        Some( Token{ typ: $typ, smap: $smap } )
    };
}