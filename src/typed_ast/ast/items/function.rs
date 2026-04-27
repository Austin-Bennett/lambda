use crate::ast::block::BlockSyntax;
use crate::ast::function::FunctionSyntax;
use crate::ast::items::modify::{MethodDecl, OperatorDecl, SelfMode};
use crate::ast::statements::vardecl::VarDeclSyntax;
use crate::ast::ty::Type;
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::ast::block::TypedBlockSyntax;
use crate::typed_ast::ast::statements::vardecl::TypedVarDecl;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::TypeId;
use std::ops::{AddAssign, Deref};

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

            for s in code.data.deref() {
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
    pub fn from_parts(
        mangled_name: &str,
        self_mode: SelfMode,
        self_type_id: Option<TypeId>,
        params: &[VarDeclSyntax],
        ret_ty: Option<&Type>,
        body: Option<&BlockSyntax>,
        is_extern: bool,
        error_smap: SourceMap,
        compiler: &mut Compiler,
        context: &mut AvailableContext<TypeId>,
    ) -> Option<Self> {
        context.push_new_scope();

        let ret = match ret_ty {
            Some(ty) => {
                let Some(id) = compiler.resolve_type(ty) else {
                    compiler.emit_compile_message(CompileMessage::new(
                        error_smap,
                        format!("Could not resolve return type {:?}", ty),
                        CompileMessageType::Error,
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

        match self_mode {
            SelfMode::ByRef => {
                let self_ref_ty = compiler.type_context.reference_to(self_type_id.unwrap());
                context.declare_identifier_in_scope("self".to_string(), self_ref_ty);
                param_names.push("self".to_string());
                param_types.push(self_ref_ty);
            }
            SelfMode::Value => {
                let self_ty = self_type_id.unwrap();
                context.declare_identifier_in_scope("self".to_string(), self_ty);
                param_names.push("self".to_string());
                param_types.push(self_ty);
            }
            SelfMode::None => {}
        }

        for p in params {
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

        let prev_self_type = compiler.current_self_type;
        compiler.current_self_type = self_type_id;

        let code = if let Some(b) = body {
            let Some(typed) = TypedBlockSyntax::from_ast(b, ret, compiler, context) else {
                compiler.current_self_type = prev_self_type;
                context.pop_last_scope();
                return None;
            };
            Some(typed)
        } else {
            None
        };

        compiler.current_self_type = prev_self_type;
        context.pop_last_scope();

        Some(Self { is_extern, signature: sig, code, params: param_names })
    }

    pub fn from_method(
        method: &MethodDecl,
        self_type_id: TypeId,
        mangled_name: &str,
        compiler: &mut Compiler,
        context: &mut AvailableContext<TypeId>,
    ) -> Option<Self> {
        Self::from_parts(
            mangled_name,
            method.self_mode.clone(),
            Some(self_type_id),
            &method.params,
            method.ret.as_ref(),
            method.body.as_ref(),
            false,
            method.body.as_ref().map(|b| b.smap.clone()).unwrap_or_default(),
            compiler,
            context,
        )
    }

    pub fn from_operator(
        op: &OperatorDecl,
        self_type_id: TypeId,
        mangled_name: &str,
        compiler: &mut Compiler,
        context: &mut AvailableContext<TypeId>,
    ) -> Option<Self> {
        Self::from_parts(
            mangled_name,
            op.self_mode.clone(),
            Some(self_type_id),
            &op.params,
            op.ret.as_ref(),
            op.body.as_ref(),
            false,
            op.body.as_ref().map(|b| b.smap.clone()).unwrap_or_default(),
            compiler,
            context,
        )
    }

    pub fn from_ast(func: &FunctionSyntax, compiler: &mut Compiler, context: &mut AvailableContext<TypeId>) -> Option<Self> {
        let result = Self::from_parts(
            &func.data.name,
            SelfMode::None,
            None,
            &func.data.parameters,
            func.data.ty.as_ref(),
            func.data.body.as_ref(),
            func.data.is_extern,
            func.smap.clone(),
            compiler,
            context,
        )?;

        if result.signature.name == "main" && result.signature.ret != compiler.type_context.int8 {
            compiler.emit_compile_message(CompileMessage::new(
                func.smap.clone(),
                "main function must return an int8 denoting the programs exit status!".to_string(),
                CompileMessageType::Error,
            ));
            return None;
        }

        Some(result)
    }
}