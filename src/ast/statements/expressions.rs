
use crate::lexer::iter::TokenIterator;
use std::ptr;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter, Write};
use crate::ast::{GenericSyntax, Syntax};
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
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

pub struct CallOperation {
    pub caller: Expr,
    pub arguments: Vec<Expr>,
}

pub enum Expr {
    Identifier(ModulePath),
    IntLiteral(IntegerLiteral),

    Tuple(Vec<Expr>),

    BinaryOp(Box<BinaryOperation>),
    UnaryOp(Box<UnaryOperation>),
    CallOp(Box<CallOperation>),
}

pub type ExprSyntax = GenericSyntax<Expr>;

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        match self {
            Expr::Identifier(ident) => {
                write!(f, "{:?}", ident)?;
            }
            Expr::IntLiteral(int) => {
                write!(f, "{:?}", int)?;
            }
            Expr::Tuple(tuple) => {
                f.write_str("(")?;
                let mut first = true;
                for expr in tuple {

                    if !first {
                        f.write_str(", ")?;
                    }
                    first = false;

                    write!(f, "{:?}", expr)?;
                }
                f.write_str(")")?;
            }
            Expr::BinaryOp(bop) => {
                write!(f, "({:?} {} {:?})", bop.lhs, bop.op.tk, bop.rhs)?;
            }
            Expr::UnaryOp(uop) => {
                write!(f, "{}{:?}", uop.op.tk, uop.operand)?;
            }
            Expr::CallOp(call) => {

                write!(f, "{:?}(", call.caller)?;

                let mut first = true;
                for expr in &call.arguments {

                    if !first {
                        f.write_str(", ")?;
                    }
                    first = false;

                    write!(f, "{:?}", expr)?;
                }
                f.write_str(")")?;
            }
        }


        Ok(())
    }
}

impl ExprSyntax {



    pub fn make_binary_op(&mut self, op: Operator, other: Expr) {

        unsafe{
            //SAFETY this is used to move out the old expression and turn it into this
            //this is safe, because if self were just a regular old mut value, then
            //i wouldnt even need the pointer semantics
            let expr = ptr::read(&raw mut self.data);
            ptr::write(&raw mut self.data, Expr::BinaryOp(
                Box::new(
                    BinaryOperation{
                        op,
                        lhs: expr,
                        rhs: other
                    }
                )
            ));
        }

    }

    pub fn make_call_expression(&mut self, args: Vec<Expr>) {

        unsafe{
            //SAFETY this is used to move out the old expression and turn it into this
            //this is safe, because if self were just a regular old mut value, then
            //i wouldnt even need the pointer semantics
            let expr = ptr::read(&raw mut self.data);
            ptr::write(&raw mut self.data, Expr::CallOp(
                Box::new(
                    CallOperation{
                        caller: expr,
                        arguments: args
                    }
                )
            ));
        }
    }
    
    pub fn parse_parentheses(tokens: &mut VecDeque<Token>) -> Outcome<(Vec<Expr>, SourceMap), CompileMessage> {
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
            
            res.push(expr.data);

            if let Some((ExpressionToken::CloseParentheses, _emap)) = tokens.peek_expression() {
                let Some((ExpressionToken::CloseParentheses, emap)) = tokens.next_expression() else { panic!("Shouldn't happen") };

                smap.extend(emap);
                break;
            } else if let Some(Token{ typ: TokenType::Feature(FeatureToken::Comma), smap: _emap }) = tokens.get(0) {
                let Some(Token{ typ: TokenType::Feature(FeatureToken::Comma), smap: emap }) = tokens.pop_front() else { panic!("Shouldn't happen") };

                smap.extend(emap);
            } else if let Some(tk) = tokens.pop_front() {
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
    
    pub fn make_expression(tokens: &mut VecDeque<Token>, minbp: u8) -> Outcome<Self, CompileMessage> {
        //get the first expression token
        let Some((expr, mut smap)) = tokens.next_expression() else {
            return Outcome::None;
        };

        let mut lhs = match expr {
            //identifiers or literals
            ExpressionToken::Identifier(ident) => {
                ExprSyntax{
                    data: Expr::Identifier(ModulePath::from_module_path(ident)),
                    smap
                }
            }
            ExpressionToken::IntegerLiteral(i) => {
                ExprSyntax{
                    data: Expr::IntLiteral(i),
                    smap
                }
            }
            ExpressionToken::Operator(op) => {

                if !op.bp.is_unary() {
                    return Outcome::Err(
                        CompileMessage::new(
                            smap,
                            format!("operator \'{}\' can not be used in a unary operation", op.tk),
                            CompileMessageType::Error,
                        )
                    )
                }

                //must be a unary operator
                let rhs = match Self::make_expression(tokens, 0)? {
                    Some(v) => v,
                    None => return Outcome::Err(CompileMessage::new(smap, format!("Expected expression after operator \'{}\'", op.tk), CompileMessageType::Error)),
                };



                smap.extend(rhs.smap);


                
                ExprSyntax{
                    data: Expr::UnaryOp(
                        Box::new(
                            UnaryOperation{
                                op,
                                operand: rhs.data,
                            }
                        )
                    ),
                    smap
                }
            }
            //an open parenthese
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
                        data: tuple.remove(0)
                    }
                } else {
                    ExprSyntax{
                        smap,
                        data: Expr::Tuple(tuple)
                    }
                }
            }

            //todo: parse array expressions [...]
            _ => return Outcome::None,
        };
        
        loop {

            //grab the next token
            let Some((rhs, emap)) = tokens.peek_expression() else {
                break;
            };

            match rhs {
                ExpressionToken::Identifier(_) | ExpressionToken::IntegerLiteral(_) => {
                    //expressions such as 2a will go here, this is multiplication, so we will inline the
                    //multiplication parse operation here
                    if Operator::MUL.bp.effective_lbp() < minbp { break }

                    //there is guaranteed to at least be a identifier or integer
                    let rhs = unsafe { Self::make_expression(tokens, Operator::MUL.bp.effective_rbp())?.unwrap_unchecked() };

                    lhs.smap.extend(rhs.smap);
                    lhs.make_binary_op(Operator::MUL, rhs.data);
                }
                ExpressionToken::Operator(op) => {

                    if op.bp.effective_lbp() < minbp {
                        break;
                    }

                    let Some((ExpressionToken::Operator(op), emap)) = tokens.next_expression() else {
                        panic!("Shouldn't happen")
                    };

                    if !op.bp.is_binary() {
                        return Outcome::Err(CompileMessage::new(
                            emap,
                            format!("operator {} cannot be used in binary operation", op.tk),
                            CompileMessageType::Error
                        ));
                    }






                    //must be a binary operator
                    let rhs = match Self::make_expression(tokens, op.bp.effective_rbp())? {
                        Some(v) => v,
                        //todo: postfix
                        None => return Outcome::Err(
                            CompileMessage::new(
                                emap,
                                format!("expected expression after operator \'{}\'", op.tk),
                                CompileMessageType::Error
                        ))
                    };
                    lhs.smap.extend(emap);
                    lhs.smap.extend(rhs.smap);


                    lhs.make_binary_op(op, rhs.data);
                }
                ExpressionToken::OpenParentheses => {
                    //a call expression
                    let emap = emap.clone();
                    let (args, emap) = match Self::parse_parentheses(tokens)? {
                        Some(args) => args,
                        None => return Outcome::Err(CompileMessage::new(
                            emap,
                            "expected ')' to close this tuple expressions!".to_string(),
                            CompileMessageType::Error
                        ))
                    };


                    lhs.smap.extend(emap);
                    lhs.make_call_expression(args)
                }
                ExpressionToken::CloseParentheses => {
                    break;
                }
                _ => break
            }


        }
        
        Outcome::Ok(lhs)
    }
}

impl Syntax for ExprSyntax {
    fn parse<'a>(tokens: &mut VecDeque<Token>, compiler: &mut Compiler) -> Option<Self>
    where
        Self: Sized
    {
        match Self::make_expression(tokens, 0) {
            Outcome::Ok(v) => { Some(v) }
            Outcome::None => { None }
            Outcome::Err(e) => { 
                compiler.emit_compile_message(e);
                None
            }
        }
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}