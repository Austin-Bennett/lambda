use crate::lambda_parser::tokenization::{tokenize, BindingPower, Token};
use crate::lambda_parser::ExprNode::{BinaryOperation, CallOperation, Error, Void};
use crate::lambda_parser::{NumericalLiteral, Operator, TokenHelpers, MINBP};
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::process::abort;

#[derive(Clone, PartialEq)]
pub enum ExprNode {
    CallOperation{caller: String, args: Vec<ExprNode>},
    FBinaryOperation{op: &'static str, operands: Box<(ExprNode, ExprNode)>},
    IBinaryOperation{op: &'static str, operands: Box<(ExprNode, ExprNode)>},
    FUnaryOperation{op: &'static str, operand: Box<ExprNode>},
    IUnaryOperation{op: &'static str, operand: Box<ExprNode>},
    CastInt(Box<ExprNode>),
    CastFloat(Box<ExprNode>),
    Ident(String),
    IntNum(i64),
    FloatNum(f64),
    Void,
    Error(String), //internal, shouldn't show up in the tree
}



impl ExprNode {


    //false if floating, true if integer
    pub fn is_integer_operation(&self) -> Option<bool> {
        match self {
            ExprNode::CallOperation { caller, args } => {
                if args.is_empty() {
                    //assume floating
                    None
                } else {
                    args[0].is_integer_operation()
                }
            }
            ExprNode::FBinaryOperation { .. } => {
                Some(false)
            }
            ExprNode::IBinaryOperation { .. } => {
                Some(true)
            }
            ExprNode::FUnaryOperation { .. } => {
                Some(false)
            }
            ExprNode::IUnaryOperation { .. } => {
                Some(true)
            }
            ExprNode::CastInt(_) => {
                Some(true)
            }
            ExprNode::CastFloat(_) => {
                Some(false)
            }
            ExprNode::Ident(ident) => {
                None
            }
            ExprNode::IntNum(_) => {
                Some(true)
            }
            ExprNode::FloatNum(_) => {
                Some(false)
            }
            Void => {
                None
            }
            Error(_) => {
                None
            }
        }
    }



    pub fn from_tokens(tks: &mut VecDeque<Token>) -> Result<Self, String> {
        Self::make(tks, MINBP)
    }

    fn parse_call(tks: &mut VecDeque<Token>) -> Result<Vec<Self>, String> {
        let mut res = Vec::new();

        while tks.next_is_expr() {
            let arg = Self::make(tks, MINBP)?;
            res.push(arg);

            if let Some(Token::Operator(Operator{ token: _, bp: BindingPower::Seperator })) = tks.front() {
                tks.pop_front();
            }
        }

        Ok(res)
    }

    fn make(tks: &mut VecDeque<Token>, min_bp: f32) -> Result<Self, String> {

        if !tks.next_is_expr() { return Ok(Error("Didnt expect end of tokens".to_string())); }


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
                Self::make(&mut grp, MINBP)?
            }
            _ => Void
        };


        while tks.next_is_expr() {


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
                    if let Some(Token::ParenthesesGroup(mut arg_tks)) = tks.pop_front() {
                        let args = Self::parse_call(&mut arg_tks)?;
                        // check if the next token is '=', if so, this is (probably) a function declaration todo: ?
                        if let ExprNode::Ident(s) = lhs {

                            CallOperation { caller: s, args }
                        } else {
                            Error("Expected function identifier, got expression".to_string())
                        }

                    } else {
                        abort(); //shouldn't ever happen
                    }
                }
                _ => Void
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
                f.write_str(&caller)?;
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
