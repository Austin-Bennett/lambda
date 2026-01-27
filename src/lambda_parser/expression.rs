use crate::lambda_parser::tokenization::{tokenize, BindingPower, Token};
use crate::lambda_parser::ExprNode::{BinaryOperation, CallOperation, Error};
use crate::lambda_parser::{Operator, MINBP};
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter, Write};
use std::process::abort;
use rust_decimal::Decimal;
use crate::lambda_jit::lambda_il::Instruction;

#[derive(Clone, PartialEq)]
pub enum ExprNode {
    CallOperation{caller: Box<ExprNode>, args: Vec<ExprNode>},
    BinaryOperation{op: &'static str, operands: Box<(ExprNode, ExprNode)>},
    UnaryOperation{op: &'static str, operand: Box<ExprNode>},
    Ident(String),
    Num(Decimal),
    FunctionDecl{ident: String, expr: Box<ExprNode>},
    Argument(usize),
    InlineInstruction(Instruction),
    Void,
    Error(String), //internal, shouldn't show up in the tree
}



impl ExprNode {


    pub fn replace_identifier(&mut self, ident: &impl AsRef<str>, node: ExprNode) {
        //walks the tree and replaces the corresponding identifier with the node
        match self {
            ExprNode::CallOperation { caller, args } => {
                caller.replace_identifier(ident, node.clone());
                for n in args {
                    n.replace_identifier(ident, node.clone());
                }
            }
            ExprNode::BinaryOperation { op, operands } => {
                operands.0.replace_identifier(ident, node.clone());
                operands.1.replace_identifier(ident, node.clone());
            }
            ExprNode::UnaryOperation { op, operand } => {
                operand.replace_identifier(ident, node)
            }
            ExprNode::Ident(id) => {
                if id == ident.as_ref() {
                    *self = node;
                }
            }
            ExprNode::InlineInstruction(_) => {}
            ExprNode::Num(_) => {}
            ExprNode::FunctionDecl { expr, .. } => {
                expr.replace_identifier(ident, node)
            }
            ExprNode::Argument(_) => {}
            ExprNode::Void => {}
            Error(_) => {}
        }

    }

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
        Self::make(&mut tks, MINBP)
    }

    fn parse_call(tks: &mut VecDeque<Token>) -> Result<Vec<Self>, String> {
        let mut res = Vec::new();

        while !tks.is_empty() {
            let arg = Self::make(tks, MINBP)?;
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
                Self::make(&mut grp, MINBP)?
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
                    if let Some(Token::ParenthesesGroup(mut arg_tks)) = tks.pop_front() {
                        let args = Self::parse_call(&mut arg_tks)?;
                        // check if the next token is '=', if so, this is (probably) a function declaration
                        if let ExprNode::Ident(id) = &mut lhs &&
                            let Some(Token::Operator(Operator{ token: "=", bp })) = tks.front() {
                            tks.pop_front();

                            let mut argvec = Vec::new();
                            for a in args {
                                if let ExprNode::Ident(s) = a {
                                    argvec.push(s);
                                } else {
                                    return Err(format!("Expected identifier for function argument, got {:?}", a));
                                }
                            }

                            let mut rhs = Self::make(tks, f32::NEG_INFINITY)?;

                            for (i, a) in argvec.iter().enumerate() {
                                rhs.replace_identifier(a, ExprNode::Argument(argvec.len()-i-1))
                            }

                            ExprNode::FunctionDecl { ident: std::mem::replace(id, String::new()), expr: Box::new(rhs) }
                        } else {
                            CallOperation { caller: Box::new(lhs), args }
                        }
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
            ExprNode::InlineInstruction(i) => write!(f, "{:?}", i),
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
            ExprNode::Argument(u) => write!(f, "Argument({})", u),
            ExprNode::FunctionDecl { ident, expr } => write!(f, "{}() = {:?}", ident, expr),
        }
    }
}
