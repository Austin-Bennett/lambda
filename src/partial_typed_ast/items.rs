use crate::ast;
use crate::ast::block::Block;
use crate::ast::statements::vardecl::VarDecl;
use crate::ast::ty::Type;
use crate::common::type_context::TypeContext;
use crate::common::utils::modulepath::ModulePath;

pub struct PartiallyTypedFunction {
    name: ModulePath,
    parameters: Vec<VarDecl>,
    body: Block,
    ty: Option<Type>, //None for void
}

impl PartiallyTypedFunction {
    pub fn make_from_untyped(context: &TypeContext, function: ast::items::function::Function) {
        
    }
}