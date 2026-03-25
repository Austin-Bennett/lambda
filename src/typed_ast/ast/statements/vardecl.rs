use crate::ast::statements::vardecl::VarDecl;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::ast::statements::expression::TypedExpr;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::ty::TypeId;

pub struct TypedVarDecl {
    name: ModulePath,
    ty: TypeId,
    val: Option<TypedExpr>
}


impl TypedVarDecl {
    pub fn from_ast(vd: &VarDecl, smap: &SourceMap, compiler: &mut Compiler, context: &AvailableContext) -> Option<Self> {
        let ty = match compiler.type_context.get_type_id(&vd.ty) {
            Some(v) => v,
            None => {
                compiler.emit_compile_message(
                    CompileMessage::new(
                        smap.clone(),
                        format!("Unknown type: {:?}", vd.ty),
                        CompileMessageType::Error,
                    )
                );
                return None;
            }
        };

        let expr = match &vd.value {
            Some(e) => Some(TypedExpr::from_ast(e, smap, compiler, context)?),
            None => None
        };

        Some(
            Self{
                name: vd.name.clone(),
                ty,
                val: expr,
            }
        )

    }
}