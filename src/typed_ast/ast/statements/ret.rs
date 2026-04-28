use crate::ast::statements::ret::Return;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::ast::statements::expression::TypedExpr;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::ty::{TypeId, TypeKind};

pub struct TypedReturn(pub TypedExpr);

impl TypedReturn {
    pub fn from_ast(ast: &Return, function_return: TypeId, compiler: &mut Compiler, context: &AvailableContext<TypeId>) -> Option<Self> {
        // When the declared return type is a reference (T&), skip coerce_ref so the reference
        // isn't stripped before the type comparison.
        let is_ref_return = matches!(
            compiler.type_context.get_by_id(function_return).map(|i| &i.kind),
            Some(TypeKind::Reference(_))
        );
        let expr = if is_ref_return {
            let (value, ty) = TypedExpr::from_node(&ast.0, compiler, context)?;
            TypedExpr { value, ty, smap: ast.0.smap.clone() }
        } else {
            TypedExpr::from_ast(&ast.0, compiler, context)?
        };
        let expr = TypedExpr::coerce_literal(&compiler.type_context, expr, function_return);
        
        if expr.ty != function_return {
            compiler.emit_compile_message(
                CompileMessage::new(
                    expr.smap,
                    format!("Expected expression of type {}, got {}", 
                            compiler.type_context.name_of(function_return).unwrap(), 
                            compiler.type_context.name_of(expr.ty).unwrap()
                    ),
                    CompileMessageType::Error
                )
            );
            return None;
        }
        
        Some(Self(expr))
    }
}