use std::ops::{Deref, DerefMut};
use crate::ast::block::BlockSyntax;
use crate::ast::GenericSyntax;
use crate::ast::statements::Statement;
use crate::common::sourcemap::SourceMap;
use crate::compiler::Compiler;
use crate::typed_ast::ast::statements::expression::{
    TypedCallOperation, TypedExpr, TypedExprNode, TypedUnaryOperation, UnaryOperator,
};
use crate::typed_ast::ast::statements::TypedStatement;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::ty::TypeId;




pub struct TypedBlock {
    pub code: Vec<TypedStatement>,
    pub drops: Vec<TypedExpr>,
}

impl Deref for TypedBlock {
    type Target = Vec<TypedStatement>;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl DerefMut for TypedBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.code
    }
}


pub type TypedBlockSyntax = GenericSyntax<TypedBlock>;



impl TypedBlockSyntax {
    pub fn from_ast(blk: &BlockSyntax, function_return: TypeId, compiler: &mut Compiler, context: &mut AvailableContext<TypeId>) -> Option<Self> {
        context.push_new_scope();

        let mut statements = Vec::new();
        // Variables declared in this block that need end-of-scope drops, in declaration order.
        let mut drop_vars: Vec<(String, TypeId, String, SourceMap)> = Vec::new();

        for s in &blk.data {
            // Redeclaration: if `let x = expr` shadows a droppable `x` in this block,
            // record the old drop info so codegen can drop AFTER evaluating the init expr.
            let mut redecl_drop: Option<(TypeId, String)> = None;
            if let Statement::VariableDeclaration(vd) = s {
                if let Some(pos) = drop_vars.iter().position(|(name, ..)| *name == vd.data.name) {
                    let (_, old_ty, old_drop_mangled, _) = drop_vars.remove(pos);
                    redecl_drop = Some((old_ty, old_drop_mangled));
                }
            }

            let mut stmt = TypedStatement::from_ast(s, function_return, compiler, context)?;

            // Attach the shadowed drop info onto the VarDecl node for codegen.
            if let Some(drop_info) = redecl_drop {
                if let TypedStatement::VarDecl(ref mut vd) = stmt {
                    vd.shadowed_drop = Some(drop_info);
                }
            }

            // Track droppable variables declared in this block for end-of-scope cleanup.
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

        // Build end-of-scope drops in LIFO order (last declared → dropped first).
        let mut drops = Vec::new();
        for (name, ty, drop_mangled, smap) in drop_vars.into_iter().rev() {
            if let Some(drop_call) = make_drop_call(&name, ty, &drop_mangled, &smap, compiler, context) {
                drops.push(drop_call);
            }
        }

        context.pop_last_scope();

        Some(Self{
            data: TypedBlock {
                code: statements,
                drops,
            },
            smap: blk.smap.clone(),
        })
    }
}

/// Build a `drop(T&)` call expression for a named local variable.
fn make_drop_call(
    name: &str,
    ty: TypeId,
    drop_mangled: &str,
    smap: &SourceMap,
    compiler: &mut Compiler,
    context: &AvailableContext<TypeId>,
) -> Option<TypedExpr> {
    let fn_type_id = *context.get_identifier(&drop_mangled.to_string())?;
    let self_ref_ty = compiler.type_context.reference_to(ty);
    let var_expr = TypedExpr {
        ty,
        smap: smap.clone(),
        value: TypedExprNode::Identifier(name.to_string()),
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
        value: TypedExprNode::Identifier(drop_mangled.to_string()),
    };
    Some(TypedExpr {
        ty: compiler.type_context.none,
        smap: smap.clone(),
        value: TypedExprNode::CallOp(Box::new(TypedCallOperation {
            caller: fn_expr,
            arguments: vec![self_ref],
        })),
    })
}
