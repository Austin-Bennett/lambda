use crate::ast::function::FunctionSyntax;
use crate::ast::items::modify::{MethodDecl, OperatorDecl};
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::ast::block::TypedBlockSyntax;
use crate::typed_ast::ast::statements::vardecl::TypedVarDecl;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::TypeId;
use std::ops::AddAssign;

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
    pub is_extern: bool,
    pub signature: FunctionSignature,
    pub code: Option<TypedBlockSyntax>,
    pub params: Vec<String>,
}

impl Function {
    pub fn to_string(&self, context: &TypeContext) -> String {
        let mut f = String::new();

        
        
        if let Some(code) = &self.code {
            f.add_assign("{");
            
            if !code.data.is_empty() {
                f.add_assign("\n");
            }

            for s in &code.data {
                f.add_assign("\t");
                f.add_assign(s.to_string(context).as_str());
                f.add_assign("\n");
            }

            f.add_assign("}");
        }
        

        f
    }
}



impl Function {
    pub fn from_method(
        method: &MethodDecl,
        self_type_id: TypeId,
        mangled_name: &str,
        compiler: &mut Compiler,
        context: &mut AvailableContext<TypeId>,
    ) -> Option<Self> {
        context.push_new_scope();

        let ret = match &method.ret {
            Some(ty) => {
                let Some(id) = compiler.resolve_type(ty) else {
                    compiler.emit_compile_message(crate::compiler::CompileMessage::new(
                        method.body.as_ref().map(|b| b.smap.clone()).unwrap_or_default(),
                        format!("Could not resolve return type {:?}", ty),
                        crate::compiler::CompileMessageType::Error,
                    ));
                    context.pop_last_scope();
                    return None;
                };
                id
            }
            None => compiler.type_context.none,
        };

        let mut param_names = Vec::new();
        let mut param_types = Vec::new();

        if method.has_self {
            let self_ref_ty = compiler.type_context.reference_to(self_type_id);
            context.declare_identifier_in_scope("self".to_string(), self_ref_ty);
            param_names.push("self".to_string());
            param_types.push(self_ref_ty);
        }

        for p in &method.params {
            let Some(typed) = TypedVarDecl::from_ast(p, compiler, context) else {
                context.pop_last_scope();
                return None;
            };
            param_names.push(typed.name.clone());
            param_types.push(typed.ty);
        }

        let sig = FunctionSignature {
            name: mangled_name.to_string(),
            ret,
            params: param_types,
        };

        let code = if let Some(body) = &method.body {
            Some(TypedBlockSyntax::from_ast(body, ret, compiler, context)?)
        } else {
            None
        };

        context.pop_last_scope();

        Some(Self {
            is_extern: false,
            signature: sig,
            code,
            params: param_names,
        })
    }

    pub fn from_operator(
        op: &OperatorDecl,
        self_type_id: TypeId,
        mangled_name: &str,
        compiler: &mut Compiler,
        context: &mut AvailableContext<TypeId>,
    ) -> Option<Self> {
        context.push_new_scope();

        let ret = match &op.ret {
            Some(ty) => {
                let Some(id) = compiler.resolve_type(ty) else {
                    compiler.emit_compile_message(crate::compiler::CompileMessage::new(
                        op.body.as_ref().map(|b| b.smap.clone()).unwrap_or_default(),
                        format!("Could not resolve return type {:?}", ty),
                        crate::compiler::CompileMessageType::Error,
                    ));
                    context.pop_last_scope();
                    return None;
                };
                id
            }
            None => compiler.type_context.none,
        };

        let mut param_names = Vec::new();
        let mut param_types = Vec::new();

        if op.has_self {
            let self_ref_ty = compiler.type_context.reference_to(self_type_id);
            context.declare_identifier_in_scope("self".to_string(), self_ref_ty);
            param_names.push("self".to_string());
            param_types.push(self_ref_ty);
        }

        for p in &op.params {
            let Some(typed) = TypedVarDecl::from_ast(p, compiler, context) else {
                context.pop_last_scope();
                return None;
            };
            param_names.push(typed.name.clone());
            param_types.push(typed.ty);
        }

        let sig = FunctionSignature {
            name: mangled_name.to_string(),
            ret,
            params: param_types,
        };

        let code = if let Some(body) = &op.body {
            Some(TypedBlockSyntax::from_ast(body, ret, compiler, context)?)
        } else {
            None
        };

        context.pop_last_scope();

        Some(Self {
            is_extern: false,
            signature: sig,
            code,
            params: param_names,
        })
    }

    pub fn from_ast(func: &FunctionSyntax, compiler: &mut Compiler, context: &mut AvailableContext<TypeId>) -> Option<Self> {
        context.push_new_scope();

        let ret = match &func.data.ty {
            Some(ty) => {
                let Some(id) = compiler.resolve_type(ty) else {

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

        if sig.name == "main" && sig.ret != compiler.type_context.int8 {
            compiler.emit_compile_message(
                CompileMessage::new(
                    func.smap.clone(),
                    "main function must return an i32 denoting the programs return type!".to_string(),
                    CompileMessageType::Error
                )
            );
            return None;
        }
        
        let code = if let Some(body) = &func.data.body {
            Some(TypedBlockSyntax::from_ast(body, ret, compiler, context)?)
        } else {
            None
        };

        let res = Some(

            Self{
                code,
                params: params.iter().map(|v| v.name.clone()).collect(),
                signature: sig,
                is_extern: func.data.is_extern,
            }

        );


        context.pop_last_scope();

        res
    }
}