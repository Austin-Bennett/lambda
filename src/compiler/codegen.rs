use crate::compiler::Compiler;
use crate::typed_ast::ast::items::function::Function;
use crate::lexer::literal::LiteralValue;
use crate::typed_ast::ast::statements::expression::{BinaryOperator, TypedExpr, TypedExprNode, UnaryOperator};
use crate::typed_ast::ast::statements::TypedStatement;
use crate::typed_ast::typing::ty::TypeKind;
use inkwell::builder::Builder;
use inkwell::types::{AnyTypeEnum, BasicMetadataTypeEnum, BasicTypeEnum};
use inkwell::values::{AnyValueEnum, BasicMetadataValueEnum, BasicValueEnum, FunctionValue, PointerValue};
use std::collections::HashMap;
use std::mem;
use std::ops::Deref;
use inkwell::basic_block::BasicBlock;
use inkwell::module::Linkage;
use crate::typed_ast::ast::block::TypedBlock;
use crate::typed_ast::ast::statements::if_stmt::{TypedElseStatement, TypedIfSyntax};
use crate::typed_ast::typing::scope::AvailableContext;

impl Compiler {

    pub fn compile(&mut self) -> inkwell::module::Module<'static> {
        let mut builder = self.llvm_context.create_builder();
        let module = self.llvm_context.create_module("lambda_program");

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
                    _ => unreachable!(),
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
                self.compile_function(&mut builder, &globals, func, fn_val);
            }
        }


        self.typed_modules = modules;

        module
    }

    pub fn compile_function(
        &mut self,
        builder: &mut Builder<'static>,
        globals: &HashMap<String, AnyValueEnum<'static>>,
        func: &Function,
        fn_val: FunctionValue<'static>,
    ) {
        if func.is_extern && func.code.is_none() {
            return;
        }



        let entry = self.llvm_context.append_basic_block(fn_val, "entry");
        let ret_block = self.llvm_context.append_basic_block(fn_val, "return");

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

        //allocate the return variable
        let ret_ty = func.signature.ret;
        let ret_info = self.type_context.get_by_id(ret_ty).unwrap();
        let return_result_val: Result<BasicTypeEnum, _> = ret_info.llvm_type.try_into();
        let (return_alloca, ret_ty) = if let Ok(bt) = return_result_val {
            (Some(builder.build_alloca(bt, "return_var").unwrap()), Some(bt))
        } else {
            (None, None)
        };


        let code = self.compile_block(
            builder,
            fn_val,
            return_alloca,
            ret_block,
            globals,
            &mut locals,
            &func.code.as_ref().unwrap().data,
        );

        if let Some(blk) = builder.get_insert_block() {
            if blk.get_terminator().is_none() {
                builder.build_unconditional_branch(ret_block).unwrap();
            }
        }

        builder.position_at_end(entry);
        builder.build_unconditional_branch(code).unwrap();





        builder.position_at_end(ret_block);

        //todo: dropping function parameters


        match return_alloca {
            Some(v) => {
                let v = builder.build_load(ret_ty.unwrap(), v, "return_val").unwrap();
                builder.build_return(Some(& v)).unwrap();
            },
            None => {
                builder.build_return(None).unwrap();
            }
        }

        locals.pop_last_scope();
    }

    //returns the entry
    pub fn compile_block(&mut self,
                         builder: &mut Builder<'static>,
                         function: FunctionValue<'static>,
                         ret_var: Option<PointerValue>,
                         ret_block: BasicBlock,
                         globals: &HashMap<String, AnyValueEnum<'static>>,
                         locals: &mut AvailableContext<AnyValueEnum<'static>>,
                         block: &TypedBlock,
    ) -> BasicBlock<'_> {
        /*
            blocks have 3 sections
            code:
                ...code
            drop_and_return:
                ...drop variables
                jump to outer return section
            drop_and_merge:
                ...drop variables

        */
        let code_section_blk = self.llvm_context.append_basic_block(function, "code_");
        let drop_and_return_blk = self.llvm_context.append_basic_block(function, "drop_and_return_");
        let drop_and_merge_blk = self.llvm_context.append_basic_block(function, "drop_and_merge_");

        //compile code first

        builder.position_at_end(code_section_blk);

        locals.push_new_scope();

        for s in &block.code {
            self.compile_statement(
                builder,
                function,
                ret_var,
                drop_and_return_blk,
                globals,
                locals,
                s
            )
        }



        if let Some(blk) = builder.get_insert_block() {
            if blk.get_terminator().is_none() {
                //basically: if the inner scopes return early, then this will never run, but the outer functions will jump to the
                //drop_and_return block
                builder.build_unconditional_branch(drop_and_merge_blk).unwrap();
            }
        }

        builder.position_at_end(drop_and_return_blk);
        for d in &block.drops {
            self.compile_expression(
                builder,
                ret_var,
                globals,
                locals,
                d
            );
        }
        builder.build_unconditional_branch(ret_block).unwrap();

        builder.position_at_end(drop_and_merge_blk);
        //drop all variables, code continues from this merge block
        for d in &block.drops {
            self.compile_expression(
                builder,
                ret_var,
                globals,
                locals,
                d
            );
        }


        locals.pop_last_scope();

        code_section_blk
    }

    pub fn compile_statement(
        &mut self,
        builder: &mut Builder<'static>,
        function: FunctionValue<'static>,
        ret_var: Option<PointerValue>,
        ret_block: BasicBlock,
        globals: &HashMap<String, AnyValueEnum<'static>>,
        locals: &mut AvailableContext<AnyValueEnum<'static>>,
        s: &TypedStatement,
    ) {
        // Skip dead code after a terminator (e.g. injected drop calls after an explicit return).
        if builder.get_insert_block().map_or(false, |bb| bb.get_terminator().is_some()) {
            return;
        }
        match s {
            TypedStatement::Expr(e) => {
                self.compile_expression(builder, ret_var, globals, locals, e);
                
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
                    if let Some(val) = self.compile_expression(builder, ret_var, globals, locals, val_expr) {
                        if let Ok(basic_val) = BasicValueEnum::try_from(val) {
                            builder.build_store(alloca, basic_val).unwrap();
                        }
                    }
                }
            }

            TypedStatement::Return(ret) => {
                if let Some(val) = self.compile_expression(builder, ret_var, globals, locals, &ret.0) {
                    if let Ok(basic_val) = BasicValueEnum::try_from(val) {
                        ret_var.map(|v| builder.build_store(v, basic_val));
                    }
                    builder.build_unconditional_branch(ret_block).unwrap();
                }
            },

            TypedStatement::If(i4) => {

                self.compile_if_statement(
                    builder, function, ret_var, ret_block, globals, locals, i4
                );

            }
            TypedStatement::While(wh1le) => {


                let loop_start_block = self.llvm_context.append_basic_block(function, "loop_start");
                let loop_block = self.llvm_context.append_basic_block(function, "loop");
                let loop_merge_block = self.llvm_context.append_basic_block(function, "loop_merge");

                //to "terminate" the last block
                builder.build_unconditional_branch(loop_start_block).unwrap();

                builder.position_at_end(loop_start_block);

                let condition = self.compile_expression(
                    builder,
                    ret_var,
                    globals,
                    locals,
                    &wh1le.data.condition
                ).unwrap().into_int_value();

                builder.build_conditional_branch(condition, loop_block, loop_merge_block).unwrap();


                let loop_scope = self.compile_block(
                    builder,
                    function,
                    ret_var,
                    ret_block,
                    globals,
                    locals,
                    &wh1le.data.code.data
                );
                builder.build_unconditional_branch(loop_start_block).unwrap();

                builder.position_at_end(loop_block);
                builder.build_unconditional_branch(loop_scope).unwrap();


                builder.position_at_end(loop_merge_block);
            }
        }
    }


    pub fn compile_if_statement(
        &mut self,
        builder: &mut Builder<'static>,
        function: FunctionValue<'static>,
        ret_var: Option<PointerValue>,
        ret_block: BasicBlock,
        globals: &HashMap<String, AnyValueEnum<'static>>,
        locals: &mut AvailableContext<AnyValueEnum<'static>>,
        i4: &TypedIfSyntax,
    ) {
        let then_block  = self.llvm_context.append_basic_block(function, "then");
        let else_block  = self.llvm_context.append_basic_block(function, "else");
        let merge_block = self.llvm_context.append_basic_block(function, "if_merge");

        let pred = self.compile_expression(builder, ret_var, globals, locals, &i4.data.predicate)
            .unwrap()
            .into_int_value();

        builder.build_conditional_branch(pred, then_block, else_block).unwrap();


        let if_scope = self.compile_block(
            builder,
            function,
            ret_var,
            ret_block,
            globals,
            locals,
            &i4.data.code.data
        );

        if builder.get_insert_block().unwrap().get_terminator().is_none() {
            builder.build_unconditional_branch(merge_block).unwrap();
        }

        // Compile then branch.
        builder.position_at_end(then_block);

        builder.build_unconditional_branch(if_scope).unwrap();





        // Compile else branch.
        if let Some(e1se) = &i4.data.otherwise {
            match e1se.deref() {
                TypedElseStatement::If(if_statement) => {
                    self.compile_if_statement(builder, function, ret_var, ret_block, globals, locals, if_statement);
                    // compile_if_statement leaves the builder at its own merge block; branch to ours.
                    if builder.get_insert_block().unwrap().get_terminator().is_none() {
                        builder.build_unconditional_branch(merge_block).unwrap();
                    }
                }
                TypedElseStatement::Else(block) => {

                    let else_scope = self.compile_block(
                        builder,
                        function,
                        ret_var,
                        ret_block,
                        globals,
                        locals,
                        &block.data
                    );

                    if builder.get_insert_block().unwrap().get_terminator().is_none() {
                        builder.build_unconditional_branch(merge_block).unwrap();
                    }

                    builder.position_at_end(else_block);
                    builder.build_unconditional_branch(else_scope).unwrap();

                }
            }
        } else {
            builder.position_at_end(else_block);
            builder.build_unconditional_branch(merge_block).unwrap();
        }

        builder.position_at_end(merge_block);
    }

    pub fn compile_expression(
        &mut self,
        builder: &mut Builder<'static>,
        ret_var: Option<PointerValue>,
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
                let ref_ptr = self.compile_expression(builder, ret_var, globals, locals, inner).unwrap();
                let inner_ty: BasicTypeEnum<'static> = self
                    .type_context
                    .get_by_id(e.ty)
                    .and_then(|info| info.llvm_type.try_into().ok()).unwrap();
                Some(builder.build_load(inner_ty, ref_ptr.into_pointer_value(), "refread").unwrap().into())
            }

            TypedExprNode::Identifier(ident) => {
                if let Some(alloca_val) = locals.get_identifier(ident) {
                    // Local variable: load from its alloca slot.
                    let ptr = alloca_val.into_pointer_value();
                    let basic_ty: BasicTypeEnum<'static> = self
                        .type_context
                        .get_by_id(e.ty)
                        .and_then(|info| info.llvm_type.try_into().ok()).unwrap();
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

                let elem_ty: BasicTypeEnum = self.type_context.get_by_id(elem_ty).unwrap().llvm_type.try_into().unwrap();
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

                        let compiled_expr = self.compile_expression(builder, ret_var, globals, locals, expr).unwrap();
                        let basic: BasicValueEnum = compiled_expr.try_into().unwrap();

                        builder.build_store(elem_ptr, basic).unwrap();
                    }
                }

                Some(ptr.into())

            },
            TypedExprNode::Index(index) => {
                let compiled_operand = self.compile_expression(builder, ret_var, globals, locals, &index.operand).unwrap();
                let compiled_index = self.compile_expression(builder, ret_var, globals, locals, &index.index).unwrap();

                let info = self.type_context.get_by_id(index.operand.ty).unwrap();

                let maker = &info.ops.index[&index.index.ty].1;
                Some(maker(builder, compiled_operand, compiled_index))
            },

            TypedExprNode::BinaryOp(bop) => {
                if let BinaryOperator::Assign = &bop.op {
                    let rhs_val = self.compile_expression(builder, ret_var, globals, locals, &bop.rhs).unwrap();
                    match &bop.lhs.value {
                        TypedExprNode::RefRead(ref_inner) => {
                            // Store through the reference pointer.
                            let ref_ptr = self.compile_expression(builder, ret_var, globals, locals, ref_inner).unwrap();
                            let basic: BasicValueEnum<'static> = rhs_val.try_into().ok().unwrap();
                            builder.build_store(ref_ptr.into_pointer_value(), basic).unwrap();
                        }
                        TypedExprNode::Identifier(name) => {
                            let alloca = locals.get_identifier(name).unwrap().into_pointer_value();
                            let ops = &self.type_context.get_by_id(bop.lhs.ty).unwrap().ops;
                            let maker = ops.assign.get(&bop.rhs.ty).unwrap();
                            maker(builder, alloca, rhs_val);
                        }
                        TypedExprNode::UnaryOp(uop) if matches!(uop.op, UnaryOperator::Dereference) => {
                            // Compile the pointer operand to get the address, then store through it.
                            let ptr_val = self.compile_expression(builder, ret_var, globals, locals, &uop.operand).unwrap();
                            let basic: BasicValueEnum<'static> = rhs_val.try_into().ok().unwrap();
                            builder.build_store(ptr_val.into_pointer_value(), basic).unwrap();
                        }
                        TypedExprNode::MemberAccess(ma) => {
                            // Get a pointer to the struct field, then store through it.
                            let field_ptr = match &ma.object.value {
                                TypedExprNode::Identifier(name) => {
                                    // Direct struct variable: alloca is the base pointer
                                    let alloca = locals.get_identifier(name).unwrap().into_pointer_value();
                                    let TypeKind::Struct(sid) = self.type_context.get_by_id(ma.object.ty).unwrap().kind.clone() else { return None; };
                                    let struct_ty = self.type_context.structs[sid as usize].llvm_struct;
                                    builder.build_struct_gep(struct_ty, alloca, ma.member_index as u32, "field_ptr").ok().unwrap()
                                }
                                TypedExprNode::RefRead(inner) => {
                                    // Reference to struct: inner compiles to the struct pointer
                                    let struct_ptr = self.compile_expression(builder, ret_var, globals, locals, inner).unwrap().into_pointer_value();
                                    let TypeKind::Struct(sid) = self.type_context.get_by_id(ma.object.ty).unwrap().kind.clone() else { return None; };
                                    let struct_ty = self.type_context.structs[sid as usize].llvm_struct;
                                    builder.build_struct_gep(struct_ty, struct_ptr, ma.member_index as u32, "field_ptr").ok().unwrap()
                                }
                                _ => return None,
                            };
                            let basic: BasicValueEnum<'static> = rhs_val.try_into().ok().unwrap();
                            builder.build_store(field_ptr, basic).unwrap();
                        }
                        _ => return None,
                    }
                    return Some(rhs_val);
                }

                let lhs_val = self.compile_expression(builder, ret_var, globals, locals, &bop.lhs).unwrap();
                let rhs_val = self.compile_expression(builder, ret_var, globals, locals, &bop.rhs).unwrap();

                // Look up the operator callback. All borrows of type_context are released
                // before the next compile_expression call, so no conflict.
                let result = {
                    let ops = &self.type_context.get_by_id(bop.lhs.ty).unwrap().ops;
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
                        let ptr_val = self.compile_expression(builder, ret_var, globals, locals, &uop.operand).unwrap();
                        //return the pointer
                        // let inner_ty = match self.type_context.get_by_id(uop.operand.ty).unwrap().kind.clone() {
                        //     TypeKind::Pointer(id) => id,
                        //     _ => return None,
                        // };
                        // let llvm_ty: BasicTypeEnum = self.type_context.get_by_id(inner_ty).unwrap().llvm_type.try_into().ok().unwrap();
                        // return Some(builder.build_load(llvm_ty, ptr_val.into_pointer_value(), "deref").unwrap().into());
                        return Some(ptr_val)
                    }
                    UnaryOperator::Neg => {}
                }

                let operand_val = self.compile_expression(builder, ret_var, globals, locals, &uop.operand).unwrap();

                let result = {
                    let ops = &self.type_context.get_by_id(uop.operand.ty).unwrap().ops;
                    ops.neg.as_ref().map(|(_, maker)| maker(builder, operand_val))
                };
                result
            }

            TypedExprNode::Cast(cast) => {
                let val = self.compile_expression(builder, ret_var, globals, locals, &cast.expr).unwrap();
                let ops = &self.type_context.get_by_id(cast.expr.ty).unwrap().ops;
                ops.conversion_ops.get(&cast.target).map(|maker| maker(builder, val))
            }
            TypedExprNode::CallOp(call) => {
                let caller_val = self.compile_expression(builder, ret_var, globals, locals, &call.caller).unwrap();

                // Compile each argument.
                let mut args: Vec<BasicMetadataValueEnum<'static>> = Vec::new();
                for arg in &call.arguments {
                    let val = self.compile_expression(builder, ret_var, globals, locals, arg).unwrap();
                    let basic: BasicValueEnum<'static> = val.try_into().ok().unwrap();
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
                    let val = self.compile_expression(builder, ret_var, globals, locals, field_expr).unwrap();
                    let basic: BasicValueEnum<'static> = val.try_into().ok().unwrap();
                    agg = builder.build_insert_value(agg.into_struct_value(), basic, i as u32, "sf")
                        .unwrap()
                        .into_struct_value()
                        .into();
                }
                Some(agg.into())
            }
            TypedExprNode::MemberAccess(ma) => {
                // object is always a plain struct value here (reference already unwrapped via RefRead)
                let obj_val = self.compile_expression(builder, ret_var, globals, locals, &ma.object).unwrap();
                let sv = BasicValueEnum::try_from(obj_val).ok().unwrap().into_struct_value();
                Some(builder.build_extract_value(sv, ma.member_index as u32, "member").unwrap().into())
            }
            TypedExprNode::BoundMethod(bm) => {
                globals.get(&bm.mangled_name).copied()
            }

            TypedExprNode::CompilerIntrinsic(ci) => {
                let maker = {
                    let m = self.intrinsics.get(&ci.name).unwrap();
                    m.clone()
                };

                let compiled_args: Vec<Option<AnyValueEnum<'static>>> = ci.args.iter()
                    .map(|arg| self.compile_expression(builder, ret_var, globals, locals, arg))
                    .collect();

                maker(builder, globals, locals, &ci.args, compiled_args)
            }
        }
    }
}
