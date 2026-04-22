use crate::ast::statements::ret::Return;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::ast::statements::expression::TypedExpr;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::ty::TypeId;

pub struct TypedReturn(pub TypedExpr);

impl TypedReturn {
    pub fn from_ast(ast: &Return, function_return: TypeId, compiler: &mut Compiler, context: &AvailableContext<TypeId>) -> Option<Self> {
        
        let expr = TypedExpr::from_ast(&ast.0, compiler, context)?;
        let expr = TypedExpr::coerce_ref(expr, &compiler.type_context);
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