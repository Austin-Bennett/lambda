use crate::ast::block::{Block, BlockSyntax};
use crate::ast::GenericSyntax;
use crate::compiler::Compiler;
use crate::typed_ast::ast::statements::TypedStatement;
use crate::typed_ast::typing::scope::AvailableContext;

pub type TypedBlock = Vec<TypedStatement>;

pub type TypedBlockSyntax = GenericSyntax<TypedBlock>;


impl TypedBlockSyntax {
    pub fn from_ast(blk: &BlockSyntax, compiler: &mut Compiler, context: &mut AvailableContext) -> Option<Self> {
        let mut statements = Vec::new();
        for s in &blk.data {
            statements.push(TypedStatement::from_ast(s, compiler, context)?);
        }
        
        Some(Self{
            data: statements,
            smap: blk.smap.clone(),
        })
    }
}