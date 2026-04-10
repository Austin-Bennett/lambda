use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign};
use crate::ast::function::FunctionSyntax;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::ast::block::{TypedBlockSyntax};
use crate::typed_ast::ast::statements::vardecl::TypedVarDecl;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::TypeId;

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct FunctionSignature {
    pub name: String,
    pub ret: TypeId,
    pub params: Vec<TypeId>,
}

impl FunctionSignature {
    pub fn to_string(&self, context: &TypeContext) -> String {
        format!("{}({})", self.name, self.params
            .iter()
            .map(|id| context.name_of(*id).unwrap())
            .collect::<Vec<String>>()
            .join(", ")
        )
    }
}

pub struct Function {
    //we might need it, not sure right now, and id like to avoid unnecessary clones
    //sig: FunctionSignature,
    pub code: TypedBlockSyntax,
    pub params: Vec<String>,
}

impl Function {
    pub fn to_string(&self, context: &TypeContext) -> String {
        let mut f = String::new();

        f.add_assign("{");
        if !self.code.data.is_empty() {
            f.add_assign("\n");
        }

        for s in &self.code.data {
            f.add_assign("\t");
            f.add_assign(s.to_string(context).as_str());
            f.add_assign("\n");
        }

        f.add_assign("}");

        f
    }
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
            //this will also declare the parameters in the scope
            params.push(TypedVarDecl::from_ast(p, compiler, context)?)
        }

        //resolve the signature
        let sig = FunctionSignature {
            name: func.data.name.clone(),
            ret,
            params: params.iter().map(|v| v.ty).collect()
        };

        

        let res = Some(
            (
                sig,
                Self{
                    code: TypedBlockSyntax::from_ast(&func.data.body, compiler, context)?,
                    params: params.iter().map(|v| v.name.clone()).collect()
                }
            )
        );

        context.pop_last_scope();

        res
    }
}