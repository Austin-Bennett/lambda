use crate::ast::statements::expressions::{Expr, ExprSyntax};
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::literal::{IntegerLiteral, LiteralValue};
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::{StructId, TypeId, TypeKind};
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
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

impl Debug for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => f.write_str("+"),
            BinaryOperator::Sub => f.write_str("-"),
            BinaryOperator::Mul => f.write_str("*"),
            BinaryOperator::Div => f.write_str("/"),
            BinaryOperator::Assign => f.write_str("="),
            BinaryOperator::Eq => f.write_str("=="),
            BinaryOperator::Ne => f.write_str("!="),
            BinaryOperator::Lt => f.write_str("<"),
            BinaryOperator::Gt => f.write_str(">"),
            BinaryOperator::Le => f.write_str("<="),
            BinaryOperator::Ge => f.write_str(">="),
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

pub struct TypedMemberAccess {
    pub object: TypedExpr,
    pub member_index: usize,
}

pub struct TypedStructConstruct {
    pub struct_id: StructId,
    pub fields: Vec<TypedExpr>,
}

/// A method with its receiver already attached: `p.length2` stores `&p` and the mangled name.
/// When called, the receiver is prepended as the first argument automatically.
pub struct TypedBoundMethod {
    pub self_expr: TypedExpr,
    pub mangled_name: String,
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
    MemberAccess(Box<TypedMemberAccess>),
    StructConstruct(Box<TypedStructConstruct>),
    BoundMethod(Box<TypedBoundMethod>),
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

    pub fn as_identifier(&self) -> Option<&str> {
        match self {
            TypedExprNode::Identifier(name) => Some(name),
            _ => None,
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
            TypedExprNode::MemberAccess(ma) => {
                write!(f, "{:?}.{}", ma.object, ma.member_index)
            }
            TypedExprNode::StructConstruct(sc) => {
                write!(f, "struct_construct({:?})", sc.fields)
            }
            TypedExprNode::BoundMethod(bm) => {
                write!(f, "{:?}.{}", bm.self_expr, bm.mangled_name)
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
    
    /// Find a user-defined binary op entry, coercing a literal rhs type if needed.
    /// Returns `(coerced_rhs_ty, result_ty, mangled_name)`.
    fn find_user_binop(
        user_ops: &HashMap<TypeId, (TypeId, String)>,
        rhs_ty: TypeId,
        context: &TypeContext,
    ) -> Option<(TypeId, TypeId, String)> {
        if let Some((ret, mangled)) = user_ops.get(&rhs_ty) {
            return Some((rhs_ty, *ret, mangled.clone()));
        }
        if context.is_literal(rhs_ty) {
            for (reg_ty, (ret, mangled)) in user_ops {
                if context.get_by_id(*reg_ty)
                    .map_or(false, |i| i.ops.from_literal.contains_key(&rhs_ty))
                {
                    return Some((*reg_ty, *ret, mangled.clone()));
                }
            }
        }
        None
    }

    /// Find a user-defined cmp op entry, coercing a literal rhs type if needed.
    /// Returns `(coerced_rhs_ty, mangled_name)`.
    fn find_user_cmp(
        user_cmp: &HashMap<TypeId, String>,
        rhs_ty: TypeId,
        context: &TypeContext,
    ) -> Option<(TypeId, String)> {
        if let Some(mangled) = user_cmp.get(&rhs_ty) {
            return Some((rhs_ty, mangled.clone()));
        }
        if context.is_literal(rhs_ty) {
            for (reg_ty, mangled) in user_cmp {
                if context.get_by_id(*reg_ty)
                    .map_or(false, |i| i.ops.from_literal.contains_key(&rhs_ty))
                {
                    return Some((*reg_ty, mangled.clone()));
                }
            }
        }
        None
    }

    /// Build a call to a user-defined binary operator: `mangled(&lhs, rhs)`.
    fn make_user_binop_call(
        mangled: String,
        result_ty: TypeId,
        lhs: TypedExpr,
        rhs: TypedExpr,
        context: &AvailableContext<TypeId>,
        compiler: &mut Compiler,
        smap: SourceMap,
    ) -> Option<(TypedExprNode, TypeId)> {
        let fn_type_id = *context.get_identifier(&mangled)?;
        let self_ref_ty = compiler.type_context.reference_to(lhs.ty);
        let self_ref = TypedExpr {
            smap: lhs.smap.clone(),
            ty: self_ref_ty,
            value: TypedExprNode::UnaryOp(Box::new(TypedUnaryOperation {
                op: UnaryOperator::Reference,
                operand: lhs,
            })),
        };
        let fn_expr = TypedExpr {
            smap: smap.clone(),
            ty: fn_type_id,
            value: TypedExprNode::Identifier(mangled),
        };
        Some((TypedExprNode::CallOp(Box::new(TypedCallOperation {
            caller: fn_expr,
            arguments: vec![self_ref, rhs],
        })), result_ty))
    }

    /// Build a comparison using a user-defined `cmp` operator.
    /// Generates `T__op_cmp(&lhs, rhs) OP 0` where OP is the original comparison.
    fn make_user_cmp_call(
        op_tk: &str,
        mangled: String,
        lhs: TypedExpr,
        rhs: TypedExpr,
        context: &AvailableContext<TypeId>,
        compiler: &mut Compiler,
        smap: SourceMap,
    ) -> Option<(TypedExprNode, TypeId)> {
        let fn_type_id = *context.get_identifier(&mangled)?;
        let int8_id = compiler.type_context.int8;
        let bool_id = compiler.type_context.bool;
        let self_ref_ty = compiler.type_context.reference_to(lhs.ty);
        let self_ref = TypedExpr {
            smap: lhs.smap.clone(),
            ty: self_ref_ty,
            value: TypedExprNode::UnaryOp(Box::new(TypedUnaryOperation {
                op: UnaryOperator::Reference,
                operand: lhs,
            })),
        };
        let fn_expr = TypedExpr {
            smap: smap.clone(),
            ty: fn_type_id,
            value: TypedExprNode::Identifier(mangled),
        };
        let cmp_call = TypedExpr {
            ty: int8_id,
            smap: smap.clone(),
            value: TypedExprNode::CallOp(Box::new(TypedCallOperation {
                caller: fn_expr,
                arguments: vec![self_ref, rhs],
            })),
        };
        let zero = TypedExpr {
            ty: int8_id,
            smap: smap.clone(),
            value: TypedExprNode::Literal(LiteralValue::Integer(IntegerLiteral { value: 0, negative: false })),
        };
        let op_variant = match op_tk {
            "==" => BinaryOperator::Eq,
            "!=" => BinaryOperator::Ne,
            "<"  => BinaryOperator::Lt,
            ">"  => BinaryOperator::Gt,
            "<=" => BinaryOperator::Le,
            ">=" => BinaryOperator::Ge,
            _    => unreachable!(),
        };
        Some((TypedExprNode::BinaryOp(Box::new(TypedBinaryOperation {
            op: op_variant,
            lhs: cmp_call,
            rhs: zero,
        })), bool_id))
    }

    pub fn from_node(expr: &ExprSyntax, compiler: &mut Compiler, context: &AvailableContext<TypeId>) -> Option<(TypedExprNode, TypeId)> {
        match &expr.data {
            Expr::Identifier(ident) => {
                if let Some(ty) = context.get_identifier(ident) {
                    return Some((TypedExprNode::Identifier(ident.clone()), *ty));
                }
                if let Some(&ty) = compiler.type_context.intrinsics.get(ident) {
                    return Some((TypedExprNode::Identifier(ident.clone()), ty));
                }
                // allow struct type names to be used as constructors
                if let Some(ty_id) = compiler.resolve_typename(ident) {
                    return Some((TypedExprNode::Identifier(ident.clone()), ty_id));
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
                let lhs_raw = TypedExpr::from_ast(&bin.lhs, compiler, context)?;
                let rhs_raw = TypedExpr::from_ast(&bin.rhs, compiler, context)?;
                // Auto-deref references for non-assignment binary ops so that `self: T&` works naturally
                let mut lhs = if bin.op.tk != "=" { Self::coerce_ref(lhs_raw, &compiler.type_context) } else { lhs_raw };
                let mut rhs = Self::coerce_ref(rhs_raw, &compiler.type_context);

                if bin.op.tk != "=" {
                    Self::infer_literals_binary(&compiler.type_context, &mut lhs, &mut rhs);

                    // Check user-defined binary/cmp operators (take priority over built-ins)
                    match bin.op.tk {
                        "+" | "-" | "*" | "/" => {
                            let found = {
                                let info = compiler.type_context.get_by_id(lhs.ty)?;
                                let user_ops = match bin.op.tk {
                                    "+" => &info.ops.user_add,
                                    "-" => &info.ops.user_sub,
                                    "*" => &info.ops.user_mul,
                                    "/" => &info.ops.user_div,
                                    _   => unreachable!(),
                                };
                                Self::find_user_binop(user_ops, rhs.ty, &compiler.type_context)
                            };
                            if let Some((coerced_ty, result_ty, mangled)) = found {
                                rhs.ty = coerced_ty;
                                return Some(Self::make_user_binop_call(
                                    mangled, result_ty, lhs, rhs, context, compiler, expr.smap.clone(),
                                )?);
                            }
                        }
                        "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                            let found = {
                                let info = compiler.type_context.get_by_id(lhs.ty)?;
                                Self::find_user_cmp(&info.ops.user_cmp, rhs.ty, &compiler.type_context)
                            };
                            if let Some((coerced_ty, mangled)) = found {
                                rhs.ty = coerced_ty;
                                return Some(Self::make_user_cmp_call(
                                    bin.op.tk, mangled, lhs, rhs, context, compiler, expr.smap.clone(),
                                )?);
                            }
                        }
                        _ => {}
                    }
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

                    "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                        let op_variant = match bin.op.tk {
                            "==" => BinaryOperator::Eq,
                            "!=" => BinaryOperator::Ne,
                            "<"  => BinaryOperator::Lt,
                            ">"  => BinaryOperator::Gt,
                            "<=" => BinaryOperator::Le,
                            ">=" => BinaryOperator::Ge,
                            _    => unreachable!(),
                        };

                        let bool_id = compiler.type_context.bool;
                        let lhs_inf = compiler.type_context.get_by_id(lhs.ty).unwrap();

                        if lhs_inf.ops.cmp.contains_key(&rhs.ty) {
                            Some((TypedExprNode::BinaryOp(Box::new(TypedBinaryOperation {
                                op: op_variant, lhs, rhs,
                            })), bool_id))
                        } else {
                            compiler.emit_compile_message(CompileMessage::new(
                                expr.smap.clone(),
                                format!("Cannot compare {} with {}",
                                    compiler.type_context.name_of(lhs.ty).unwrap_or_default(),
                                    compiler.type_context.name_of(rhs.ty).unwrap_or_default()),
                                CompileMessageType::Error,
                            ));
                            None
                        }
                    }

                    "=" => {
                        // Determine the target type and whether the LHS is a pointer dereference.
                        let (target_ty, lhs_is_deref) = match &lhs.value {
                            TypedExprNode::Identifier(_) | TypedExprNode::RefRead(_) | TypedExprNode::MemberAccess(_) => (lhs.ty, false),
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

                        // Check user-defined assign operator (takes priority over built-in copy)
                        if !lhs_is_deref {
                            let found = {
                                let info = compiler.type_context.get_by_id(target_ty);
                                info.and_then(|i| {
                                    Self::find_user_binop(&i.ops.user_assign, rhs.ty, &compiler.type_context)
                                })
                            };
                            if let Some((coerced_ty, result_ty, mangled)) = found {
                                rhs.ty = coerced_ty;
                                return Some(Self::make_user_binop_call(
                                    mangled, result_ty, lhs, rhs, context, compiler, expr.smap.clone(),
                                )?);
                            }
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
                let lhs_expr = TypedExpr { value: lhs, ty: lhs_ty, smap: op.operand.smap.clone() };
                // Auto-deref references for all unary ops except address-of


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

                // Method call: MemberAccess already resolved this to a BoundMethod.
                if let TypedExprNode::BoundMethod(bm) = caller {
                    let TypeKind::Function { params, ret } = compiler.type_context
                        .get_by_id(caller_ty)
                        .unwrap()
                        .kind
                        .clone()
                    else {
                        return None;
                    };

                    let mut args = vec![bm.self_expr];
                    for (i, arg_expr) in call.arguments.iter().enumerate() {
                        let mut typed = TypedExpr::from_ast(arg_expr, compiler, context)?;
                        if let Some(&expected) = params.get(i + 1) {
                            typed = Self::coerce_literal(&compiler.type_context, typed, expected);
                            if typed.ty != expected {
                                compiler.emit_compile_message(CompileMessage::new(
                                    arg_expr.smap.clone(),
                                    format!(
                                        "method argument type mismatch: expected {}, got {}",
                                        compiler.type_context.name_of(expected).unwrap_or_default(),
                                        compiler.type_context.name_of(typed.ty).unwrap_or_default()
                                    ),
                                    CompileMessageType::Error,
                                ));
                                return None;
                            }
                        }
                        args.push(typed);
                    }

                    let caller_expr = TypedExpr {
                        value: TypedExprNode::Identifier(bm.mangled_name),
                        ty: caller_ty,
                        smap: call.caller.smap.clone(),
                    };
                    return Some((
                        TypedExprNode::CallOp(Box::new(TypedCallOperation {
                            caller: caller_expr,
                            arguments: args,
                        })),
                        ret,
                    ));
                }
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

                // struct construction: MyStruct(field0, field1, ...)
                if let Some(TypeKind::Struct(sid)) = compiler.type_context.get_by_id(caller_ty).map(|i| i.kind.clone()) {
                    let members: Vec<_> = compiler.type_context.structs[sid as usize].members.clone();
                    if call.arguments.len() != members.len() {
                        compiler.emit_compile_message(CompileMessage::new(
                            expr.smap.clone(),
                            format!("struct '{}' has {} fields but {} arguments were provided",
                                compiler.type_context.structs[sid as usize].name,
                                members.len(),
                                call.arguments.len()),
                            CompileMessageType::Error,
                        ));
                        return None;
                    }
                    let mut fields = Vec::new();
                    for (arg, member) in call.arguments.iter().zip(members.iter()) {
                        let mut typed = TypedExpr::from_ast(arg, compiler, context)?;
                        typed = Self::coerce_literal(&compiler.type_context, typed, member.ty);
                        if typed.ty != member.ty {
                            compiler.emit_compile_message(CompileMessage::new(
                                arg.smap.clone(),
                                format!("field '{}' expects type {} but got {}",
                                    member.name,
                                    compiler.type_context.name_of(member.ty).unwrap_or_default(),
                                    compiler.type_context.name_of(typed.ty).unwrap_or_default()),
                                CompileMessageType::Error,
                            ));
                            return None;
                        }
                        fields.push(typed);
                    }
                    let struct_ty = caller_ty;
                    return Some((TypedExprNode::StructConstruct(Box::new(TypedStructConstruct { struct_id: sid, fields })), struct_ty));
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
            Expr::MemberAccess(op) => {
                let object = TypedExpr::from_ast(&op.object, compiler, context)?;

                // Compute effective type (unwrap one reference level) for method/member lookup
                let effective_ty = match compiler.type_context.get_by_id(object.ty).map(|i| i.kind.clone()) {
                    Some(TypeKind::Reference(inner)) => inner,
                    _ => object.ty,
                };

                // Check for a method on the effective type first
                if let Some((mangled, fn_type_id, public)) = compiler.type_context
                    .get_by_id(effective_ty)
                    .and_then(|info| info.methods.get(&op.member))
                    .cloned()
                {
                    if !public {
                        compiler.emit_compile_message(CompileMessage::new(
                            expr.smap.clone(),
                            format!("method '{}' is private", op.member),
                            CompileMessageType::Error,
                        ));
                        return None;
                    }

                    // A static method's first param is NOT the self-reference type.
                    // Detect this and return a plain identifier — no receiver binding needed.
                    let self_ref_ty = compiler.type_context.reference_to(effective_ty);
                    let is_static = match compiler.type_context.get_by_id(fn_type_id).map(|i| i.kind.clone()) {
                        Some(TypeKind::Function { params, .. }) => params.first() != Some(&self_ref_ty),
                        _ => false,
                    };

                    if is_static {
                        return Some((TypedExprNode::Identifier(mangled), fn_type_id));
                    }

                    // Instance method: self must be an lvalue so we can take its address.
                    let TypedExprNode::Identifier(_) = &object.value else {
                        compiler.emit_compile_message(CompileMessage::new(
                            op.object.smap.clone(),
                            "method receiver must be a variable".into(),
                            CompileMessageType::Error,
                        ));
                        return None;
                    };
                    let self_expr = if object.ty == effective_ty {
                        // direct struct — take address
                        TypedExpr {
                            smap: object.smap.clone(),
                            ty: self_ref_ty,
                            value: TypedExprNode::UnaryOp(Box::new(TypedUnaryOperation {
                                op: UnaryOperator::Reference,
                                operand: object,
                            })),
                        }
                    } else {
                        // already a reference — pass as-is
                        object
                    };
                    return Some((TypedExprNode::BoundMethod(Box::new(TypedBoundMethod {
                        self_expr,
                        mangled_name: mangled,
                    })), fn_type_id));
                }

                // Resolve the struct id from the effective type
                let sid = match compiler.type_context.get_by_id(effective_ty).map(|i| i.kind.clone()) {
                    Some(TypeKind::Struct(sid)) => sid,
                    _ => {
                        compiler.emit_compile_message(CompileMessage::new(
                            expr.smap.clone(),
                            format!("cannot access member '{}' on non-struct type {}",
                                op.member,
                                compiler.type_context.name_of(object.ty).unwrap_or_default()),
                            CompileMessageType::Error,
                        ));
                        return None;
                    }
                };

                let members = &compiler.type_context.structs[sid as usize].members;
                let Some((member_index, member)) = members.iter().enumerate().find(|(_, m)| m.name == op.member) else {
                    compiler.emit_compile_message(CompileMessage::new(
                        expr.smap.clone(),
                        format!("struct '{}' has no member '{}'",
                            compiler.type_context.structs[sid as usize].name,
                            op.member),
                        CompileMessageType::Error,
                    ));
                    return None;
                };
                let member_ty = member.ty;
                let object = Self::coerce_ref(object, &compiler.type_context);
                Some((TypedExprNode::MemberAccess(Box::new(TypedMemberAccess { object, member_index })), member_ty))
            }
        }
    }
    
    pub fn from_ast(expr: &ExprSyntax, compiler: &mut Compiler, context: &AvailableContext<TypeId>) -> Option<Self> {
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