use std::collections::VecDeque;
use std::fmt::{Debug, Formatter, Pointer};
use crate::lambda_parser::{ExprNode, Keyword, Operator, Token, TokenHelpers};
use crate::match_tokens;

pub type Block = Vec<Statement>;

pub enum Statement {
    Expression(ExprNode),
    FunctionDeclaration{name: String, args: Vec<String>, body: Block},
    IfStatement{pred: ExprNode, code: Block, else_block: Block},
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Expression(e) => e.fmt(f),
            Statement::FunctionDeclaration {name, args, body} => write!(f, "{:?}({:?}) = {:?}", name, args, body),
            Statement::IfStatement { pred, code, else_block } =>
                write!(f, "if {:?} {{ {:?} }} else {{ {:?} }}", pred, code, else_block),
        }
    }
}

impl Statement {
    fn parse_argument_decl(args: VecDeque<Token>) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let mut iter = args.iter().peekable();
        while let Some(arg) = iter.next() {
            if let Token::Newline = arg {
                continue;
            }
            if let Token::Ident(s) = arg {
                result.push(s.clone());
                //next must be a comma, otherwise return error
                if let None = iter.peek() {
                    break;
                }
                if let Some(Token::Operator(Operator{ token: ",", bp: _ })) = iter.next() {

                } else {
                    return Err(String::from("Error, expected comma"))
                }
            } else {
                return Err(format!("Expected identifier, got {arg:?}"));
            }
        }

        Ok(result)
    }
    pub fn from_tokens(tks: &mut VecDeque<Token>) -> Result<Block, String> {
        let mut result = Block::new();

        while !tks.is_empty() {
            let s = if match_tokens!(tks, Token::Ident(_), Token::ParenthesesGroup(_),
                Token::Operator(Operator{ token: "=", bp: _ })) {
                //this is a function declaration
                let Some(Token::Ident(s)) = tks.pop_front() else { return Err("THIS SHOULDN'T HAPPEN".to_string()) };
                let Some(Token::ParenthesesGroup(args)) = tks.pop_front() else { return Err("THIS SHOULDN'T HAPPEN".to_string()) };
                tks.pop_front(); //consume the operator '=';

                let body = if let Some(Token::OpenBrace) = tks.front() {
                    tks.pop_front();
                    Statement::from_tokens(tks)?
                } else {
                    vec![Statement::Expression(ExprNode::from_tokens(tks)?)]
                };

                Statement::FunctionDeclaration { name: s, args: Statement::parse_argument_decl(args)?, body }
            } else if let Some(Token::Keyword(Keyword::If)) = tks.front() {
                tks.pop_front();
                let pred = ExprNode::from_tokens(tks)?;
                let code = if let Some(Token::OpenBrace) = tks.pop_front() {
                    Self::from_tokens(tks)?
                } else {
                    return Err("Expected code block after 'if'".to_string());
                };

                let else_block = if let Some(Token::Keyword(Keyword::Else)) = tks.front() {
                    tks.pop_front();
                    if let Some(Token::OpenBrace) = tks.front() {
                        tks.pop_front();
                    }
                    Self::from_tokens(tks)?
                } else {
                    Block::new()
                };


                Statement::IfStatement { pred, code, else_block }
            } else if let Some(Token::CloseBrace) = tks.front() {
                tks.pop_front();
                break;
            } else if let Some(Token::Newline) = tks.front() {
                tks.pop_front();
                continue;
            } else {
                Statement::Expression(ExprNode::from_tokens(tks)?)
            }
            ;


            result.push(s);
        }

        Ok(result)
    }
}