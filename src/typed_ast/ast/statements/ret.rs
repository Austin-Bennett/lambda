use crate::ast::GenericSyntax;
use crate::ast::statements::ret::Return;
use crate::common::sourcemap::SourceMap;
use crate::compiler::Compiler;
use crate::typed_ast::ast::statements::expression::{TypedExpr, TypedExprNode};
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::ty::TypeKind;

pub struct TypedReturn(pub TypedExpr);

impl TypedReturn {
    pub fn from_ast(ast: &Return, compiler: &mut Compiler, context: &AvailableContext) -> Option<Self> {
        
         
        
        
        Some(Self(TypedExpr::coerce_ref(TypedExpr::from_ast(&ast.0, compiler, context)?, &compiler.type_context)))
    }
}