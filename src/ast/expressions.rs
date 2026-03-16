use crate::lexer::iter::TokenIterator;
use std::iter::Peekable;
use std::process::abort;
use crate::ast::Syntax;
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::compiler::{CompileMessage};
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
            let end = expr.smap.offset + expr.smap.len;
            smap.len = end - smap.offset;
            
            res.push(expr.expression);

            if let Some((ExpressionToken::CloseParentheses, emap)) = tokens.peek_expression() {
                let Some((ExpressionToken::CloseParentheses, emap)) = tokens.next_expression() else { panic!("Shouldn't happen") };

                let end = emap.offset + emap.len;
                smap.len = end - smap.offset;
                break;
            } else if let Some(Token{ typ: TokenType::Feature(FeatureToken::Comma), smap: emap }) = tokens.peek() {
                let Some((ExpressionToken::CloseParentheses, emap)) = tokens.next_expression() else { panic!("Shouldn't happen") };

                let end = emap.offset + emap.len;
                smap.len = end - smap.offset;
                break;
            } else if let Some(tk) = tokens.next() {
                return Outcome::Err(
                    CompileMessage::new(tk.smap, format!("Expected ')', got token: {:?}", tk.typ))
                        .add_note(
                            CompileMessage::new(smap, "Note: to end this tuple expression".into())
                        )
                )
            } else {
                return Outcome::Err(
                    CompileMessage::new(smap, "Expected ')' to end this tuple expression".into())
                )
            }
        }
        
        

        Outcome::Ok((res, smap))
    }
    
    fn make_expression(tokens: &mut Peekable<impl Iterator<Item=Token>>, minbp: u8) -> Outcome<Self, CompileMessage> {
        let Some((expr, smap)) = tokens.next_expression() else {
            return Outcome::None;
        };

        let lhs = match expr {
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
                let rhs = Self::make_expression(tokens, 0)?;
                
                let end = rhs.smap.offset + rhs.smap.len;
                smap.len = end - smap.offset;
                
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
                
            }
            _ => return Err(None)
        };
        
        
        
        lhs
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