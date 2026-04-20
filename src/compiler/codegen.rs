use crate::compiler::Compiler;
use crate::typed_ast::ast::items::function::Function;
use crate::typed_ast::ast::statements::expression::{BinaryOperator, UnaryOperator, TypedExpr, TypedExprNode};
use crate::typed_ast::ast::statements::TypedStatement;
use crate::typed_ast::typing::ty::{TypeKind, TypeId};
use inkwell::builder::Builder;
use inkwell::types::{AnyTypeEnum, BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{AnyValueEnum, BasicMetadataValueEnum, BasicValueEnum, FunctionValue};
use std::collections::HashMap;
use std::mem;
use std::process::abort;
use inkwell::module::Linkage;

impl Compiler {

    pub fn compile(&mut self) -> inkwell::module::Module<'static> {
        let mut builder = self.llvm_context.create_builder();
        let mut module = self.llvm_context.create_module("lambda_program");

        let mut globals: HashMap<String, AnyValueEnum> = HashMap::new();

        let modules = mem::take(&mut self.typed_modules);

        // First pass: declare all functions so they can be called before definition.
        for (_, tmod) in &modules {
            for func in &tmod.functions {
                let ret_type = self.type_context.get_by_id(func.signature.ret).unwrap();

                let llvm_params: Vec<BasicMetadataTypeEnum> = func.signature.params.iter()
                    .map(|p| {
                        let info = self.type_context.get_by_id(*p).unwrap();
                        info.llvm_type.try_into().unwrap()
                    })
                    .collect();

                let func_ty = match ret_type.llvm_type {
                    AnyTypeEnum::ArrayType(t)   => t.fn_type(&llvm_params, false),
                    AnyTypeEnum::FloatType(t)   => t.fn_type(&llvm_params, false),
                    AnyTypeEnum::IntType(t)     => t.fn_type(&llvm_params, false),
                    AnyTypeEnum::PointerType(t) => t.fn_type(&llvm_params, false),
                    AnyTypeEnum::StructType(t)  => t.fn_type(&llvm_params, false),
                    AnyTypeEnum::VoidType(t)    => t.fn_type(&llvm_params, false),
                    _ => abort(),
                };

                let fn_val: FunctionValue = module.add_function(&func.signature.name, func_ty,
                                                                match func.is_extern {
                                                                    true => Some(Linkage::External),
                                                                    false => None,
                                                                });
                globals.insert(func.signature.name.clone(), fn_val.into());
            }
        }

        // Second pass: compile each function body.
        for (_, tmod) in &modules {
            for func in &tmod.functions {
                let fn_val = globals[&func.signature.name].into_function_value();
                self.compile_function(&mut builder, &mut module, &globals, func, fn_val);
            }
        }

        self.typed_modules = modules;

        module
    }

    pub fn compile_function(
        &mut self,
        builder: &mut Builder<'static>,
        module: &mut inkwell::module::Module<'static>,
        globals: &HashMap<String, AnyValueEnum<'static>>,
        func: &Function,
        fn_val: FunctionValue<'static>,
    ) {
        if func.is_extern && func.code.is_none() {
            return;
        }

        let entry = self.llvm_context.append_basic_block(fn_val, "entry");
        builder.position_at_end(entry);

        let mut locals: HashMap<String, AnyValueEnum<'static>> = HashMap::new();

        // Allocate stack slots for each parameter and store the incoming value.
        // All locals in the `locals` map are alloca pointers; we always load on read.
        for (name, param) in func.params.iter().zip(fn_val.get_param_iter()) {
            let alloca = builder.build_alloca(param.get_type(), name).unwrap();
            builder.build_store(alloca, param).unwrap();
            locals.insert(name.clone(), alloca.into());
        }

        if let Some(code) = &func.code {
            for statement in &code.data {
                self.compile_statement(builder, globals, &mut locals, statement);
            }
        }


        // If the current block still has no terminator (e.g. a void function with
        // no explicit return statement), emit an implicit `ret void`.
        let ret_is_void = matches!(
            self.type_context.get_by_id(func.signature.ret).map(|i| &i.kind),
            Some(TypeKind::None)
        );
        if ret_is_void {
            if let Some(block) = builder.get_insert_block() {
                if block.get_terminator().is_none() {
                    builder.build_return(None).unwrap();
                }
            }
        }
    }

    pub fn compile_statement(
        &mut self,
        builder: &mut Builder<'static>,
        globals: &HashMap<String, AnyValueEnum<'static>>,
        locals: &mut HashMap<String, AnyValueEnum<'static>>,
        s: &TypedStatement,
    ) {
        match s {
            TypedStatement::Expr(e) => {
                self.compile_expression(builder, globals, locals, e);
            }

            TypedStatement::VarDecl(vd) => {
                // Resolve the LLVM type before taking any mutable borrow.
                let basic_ty: BasicTypeEnum<'static> = self
                    .type_context
                    .get_by_id(vd.ty)
                    .and_then(|info| info.llvm_type.try_into().ok())
                    .unwrap();

                let alloca = builder.build_alloca(basic_ty, &vd.name).unwrap();
                locals.insert(vd.name.clone(), alloca.into());

                if let Some(val_expr) = &vd.val {
                    if let Some(val) = self.compile_expression(builder, globals, locals, val_expr) {
                        if let Ok(basic_val) = BasicValueEnum::try_from(val) {
                            builder.build_store(alloca, basic_val).unwrap();
                        }
                    }
                }
            }

            TypedStatement::Return(ret) => {
                if let Some(val) = self.compile_expression(builder, globals, locals, &ret.0) {
                    if let Ok(basic_val) = BasicValueEnum::try_from(val) {
                        builder.build_return(Some(&basic_val)).unwrap();
                        return;
                    }
                }
                builder.build_return(None).unwrap();
            }
        }
    }

    pub fn compile_expression(
        &mut self,
        builder: &mut Builder<'static>,
        globals: &HashMap<String, AnyValueEnum<'static>>,
        locals: &mut HashMap<String, AnyValueEnum<'static>>,
        e: &TypedExpr,
    ) -> Option<AnyValueEnum<'static>> {
        match &e.value {
            TypedExprNode::IntLiteral(il) => {
                // Copy the context reference (it's &'static, so Copy) before borrowing type_context.
                let ctx = self.llvm_context;
                let maker_result = self
                    .type_context
                    .get_by_id(e.ty)
                    .and_then(|info| info.ops.from_int_literal.as_ref().map(|m| m(ctx, *il)));
                maker_result
            }

            TypedExprNode::RefRead(inner) => {
                // Evaluate the inner expr to get the reference pointer, then load through it.
                let ref_ptr = self.compile_expression(builder, globals, locals, inner)?;
                let inner_ty: BasicTypeEnum<'static> = self
                    .type_context
                    .get_by_id(e.ty)
                    .and_then(|info| info.llvm_type.try_into().ok())?;
                Some(builder.build_load(inner_ty, ref_ptr.into_pointer_value(), "refread").unwrap().into())
            }

            TypedExprNode::Identifier(ident) => {
                if let Some(alloca_val) = locals.get(ident) {
                    // Local variable: load from its alloca slot.
                    let ptr = alloca_val.into_pointer_value();
                    let basic_ty: BasicTypeEnum<'static> = self
                        .type_context
                        .get_by_id(e.ty)
                        .and_then(|info| info.llvm_type.try_into().ok())?;
                    Some(builder.build_load(basic_ty, ptr, ident).unwrap().into())
                } else {
                    // Global (function): return as-is.
                    globals.get(ident).copied()
                }
            }

            TypedExprNode::Tuple(_) => {
                todo!("tuple expressions")
            }
            TypedExprNode::Array(ray) => {
                let ty = self.type_context.get_by_id(e.ty).unwrap();

                let TypeKind::Array { ty: elem_ty, size } = ty.kind else { unreachable!() };

                let elem_ty: BasicTypeEnum = self.type_context.get_by_id(elem_ty)?.llvm_type.try_into().unwrap();
                let llvm_ty = ty.llvm_type.into_pointer_type();
                let usize_info = self.type_context.get_by_id(self.type_context.usize).unwrap().llvm_type.into_int_type();


                let ptr = builder.build_array_alloca(elem_ty,
                                                     self.llvm_context.i64_type().const_int(ray.len() as u64, false),
                                                     "array_ptr"
                ).unwrap();

                for (i, expr) in ray.iter().enumerate() {
                    let ty = self.type_context.get_by_id(expr.ty).unwrap();
                    let basic: BasicTypeEnum = ty.llvm_type.try_into().unwrap();
                    unsafe {
                        let elem_ptr = builder.build_gep(basic, ptr,
                        &[
                            usize_info.const_int(0, false),
                            usize_info.const_int(i as u64, false),
                        ], "elem")
                            .unwrap();

                        let compiled_expr = self.compile_expression(builder, globals, locals, expr)?;
                        let basic: BasicValueEnum = compiled_expr.try_into().unwrap();

                        builder.build_store(elem_ptr, basic).unwrap();
                    }
                }

                Some(ptr.into())

            },
            TypedExprNode::Index(index) => {
                let compiled_operand = self.compile_expression(builder, globals, locals, &index.operand)?;
                let compiled_index = self.compile_expression(builder, globals, locals, &index.index)?;

                let info = self.type_context.get_by_id(index.operand.ty).unwrap();

                let maker = &info.ops.index[&index.index.ty].1;
                Some(maker(builder, compiled_operand, compiled_index))
            },

            TypedExprNode::BinaryOp(bop) => {
                if let BinaryOperator::Assign = &bop.op {
                    let rhs_val = self.compile_expression(builder, globals, locals, &bop.rhs)?;
                    match &bop.lhs.value {
                        TypedExprNode::RefRead(ref_inner) => {
                            // Store through the reference pointer.
                            let ref_ptr = self.compile_expression(builder, globals, locals, ref_inner)?;
                            let basic: BasicValueEnum<'static> = rhs_val.try_into().ok()?;
                            builder.build_store(ref_ptr.into_pointer_value(), basic).unwrap();
                        }
                        TypedExprNode::Identifier(name) => {
                            let alloca = locals.get(name)?.into_pointer_value();
                            let ops = &self.type_context.get_by_id(bop.lhs.ty)?.ops;
                            let maker = ops.assign.get(&bop.rhs.ty)?;
                            maker(builder, alloca, rhs_val);
                        }
                        TypedExprNode::UnaryOp(uop) if matches!(uop.op, UnaryOperator::Dereference) => {
                            // Compile the pointer operand to get the address, then store through it.
                            let ptr_val = self.compile_expression(builder, globals, locals, &uop.operand)?;
                            let basic: BasicValueEnum<'static> = rhs_val.try_into().ok()?;
                            builder.build_store(ptr_val.into_pointer_value(), basic).unwrap();
                        }
                        _ => return None,
                    }
                    return Some(rhs_val);
                }

                let lhs_val = self.compile_expression(builder, globals, locals, &bop.lhs)?;
                let rhs_val = self.compile_expression(builder, globals, locals, &bop.rhs)?;

                // Look up the operator callback. All borrows of type_context are released
                // before the next compile_expression call, so no conflict.
                let result = {
                    let ops = &self.type_context.get_by_id(bop.lhs.ty)?.ops;
                    let entry = match &bop.op {
                        BinaryOperator::Add => ops.add.get(&bop.rhs.ty),
                        BinaryOperator::Sub => ops.sub.get(&bop.rhs.ty),
                        BinaryOperator::Mul => ops.mul.get(&bop.rhs.ty),
                        BinaryOperator::Div => ops.div.get(&bop.rhs.ty),
                        BinaryOperator::Assign => unreachable!("assign handled above"),
                    };
                    entry.map(|(_, maker)| maker(builder, lhs_val, rhs_val))
                };
                result
            }

            TypedExprNode::UnaryOp(uop) => {
                match &uop.op {
                    UnaryOperator::Reference => {
                        let TypedExprNode::Identifier(name) = &uop.operand.value else { return None; };
                        if let Some(alloca) = locals.get(name) {
                            return Some(alloca.into_pointer_value().into());
                        }
                        // functions are already pointers in LLVM's opaque pointer model
                        if let Some(&global) = globals.get(name) {
                            return Some(global);
                        }
                        return None;
                    }
                    UnaryOperator::Dereference => {

                        //dereferencing a pointer is syntactic sugar, we basically just return the pointer and let the "read ref"
                        //do the rest
                        let ptr_val = self.compile_expression(builder, globals, locals, &uop.operand)?;
                        //return the pointer
                        // let inner_ty = match self.type_context.get_by_id(uop.operand.ty)?.kind.clone() {
                        //     TypeKind::Pointer(id) => id,
                        //     _ => return None,
                        // };
                        // let llvm_ty: BasicTypeEnum = self.type_context.get_by_id(inner_ty)?.llvm_type.try_into().ok()?;
                        // return Some(builder.build_load(llvm_ty, ptr_val.into_pointer_value(), "deref").unwrap().into());
                        return Some(ptr_val)
                    }
                    UnaryOperator::Neg => {}
                }

                let operand_val = self.compile_expression(builder, globals, locals, &uop.operand)?;

                let result = {
                    let ops = &self.type_context.get_by_id(uop.operand.ty)?.ops;
                    ops.neg.as_ref().map(|(_, maker)| maker(builder, operand_val))
                };
                result
            }

            TypedExprNode::Cast(cast) => {
                let val = self.compile_expression(builder, globals, locals, &cast.expr)?;
                let ops = &self.type_context.get_by_id(cast.expr.ty)?.ops;
                ops.conversion_ops.get(&cast.target).map(|maker| maker(builder, val))
            }
            TypedExprNode::CallOp(call) => {
                let caller_val = self.compile_expression(builder, globals, locals, &call.caller)?;

                // Compile each argument.
                let mut args: Vec<BasicMetadataValueEnum<'static>> = Vec::new();
                for arg in &call.arguments {
                    let val = self.compile_expression(builder, globals, locals, arg)?;
                    let basic: BasicValueEnum<'static> = val.try_into().ok()?;
                    args.push(basic.into());
                }

                // Direct function call (the common case).
                // Indirect / function-pointer calls are a TODO.
                if let AnyValueEnum::FunctionValue(fn_val) = caller_val {
                    let call_site = builder.build_call(fn_val, &args, "call").unwrap();

                    // Determine whether the return type is void.
                    let ret_is_void = matches!(
                        self.type_context.get_by_id(e.ty).map(|i| &i.kind),
                        Some(TypeKind::None)
                    );

                    if ret_is_void {
                        None
                    } else {
                        call_site.try_as_basic_value().basic().map(|v| v.into())
                    }
                } else {
                    None
                }
            }
        }
    }
}
