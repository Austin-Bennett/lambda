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

                // Always emit allocas in the function entry block so that loop bodies
                // don't adjust rsp on every iteration, which would overflow the stack.
                let current_block = builder.get_insert_block().unwrap();
                let entry_block = function.get_first_basic_block().unwrap();
                builder.position_at_end(entry_block);
                let alloca = builder.build_alloca(basic_ty, &vd.name).unwrap();
                builder.position_at_end(current_block);

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
                let val = self.compile_expression(builder, ret_var, globals, locals, &ret.0);
                if let Some(val) = val {
                    if let Ok(basic_val) = BasicValueEnum::try_from(val) {
                        ret_var.map(|v| builder.build_store(v, basic_val));
                    }
                }
                builder.build_unconditional_branch(ret_block).unwrap();
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
                    // else_block is the entry point when the outer condition is false;
                    // the recursive if-statement's predicate must be emitted there.
                    builder.position_at_end(else_block);
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
                // --- Assign ---
                if let BinaryOperator::Assign = &bop.op {
                    let rhs_val = self.compile_expression(builder, ret_var, globals, locals, &bop.rhs).unwrap();
                    match &bop.lhs.value {
                        TypedExprNode::RefRead(ref_inner) => {
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
                            let ptr_val = self.compile_expression(builder, ret_var, globals, locals, &uop.operand).unwrap();
                            let basic: BasicValueEnum<'static> = rhs_val.try_into().ok().unwrap();
                            builder.build_store(ptr_val.into_pointer_value(), basic).unwrap();
                        }
                        TypedExprNode::MemberAccess(ma) => {
                            let field_ptr = match &ma.object.value {
                                TypedExprNode::Identifier(name) => {
                                    let alloca = locals.get_identifier(name).unwrap().into_pointer_value();
                                    let TypeKind::Struct(sid) = self.type_context.get_by_id(ma.object.ty).unwrap().kind.clone() else { return None; };
                                    let struct_ty = self.type_context.structs[sid as usize].llvm_struct;
                                    builder.build_struct_gep(struct_ty, alloca, ma.member_index as u32, "field_ptr").ok().unwrap()
                                }
                                TypedExprNode::RefRead(inner) => {
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

                // --- Short-circuit && ---
                if let BinaryOperator::BoolAnd = &bop.op {
                    let function = builder.get_insert_block().unwrap().get_parent().unwrap();
                    let lhs_val = self.compile_expression(builder, ret_var, globals, locals, &bop.lhs)?.into_int_value();
                    let lhs_block = builder.get_insert_block().unwrap();
                    let rhs_block   = self.llvm_context.append_basic_block(function, "and_rhs");
                    let merge_block = self.llvm_context.append_basic_block(function, "and_merge");
                    builder.build_conditional_branch(lhs_val, rhs_block, merge_block).unwrap();

                    builder.position_at_end(rhs_block);
                    let rhs_val = self.compile_expression(builder, ret_var, globals, locals, &bop.rhs)?.into_int_value();
                    let rhs_final = builder.get_insert_block().unwrap();
                    builder.build_unconditional_branch(merge_block).unwrap();

                    builder.position_at_end(merge_block);
                    let bool_ty = self.llvm_context.custom_width_int_type(1);
                    let false_val = bool_ty.const_int(0, false);
                    let phi = builder.build_phi(bool_ty, "and_result").unwrap();
                    phi.add_incoming(&[(&false_val, lhs_block), (&rhs_val, rhs_final)]);
                    return Some(phi.as_basic_value().into());
                }

                // --- Short-circuit || ---
                if let BinaryOperator::BoolOr = &bop.op {
                    let function = builder.get_insert_block().unwrap().get_parent().unwrap();
                    let lhs_val = self.compile_expression(builder, ret_var, globals, locals, &bop.lhs)?.into_int_value();
                    let lhs_block = builder.get_insert_block().unwrap();
                    let rhs_block   = self.llvm_context.append_basic_block(function, "or_rhs");
                    let merge_block = self.llvm_context.append_basic_block(function, "or_merge");
                    builder.build_conditional_branch(lhs_val, merge_block, rhs_block).unwrap();

                    builder.position_at_end(rhs_block);
                    let rhs_val = self.compile_expression(builder, ret_var, globals, locals, &bop.rhs)?.into_int_value();
                    let rhs_final = builder.get_insert_block().unwrap();
                    builder.build_unconditional_branch(merge_block).unwrap();

                    builder.position_at_end(merge_block);
                    let bool_ty = self.llvm_context.custom_width_int_type(1);
                    let true_val = bool_ty.const_int(1, false);
                    let phi = builder.build_phi(bool_ty, "or_result").unwrap();
                    phi.add_incoming(&[(&true_val, lhs_block), (&rhs_val, rhs_final)]);
                    return Some(phi.as_basic_value().into());
                }

                // --- Compound assignment (a op= b) ---
                let compound_inner_op: Option<fn(&inkwell::builder::Builder<'static>,
                    AnyValueEnum<'static>, AnyValueEnum<'static>,
                    &crate::typed_ast::typing::operator::OperatorOverloads,
                    crate::typed_ast::typing::ty::TypeId) -> Option<AnyValueEnum<'static>>>
                    = match &bop.op {
                    BinaryOperator::AddAssign    => Some(|b,l,r,ops,rty| ops.add.get(&rty).map(|(_,m)| m(b,l,r))),
                    BinaryOperator::SubAssign    => Some(|b,l,r,ops,rty| ops.sub.get(&rty).map(|(_,m)| m(b,l,r))),
                    BinaryOperator::MulAssign    => Some(|b,l,r,ops,rty| ops.mul.get(&rty).map(|(_,m)| m(b,l,r))),
                    BinaryOperator::DivAssign    => Some(|b,l,r,ops,rty| ops.div.get(&rty).map(|(_,m)| m(b,l,r))),
                    BinaryOperator::BitAndAssign => Some(|b,l,r,ops,rty| ops.bit_and.get(&rty).map(|(_,m)| m(b,l,r))),
                    BinaryOperator::BitOrAssign  => Some(|b,l,r,ops,rty| ops.bit_or.get(&rty).map(|(_,m)| m(b,l,r))),
                    BinaryOperator::BitXorAssign => Some(|b,l,r,ops,rty| ops.bit_xor.get(&rty).map(|(_,m)| m(b,l,r))),
                    BinaryOperator::ShlAssign    => Some(|b,l,r,ops,rty| ops.shl.get(&rty).map(|(_,m)| m(b,l,r))),
                    BinaryOperator::ShrAssign    => Some(|b,l,r,ops,rty| ops.shr.get(&rty).map(|(_,m)| m(b,l,r))),
                    _ => None,
                };
                if let Some(inner_fn) = compound_inner_op {
                    let TypedExprNode::Identifier(name) = &bop.lhs.value else { return None; };
                    let alloca = locals.get_identifier(name).unwrap().into_pointer_value();
                    let lhs_ty: BasicTypeEnum<'static> = self.type_context
                        .get_by_id(bop.lhs.ty).unwrap().llvm_type.try_into().ok().unwrap();
                    let current_val: AnyValueEnum<'static> = builder.build_load(lhs_ty, alloca, "ca_load").unwrap().into();
                    let rhs_val = self.compile_expression(builder, ret_var, globals, locals, &bop.rhs)?;
                    let new_val = {
                        let ops = &self.type_context.get_by_id(bop.lhs.ty).unwrap().ops;
                        inner_fn(builder, current_val, rhs_val, ops, bop.rhs.ty)?
                    };
                    let basic_new: BasicValueEnum<'static> = new_val.try_into().ok().unwrap();
                    builder.build_store(alloca, basic_new).unwrap();
                    return Some(new_val);
                }

                // --- Compound &&= / ||= (short-circuit) ---
                if matches!(bop.op, BinaryOperator::BoolAndAssign | BinaryOperator::BoolOrAssign) {
                    let is_and = matches!(bop.op, BinaryOperator::BoolAndAssign);
                    let function = builder.get_insert_block().unwrap().get_parent().unwrap();
                    let TypedExprNode::Identifier(name) = &bop.lhs.value else { return None; };
                    let alloca = locals.get_identifier(name).unwrap().into_pointer_value();
                    let bool_ty = self.llvm_context.custom_width_int_type(1);
                    let lhs_val = builder.build_load(bool_ty, alloca, "ca_load").unwrap().into_int_value();
                    let lhs_block   = builder.get_insert_block().unwrap();
                    let rhs_block   = self.llvm_context.append_basic_block(function, "bca_rhs");
                    let merge_block = self.llvm_context.append_basic_block(function, "bca_merge");
                    if is_and {
                        builder.build_conditional_branch(lhs_val, rhs_block, merge_block).unwrap();
                    } else {
                        builder.build_conditional_branch(lhs_val, merge_block, rhs_block).unwrap();
                    }
                    builder.position_at_end(rhs_block);
                    let rhs_val = self.compile_expression(builder, ret_var, globals, locals, &bop.rhs)?.into_int_value();
                    let rhs_final = builder.get_insert_block().unwrap();
                    builder.build_unconditional_branch(merge_block).unwrap();
                    builder.position_at_end(merge_block);
                    let short_val = bool_ty.const_int(if is_and { 0 } else { 1 }, false);
                    let phi = builder.build_phi(bool_ty, "bca_result").unwrap();
                    phi.add_incoming(&[(&short_val, lhs_block), (&rhs_val, rhs_final)]);
                    let result_val: BasicValueEnum = phi.as_basic_value();
                    builder.build_store(alloca, result_val).unwrap();
                    return Some(result_val.into());
                }

                // --- Generic binary ops (both sides evaluated eagerly) ---
                let lhs_val = self.compile_expression(builder, ret_var, globals, locals, &bop.lhs).unwrap();
                let rhs_val = self.compile_expression(builder, ret_var, globals, locals, &bop.rhs).unwrap();

                let result = {
                    let ops = &self.type_context.get_by_id(bop.lhs.ty).unwrap().ops;
                    match &bop.op {
                        BinaryOperator::Add => ops.add.get(&bop.rhs.ty).map(|(_, m)| m(builder, lhs_val, rhs_val)),
                        BinaryOperator::Sub => ops.sub.get(&bop.rhs.ty).map(|(_, m)| m(builder, lhs_val, rhs_val)),
                        BinaryOperator::Mul => ops.mul.get(&bop.rhs.ty).map(|(_, m)| m(builder, lhs_val, rhs_val)),
                        BinaryOperator::Div => ops.div.get(&bop.rhs.ty).map(|(_, m)| m(builder, lhs_val, rhs_val)),
                        BinaryOperator::BitAnd => ops.bit_and.get(&bop.rhs.ty).map(|(_, m)| m(builder, lhs_val, rhs_val)),
                        BinaryOperator::BitOr  => ops.bit_or.get(&bop.rhs.ty).map(|(_, m)| m(builder, lhs_val, rhs_val)),
                        BinaryOperator::BitXor => ops.bit_xor.get(&bop.rhs.ty).map(|(_, m)| m(builder, lhs_val, rhs_val)),
                        BinaryOperator::Shl    => ops.shl.get(&bop.rhs.ty).map(|(_, m)| m(builder, lhs_val, rhs_val)),
                        BinaryOperator::Shr    => ops.shr.get(&bop.rhs.ty).map(|(_, m)| m(builder, lhs_val, rhs_val)),
                        BinaryOperator::Assign => unreachable!("assign handled above"),
                        BinaryOperator::BoolAnd | BinaryOperator::BoolOr => unreachable!("handled above"),
                        BinaryOperator::Eq => ops.cmp.get(&bop.rhs.ty).map(|c| (c.eq)(builder, lhs_val, rhs_val)),
                        BinaryOperator::Ne => ops.cmp.get(&bop.rhs.ty).map(|c| (c.ne)(builder, lhs_val, rhs_val)),
                        BinaryOperator::Lt => ops.cmp.get(&bop.rhs.ty).map(|c| (c.lt)(builder, lhs_val, rhs_val)),
                        BinaryOperator::Gt => ops.cmp.get(&bop.rhs.ty).map(|c| (c.gt)(builder, lhs_val, rhs_val)),
                        BinaryOperator::Le => ops.cmp.get(&bop.rhs.ty).map(|c| (c.le)(builder, lhs_val, rhs_val)),
                        BinaryOperator::Ge => ops.cmp.get(&bop.rhs.ty).map(|c| (c.ge)(builder, lhs_val, rhs_val)),
                        _ => unreachable!("compound assign handled above"),
                    }
                };
                result
            }

            TypedExprNode::UnaryOp(uop) => {
                match &uop.op {
                    UnaryOperator::Reference => {
                        match &uop.operand.value {
                            TypedExprNode::Identifier(name) => {
                                if let Some(alloca) = locals.get_identifier(name) {
                                    return Some(alloca.into_pointer_value().into());
                                }
                                // functions are already pointers in LLVM's opaque pointer model
                                if let Some(&global) = globals.get(name) {
                                    return Some(global);
                                }
                                return None;
                            }
                            TypedExprNode::Index(idx) => {
                                // &arr[i] — GEP without load
                                let array_ptr = self.compile_expression(builder, ret_var, globals, locals, &idx.operand)
                                    .unwrap().into_pointer_value();
                                let idx_val: inkwell::values::IntValue<'static> = self
                                    .compile_expression(builder, ret_var, globals, locals, &idx.index)
                                    .unwrap().try_into().ok().unwrap();
                                let array_info = self.type_context.get_by_id(idx.operand.ty).unwrap();
                                let elem_ty_id = match array_info.kind {
                                    TypeKind::Array { ty, .. } => ty,
                                    TypeKind::Slice(ty) => ty,
                                    _ => return None,
                                };
                                let elem_llvm: BasicTypeEnum<'static> = self.type_context
                                    .get_by_id(elem_ty_id).unwrap().llvm_type.try_into().unwrap();
                                let elem_ptr = unsafe {
                                    builder.build_gep(elem_llvm, array_ptr, &[idx_val], "elem_ptr").unwrap()
                                };
                                return Some(elem_ptr.into());
                            }
                            _ => return None,
                        }
                    }
                    UnaryOperator::Dereference => {
                        let ptr_val = self.compile_expression(builder, ret_var, globals, locals, &uop.operand).unwrap();
                        return Some(ptr_val)
                    }
                    UnaryOperator::Neg | UnaryOperator::Not => {}
                }

                let operand_val = self.compile_expression(builder, ret_var, globals, locals, &uop.operand).unwrap();

                let result = {
                    let ops = &self.type_context.get_by_id(uop.operand.ty).unwrap().ops;
                    match &uop.op {
                        UnaryOperator::Neg => ops.neg.as_ref().map(|(_, maker)| maker(builder, operand_val)),
                        UnaryOperator::Not => ops.not.as_ref().map(|(_, maker)| maker(builder, operand_val)),
                        _ => unreachable!(),
                    }
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
            TypedExprNode::SliceConstruct(sc) => {
                let slice_ty = self.type_context.get_by_id(e.ty).unwrap().llvm_type
                    .try_into().ok().unwrap();
                let BasicTypeEnum::StructType(slice_struct_ty) = slice_ty else { return None; };
                let len_val: BasicValueEnum<'static> = self.compile_expression(builder, ret_var, globals, locals, &sc.len)?.try_into().ok()?;
                let ptr_val: BasicValueEnum<'static> = self.compile_expression(builder, ret_var, globals, locals, &sc.ptr)?.try_into().ok()?;
                let mut agg: BasicValueEnum<'static> = slice_struct_ty.get_undef().into();
                agg = builder.build_insert_value(agg.into_struct_value(), len_val, 0, "sl_len").unwrap().into_struct_value().into();
                agg = builder.build_insert_value(agg.into_struct_value(), ptr_val, 1, "sl_ptr").unwrap().into_struct_value().into();
                Some(agg.into())
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
