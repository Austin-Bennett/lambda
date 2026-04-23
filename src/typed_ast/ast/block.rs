use crate::ast::block::BlockSyntax;
use crate::ast::GenericSyntax;
use crate::common::sourcemap::SourceMap;
use crate::compiler::Compiler;
use crate::typed_ast::ast::statements::expression::{
    TypedCallOperation, TypedExpr, TypedExprNode, TypedUnaryOperation, UnaryOperator,
};
use crate::typed_ast::ast::statements::TypedStatement;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::ty::TypeId;

pub type TypedBlock = Vec<TypedStatement>;


pub type TypedBlockSyntax = GenericSyntax<TypedBlock>;



impl TypedBlockSyntax {
    pub fn from_ast(blk: &BlockSyntax, function_return: TypeId, compiler: &mut Compiler, context: &mut AvailableContext<TypeId>) -> Option<Self> {
        context.push_new_scope();

        let mut statements = Vec::new();
        // Track variables declared in this block that need drop calls, in declaration order.
        let mut drop_vars: Vec<(String, TypeId, String, SourceMap)> = Vec::new();

        for s in &blk.data {
            let stmt = TypedStatement::from_ast(s, function_return, compiler, context)?;
            // If this declares a variable whose type has a drop operator, record it.
            if let TypedStatement::VarDecl(ref vd) = stmt {
                if let Some(drop_mangled) = compiler.type_context.get_by_id(vd.ty)
                    .and_then(|info| info.ops.drop.as_ref())
                    .cloned()
                {
                    drop_vars.push((vd.name.clone(), vd.ty, drop_mangled, vd.smap.clone()));
                }
            }
            statements.push(stmt);
        }

        // Append drop calls in LIFO order (last declared → dropped first).
        for (name, ty, drop_mangled, smap) in drop_vars.into_iter().rev() {
            let fn_type_id = match context.get_identifier(&drop_mangled) {
                Some(&id) => id,
                None => continue,
            };
            let self_ref_ty = compiler.type_context.reference_to(ty);
            let var_expr = TypedExpr {
                ty,
                smap: smap.clone(),
                value: TypedExprNode::Identifier(name),
            };
            let self_ref = TypedExpr {
                ty: self_ref_ty,
                smap: smap.clone(),
                value: TypedExprNode::UnaryOp(Box::new(TypedUnaryOperation {
                    op: UnaryOperator::Reference,
                    operand: var_expr,
                })),
            };
            let fn_expr = TypedExpr {
                ty: fn_type_id,
                smap: smap.clone(),
                value: TypedExprNode::Identifier(drop_mangled),
            };
            let drop_call = TypedExpr {
                ty: compiler.type_context.none,
                smap: smap.clone(),
                value: TypedExprNode::CallOp(Box::new(TypedCallOperation {
                    caller: fn_expr,
                    arguments: vec![self_ref],
                })),
            };
            statements.push(TypedStatement::Expr(drop_call));
        }

        context.pop_last_scope();

        Some(Self{
            data: statements,
            smap: blk.smap.clone(),
        })
    }
}