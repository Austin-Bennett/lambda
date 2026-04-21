use std::fmt::{Debug, Formatter};
use crate::ast::block::BlockSyntax;
use crate::ast::GenericSyntax;
use crate::compiler::Compiler;
use crate::typed_ast::ast::statements::TypedStatement;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::ty::TypeId;

pub type TypedBlock = Vec<TypedStatement>;


pub type TypedBlockSyntax = GenericSyntax<TypedBlock>;



impl TypedBlockSyntax {
    pub fn from_ast(blk: &BlockSyntax, function_return: TypeId, compiler: &mut Compiler, context: &mut AvailableContext<TypeId>) -> Option<Self> {
        context.push_new_scope();

        let mut statements = Vec::new();
        for s in &blk.data {
            statements.push(TypedStatement::from_ast(s, function_return, compiler, context)?);
        }

        context.pop_last_scope();

        Some(Self{
            data: statements,
            smap: blk.smap.clone(),
        })
    }
}