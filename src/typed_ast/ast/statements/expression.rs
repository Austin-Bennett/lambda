use std::fmt::{Debug, Formatter, Pointer, Write};
use crate::ast::GenericSyntax;
use crate::ast::statements::expressions::{Expr, ExprSyntax};
use crate::common::operator::Operator;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::literal::IntegerLiteral;
use crate::typed_ast::ast::type_inference::TypeInferencer;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::TypeId;

pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Debug for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => f.write_str("+"),
            BinaryOperator::Sub => f.write_str("-"),
            BinaryOperator::Mul => f.write_str("*"),
            BinaryOperator::Div => f.write_str("/")
        }
    }
}



pub enum UnaryOperator {
    Neg,
}

impl Debug for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { 
            UnaryOperator::Neg => f.write_str("-"),
        }
    }
}

pub struct TypedBinaryOperation {
    pub op: BinaryOperator,
    pub lhs: TypedExprNode,
    pub rhs: TypedExprNode,
}

pub struct TypedUnaryOperation {
    pub op: UnaryOperator,
    pub operand: TypedExprNode,
}

pub struct TypedCallOperation {
    pub caller: TypedExprNode,
    pub arguments: Vec<TypedExpr>,
}

pub enum TypedExprNode {
    IntLiteral(IntegerLiteral),
    Identifier(String),

    Tuple(Vec<TypedExprNode>),

    BinaryOp(Box<TypedBinaryOperation>),
    UnaryOp(Box<TypedUnaryOperation>),
    CallOp(Box<TypedCallOperation>),
}

impl TypedExprNode {
    pub fn is_int_literal_expr(&self) -> bool {
        match self {
            TypedExprNode::IntLiteral(_) => { true }
            TypedExprNode::Identifier(_) => { false }
            TypedExprNode::Tuple(_) => { false }
            TypedExprNode::BinaryOp(bop) => { bop.lhs.is_int_literal_expr() && bop.rhs.is_int_literal_expr() }
            TypedExprNode::UnaryOp(uop) => { uop.operand.is_int_literal_expr() }
            TypedExprNode::CallOp(call) => { false }
        }
    }
}

pub struct TypedExpr {
    pub value: TypedExprNode,
    pub ty: TypeId,
    pub smap: SourceMap
}

impl Debug for TypedExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl Debug for TypedExprNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypedExprNode::IntLiteral(i) => { i.fmt(f) }
            TypedExprNode::Identifier(ident) => { write!(f, "{}", ident) }
            TypedExprNode::Tuple(_) => { todo!() }
            TypedExprNode::BinaryOp(op) => {
                write!(f, "({:?} {:?} {:?})", op.lhs, op.op, op.rhs)
            }
            TypedExprNode::UnaryOp(op) => {
                write!(f, "{:?}{:?}", op.op, op.operand)}
            TypedExprNode::CallOp(call) => {
                write!(f, "{:?}({:?})", call.caller, call.arguments)
            }
        }
    }
}
impl TypedExpr {

    pub fn is_inferred(&self, context: &TypeContext) -> bool {
        self.ty == context.infer
    }
    
    pub fn from_node(expr: &ExprSyntax, compiler: &mut Compiler, context: &AvailableContext) -> Option<(TypedExprNode, TypeId)> {
        match &expr.data {
            Expr::Identifier(ident) => {
                let Some(ty) = context.get_identifier_type(ident) else {
                    compiler.emit_compile_message(
                        CompileMessage::new(
                            expr.smap.clone(),
                            format!("unknown identifier: {:?}", ident),
                            CompileMessageType::Error
                        )
                    );
                    return None;
                };
                
                Some((TypedExprNode::Identifier(ident.clone()), ty))
            }
            Expr::IntLiteral(il) => {
                Some((TypedExprNode::IntLiteral(*il), compiler.type_context.infer))
            }
            Expr::Tuple(_) => { todo!() }
            Expr::BinaryOp(bin) => {
                let mut lhs = TypedExpr::from_ast(&bin.lhs, compiler, context)?;
                let mut rhs = TypedExpr::from_ast(&bin.rhs, compiler, context)?;

                //infer the types
                if !TypeInferencer::infer_binary(&compiler.type_context, &mut lhs, &mut rhs) {
                    compiler.emit_compile_message(
                        CompileMessage::new(
                            expr.smap.clone(),
                            if lhs.ty == compiler.type_context.infer {
                                format!("could not infer type of expression: {:?}", lhs.value)
                            } else {
                                format!("could not infer type of expression: {:?}", rhs.value)
                            },
                            CompileMessageType::Error
                        )
                    );
                    return None;
                }
                //lhs must have an operator overload that accepts rhs

                let lhs_inf = compiler.type_context.get_by_id(lhs.ty).unwrap();
                let rhs_inf = compiler.type_context.get_by_id(rhs.ty).unwrap();

                match bin.op.tk {
                    "+" => {
                        //ensure we can add rhs to lhs
                        let Some(add_res) = lhs_inf.ops.add.get(&rhs.ty) else {
                            compiler.emit_compile_message(
                                CompileMessage::new(
                                    expr.smap.clone(),
                                    format!("Cannot add {} to {}",
                                        compiler.type_context.name_of(lhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
                                        compiler.type_context.name_of(lhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
                                    ),
                                    CompileMessageType::Error
                                )
                            );

                            return None;
                        };

                        Some((TypedExprNode::BinaryOp(
                            Box::new(
                                TypedBinaryOperation{
                                    op: BinaryOperator::Add,
                                    lhs: lhs.value,
                                    rhs: rhs.value,
                                }
                            )
                        ), *add_res))
                    },
                    "-" => {
                        //ensure we can add rhs to lhs
                        let Some(sub_res) = lhs_inf.ops.sub.get(&rhs.ty) else {
                            compiler.emit_compile_message(
                                CompileMessage::new(
                                    expr.smap.clone(),
                                    format!("Cannot subtract {} from {}",
                                            compiler.type_context.name_of(lhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
                                            compiler.type_context.name_of(lhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
                                    ),
                                    CompileMessageType::Error
                                )
                            );

                            return None;
                        };

                        Some((TypedExprNode::BinaryOp(
                            Box::new(
                                TypedBinaryOperation{
                                    op: BinaryOperator::Sub,
                                    lhs: lhs.value,
                                    rhs: rhs.value,
                                }
                            )
                        ), *sub_res))
                    },
                    "*" => {
                        //ensure we can add rhs to lhs
                        let Some(mul_res) = lhs_inf.ops.mul.get(&rhs.ty) else {
                            compiler.emit_compile_message(
                                CompileMessage::new(
                                    expr.smap.clone(),
                                    format!("Cannot multiply {} by {}",
                                            compiler.type_context.name_of(lhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
                                            compiler.type_context.name_of(lhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
                                    ),
                                    CompileMessageType::Error
                                )
                            );

                            return None;
                        };

                        Some((TypedExprNode::BinaryOp(
                            Box::new(
                                TypedBinaryOperation{
                                    op: BinaryOperator::Add,
                                    lhs: lhs.value,
                                    rhs: rhs.value,
                                }
                            )
                        ), *mul_res))
                    },
                    "/" => {
                        //ensure we can add rhs to lhs
                        let Some(div_res) = lhs_inf.ops.div.get(&rhs.ty) else {
                            compiler.emit_compile_message(
                                CompileMessage::new(
                                    expr.smap.clone(),
                                    format!("Cannot divide {} by {}",
                                            compiler.type_context.name_of(lhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
                                            compiler.type_context.name_of(lhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
                                    ),
                                    CompileMessageType::Error
                                )
                            );

                            return None;
                        };

                        Some((TypedExprNode::BinaryOp(
                            Box::new(
                                TypedBinaryOperation{
                                    op: BinaryOperator::Div,
                                    lhs: lhs.value,
                                    rhs: rhs.value,
                                }
                            )
                        ), *div_res))
                    }

                    _ => {
                        //we really shouldn't get this far
                        panic!("Unknown op: {}", bin.op.tk);
                    }
                }
            }
            Expr::UnaryOp(op) => {
                let (lhs, lhs_ty) = TypedExpr::from_node(&op.operand, compiler, context)?;
                //lhs must have a unary operator overload for the specified operator

                let lhs_inf = compiler.type_context.get_by_id(lhs_ty).unwrap();

                match op.op.tk {
                    "-" => {
                        let Some(neg) = lhs_inf.ops.neg else {
                            compiler.emit_compile_message(
                                CompileMessage::new(
                                    expr.smap.clone(),
                                    format!("Cannot negate {}",
                                            compiler.type_context.name_of(lhs_ty).unwrap(),
                                    ),
                                    CompileMessageType::Error,
                                )
                            );

                            return None;
                        };

                        Some(
                            (TypedExprNode::UnaryOp(
                                Box::new(
                                    TypedUnaryOperation{
                                        op: UnaryOperator::Neg,
                                        operand: lhs,
                                    }
                                ),
                            ),
                                neg
                            )
                        )
                    }
                    _ => {
                        panic!("Unknown unary op: {}", op.op.tk)
                    }
                }
            }
            Expr::CallOp(call) => {
                let (caller, caller_ty) = TypedExpr::from_node(&call.caller, compiler, context)?;



                let mut params = Vec::new();
                let mut param_types = Vec::new();
                let mut param_smaps = Vec::new();

                for p in &call.arguments {
                    let (expr, ty) = TypedExpr::from_node(p, compiler, context)?;
                    params.push(expr);
                    param_types.push(ty);
                    param_smaps.push(p.smap.clone())
                }

                let caller_inf = compiler.type_context.get_by_id(caller_ty).unwrap();

                //caller must have a call overload accepting params
                let Some(call) = TypeInferencer::infer_call(&compiler.type_context, &mut param_types, &mut params, &caller_inf.ops.call) else {
                    compiler.emit_compile_message(
                        CompileMessage::new(
                            expr.smap.clone(),
                            format!("Cannot call type {} with parameters: (", compiler.type_context.name_of(caller_ty).unwrap()) + &{
                                let mut s = String::new();
                                let mut first = true;
                                for p in &param_types {
                                    if !first {
                                        s += ", ";
                                    }
                                    first = false;
                                    s += &compiler.type_context.name_of(*p).unwrap();
                                }
                                s += ")";
                                s
                            },
                            CompileMessageType::Error
                        )
                    );

                    return None
                };

                Some((
                    TypedExprNode::CallOp(
                        Box::new(
                            TypedCallOperation{
                                caller,
                                arguments: params
                                    .into_iter()
                                    .zip(param_types)
                                    .zip(param_smaps)
                                    .map(
                                        |((p, ty), smap)| TypedExpr{ value: p, smap, ty }
                                    ).collect()
                            }
                        )
                    ),
                    call
                    ))
            }
        }
    }
    
    pub fn from_ast(expr: &ExprSyntax, compiler: &mut Compiler, context: &AvailableContext) -> Option<Self> {
        let (value, ty) = TypedExpr::from_node(expr, compiler, context)?;

        Some(
            Self{
                value,
                ty,
                smap: expr.smap.clone(),
            }
        )
    }
}