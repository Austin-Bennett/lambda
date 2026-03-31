pub mod literals;


use std::collections::HashMap;
use crate::typed_ast::ast::statements::expression::{TypedExpr, TypedExprNode};
use crate::typed_ast::ast::type_inference::literals::{BIntLiteralInferencer};
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::TypeId;

pub trait BinaryInferencer {

    //returns true if it was able to infer the types of the nodes
    fn infer_type(&self, context: &TypeContext, unknown: &TypedExprNode, known: TypeId) -> Option<TypeId>;
}





pub struct TypeInferencer;


impl TypeInferencer {
    pub const BINARY_INFERENCERS: &[&dyn BinaryInferencer] = &[
        &BIntLiteralInferencer,
    ];



    //infers the type for a binary expression between 2 numbers
    pub fn infer_binary(context: &TypeContext, expr1: &mut TypedExpr, expr2: &mut TypedExpr) -> bool {
        if expr1.ty == expr2.ty && expr2.ty == context.infer { return true; }
        if expr1.ty != context.infer && expr2.ty != context.infer { return true; }

        let (uk, k) = if expr1.ty == context.infer {
            (expr1, expr2.ty)
        } else {
            (expr2, expr1.ty)
        };

        Self::infer_unknown(context, uk, k)
    }
    
    pub fn infer_unknown(context: &TypeContext, unknown: &mut TypedExpr, known_ty: TypeId) -> bool {
        if unknown.ty != context.infer { return true; }
        
        
        for i in Self::BINARY_INFERENCERS {
            if let Some(v) = i.infer_type(context, &unknown.value, known_ty) {

                unknown.ty = v;

                return true;
            }
        }
        
        false
    }

    pub fn infer_call(context: &TypeContext, params: &mut Vec<TypeId>, exprs: &mut Vec<TypedExprNode>, call_op: &HashMap<Vec<TypeId>, TypeId>) -> Option<TypeId> {
        if let Some(ty) = call_op.get(params) {
            return Some(*ty);
        }

        let mut tys = vec![0; params.len()];

        'outer: for (sig, res_ty) in call_op {

            if sig.len() != params.len() { continue; }

            for (i, ((ty, expr), param_ty)) in params
                .iter().copied()
                .zip(exprs.iter())
                .zip(sig.iter().copied())
                .enumerate() {
                
                if ty == context.infer {
                    for inferer in Self::BINARY_INFERENCERS {
                        if let Some(v) = inferer.infer_type(context, expr, param_ty) {
                            tys[i] = v;
                        } else {
                            continue 'outer;
                        }
                    }
                } else {
                    tys[i] = ty;
                }
            }
            
            for (i, t) in tys.iter().enumerate() {
                params[i] = *t;
            }
            
            //reaching here means we have succesfully inferred all the parameters
            return Some(*res_ty)
        }


        None
    }
}