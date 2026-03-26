use crate::ast::function::FunctionSyntax;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::ast::block::{TypedBlockSyntax};
use crate::typed_ast::ast::statements::vardecl::TypedVarDecl;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::ty::TypeId;

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct FunctionSignature {
    name: String,
    ret: TypeId,
    params: Vec<TypeId>,
}

pub struct Function {
    //we might need it, not sure right now, and id like to avoid unnecessary clones
    //sig: FunctionSignature,
    pub code: TypedBlockSyntax,
}

impl Function {
    pub fn from_ast(func: &FunctionSyntax, compiler: &mut Compiler, context: &mut AvailableContext) -> Option<(FunctionSignature, Self)> {
        context.push_new_scope();

        let ret = match &func.data.ty {
            Some(ty) => {
                let Some((_, id)) = compiler.resolve_type(ty) else {

                    compiler.emit_compile_message(
                        CompileMessage::new(
                            func.smap.clone(),
                            format!("Could not resolve type {:?}", ty),
                            CompileMessageType::Error,
                        )
                    );

                    return None;
                };

                id
            },
            None => compiler.type_context.none
        };

        let mut params = Vec::new();

        for p in &func.data.parameters {
            //this will also declare the variables in the scope
            params.push(TypedVarDecl::from_ast(p, compiler, context)?)
        }

        //resolve the signature
        let sig = FunctionSignature {
            name: func.data.name.clone(),
            ret,
            params: params.iter().map(|f| f.ty).collect()
        };

        
        context.pop_last_scope();
        Some(
            (
                sig,
                Self{
                    code: TypedBlockSyntax::from_ast(&func.data.body, compiler, context)?
                }
            )
        )
    }
}