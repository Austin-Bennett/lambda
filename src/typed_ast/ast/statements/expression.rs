use crate::ast::statements::expressions::{Expr, ExprSyntax, CastOperation};
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::literal::IntegerLiteral;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::{TypeId, TypeKind};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Pointer, Write};
use crate::ast::ty::Type;

pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
}

impl Debug for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => f.write_str("+"),
            BinaryOperator::Sub => f.write_str("-"),
            BinaryOperator::Mul => f.write_str("*"),
            BinaryOperator::Div => f.write_str("/"),
            BinaryOperator::Assign => f.write_str("=")
        }
    }
}



pub enum UnaryOperator {
    Neg,
    Reference,
    Dereference,
}

impl Debug for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { 
            UnaryOperator::Neg => f.write_str("-"),
            UnaryOperator::Reference => f.write_str("&"),
            UnaryOperator::Dereference => f.write_str("*"),
        }
    }
}

pub struct TypedBinaryOperation {
    pub op: BinaryOperator,
    pub lhs: TypedExpr,
    pub rhs: TypedExpr,
}

pub struct TypedUnaryOperation {
    pub op: UnaryOperator,
    pub operand: TypedExpr,
}

pub struct TypedCallOperation {
    pub caller: TypedExpr,
    pub arguments: Vec<TypedExpr>,
}

pub struct TypedCastOperation {
    pub expr: TypedExpr,
    pub target: TypeId,
}

pub enum TypedExprNode {
    IntLiteral(IntegerLiteral),
    Identifier(String),
    /// Transparent read through a reference. Inner expr has type `Reference(T)`; this node's `ty` is `T`.
    RefRead(Box<TypedExpr>),

    Tuple(Vec<TypedExpr>),

    BinaryOp(Box<TypedBinaryOperation>),
    UnaryOp(Box<TypedUnaryOperation>),
    CallOp(Box<TypedCallOperation>),
    Cast(Box<TypedCastOperation>),
}

impl TypedExprNode {
    pub fn is_int_literal_expr(&self) -> bool {
        match self {
            TypedExprNode::IntLiteral(_) => { true }
            TypedExprNode::Identifier(_) => { false }
            TypedExprNode::RefRead(_) => { false }
            TypedExprNode::Tuple(_) => { false }
            TypedExprNode::BinaryOp(bop) => { bop.lhs.value.is_int_literal_expr() && bop.rhs.value.is_int_literal_expr() }
            TypedExprNode::UnaryOp(uop) => { uop.operand.value.is_int_literal_expr() }
            TypedExprNode::CallOp(call) => { false }
            TypedExprNode::Cast(_) => { false }
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
            TypedExprNode::RefRead(inner) => { write!(f, "refread({:?})", inner) }
            TypedExprNode::Tuple(_) => { todo!() }
            TypedExprNode::BinaryOp(op) => {
                write!(f, "({:?} {:?} {:?})", op.lhs, op.op, op.rhs)
            }
            TypedExprNode::UnaryOp(op) => {
                write!(f, "{:?}{:?}", op.op, op.operand)}
            TypedExprNode::CallOp(call) => {
                write!(f, "{:?}({:?})", call.caller, call.arguments)
            }
            TypedExprNode::Cast(cast) => {
                write!(f, "({:?} as {})", cast.expr, cast.target)
            }
        }
    }
}
impl TypedExpr {




    /// If `expr` has a reference type `T&`, wrap it in `RefRead` so the result has type `T`.
    pub fn coerce_ref(expr: TypedExpr, context: &TypeContext) -> TypedExpr {
        if let TypeKind::Reference(inner_id) = context.get_by_id(expr.ty).map(|i| i.kind.clone()).unwrap_or(TypeKind::None) {
            let smap = expr.smap.clone();
            TypedExpr { value: TypedExprNode::RefRead(Box::new(expr)), ty: inner_id, smap }
        } else {
            expr
        }
    }

    pub fn infer_ints_binary(context: &TypeContext, lhs: &mut TypedExpr, rhs: &mut TypedExpr) -> bool {
        if lhs.ty == rhs.ty { return true; }
        else if lhs.ty == context.int_literal || rhs.ty == context.int_literal {
            if lhs.ty == context.int_literal && context.is_int(rhs.ty) {

                lhs.ty = rhs.ty;

                return true;
            }
        }
        if rhs.ty == context.int_literal && context.is_int(lhs.ty) {
            rhs.ty = lhs.ty;
            true
        } else {
            false
        }
    }
    
    

    pub fn infer_ints_call(context: &TypeContext, call_params: &mut Vec<TypedExpr>, call_op: &HashMap<Vec<TypeId>, TypeId>) -> Option<TypeId> {
        let mut tys: Vec<_> = call_params.iter().map(|p| p.ty).collect();
        let mut ret = None;

        let mut resolved = false;

        'outer: for (params, res) in call_op {
            if call_params.len() != params.len() {
                continue;
            }

            for (i, (p, actual)) in call_params.iter().zip(params).enumerate() {
                if p.ty == *actual { continue; }
                else if p.ty == context.int_literal && context.is_int(*actual) {
                    tys[i] = *actual;
                } else {
                    continue 'outer;
                }
            }
            ret = Some(*res);
            resolved = true;
            //getting here means the last check was a success
            break;
        }
        if resolved {
            for (i, t) in tys.iter().enumerate() {
                call_params[i].ty = *t;
            }
        }

        ret
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

                // If the declared type is a reference, transparently read through it.
                if let TypeKind::Reference(inner_id) = compiler.type_context.get_by_id(ty).unwrap().kind {
                    let ref_expr = TypedExpr { value: TypedExprNode::Identifier(ident.clone()), ty, smap: expr.smap.clone() };
                    return Some((TypedExprNode::RefRead(Box::new(ref_expr)), inner_id));
                }

                Some((TypedExprNode::Identifier(ident.clone()), ty))
            }
            Expr::IntLiteral(il) => {
                Some((TypedExprNode::IntLiteral(*il), compiler.type_context.int_literal))
            }
            Expr::Tuple(_) => { todo!() }
            Expr::BinaryOp(bin) => {
                let mut lhs = TypedExpr::from_ast(&bin.lhs, compiler, context)?;
                let mut rhs = TypedExpr::from_ast(&bin.rhs, compiler, context)?;

                // Coerce references on both sides (except `=` LHS which is handled separately).
                if bin.op.tk != "=" {
                    lhs = Self::coerce_ref(lhs, &compiler.type_context);
                }
                rhs = Self::coerce_ref(rhs, &compiler.type_context);

                if bin.op.tk != "=" && !Self::infer_ints_binary(&compiler.type_context, &mut lhs, &mut rhs) {
                    let mut smap = lhs.smap.clone();
                    smap.extend(&rhs.smap);
                    compiler.emit_compile_message(
                        CompileMessage::new(
                            smap,
                            format!("Cannot infer types for binary operation between {} and {}",
                                compiler.type_context.name_of(lhs.ty).unwrap(), compiler.type_context.name_of(rhs.ty).unwrap()
                            ),
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
                        let Some((add_res, _)) = lhs_inf.ops.add.get(&rhs.ty) else {
                            compiler.emit_compile_message(
                                CompileMessage::new(
                                    expr.smap.clone(),
                                    format!("Cannot add {} to {}",
                                        compiler.type_context.name_of(lhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
                                        compiler.type_context.name_of(rhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
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
                                    lhs,
                                    rhs,
                                }
                            )
                        ), *add_res))
                    },
                    "-" => {
                        let Some((sub_res, _)) = lhs_inf.ops.sub.get(&rhs.ty) else {
                            compiler.emit_compile_message(
                                CompileMessage::new(
                                    expr.smap.clone(),
                                    format!("Cannot subtract {} from {}",
                                            compiler.type_context.name_of(rhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
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
                                    lhs,
                                    rhs,
                                }
                            )
                        ), *sub_res))
                    },
                    "*" => {
                        let Some((mul_res, _)) = lhs_inf.ops.mul.get(&rhs.ty) else {
                            compiler.emit_compile_message(
                                CompileMessage::new(
                                    expr.smap.clone(),
                                    format!("Cannot multiply {} by {}",
                                            compiler.type_context.name_of(lhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
                                            compiler.type_context.name_of(rhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
                                    ),
                                    CompileMessageType::Error
                                )
                            );

                            return None;
                        };

                        Some((TypedExprNode::BinaryOp(
                            Box::new(
                                TypedBinaryOperation{
                                    op: BinaryOperator::Mul,
                                    lhs,
                                    rhs,
                                }
                            )
                        ), *mul_res))
                    },
                    "/" => {
                        let Some((div_res, _)) = lhs_inf.ops.div.get(&rhs.ty) else {
                            compiler.emit_compile_message(
                                CompileMessage::new(
                                    expr.smap.clone(),
                                    format!("Cannot divide {} by {}",
                                            compiler.type_context.name_of(lhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
                                            compiler.type_context.name_of(rhs.ty).unwrap_or("UNKNOWN TYPE".to_string()),
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
                                    lhs,
                                    rhs,
                                }
                            )
                        ), *div_res))
                    }

                    "=" => {
                        // Determine the target type and whether the LHS is a pointer dereference.
                        let (target_ty, lhs_is_deref) = match &lhs.value {
                            TypedExprNode::Identifier(_) | TypedExprNode::RefRead(_) => (lhs.ty, false),
                            TypedExprNode::UnaryOp(uop) if matches!(uop.op, UnaryOperator::Dereference) => {
                                match compiler.type_context.get_by_id(lhs.ty).map(|i| i.kind.clone()) {
                                    Some(TypeKind::Reference(inner)) => (inner, true),
                                    _ => {
                                        compiler.emit_compile_message(CompileMessage::new(
                                            expr.smap.clone(),
                                            "invalid dereference lvalue in assignment".into(),
                                            CompileMessageType::Error,
                                        ));
                                        return None;
                                    }
                                }
                            }
                            _ => {
                                compiler.emit_compile_message(CompileMessage::new(
                                    expr.smap.clone(),
                                    "left-hand side of `=` must be a variable or dereference expression".into(),
                                    CompileMessageType::Error,
                                ));
                                return None;
                            }
                        };

                        // Coerce int_literal RHS to target type.
                        if rhs.ty == compiler.type_context.int_literal && compiler.type_context.is_int(target_ty) {
                            rhs.ty = target_ty;
                        }

                        if lhs_is_deref {
                            if rhs.ty != target_ty {
                                compiler.emit_compile_message(CompileMessage::new(
                                    expr.smap.clone(),
                                    format!("cannot assign {} through pointer to {}",
                                        compiler.type_context.name_of(rhs.ty).unwrap_or_default(),
                                        compiler.type_context.name_of(target_ty).unwrap_or_default()),
                                    CompileMessageType::Error,
                                ));
                                return None;
                            }
                        } else {
                            let lhs_info = compiler.type_context.get_by_id(target_ty).unwrap();
                            if !lhs_info.ops.assign.contains_key(&rhs.ty) {
                                compiler.emit_compile_message(CompileMessage::new(
                                    expr.smap.clone(),
                                    format!("cannot assign {} to variable of type {}",
                                        compiler.type_context.name_of(rhs.ty).unwrap_or_default(),
                                        compiler.type_context.name_of(target_ty).unwrap_or_default()),
                                    CompileMessageType::Error,
                                ));
                                return None;
                            }
                        }

                        Some((TypedExprNode::BinaryOp(Box::new(TypedBinaryOperation {
                            op: BinaryOperator::Assign,
                            lhs,
                            rhs,
                        })), target_ty))
                    }

                    _ => {
                        //we really shouldn't get this far
                        panic!("Unknown op: {}", bin.op.tk);
                    }
                }
            }
            Expr::UnaryOp(op) => {
                let (lhs, lhs_ty) = TypedExpr::from_node(&op.operand, compiler, context)?;
                let mut lhs_expr = TypedExpr { value: lhs, ty: lhs_ty, smap: op.operand.smap.clone() };

                // Coerce references for arithmetic ops; `&` and `*` need the raw type.
                if op.op.tk == "-" {
                    lhs_expr = Self::coerce_ref(lhs_expr, &compiler.type_context);
                }
                let lhs_ty = lhs_expr.ty;
                let lhs = lhs_expr.value;

                let lhs_inf = compiler.type_context.get_by_id(lhs_ty).unwrap();

                match op.op.tk {
                    "-" => {
                        let Some((neg_ty, _)) = &lhs_inf.ops.neg else {
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
                                        operand: TypedExpr{ value: lhs, ty: lhs_ty, smap: expr.smap.clone() },
                                    }
                                ),
                            ),
                                *neg_ty
                            )
                        )
                    },
                    "&" => {
                        //lhs MUST be a identifier
                        let TypedExprNode::Identifier(_) = &lhs else {
                            compiler.emit_compile_message(
                                CompileMessage::new(
                                    expr.smap.clone(),
                                    "Cannot take address of rvalue!".to_string(),
                                    CompileMessageType::Error,
                                )
                            );

                            return None;
                        };


                        let ref_type = compiler.type_context.reference_to(lhs_ty);

                        Some((TypedExprNode::UnaryOp(
                            Box::new(
                                TypedUnaryOperation{
                                    op: UnaryOperator::Reference,
                                    operand: TypedExpr{ value: lhs, ty: ref_type, smap: expr.smap.clone() }
                                }
                            ),

                        ), ref_type))
                    },
                    "*" => {
                        //can be any POINTER type
                        let result = match &lhs_inf.kind {
                            TypeKind::Pointer(i) => compiler.type_context.reference_to(*i),
                            ty => {
                                compiler.emit_compile_message(
                                    CompileMessage::new(
                                        expr.smap.clone(),
                                        format!("Cannot dereference type {}", compiler.type_context.name_of(lhs_ty).unwrap()),
                                        CompileMessageType::Error
                                    )
                                );
                                return None;
                            }
                        };

                        Some(
                            (TypedExprNode::UnaryOp(
                                Box::new(
                                    TypedUnaryOperation{
                                        op: UnaryOperator::Dereference,
                                        operand: TypedExpr{ value: lhs, ty: result, smap: expr.smap.clone() }
                                    }
                                )
                            ), result)
                        )
                    }
                    _ => {
                        panic!("Unknown unary op: {}", op.op.tk)
                    }
                }
            }
            Expr::CastOp(cast) => {
                let inner = TypedExpr::from_ast(&cast.expr, compiler, context)?;
                let Some(target) = compiler.resolve_type(&cast.ty) else {
                    compiler.emit_compile_message(CompileMessage::new(
                        expr.smap.clone(),
                        format!("unknown cast target type {:?}", cast.ty),
                        CompileMessageType::Error,
                    ));
                    return None;
                };
                Some((TypedExprNode::Cast(Box::new(TypedCastOperation { expr: inner, target })), target))
            }
            Expr::CallOp(call) => {
                let (caller, caller_ty) = TypedExpr::from_node(&call.caller, compiler, context)?;
                let expr_call = call;




                let mut params = Vec::new();

                for p in &call.arguments {
                    let (expr, ty) = TypedExpr::from_node(p, compiler, context)?;
                    params.push(TypedExpr{
                        value: expr,
                        ty,
                        smap: p.smap.clone(),
                    });
                }

                let caller_inf = compiler.type_context.get_by_id(caller_ty).unwrap();

                //caller must have a call overload accepting params
                let Some(call) = Self::infer_ints_call(&compiler.type_context, &mut params, &caller_inf.ops.call) else {
                    compiler.emit_compile_message(
                        CompileMessage::new(
                            expr.smap.clone(),
                            format!("Cannot call type {} with parameters: (", compiler.type_context.name_of(caller_ty).unwrap()) + &{
                                let mut s = String::new();
                                let mut first = true;
                                for p in &params {
                                    if !first {
                                        s += ", ";
                                    }
                                    first = false;
                                    s += &compiler.type_context.name_of(p.ty).unwrap();
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
                                caller: TypedExpr{ value: caller, ty: caller_ty, smap: expr_call.caller.smap.clone() },
                                arguments: params
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