use crate::ast::statements::vardecl::VarDeclSyntax;
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::ast::statements::expression::TypedExpr;
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
        let ty = match compiler.resolve_type(&vd.data.ty) {
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




                e = TypedExpr::coerce_literal_array(
                    TypedExpr::coerce_literal(&compiler.type_context, e, ty),
                    &mut compiler.type_context, ty
                );




                if e.ty != ty {
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