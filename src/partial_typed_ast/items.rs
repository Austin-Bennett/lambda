
use crate::partial_typed_ast;
use crate::ast;
use crate::ast::block::Block;
use crate::ast::function::Function;
use crate::ast::GenericSyntax;
use crate::common::sourcemap::SourceMap;
use crate::common::type_context::{PartialType, TypeContext};
use crate::common::utils::modulepath::ModulePath;
use crate::compiler::Compiler;
use crate::partial_typed_ast::vardecl::PTVarDecl;

//PT stands for Partially Typed
pub struct PTFunction {
    name: ModulePath,
    parameters: Vec<PTVarDecl>,
    body: Block,
    ty: Option<PartialType>, //None for void
}

pub type PTFunctionSyntax = GenericSyntax<PTFunction>;

impl PTFunctionSyntax {
    pub fn from_untyped(context: &TypeContext, function: Function, smap: SourceMap, compiler: &mut Compiler) -> Option<Self> {
        let mut parameters = Vec::new();

        for p in function.parameters {
            let t = PartialType::from_untyped(context, p.ty, smap.clone(), compiler)?;
            parameters.push(
                PTVarDecl{
                    name: p.name,
                    ty: t,
                    value: p.value,
                }
            )
        }

        let ty = match function.ty {
            Some(t) => Some(PartialType::from_untyped(context, t, smap.clone(), compiler)?),
            None => None
        };

        Some(PTFunctionSyntax {
            data: PTFunction {
                name: function.name,
                body: function.body,
                ty,
                parameters
            },
            smap
        })
    }
}