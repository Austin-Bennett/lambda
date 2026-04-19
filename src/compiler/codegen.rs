use std::collections::HashMap;
use std::mem;
use std::process::abort;
use inkwell::builder::Builder;
use inkwell::types::{AnyTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FunctionValue};
use crate::compiler::Compiler;
use crate::typed_ast::ast::items::function::{Function, FunctionSignature};
use crate::typed_ast::ast::statements::expression::{TypedExpr, TypedExprNode};
use crate::typed_ast::ast::statements::TypedStatement;
use crate::typed_ast::typing::ty::TypeInfo;

impl Compiler {

    pub fn compile(& mut self) -> inkwell::module::Module<'static> {
        let mut builder = self.llvm_context.create_builder();
        let mut module = self.llvm_context.create_module("lambda_program");

        let modules = mem::take(&mut self.typed_modules);
        let this = self as *mut Self;

        for (_, tmod) in &modules {
            for func in &tmod.functions {
                unsafe{ &*this }.compile_function(&mut builder, &mut module, func);
            }
        }

        self.typed_modules = modules;

        module
    }

    pub fn compile_function(& self, builder: &mut Builder, module: &mut inkwell::module::Module<'static>, func: &Function) {
        let ret_type = self.type_context.get_by_id(func.signature.ret).unwrap();

        let params: Vec<&TypeInfo> = func.signature.params.iter()
            .map(|p| self.type_context.get_by_id(*p).unwrap()).collect();

        let llvm_params: Vec<_> = params.iter().map(
            |p| p.llvm_type.try_into().unwrap()
        ).collect();


        let func_ty = match ret_type.llvm_type {
            AnyTypeEnum::ArrayType(t) => { t.fn_type(&llvm_params, false) }
            AnyTypeEnum::FloatType(t) => { t.fn_type(&llvm_params, false) }
            AnyTypeEnum::IntType(t) => { t.fn_type(&llvm_params, false) }
            AnyTypeEnum::PointerType(t) => { t.fn_type(&llvm_params, false) }
            AnyTypeEnum::StructType(t) => { t.fn_type(&llvm_params, false) }
            AnyTypeEnum::VoidType(t) => { t.fn_type(&llvm_params, false) }
            _ => { abort() }
        };

        let fn_val: FunctionValue = module.add_function(&func.signature.name, func_ty, None);


        let mut locals = HashMap::new();
        locals.extend(
            func.params.iter()
                .zip(fn_val.get_param_iter())
                .map(|(s, p)| (s.clone(), p))
        );


        let entry = self.llvm_context.append_basic_block(fn_val, "entry");

        for statement in &func.code.data {
            self.compile_statement(builder, &mut locals, statement);
        }
    }

    pub fn compile_statement(& self, builder: &mut Builder,
                             locals: &mut HashMap<String, BasicValueEnum<'static>>, s: &TypedStatement) {
        match s {
            TypedStatement::Expr(e) => {

            }
            TypedStatement::VarDecl(_) => {}
            TypedStatement::Return(_) => {}
        }
    }
    
    pub fn compile_expression(& self, builder: &mut Builder, 
                              locals: &mut HashMap<String, BasicValueEnum<'static>>, e: &TypedExpr) {
        match e.value {
            TypedExprNode::IntLiteral(il) => {

            }
            TypedExprNode::Identifier(_) => {}
            TypedExprNode::Tuple(_) => {}
            TypedExprNode::BinaryOp(_) => {}
            TypedExprNode::UnaryOp(_) => {}
            TypedExprNode::CallOp(_) => {}
        }
    }
}