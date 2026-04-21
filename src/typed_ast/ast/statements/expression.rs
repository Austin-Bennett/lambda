use crate::ast::statements::expressions::{Expr, ExprSyntax};
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::literal::LiteralValue;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::{TypeId, TypeKind};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::mem;
use crate::typed_ast::typing::operator::BinaryOperatorMaker;

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

pub struct TypedIndexOperation {
    pub operand: TypedExpr,
    pub index: TypedExpr,
}

pub struct TypedCompilerIntrinsic {
    pub name: String,
    pub args: Vec<TypedExpr>,
}

pub enum TypedExprNode {
    Literal(LiteralValue),
    Identifier(String),
    /// Transparent read through a reference. Inner expr has type `Reference(T)`; this node's `ty` is `T`.
    RefRead(Box<TypedExpr>),

    Tuple(Vec<TypedExpr>),
    Array(Vec<TypedExpr>),
    Index(Box<TypedIndexOperation>),

    BinaryOp(Box<TypedBinaryOperation>),
    UnaryOp(Box<TypedUnaryOperation>),
    CallOp(Box<TypedCallOperation>),
    Cast(Box<TypedCastOperation>),
    CompilerIntrinsic(Box<TypedCompilerIntrinsic>),
}

impl TypedExprNode {
    pub fn is_literal_expr(&self) -> bool {
        match self {
            TypedExprNode::Literal(_) => true,
            TypedExprNode::BinaryOp(bop) => bop.lhs.value.is_literal_expr() && bop.rhs.value.is_literal_expr(),
            TypedExprNode::UnaryOp(uop) => uop.operand.value.is_literal_expr(),
            _ => false,
        }
    }
}

pub struct TypedExpr {
    pub value: TypedExprNode,
    pub ty: TypeId,
    pub smap: SourceMap
}

impl Default for TypedExpr {
    fn default() -> Self {
        use crate::lexer::literal::IntegerLiteral;
        Self{
            value: TypedExprNode::Literal(LiteralValue::Integer(IntegerLiteral{ value: 0, negative: false })),
            ty: 0,
            smap: SourceMap::default(),
        }
    }
}

impl Debug for TypedExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl Debug for TypedExprNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypedExprNode::Literal(lit) => { lit.fmt(f) }
            TypedExprNode::Identifier(ident) => { write!(f, "{}", ident) }
            TypedExprNode::RefRead(inner) => { write!(f, "refread({:?})", inner) }
            TypedExprNode::Tuple(_) => { todo!() }
            TypedExprNode::Array(ray) => {
                write!(f, "[")?;

                for i in 0..ray.len() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{:?}", ray[i])?;
                }

                write!(f, "]")
            }
            TypedExprNode::Index(index) => {
                write!(f, "{:?}[{:?}]", index.operand, index.index)
            }
            TypedExprNode::BinaryOp(op) => {
                write!(f, "({:?} {:?} {:?})", op.lhs.value, op.op, op.rhs.value)
            }
            TypedExprNode::UnaryOp(op) => {
                write!(f, "{:?}{:?}", op.op, op.operand)}
            TypedExprNode::CallOp(call) => {
                write!(f, "{:?}({:?})", call.caller, call.arguments)
            }
            TypedExprNode::Cast(cast) => {
                write!(f, "({:?} as {})", cast.expr, cast.target)
            }
            TypedExprNode::CompilerIntrinsic(ci) => {
                write!(f, "{}({:?})", ci.name, ci.args)
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

    /// Coerce a literal-typed expression to a target type, if the target accepts that literal.
    pub fn coerce_literal(context: &TypeContext, mut operand: TypedExpr, ty: TypeId) -> TypedExpr {
        if context.get_by_id(ty).map_or(false, |i| i.ops.from_literal.contains_key(&operand.ty)) {
            operand.ty = ty;
        }
        operand
    }

    /// Coerce an array of literals to a concrete array type, if the element types are compatible.
    pub fn coerce_literal_array(mut expr: TypedExpr, context: &mut TypeContext, ty: TypeId) -> TypedExpr {
        let TypedExprNode::Array(_) = &expr.value else { return expr; };

        // Extract all needed TypeIds inside a block so borrows are released before mutation.
        let (inner_target, expr_inner, expr_size) = {
            let Some(info1) = context.get_by_id(ty) else { return expr; };
            let TypeKind::Array { ty: inner_target, .. } = &info1.kind else { return expr; };
            let inner_target = *inner_target;

            let Some(expr_info) = context.get_by_id(expr.ty) else { return expr; };
            let TypeKind::Array { ty: expr_inner, size: expr_size } = &expr_info.kind else { return expr; };
            (inner_target, *expr_inner, *expr_size)
        };

        let can_coerce = context.get_by_id(inner_target)
            .map_or(false, |i| i.ops.from_literal.contains_key(&expr_inner));

        if can_coerce {
            let new_ty = context.array_of(inner_target, expr_size);
            expr.ty = new_ty;
            if let TypedExprNode::Array(ray) = &mut expr.value {
                for elem in ray { elem.ty = inner_target; }
            }
        }
        expr
    }

    /// Infer concrete types when one or both sides are literals. Returns true if types now match.
    pub fn infer_literals_binary(context: &TypeContext, lhs: &mut TypedExpr, rhs: &mut TypedExpr) -> bool {
        if lhs.ty == rhs.ty { return true; }
        if context.is_literal(lhs.ty) && context.get_by_id(rhs.ty).map_or(false, |i| i.ops.from_literal.contains_key(&lhs.ty)) {
            lhs.ty = rhs.ty;
            return true;
        }
        if context.is_literal(rhs.ty) && context.get_by_id(lhs.ty).map_or(false, |i| i.ops.from_literal.contains_key(&rhs.ty)) {
            rhs.ty = lhs.ty;
            return true;
        }
        false
    }

    /// Infer the result type for an index operation, coercing a literal index if needed.
    pub fn infer_literal_index(context: &TypeContext, index: &mut TypedExpr, index_op: &HashMap<TypeId, (TypeId, BinaryOperatorMaker)>) -> Option<TypeId> {
        if !context.is_literal(index.ty) {
            return index_op.get(&index.ty).map(|v| v.0);
        }
        for (index_type, (res, _)) in index_op {
            if context.get_by_id(*index_type).map_or(false, |i| i.ops.from_literal.contains_key(&index.ty)) {
                index.ty = *index_type;
                return Some(*res);
            }
        }
        None
    }

    /// Resolve a call overload, coercing any literal-typed parameters to concrete types.
    pub fn infer_literals_call(context: &TypeContext, call_params: &mut Vec<TypedExpr>, call_op: &HashMap<Vec<TypeId>, TypeId>) -> Option<TypeId> {
        let mut tys: Vec<_> = call_params.iter().map(|p| p.ty).collect();
        let mut resolved = false;
        let mut ret = None;

        'outer: for (params, res) in call_op {
            if call_params.len() != params.len() { continue; }
            for (i, (p, actual)) in call_params.iter().zip(params).enumerate() {
                if p.ty == *actual { continue; }
                else if context.is_literal(p.ty) && context.get_by_id(*actual)
                    .map_or(false, |i| i.ops.from_literal.contains_key(&p.ty))
                {
                    tys[i] = *actual;
                } else {
                    continue 'outer;
                }
            }
            ret = Some(*res);
            resolved = true;
            break;
        }
        if resolved {
            for (i, t) in tys.iter().enumerate() { call_params[i].ty = *t; }
        }
        ret
    }
    
    pub fn from_node(expr: &ExprSyntax, compiler: &mut Compiler, context: &AvailableContext) -> Option<(TypedExprNode, TypeId)> {
        match &expr.data {
            Expr::Identifier(ident) => {
                if let Some(ty) = context.get_identifier_type(ident) {
                    return Some((TypedExprNode::Identifier(ident.clone()), ty));
                }
                if let Some(&ty) = compiler.type_context.intrinsics.get(ident) {
                    return Some((TypedExprNode::Identifier(ident.clone()), ty));
                }
                compiler.emit_compile_message(
                    CompileMessage::new(
                        expr.smap.clone(),
                        format!("unknown identifier: {:?}", ident),
                        CompileMessageType::Error
                    )
                );
                None
            }
            Expr::Literal(lit) => {
                let ty = match lit {
                    LiteralValue::Integer(_) => compiler.type_context.int_literal,
                    LiteralValue::Float(_)   => compiler.type_context.float_literal,
                    LiteralValue::Bool(_)    => compiler.type_context.bool,
                };
                Some((TypedExprNode::Literal(lit.clone()), ty))
            }
            Expr::Tuple(_) => { todo!() }
            Expr::Array(array) => {
                let mut first_known_type = None;
                let mut typed_array = Vec::new();

                for e in array {
                    let te = TypedExpr::from_ast(e, compiler, context)?;
                    if te.ty != compiler.type_context.int_literal {
                        first_known_type = Some(te.ty);
                    }
                    typed_array.push(te)
                }

                //case 1, there is a type we can coerce
                if let Some(ty) = first_known_type {
                    for i in 0..typed_array.len() {


                        typed_array[i] = Self::coerce_literal(&compiler.type_context, mem::take(&mut typed_array[i]), ty);

                        //if it STILL doesnt equal types, throw an error
                        if ty != typed_array[i].ty {
                            compiler.emit_compile_message(
                                CompileMessage::new(
                                    typed_array[i].smap.clone(),
                                    format!("Expected array member to be of type {}", compiler.type_context.name_of(ty).unwrap()),
                                    CompileMessageType::Error
                                )
                            );
                            return None;
                        }
                    }

                    //register the type
                    let array = compiler.type_context.array_of(ty, typed_array.len());

                    Some((TypedExprNode::Array(typed_array), array))
                } else {
                    //the array is either all int literals, or empty, so well just return it with the none type
                    //and allow the caller to decide what to do with it

                    let inner = if !typed_array.is_empty() {
                        compiler.type_context.int_literal
                    } else {
                        compiler.type_context.none
                    };

                    let array = compiler.type_context.array_of(inner, typed_array.len());
                    Some((TypedExprNode::Array(typed_array), array))
                }
            },

            //need indexing operators
            Expr::Index(index) => {
                let operand = TypedExpr::from_ast(&index.operand, compiler, context)?;
                let mut index = TypedExpr::from_ast(&index.index, compiler, context)?;
                let op_type = compiler.type_context.get_by_id(operand.ty).unwrap();

                if let Some(id) = Self::infer_literal_index(&compiler.type_context, &mut index, &op_type.ops.index) {

                    Some((
                        TypedExprNode::Index(Box::new(
                            TypedIndexOperation{
                                operand,
                                index
                            }
                        )), id
                    ))
                } else {
                    compiler.emit_compile_message(CompileMessage::new(
                        expr.smap.clone(),
                        format!("Cannot index type {} with type {}",
                                compiler.type_context.name_of(operand.ty).unwrap(),
                                compiler.type_context.name_of(index.ty).unwrap()),
                        CompileMessageType::Error
                    ));

                    None
                }
            },
            Expr::BinaryOp(bin) => {
                let mut lhs = TypedExpr::from_ast(&bin.lhs, compiler, context)?;
                let mut rhs = TypedExpr::from_ast(&bin.rhs, compiler, context)?;

                if bin.op.tk != "=" && !Self::infer_literals_binary(&compiler.type_context, &mut lhs, &mut rhs) {
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

                        // Coerce any literal RHS to the target type if the target accepts it.
                        rhs = Self::coerce_literal(&compiler.type_context, rhs, target_ty);

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
                let lhs_expr = TypedExpr { value: lhs, ty: lhs_ty, smap: op.operand.smap.clone() };

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
                        match &lhs_inf.kind {
                            TypeKind::Pointer(i) => {
                                let result = compiler.type_context.reference_to(*i);
                                Some((TypedExprNode::UnaryOp(Box::new(TypedUnaryOperation {
                                    op: UnaryOperator::Dereference,
                                    operand: TypedExpr { value: lhs, ty: result, smap: expr.smap.clone() }
                                })), result))
                            }
                            TypeKind::Reference(inner) => {
                                let inner = *inner;
                                Some((
                                    TypedExprNode::RefRead(Box::new(
                                        TypedExpr { value: lhs, ty: lhs_ty, smap: expr.smap.clone() }
                                    )),
                                    inner
                                ))
                            }
                            _ => {
                                compiler.emit_compile_message(CompileMessage::new(
                                    expr.smap.clone(),
                                    format!("Cannot dereference type {}", compiler.type_context.name_of(lhs_ty).unwrap()),
                                    CompileMessageType::Error
                                ));
                                None
                            }
                        }
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

                // Intercept compiler intrinsics before any other call handling.
                if let Some(TypeKind::Intrinsic { ret }) = compiler.type_context.get_by_id(caller_ty).map(|i| i.kind.clone()) {
                    let TypedExprNode::Identifier(name) = &caller else {
                        compiler.emit_compile_message(CompileMessage::new(
                            expr.smap.clone(),
                            "compiler intrinsic must be called by name".into(),
                            CompileMessageType::Error,
                        ));
                        return None;
                    };
                    let name = name.clone();
                    let mut args = Vec::new();
                    for p in &call.arguments {
                        args.push(TypedExpr::from_ast(p, compiler, context)?);
                    }
                    return Some((
                        TypedExprNode::CompilerIntrinsic(Box::new(TypedCompilerIntrinsic { name, args })),
                        ret,
                    ));
                }

                let mut params = Vec::new();

                for p in &call.arguments {
                    let (expr, ty) = TypedExpr::from_node(p, compiler, context)?;
                    params.push(TypedExpr{
                        value: expr,
                        ty,
                        smap: p.smap.clone(),
                    });
                }

                // literal(single_expr) → implicit multiplication; fall back to the literal's default type
                if compiler.type_context.is_literal(caller_ty) && params.len() == 1 {
                    let mut lhs = TypedExpr { value: caller, ty: caller_ty, smap: expr_call.caller.smap.clone() };
                    let mut rhs = params.remove(0);
                    Self::infer_literals_binary(&compiler.type_context, &mut lhs, &mut rhs);
                    for side in [&mut lhs, &mut rhs] {
                        if compiler.type_context.is_literal(side.ty) {
                            if let Some(&default) = compiler.type_context.literal_defaults.get(&side.ty) {
                                side.ty = default;
                            }
                        }
                    }
                    let result_ty = lhs.ty;
                    return Some((TypedExprNode::BinaryOp(Box::new(TypedBinaryOperation {
                        op: BinaryOperator::Mul,
                        lhs,
                        rhs,
                    })), result_ty));
                }

                let caller_inf = compiler.type_context.get_by_id(caller_ty).unwrap();

                //caller must have a call overload accepting params
                let Some(call) = Self::infer_literals_call(&compiler.type_context, &mut params, &caller_inf.ops.call) else {
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