use std::collections::HashMap;
use crate::typed_ast::ast::statements::expression::{TypedExpr, TypedExprNode};
use crate::typed_ast::ast::type_inference::{BinaryInferencer};
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::TypeId;

pub struct BIntLiteralInferencer;

impl BinaryInferencer for BIntLiteralInferencer {
    
    //one can assume that only 1 of either is inferred
    fn infer_type(&self, context: &TypeContext, unknown: &TypedExprNode, known: TypeId) -> Option<TypeId> {
        if context.is_int(known) && unknown.is_int_literal_expr() {
            Some(known)
        } else {
            None
        }
    }
}
