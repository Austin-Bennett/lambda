use crate::ast::statements::vardecl::{VarDecl, VarDeclSyntax};
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::ast::statements::expression::TypedExpr;
use crate::typed_ast::ast::type_inference::TypeInferencer;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::ty::TypeId;

pub struct TypedVarDecl {
    pub name: String,
    pub smap: SourceMap,
    pub ty: TypeId,
    pub val: Option<TypedExpr>
}


impl TypedVarDecl {
    pub fn from_ast(vd: &VarDeclSyntax, compiler: &mut Compiler, context: &mut AvailableContext) -> Option<Self> {
        let (_, ty) = match compiler.resolve_type(&vd.data.ty) {
            Some(v) => v,
            None => {
                compiler.emit_compile_message(
                    CompileMessage::new(
                        vd.smap.clone(),
                        format!("Unknown type: {:?}", vd.data.ty),
                        CompileMessageType::Error,
                    )
                );
                return None;
            }
        };

        let expr = match &vd.data.value {
            Some(e) => {
                let mut e = TypedExpr::from_ast(e, compiler, context)?;

                let info = compiler.type_context.get_by_id(e.ty).unwrap();

                TypeInferencer::infer_unknown(&compiler.type_context, &mut e, ty);

                if e.ty != ty && !info.ops.implicit_conversion.contains_key(&ty) {
                    compiler.emit_compile_message(
                        CompileMessage::new(
                            e.smap.clone(),
                            format!("Cannot set variable of type {} to expression of type {}",
                                    compiler.type_context.name_of(ty).unwrap(), compiler.type_context.name_of(e.ty).unwrap()),
                            CompileMessageType::Error,
                        )
                    )
                }

                Some(e)
            },
            None => None
        };



        context.declare_identifier_in_scope(vd.data.name.clone(), ty);

        Some(
            Self{
                name: vd.data.name.clone(),
                ty,
                val: expr,
                smap: vd.smap.clone(),
            }
        )

    }
}