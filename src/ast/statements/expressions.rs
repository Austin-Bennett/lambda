
use crate::lexer::iter::TokenIterator;
use std::ptr;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter, Write};
use std::mem::discriminant;
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::ty::{Type, TypeSyntax};
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::literal::IntegerLiteral;
use crate::lexer::token::{ExpressionToken, FeatureToken, Token, TokenType};

#[derive(Hash, Clone)]
pub struct BinaryOperation {
    pub op: Operator,
    pub lhs: ExprSyntax,
    pub rhs: ExprSyntax,
}

#[derive(Hash, Clone)]
pub struct IndexOperation {
    pub operand: ExprSyntax,
    pub index: ExprSyntax,
}

#[derive(Hash, Clone)]
pub struct UnaryOperation {
    pub op: Operator,
    pub operand: ExprSyntax,
}

#[derive(Hash, Clone)]
pub struct CallOperation {
    pub caller: ExprSyntax,
    pub arguments: Vec<ExprSyntax>,
}

#[derive(Hash, Clone)]
pub struct CastOperation {
    pub expr: ExprSyntax,
    pub ty: Type,
}

#[derive(Hash, Clone)]
pub enum Expr {
    Identifier(String),
    IntLiteral(IntegerLiteral),

    Tuple(Vec<ExprSyntax>),
    
    Array(Vec<ExprSyntax>),
    Index(Box<IndexOperation>),
    
    BinaryOp(Box<BinaryOperation>),
    UnaryOp(Box<UnaryOperation>),
    CallOp(Box<CallOperation>),
    CastOp(Box<CastOperation>),
}


pub type ExprSyntax = GenericSyntax<Expr>;

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        match self {
            Expr::Identifier(ident) => {
                write!(f, "{}", ident)?;
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

                    write!(f, "{:?}", expr.data)?;
                }
                f.write_str(")")?;
            }
            Expr::Array(array) => {
                f.write_str("[")?;
                let mut first = true;
                for expr in array {

                    if !first {
                        f.write_str(", ")?;
                    }
                    first = false;

                    write!(f, "{:?}", expr.data)?;
                }
                f.write_str("]")?;
            }
            Expr::Index(index) => {
                
                write!(f, "{:?}[{:?}]", index.operand.data, index.index.data)?;
            }
            Expr::BinaryOp(bop) => {
                write!(f, "({:?} {} {:?})", bop.lhs.data, bop.op.tk, bop.rhs.data)?;
            }
            Expr::UnaryOp(uop) => {
                write!(f, "{}{:?}", uop.op.tk, uop.operand.data)?;
            }
            Expr::CallOp(call) => {

                write!(f, "{:?}(", call.caller.data)?;

                let mut first = true;
                for expr in &call.arguments {

                    if !first {
                        f.write_str(", ")?;
                    }
                    first = false;

                    write!(f, "{:?}", expr.data)?;
                }
                f.write_str(")")?;
            }
            Expr::CastOp(cast) => {
                write!(f, "({:?} as {:?})", cast.expr.data, cast.ty)?;
            }
        }


        Ok(())
    }
}

impl ExprSyntax {



    pub fn make_binary_op(&mut self, op: Operator, other: ExprSyntax) {

        
        
        let expr = self.clone();
        self.data = Expr::BinaryOp(
            Box::new(
                BinaryOperation{
                    op,
                    lhs: expr,
                    rhs: other
                }
            )
        );
        

    }

    pub fn make_call_expression(&mut self, args: Vec<ExprSyntax>) {
        
        let expr = self.clone();
        self.data = Expr::CallOp(
            Box::new(
                CallOperation{
                    caller: expr,
                    arguments: args
                }
            )
        );
    
    }
    
    pub fn parse_parentheses(tokens: &mut VecDeque<Token>, compiler: &mut Compiler, open_token: &ExpressionToken, close_token: &ExpressionToken) -> Outcome<(Vec<ExprSyntax>, SourceMap), CompileMessage> {
        let mut smap = if let Some((e, smap)) = tokens.peek_expression() {

            if discriminant(open_token) == discriminant(e) {
                let Some((_, smap)) = tokens.next_expression() else { unreachable!() };

                smap
            } else {
                smap.clone()
            }

        } else {
            return Outcome::None
        };
        
        
        let mut res = Vec::new();
        loop {
            let expr = match Self::make_expression(tokens, 0, compiler) {
                Outcome::Ok(e) => e,
                Outcome::None => break,
                Outcome::Err(v) => return Outcome::Err(v),
            };
            smap.extend(&expr.smap);
            
            res.push(expr);

            if let Some((tk, _)) = tokens.peek_expression() && discriminant(tk) == discriminant(close_token) {
                let Some((_, emap)) = tokens.next_expression() else { unreachable!() };

                smap.extend(emap);
                break;
            } else if let Some(Token{ typ: TokenType::Feature(FeatureToken::Comma), smap: _emap }) = tokens.get(0) {
                let Some(Token{ typ: TokenType::Feature(FeatureToken::Comma), smap: emap }) = tokens.pop_front() else { unreachable!() };

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
    
    pub fn make_expression(tokens: &mut VecDeque<Token>, minbp: u8, compiler: &mut Compiler) -> Outcome<Self, CompileMessage> {
        //get the first expression token
        let Some((expr, mut smap)) = tokens.next_expression() else {
            return Outcome::None;
        };

        let mut lhs = match expr {
            //identifiers or literals
            ExpressionToken::Identifier(ident) => {
                ExprSyntax{
                    data: Expr::Identifier(ident),
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
                let rhs = match Self::make_expression(tokens, op.bp.effective_unary_rbp(), compiler)? {
                    Some(v) => v,
                    None => return Outcome::Err(CompileMessage::new(smap, format!("Expected expression after operator \'{}\'", op.tk), CompileMessageType::Error)),
                };



                smap.extend(&rhs.smap);


                
                ExprSyntax{
                    data: Expr::UnaryOp(
                        Box::new(
                            UnaryOperation{
                                op,
                                operand: rhs,
                            }
                        )
                    ),
                    smap
                }
            }
            //an open parenthese
            ExpressionToken::OpenParentheses => {
                let (mut tuple, emap) = match Self::parse_parentheses(tokens, compiler, 
                                                                      &ExpressionToken::OpenParentheses, &ExpressionToken::CloseParentheses)? {
                    Some(v) => v,
                    None => return Outcome::Err(
                        CompileMessage::new(smap, "Expected ')' to close this '('".to_string(), CompileMessageType::Error),
                    ),
                };

                smap.extend(emap);

                if tuple.len() == 1 {
                    tuple.remove(0)
                    
                } else {
                    ExprSyntax{
                        smap,
                        data: Expr::Tuple(tuple)
                    }
                }
            }

            ExpressionToken::OpenBracket => {
                let (array, emap) = match Self::parse_parentheses(tokens, compiler,
                                                                      &ExpressionToken::OpenBracket, &ExpressionToken::CloseBracket)? {
                    Some(v) => v,
                    None => return Outcome::Err(
                        CompileMessage::new(smap, "Expected ']' to close this '['".to_string(), CompileMessageType::Error),
                    ),
                };

                smap.extend(emap);

                ExprSyntax{
                    smap,
                    data: Expr::Array(array)
                }
            
            }
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
                    let rhs = unsafe { Self::make_expression(tokens, Operator::MUL.bp.effective_rbp(), compiler)?.unwrap_unchecked() };

                    lhs.smap.extend(&rhs.smap);
                    lhs.make_binary_op(Operator::MUL, rhs);
                }
                ExpressionToken::Operator(op) => {

                    if op.bp.effective_lbp() < minbp {
                        break;
                    }

                    let Some((ExpressionToken::Operator(op), emap)) = tokens.next_expression() else {
                        unreachable!()
                    };

                    if !op.bp.is_binary() {
                        return Outcome::Err(CompileMessage::new(
                            emap,
                            format!("operator {} cannot be used in binary operation", op.tk),
                            CompileMessageType::Error
                        ));
                    }






                    //must be a binary operator
                    let rhs = match Self::make_expression(tokens, op.bp.effective_rbp(), compiler)? {
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
                    lhs.smap.extend(&rhs.smap);


                    lhs.make_binary_op(op, rhs);
                }
                ExpressionToken::OpenParentheses => {
                    //a call expression
                    let emap = emap.clone();
                    let (args, emap) = match Self::parse_parentheses(tokens, compiler, 
                                                                     &ExpressionToken::OpenParentheses, &ExpressionToken::CloseParentheses)? {
                        Some(args) => args,
                        None => return Outcome::Err(CompileMessage::new(
                            emap,
                            "expected ')' to close this call expression!".to_string(),
                            CompileMessageType::Error
                        ))
                    };


                    lhs.smap.extend(emap);
                    lhs.make_call_expression(args)
                }
                ExpressionToken::OpenBracket => {
                    //index operation
                    let emap = emap.clone();
                    let (mut index, emap) = match Self::parse_parentheses(tokens, compiler,
                                                                     &ExpressionToken::OpenBracket, &ExpressionToken::CloseBracket)? {
                        Some(args) => args,
                        None => return Outcome::Err(CompileMessage::new(
                            emap,
                            "expected ']' to close this index expression!".to_string(),
                            CompileMessageType::Error
                        ))
                    };
                    
                    if index.len() != 1 {
                        return Outcome::Err(CompileMessage::new(
                            emap,
                            "Expected exactly 1 item in indexing operator!".to_string(),
                            CompileMessageType::Error,
                        ))
                    }
                    


                    lhs.data = Expr::Index(
                        Box::new(IndexOperation{
                            operand: ExprSyntax{ smap: lhs.smap.clone(), data: lhs.data },
                            index: index.remove(0),
                        })
                    );
                    lhs.smap.extend(emap);
                }
                ExpressionToken::CloseParentheses => {
                    break;
                }
                ExpressionToken::CloseBracket => {
                    break;
                }
                ExpressionToken::AsKW => {
                    let (_, as_smap) = tokens.next_expression().unwrap();
                    let Some(ty_syntax) = TypeSyntax::parse(tokens, compiler) else {
                        return Outcome::Err(CompileMessage::new(
                            as_smap,
                            "expected type after `as`".into(),
                            CompileMessageType::Error,
                        ));
                    };
                    let old_lhs = lhs.clone();
                    lhs.smap.extend(&ty_syntax.smap);
                    lhs.data = Expr::CastOp(Box::new(CastOperation {
                        expr: old_lhs,
                        ty: ty_syntax.data,
                    }));
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
        match Self::make_expression(tokens, 0, compiler) {
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