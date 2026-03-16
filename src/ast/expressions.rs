use crate::lexer::iter::TokenIterator;
use std::iter::Peekable;
use crate::ast::Syntax;
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::compiler::{CompileError, CompilerError};
use crate::lexer::literal::IntegerLiteral;
use crate::lexer::token::{ExpressionToken, Token, TokenType};

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
    
    pub fn parse_parentheses(tokens: &mut Peekable<impl Iterator<Item=Token>>) -> Result<
        (Vec<Expr>, SourceMap),
        Option<CompileError>
    > {
        let Some((ExpressionToken::OpenParentheses, mut smap)) = tokens.next_expression() else {
            return Err(None);
        };
        
        
        let mut res = Vec::new();
        'outer: loop {
            let expr = match Self::make_expression(tokens, 0) {
                Ok(e) => e,
                Err(v) => if v.is_none() { break 'outer; } else { return Err(v); }
            };
            let end = expr.smap.offset + expr.smap.len;
            smap.len = end - smap.offset;
            
            res.push(expr.expression);
        }
        
        
        Ok((res, smap))
        
    }
    
    fn make_expression(tokens: &mut Peekable<impl Iterator<Item=Token>>, minbp: u8) -> CompilerError<Self> {
        let Some((expr, mut smap)) = tokens.next_expression() else {
            return Err(None);
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
    fn parse<'a>(tokens: &mut Peekable<impl Iterator<Item=Token>>) -> CompilerError<Self>
    where
        Self: Sized
    {
        Self::make_expression(tokens, 0)
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}