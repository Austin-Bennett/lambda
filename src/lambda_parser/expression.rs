use crate::lambda_parser::tokenization::{tokenize, BindingPower, Token};
use crate::lambda_parser::ExprNode::{BinaryOperation, CallOperation, Error};
use crate::lambda_parser::Operator;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::process::abort;


#[derive(Clone, PartialOrd, PartialEq)]
pub enum ExprNode {
    CallOperation{caller: Box<ExprNode>, args: Vec<ExprNode>},
    BinaryOperation{op: &'static str, operands: Box<(ExprNode, ExprNode)>},
    UnaryOperation{op: &'static str, operand: Box<ExprNode>},
    Ident(String),
    Num(f64),
    Void,
    Error(String), //internal, shouldn't show up in the tree
}



impl ExprNode {

    #[allow(dead_code)]
    pub fn not_eof(self) -> Result<Self, String> {
        if let Self::Error(s) = self {
            Err(s)
        } else {
            Ok(self)
        }
    }

    pub fn from_str(s: impl AsRef<str>) -> Result<Self, String> {
        Self::from_tokens(tokenize(s))
    }

    pub fn from_tokens(mut tks: VecDeque<Token>) -> Result<Self, String> {
        Self::make(&mut tks, 0.0)
    }

    fn parse_call(tks: &mut VecDeque<Token>) -> Result<Vec<Self>, String> {
        let mut res = Vec::new();

        while !tks.is_empty() {
            let arg = Self::make(tks, 0.0)?;
            res.push(arg);

            if let Some(Token::Operator(Operator{ token: _, bp: BindingPower::Seperator })) = tks.front() {
                tks.pop_front();
            }
        }

        Ok(res)
    }

    fn make(tks: &mut VecDeque<Token>, min_bp: f32) -> Result<Self, String> {

        if tks.is_empty() { return Ok(Error("Didnt expect end of tokens".to_string())); }


        let mut lhs = match tks.pop_front().unwrap() {
            Token::Ident(ident) => ExprNode::Ident(ident),
            Token::Num(n) => ExprNode::Num(n),
            Token::Operator(op) => match op.bp {
                BindingPower::Seperator | BindingPower::Binary(_, _) => return Err(format!("Expected unary operator, got {}", op.token)),
                BindingPower::Unary | BindingPower::BinaryOrUnary(_, _) => {
                    let rhs = Self::make(tks, 9999.0)?;
                    ExprNode::UnaryOperation { op: op.token, operand: Box::new(rhs) }
                }
            },
            Token::ParenthesesGroup(mut grp) => {
                //we only expect 1 argument, we can just discard everything else
                Self::make(&mut grp, 0.0)?
            }
        };


        while !tks.is_empty() {


            lhs = match tks.front().unwrap().clone() {
                //identifier or number = multiplication
                Token::Ident(_) | Token::Num(_) => ExprNode::BinaryOperation { op: "*", operands: Box::new((lhs, Self::make(tks, 2.1)?)) },
                Token::Operator(Operator { token, bp }) => match bp {
                    BindingPower::Seperator => break,
                    BindingPower::Unary => return Err(format!("Expected binary operator or ',', got {}", token)),
                    BindingPower::Binary(lbp, rbp) | BindingPower::BinaryOrUnary(lbp, rbp) => if lbp < min_bp { break } else {
                        tks.pop_front();

                        BinaryOperation { op: token, operands: Box::new((lhs, Self::make(&mut *tks, rbp)?)) }
                    }
                }
                //call operation
                Token::ParenthesesGroup(_) => {
                    if let Some(Token::ParenthesesGroup(mut tks)) = tks.pop_front() {
                        CallOperation { caller: Box::new(lhs), args: Self::parse_call(&mut tks)? }
                    } else {
                        abort(); //shouldn't ever happen
                    }
                }
            }

        }


        Ok(lhs)
    }
}


impl Debug for ExprNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprNode::BinaryOperation { op, operands } => write!(f, "({:?} {} {:?})", operands.0, op, operands.1),
            ExprNode::UnaryOperation { op, operand } => write!(f, "({}{:?})", op, operand),
            ExprNode::CallOperation{ caller, args } => {
                f.write_str("(")?;
                caller.as_ref().fmt(f)?;
                f.write_str("(")?;
                let mut first = true;
                for i in args {
                    if !first {
                        f.write_str(", ")?;
                    }
                    first = false;
                    i.fmt(f)?;
                }
                f.write_str("))")
            },
            ExprNode::Ident(s) => f.write_str(s.as_str()),
            ExprNode::Num(n) => f.write_str(n.to_string().as_str()),
            Error(s) => write!(f, "(Error: {})", s),
            ExprNode::Void => f.write_str("void"),
        }
    }
}
