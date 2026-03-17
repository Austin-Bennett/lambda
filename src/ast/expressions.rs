
use crate::lexer::iter::TokenIterator;
use std::iter::Peekable;
use std::{mem, ptr};
use std::process::abort;
use crate::ast::Syntax;
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::compiler::{CompileMessage, CompileMessageType};
use crate::lexer::literal::IntegerLiteral;
use crate::lexer::token::{ExpressionToken, FeatureToken, Token, TokenType};

pub struct BinaryOperation {
    pub op: Operator,
    pub lhs: Expr,
    pub rhs: Expr,
}

pub struct UnaryOperation {
    pub op: Operator,
    pub operand: Expr,
}

pub enum Expr {
    Identifier(ModulePath),
    BinaryOp(Box<BinaryOperation>),
    UnaryOp(Box<UnaryOperation>),

    Tuple(Vec<Expr>),
    IntLiteral(IntegerLiteral),
}

pub struct ExprSyntax {
    expression: Expr,
    smap: SourceMap,
}



impl ExprSyntax {

    pub fn make_binary_op(&mut self, op: Operator, other: Expr) {
        let this = self as *mut Self;

        unsafe{
            //SAFETY this is used to move out the old expression and turn it into this
            //this is safe, because if self were just a regular old mut value, then
            //i wouldnt even need the pointer semantics
            (*this).expression = Expr::BinaryOp(
                Box::new(
                    BinaryOperation{
                        op,
                        lhs: ptr::read(&raw mut (*this).expression),
                        rhs: other
                    }
                )
            );
        }

    }
    
    pub fn parse_parentheses(tokens: &mut Peekable<impl Iterator<Item=Token>>) -> Outcome<(Vec<Expr>, SourceMap), CompileMessage> {
        let mut smap = if let Some((e, smap)) = tokens.peek_expression() {

            if let ExpressionToken::OpenParentheses = e {
                let Some((_, smap)) = tokens.next_expression() else { panic!("Shouldn't happen") };

                smap
            } else {
                smap.clone()
            }

        } else {
            return Outcome::None
        };
        
        
        let mut res = Vec::new();
        loop {
            let expr = match Self::make_expression(tokens, 0) {
                Outcome::Ok(e) => e,
                Outcome::None => break,
                Outcome::Err(v) => return Outcome::Err(v),
            };
            smap.extend(expr.smap);
            
            res.push(expr.expression);

            if let Some((ExpressionToken::CloseParentheses, emap)) = tokens.peek_expression() {
                let Some((ExpressionToken::CloseParentheses, emap)) = tokens.next_expression() else { panic!("Shouldn't happen") };

                smap.extend(emap);
                break;
            } else if let Some(Token{ typ: TokenType::Feature(FeatureToken::Comma), smap: emap }) = tokens.peek() {
                let Some((ExpressionToken::CloseParentheses, emap)) = tokens.next_expression() else { panic!("Shouldn't happen") };

                smap.extend(emap);
                break;
            } else if let Some(tk) = tokens.next() {
                return Outcome::Err(
                    CompileMessage::new(tk.smap, format!("Expected ')', got token: {:?}", tk.typ), CompileMessageType::Error)
                        .add_note(
                            CompileMessage::note(smap, "Note: to end this tuple expression".into())
                        )
                )
            } else {
                return Outcome::Err(
                    CompileMessage::new(smap, "Expected ')' to end this tuple expression".into(), CompileMessageType::Error)
                )
            }
        }
        
        

        Outcome::Ok((res, smap))
    }
    
    fn make_expression(tokens: &mut Peekable<impl Iterator<Item=Token>>, minbp: u8) -> Outcome<Self, CompileMessage> {
        let Some((expr, mut smap)) = tokens.next_expression() else {
            return Outcome::None;
        };

        let mut lhs = match expr {
            ExpressionToken::Identifier(ident) => {
                ExprSyntax{
                    expression: Expr::Identifier(ModulePath::from_module_path(ident)),
                    smap
                }
            }
            ExpressionToken::IntegerLiteral(i) => {
                ExprSyntax{
                    expression: Expr::IntLiteral(i),
                    smap
                }
            }
            ExpressionToken::Operator(op) => {
                //must be a unary operator
                let rhs = match Self::make_expression(tokens, 0)? {
                    Some(v) => v,
                    None => return Outcome::Err(CompileMessage::new(smap, format!("Expected expression after operator \'{}\'", op.tk), CompileMessageType::Error)),
                };


                smap.extend(rhs.smap);
                
                ExprSyntax{
                    expression: Expr::UnaryOp(
                        Box::new(
                            UnaryOperation{
                                op,
                                operand: rhs.expression,
                            }
                        )
                    ),
                    smap
                }
            }
            ExpressionToken::OpenParentheses => {
                let (mut tuple, emap) = match Self::parse_parentheses(tokens)? {
                    Some(v) => v,
                    None => return Outcome::Err(
                        CompileMessage::new(smap, "Expected ')' to close this '('".to_string(), CompileMessageType::Error),
                    ),
                };

                smap.extend(emap);

                if tuple.len() == 1 {
                    ExprSyntax{
                        smap,
                        expression: tuple.remove(0)
                    }
                } else {
                    ExprSyntax{
                        smap,
                        expression: Expr::Tuple(tuple)
                    }
                }
            }
            _ => return Outcome::None
        };
        
        loop {

            let Some((rhs, emap)) = tokens.next_expression() else {
                break;
            };

            //firstly, extend the sourcemap
            lhs.smap.extend(emap);

            match rhs {
                //an identifier of literal together means multiplication
                ExpressionToken::Identifier(i) => {
                lhs.make_binary_op(Operator::MUL, Expr::Identifier(ModulePath::from_module_path(i)))
                }
                ExpressionToken::IntegerLiteral(i) => {
                    
                }
                ExpressionToken::Operator(_) => {}
                ExpressionToken::OpenParentheses => {}
                ExpressionToken::CloseParentheses => {}
                ExpressionToken::OpenBracket => {}
                ExpressionToken::CloseBracket => {}
            }


        }
        
        Outcome::Ok(lhs)
    }
}

impl Syntax for ExprSyntax {
    fn parse<'a>(tokens: &mut Peekable<impl Iterator<Item=Token>>) -> Outcome<Self, CompileMessage>
    where
        Self: Sized
    {
        Self::make_expression(tokens, 0)
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}