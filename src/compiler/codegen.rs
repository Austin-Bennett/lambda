use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::ast::items::function::Function;
use crate::lexer::literal::LiteralValue;
use crate::typed_ast::ast::statements::expression::{BinaryOperator, TypedExpr, TypedExprNode, UnaryOperator};
use crate::typed_ast::ast::statements::TypedStatement;
use crate::typed_ast::typing::ty::TypeKind;
use inkwell::builder::Builder;
use inkwell::types::{AnyTypeEnum, BasicMetadataTypeEnum, BasicTypeEnum, StructType};
use inkwell::values::{AnyValueEnum, BasicMetadataValueEnum, BasicValueEnum, FunctionValue};
use std::collections::HashMap;
use std::mem;
use std::ops::Deref;
use std::process::abort;
use inkwell::basic_block::BasicBlock;
use inkwell::module::Linkage;
use crate::typed_ast::ast::statements::if_stmt::{TypedElseStatement, TypedIfStatement, TypedIfSyntax};
use crate::typed_ast::typing::scope::AvailableContext;

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

        let mut locals = AvailableContext::new();
        locals.push_new_scope();

        // Allocate stack slots for each parameter and store the incoming value.
        // All locals in the `locals` map are alloca pointers; we always load on read.
        for (name, param) in func.params.iter().zip(fn_val.get_param_iter()) {
            let alloca = builder.build_alloca(param.get_type(), name).unwrap();
            builder.build_store(alloca, param).unwrap();
            locals.declare_identifier_in_scope(name.clone(), alloca.into());
        }

        if let Some(code) = &func.code {
            for statement in &code.data {
                self.compile_statement(builder, module, fn_val, globals, &mut locals, statement);
            }
        }


        // If the current block still has no terminator (e.g. a void function with
        // no explicit return statement), emit an implicit `ret void`.
        if let Some(current_block) = builder.get_insert_block() {
            if current_block.get_terminator().is_none() {
                let ret_info = self.type_context.get_by_id(func.signature.ret).unwrap();

                match &ret_info.kind {
                    TypeKind::None => {
                        // It's a void function, just emit ret void
                        builder.build_return(None).unwrap();
                    }
                    _ => {
                        builder.build_unreachable().unwrap();
                    }
                }
            }
        }
        locals.pop_last_scope();
    }



    pub fn compile_statement(
        &mut self,
        builder: &mut Builder<'static>,
        module: &mut inkwell::module::Module<'static>,
        function: FunctionValue<'static>,
        globals: &HashMap<String, AnyValueEnum<'static>>,
        locals: &mut AvailableContext<AnyValueEnum<'static>>,
        s: &TypedStatement,
    ) {
        match s {
            TypedStatement::Expr(e) => {
                self.compile_expression(builder, module, globals, locals, e);
            }

            TypedStatement::VarDecl(vd) => {
                // Resolve the LLVM type before taking any mutable borrow.
                let basic_ty: BasicTypeEnum<'static> = self
                    .type_context
                    .get_by_id(vd.ty)
                    .and_then(|info| info.llvm_type.try_into().ok())
                    .unwrap();

                let alloca = builder.build_alloca(basic_ty, &vd.name).unwrap();
                locals.declare_identifier_in_scope(vd.name.clone(), alloca.into());

                if let Some(val_expr) = &vd.val {
                    if let Some(val) = self.compile_expression(builder, module, globals, locals, val_expr) {
                        if let Ok(basic_val) = BasicValueEnum::try_from(val) {
                            builder.build_store(alloca, basic_val).unwrap();
                        }
                    }
                }
            }

            TypedStatement::Return(ret) => {
                if let Some(val) = self.compile_expression(builder, module, globals, locals, &ret.0) {
                    if let Ok(basic_val) = BasicValueEnum::try_from(val) {
                        builder.build_return(Some(&basic_val)).unwrap();
                        return;
                    }
                }
                builder.build_return(None).unwrap();
            },

            TypedStatement::If(i4) => {

                self.compile_if_statement(
                    builder, module, function, globals, locals, i4
                );

            }
            TypedStatement::While(wh1le) => {


                let loop_start_block = self.llvm_context.append_basic_block(function, "loop_start");
                let loop_block = self.llvm_context.append_basic_block(function, "loop");
                let loop_done_block = self.llvm_context.append_basic_block(function, "loop_end");

                //to "terminate" the last block
                builder.build_unconditional_branch(loop_start_block).unwrap();

                builder.position_at_end(loop_start_block);

                let condition = self.compile_expression(
                    builder,
                    module,
                    globals,
                    locals,
                    &wh1le.data.condition
                ).unwrap().into_int_value();

                builder.build_conditional_branch(condition, loop_block, loop_done_block).unwrap();

                builder.position_at_end(loop_block);

                locals.push_new_scope();
                for s in &wh1le.data.code.data {
                    self.compile_statement(builder, module, function, globals, locals, s);
                }
                locals.pop_last_scope();

                builder.build_unconditional_branch(loop_start_block).unwrap();

                builder.position_at_end(loop_done_block);
            }
        }
    }

    pub fn compile_if_statement(
        &mut self,
        builder: &mut Builder<'static>,
        module: &mut inkwell::module::Module<'static>,
        function: FunctionValue<'static>,
        globals: &HashMap<String, AnyValueEnum<'static>>,
        locals: &mut AvailableContext<AnyValueEnum<'static>>,
        i4: &TypedIfSyntax,
    ) {
        let then_block  = self.llvm_context.append_basic_block(function, "then");
        let else_block  = self.llvm_context.append_basic_block(function, "else");
        let merge_block = self.llvm_context.append_basic_block(function, "if_merge");

        let pred = self.compile_expression(builder, module, globals, locals, &i4.data.predicate)
            .unwrap()
            .into_int_value();

        builder.build_conditional_branch(pred, then_block, else_block).unwrap();

        // Compile then branch.
        builder.position_at_end(then_block);
        locals.push_new_scope();
        for s in &i4.data.code.data {
            self.compile_statement(builder, module, function, globals, locals, s);
        }
        locals.pop_last_scope();
        if builder.get_insert_block().unwrap().get_terminator().is_none() {
            builder.build_unconditional_branch(merge_block).unwrap();
        }

        // Compile else branch.
        builder.position_at_end(else_block);
        if let Some(e1se) = &i4.data.otherwise {
            match e1se.deref() {
                TypedElseStatement::If(if_statement) => {
                    self.compile_if_statement(builder, module, function, globals, locals, if_statement);
                    // compile_if_statement leaves the builder at its own merge block; branch to ours.
                    if builder.get_insert_block().unwrap().get_terminator().is_none() {
                        builder.build_unconditional_branch(merge_block).unwrap();
                    }
                }
                TypedElseStatement::Else(block) => {
                    locals.push_new_scope();
                    for s in &block.data {
                        self.compile_statement(builder, module, function, globals, locals, s);
                    }
                    locals.pop_last_scope();
                    if builder.get_insert_block().unwrap().get_terminator().is_none() {
                        builder.build_unconditional_branch(merge_block).unwrap();
                    }
                }
            }
        } else {
            builder.build_unconditional_branch(merge_block).unwrap();
        }

        builder.position_at_end(merge_block);
    }

    pub fn compile_expression(
        &mut self,
        builder: &mut Builder<'static>,
        module: &mut inkwell::module::Module<'static>,
        globals: &HashMap<String, AnyValueEnum<'static>>,
        locals: &mut AvailableContext<AnyValueEnum<'static>>,
        e: &TypedExpr,
    ) -> Option<AnyValueEnum<'static>> {
        match &e.value {
            TypedExprNode::Literal(lit) => {
                let ctx = self.llvm_context;
                match lit {
                    LiteralValue::Bool(b) => {
                        Some(ctx.custom_width_int_type(1).const_int(*b as u64, false).into())
                    }
                    LiteralValue::Integer(_) | LiteralValue::Float(_) => {
                        let literal_key = match lit {
                            LiteralValue::Integer(_) => self.type_context.int_literal,
                            LiteralValue::Float(_)   => self.type_context.float_literal,
                            LiteralValue::Bool(_)    => unreachable!(),
                        };
                        self.type_context.get_by_id(e.ty)
                            .and_then(|info| info.ops.from_literal.get(&literal_key))
                            .map(|maker| maker(ctx, lit))
                    }
                }
            }

            TypedExprNode::RefRead(inner) => {
                // Evaluate the inner expr to get the reference pointer, then load through it.
                let ref_ptr = self.compile_expression(builder, module, globals, locals, inner)?;
                let inner_ty: BasicTypeEnum<'static> = self
                    .type_context
                    .get_by_id(e.ty)
                    .and_then(|info| info.llvm_type.try_into().ok())?;
                Some(builder.build_load(inner_ty, ref_ptr.into_pointer_value(), "refread").unwrap().into())
            }

            TypedExprNode::Identifier(ident) => {
                if let Some(alloca_val) = locals.get_identifier(ident) {
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

                let TypeKind::Array { ty: elem_ty, .. } = ty.kind else { unreachable!() };

                let elem_ty: BasicTypeEnum = self.type_context.get_by_id(elem_ty)?.llvm_type.try_into().unwrap();
                let usize_info = self.type_context.get_by_id(self.type_context.usize).unwrap().llvm_type.into_int_type();

                let ptr = builder.build_array_alloca(elem_ty,
                                                     usize_info.const_int(ray.len() as u64, false),
                                                     "array_ptr"
                ).unwrap();

                for (i, expr) in ray.iter().enumerate() {
                    let ty = self.type_context.get_by_id(expr.ty).unwrap();
                    let basic: BasicTypeEnum = ty.llvm_type.try_into().unwrap();
                    unsafe {
                        let elem_ptr = builder.build_gep(basic, ptr,
                        &[
                            usize_info.const_int(i as u64, false),
                        ], "elem")
                            .unwrap();

                        let compiled_expr = self.compile_expression(builder, module, globals, locals, expr).unwrap();
                        let basic: BasicValueEnum = compiled_expr.try_into().unwrap();

                        builder.build_store(elem_ptr, basic).unwrap();
                    }
                }

                Some(ptr.into())

            },
            TypedExprNode::Index(index) => {
                let compiled_operand = self.compile_expression(builder, module, globals, locals, &index.operand)?;
                let compiled_index = self.compile_expression(builder, module, globals, locals, &index.index)?;

                let info = self.type_context.get_by_id(index.operand.ty).unwrap();

                let maker = &info.ops.index[&index.index.ty].1;
                Some(maker(builder, compiled_operand, compiled_index))
            },

            TypedExprNode::BinaryOp(bop) => {
                if let BinaryOperator::Assign = &bop.op {
                    let rhs_val = self.compile_expression(builder, module, globals, locals, &bop.rhs)?;
                    match &bop.lhs.value {
                        TypedExprNode::MemberAccess(ma) => {
                            let obj_val = self.compile_expression(
                                builder, module, globals, locals, &ma.object
                            ).unwrap();

                            let (sv, s_ty) = if let AnyValueEnum::PointerValue(ptr) = obj_val {
                                let TypeKind::Reference(inner_id) = self.type_context.get_by_id(ma.object.ty).unwrap().kind.clone() else {
                                    return None;
                                };
                                (ptr, inner_id)
                            } else {
                                self.emit_compile_message(CompileMessage::new(
                                    e.smap.clone(),
                                    "cannot assign to a rvalue!".to_string(),
                                    CompileMessageType::Error
                                ));
                                return None;
                            };
                            let s_ty = self.type_context.get_by_id(ma.object.ty).unwrap().llvm_type
                                .into_struct_type();

                            let ref_ptr = builder.build_struct_gep(s_ty, sv, ma.member_index as u32,
                                                                   format!("s_{}_ref", ma.member_index).as_ref()).unwrap();

                            let basic: BasicValueEnum = rhs_val.try_into().ok()?;

                            builder.build_store(ref_ptr, basic).unwrap();

                        }
                        TypedExprNode::RefRead(ref_inner) => {
                            // Store through the reference pointer.
                            let ref_ptr = self.compile_expression(builder, module, globals, locals, ref_inner)?;
                            let basic: BasicValueEnum<'static> = rhs_val.try_into().ok()?;
                            builder.build_store(ref_ptr.into_pointer_value(), basic).unwrap();
                        }
                        TypedExprNode::Identifier(name) => {
                            let alloca = locals.get_identifier(name)?.into_pointer_value();
                            let ops = &self.type_context.get_by_id(bop.lhs.ty)?.ops;
                            let maker = ops.assign.get(&bop.rhs.ty)?;
                            maker(builder, alloca, rhs_val);
                        }
                        TypedExprNode::UnaryOp(uop) if matches!(uop.op, UnaryOperator::Dereference) => {
                            // Compile the pointer operand to get the address, then store through it.
                            let ptr_val = self.compile_expression(builder, module, globals, locals, &uop.operand)?;
                            let basic: BasicValueEnum<'static> = rhs_val.try_into().ok()?;
                            builder.build_store(ptr_val.into_pointer_value(), basic).unwrap();
                        }
                        _ => return None,
                    }
                    return Some(rhs_val);
                }

                let lhs_val = self.compile_expression(builder, module, globals, locals, &bop.lhs)?;
                let rhs_val = self.compile_expression(builder, module, globals, locals, &bop.rhs)?;

                // Look up the operator callback. All borrows of type_context are released
                // before the next compile_expression call, so no conflict.
                let result = {
                    let ops = &self.type_context.get_by_id(bop.lhs.ty)?.ops;
                    match &bop.op {
                        BinaryOperator::Add => ops.add.get(&bop.rhs.ty).map(|(_, m)| m(builder, lhs_val, rhs_val)),
                        BinaryOperator::Sub => ops.sub.get(&bop.rhs.ty).map(|(_, m)| m(builder, lhs_val, rhs_val)),
                        BinaryOperator::Mul => ops.mul.get(&bop.rhs.ty).map(|(_, m)| m(builder, lhs_val, rhs_val)),
                        BinaryOperator::Div => ops.div.get(&bop.rhs.ty).map(|(_, m)| m(builder, lhs_val, rhs_val)),
                        BinaryOperator::Assign => unreachable!("assign handled above"),
                        BinaryOperator::Eq => ops.cmp.get(&bop.rhs.ty).map(|c| (c.eq)(builder, lhs_val, rhs_val)),
                        BinaryOperator::Ne => ops.cmp.get(&bop.rhs.ty).map(|c| (c.ne)(builder, lhs_val, rhs_val)),
                        BinaryOperator::Lt => ops.cmp.get(&bop.rhs.ty).map(|c| (c.lt)(builder, lhs_val, rhs_val)),
                        BinaryOperator::Gt => ops.cmp.get(&bop.rhs.ty).map(|c| (c.gt)(builder, lhs_val, rhs_val)),
                        BinaryOperator::Le => ops.cmp.get(&bop.rhs.ty).map(|c| (c.le)(builder, lhs_val, rhs_val)),
                        BinaryOperator::Ge => ops.cmp.get(&bop.rhs.ty).map(|c| (c.ge)(builder, lhs_val, rhs_val)),
                    }
                };
                result
            }

            TypedExprNode::UnaryOp(uop) => {
                match &uop.op {
                    UnaryOperator::Reference => {
                        let TypedExprNode::Identifier(name) = &uop.operand.value else { return None; };
                        if let Some(alloca) = locals.get_identifier(name) {
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
                        let ptr_val = self.compile_expression(builder, module, globals, locals, &uop.operand)?;
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

                let operand_val = self.compile_expression(builder, module, globals, locals, &uop.operand)?;

                let result = {
                    let ops = &self.type_context.get_by_id(uop.operand.ty)?.ops;
                    ops.neg.as_ref().map(|(_, maker)| maker(builder, operand_val))
                };
                result
            }

            TypedExprNode::Cast(cast) => {
                let val = self.compile_expression(builder, module, globals, locals, &cast.expr)?;
                let ops = &self.type_context.get_by_id(cast.expr.ty)?.ops;
                ops.conversion_ops.get(&cast.target).map(|maker| maker(builder, val))
            }
            TypedExprNode::CallOp(call) => {
                let caller_val = self.compile_expression(builder, module, globals, locals, &call.caller)?;

                // Compile each argument.
                let mut args: Vec<BasicMetadataValueEnum<'static>> = Vec::new();
                for arg in &call.arguments {
                    let val = self.compile_expression(builder, module, globals, locals, arg)?;
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
            TypedExprNode::StructConstruct(sc) => {
                let struct_ty = self.type_context.structs[sc.struct_id as usize].llvm_struct;
                let mut agg: BasicValueEnum<'static> = struct_ty.get_undef().into();
                for (i, field_expr) in sc.fields.iter().enumerate() {
                    let val = self.compile_expression(builder, module, globals, locals, field_expr)?;
                    let basic: BasicValueEnum<'static> = val.try_into().ok()?;
                    agg = builder.build_insert_value(agg.into_struct_value(), basic, i as u32, "sf")
                        .unwrap()
                        .into_struct_value()
                        .into();
                }
                Some(agg.into())
            }
            TypedExprNode::MemberAccess(ma) => {
                let obj_val = self.compile_expression(builder, module, globals, locals, &ma.object)?;
                // If the object is a reference (pointer), load the struct through it first
                let sv = if let AnyValueEnum::PointerValue(ptr) = obj_val {
                    let TypeKind::Reference(inner_id) = self.type_context.get_by_id(ma.object.ty).unwrap().kind.clone() else {
                        return None;
                    };
                    let TypeKind::Struct(sid) = self.type_context.get_by_id(inner_id).unwrap().kind.clone() else {
                        return None;
                    };
                    let struct_ty = self.type_context.structs[sid as usize].llvm_struct;
                    builder.build_load(struct_ty, ptr, "deref_struct").unwrap().into_struct_value()
                } else {
                    BasicValueEnum::try_from(obj_val).ok()?.into_struct_value()
                };
                Some(builder.build_extract_value(sv, ma.member_index as u32, "member").unwrap().into())
            }
            TypedExprNode::BoundMethod(bm) => {
                globals.get(&bm.mangled_name).copied()
            }
            TypedExprNode::CompilerIntrinsic(ci) => {
                let maker = {
                    let m = self.intrinsics.get(&ci.name)?;
                    m.clone()
                };

                let compiled_args: Vec<Option<AnyValueEnum<'static>>> = ci.args.iter()
                    .map(|arg| self.compile_expression(builder, module, globals, locals, arg))
                    .collect();

                maker(builder, module, globals, locals, &ci.args, compiled_args)
            }
        }
    }
}
