use crate::compiler::Compiler;
use crate::typed_ast::ast::items::function::Function;
use crate::lexer::literal::LiteralValue;
use crate::typed_ast::ast::statements::expression::{BinaryOperator, TypedExpr, TypedExprNode, UnaryOperator};
use crate::typed_ast::ast::statements::TypedStatement;
use crate::typed_ast::typing::ty::TypeKind;
use inkwell::builder::Builder;
use inkwell::types::{AnyTypeEnum, AsTypeRef, BasicMetadataTypeEnum, BasicTypeEnum};
use inkwell::values::{AnyValueEnum, ArrayValue, BasicMetadataValueEnum, BasicValueEnum, FunctionValue, GlobalValue, IntValue, PointerValue};
use std::collections::HashMap;
use std::io::Read;
use std::mem;
use std::ops::Deref;
use inkwell::{AddressSpace, GlobalVisibility};
use inkwell::basic_block::BasicBlock;
use inkwell::module::Linkage;
use crate::typed_ast::ast::block::TypedBlock;
use crate::typed_ast::ast::statements::if_stmt::{TypedElseStatement, TypedIfSyntax};
use crate::typed_ast::typing::scope::AvailableContext;

pub struct GlobalVals {
    strings: HashMap<u32, GlobalValue<'static>>,
    cstrings: HashMap<u32, GlobalValue<'static>>,
}

impl Compiler {

    pub fn compile(&mut self) -> inkwell::module::Module<'static> {
        let mut builder = self.llvm_context.create_builder();
        let module = self.llvm_context.create_module("lambda_program");

        let mut globals: HashMap<String, AnyValueEnum> = HashMap::new();
        let mut string_literals: HashMap<u32, GlobalValue> = HashMap::new();
        let mut c_string_literals: HashMap<u32, GlobalValue> = HashMap::new();

        let str_type = self.type_context.get_by_id(self.type_context.str).unwrap();
        let basic = str_type.llvm_type.into_struct_type();

        let size_t = self.type_context.get_by_id(self.type_context.usize).unwrap();
        let size_t = size_t.llvm_type.into_int_type();

        let u8 = self.type_context.get_by_id(self.type_context.uint8).unwrap();
        let u8 = u8.llvm_type.into_int_type();

        for (s, id) in &self.str_literal_reg.item_map {
            let global = module.add_global(
                basic,
                None,
                "string_literal"
            );

            let bytes: Vec<_> = s.bytes().into_iter().map(|u| u8.const_int(u as u64, false).into()).collect();


            let const_array = u8.const_array(&bytes);

            let internal_blob = module.add_global(const_array.get_type(), None, ".str_blob");
            internal_blob.set_initializer(&const_array);
            internal_blob.set_linkage(Linkage::Private); // Keep it clean
            internal_blob.set_constant(true);


            let constant_struct = basic.const_named_struct(&[
                size_t.const_int(s.len() as u64, false).into(),
                internal_blob.as_pointer_value().into(),
            ]);
            global.set_initializer(&constant_struct);
            global.set_linkage(Linkage::Private);

            string_literals.insert(*id, global);
        }

        for (s, id) in &self.cstr_literal_reg.item_map {
            let bytes: Vec<_> = s.iter().map(|u| u8.const_int(*u as u64, false).into()).collect();

            let const_array = u8.const_array(&bytes);

            let global = module.add_global(
                const_array.get_type(),
                None,
                "cstring_literal"
            );


            global.set_initializer(&const_array);
            global.set_linkage(Linkage::Private);
            global.set_constant(true);


            c_string_literals.insert(*id, global);
        }

        let global_vals = GlobalVals{
            strings: string_literals,
            cstrings: c_string_literals,
        };

        let modules = mem::take(&mut self.typed_modules);
        let mono_fns = mem::take(&mut self.pending_mono_fns);

        // First pass: declare all functions so they can be called before definition.
        for (_, tmod) in &modules {
            for func in &tmod.functions {
                let ret_type = self.type_context.get_by_id(func.signature.ret).unwrap();
                let llvm_params: Vec<BasicMetadataTypeEnum> = func.signature.params.iter()
                    .map(|p| self.type_context.get_by_id(*p).unwrap().llvm_type.try_into().unwrap())
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
                    if func.is_extern { Some(Linkage::External) } else { None });
                globals.insert(func.signature.name.clone(), fn_val.into());
            }
        }
        for func in &mono_fns {
            let ret_type = self.type_context.get_by_id(func.signature.ret).unwrap();
            let llvm_params: Vec<BasicMetadataTypeEnum> = func.signature.params.iter()
                .map(|p| self.type_context.get_by_id(*p).unwrap().llvm_type.try_into().unwrap())
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
                if func.is_extern { Some(Linkage::External) } else { None });
            globals.insert(func.signature.name.clone(), fn_val.into());
        }

        // Second pass: compile each function body.
        for (_, tmod) in &modules {
            for func in &tmod.functions {
                let fn_val = globals[&func.signature.name].into_function_value();
                self.compile_function(&mut builder, &globals, &global_vals, func, fn_val);
            }
        }
        for func in &mono_fns {
            let fn_val = globals[&func.signature.name].into_function_value();
            self.compile_function(&mut builder, &globals, &global_vals, func, fn_val);
        }


        self.typed_modules = modules;
        self.pending_mono_fns = mono_fns;

        module
    }

    pub fn compile_function(
        &mut self,
        builder: &mut Builder<'static>,
        globals: &HashMap<String, AnyValueEnum<'static>>,
        global_vals: &GlobalVals,
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
            global_vals,
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
                         global_vals: &GlobalVals,
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
                global_vals,
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
                global_vals,
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
                global_vals,
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
        global_vals: &GlobalVals,
        locals: &mut AvailableContext<AnyValueEnum<'static>>,
        s: &TypedStatement,
    ) {
        // Skip dead code after a terminator (e.g. injected drop calls after an explicit return).
        if builder.get_insert_block().map_or(false, |bb| bb.get_terminator().is_some()) {
            return;
        }
        match s {
            TypedStatement::Expr(e) => {
                self.compile_expression(builder, ret_var, globals, global_vals, locals, e);
                
            }

            TypedStatement::VarDecl(vd) => {
                let basic_ty: BasicTypeEnum<'static> = self
                    .type_context
                    .get_by_id(vd.ty)
                    .and_then(|info| info.llvm_type.try_into().ok())
                    .unwrap();

                // Save old alloca before init (needed for shadowed drop).
                let old_alloca = vd.shadowed_drop.as_ref()
                    .and_then(|_| locals.get_identifier(&vd.name))
                    .map(|v| v.into_pointer_value());

                // 1. Compute init first — may legitimately read the old value of `x`.
                let init_val = if let Some(val_expr) = &vd.val {
                    self.compile_expression(builder, ret_var, globals, global_vals, locals, val_expr)
                } else {
                    None
                };

                // 2. Drop old value after init is computed but before new alloca is stored.
                if let (Some(old_ptr), Some((_, drop_mangled))) = (old_alloca, &vd.shadowed_drop) {
                    if let Some(&drop_fn) = globals.get(drop_mangled.as_str()) {
                        builder.build_call(drop_fn.into_function_value(), &[old_ptr.into()], "").unwrap();
                    }
                }

                // 3. Always emit the alloca in the function entry block (loop-body safety).
                let current_block = builder.get_insert_block().unwrap();
                let entry_block = function.get_first_basic_block().unwrap();
                builder.position_at_end(entry_block);
                let alloca = builder.build_alloca(basic_ty, &vd.name).unwrap();
                builder.position_at_end(current_block);

                locals.declare_identifier_in_scope(vd.name.clone(), alloca.into());

                // 4. Store init value.
                if let Some(val) = init_val {
                    if let Ok(basic_val) = BasicValueEnum::try_from(val) {
                        builder.build_store(alloca, basic_val).unwrap();
                    }
                }
            }

            TypedStatement::Return(ret) => {
                let val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &ret.0);
                if let Some(val) = val {
                    if let Ok(basic_val) = BasicValueEnum::try_from(val) {
                        ret_var.map(|v| builder.build_store(v, basic_val));
                    }
                }
                builder.build_unconditional_branch(ret_block).unwrap();
            },

            TypedStatement::If(i4) => {

                self.compile_if_statement(
                    builder, function, ret_var, ret_block, globals, global_vals, locals, i4
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
                    global_vals,
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
                    global_vals,
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
        global_vals: &GlobalVals,
        locals: &mut AvailableContext<AnyValueEnum<'static>>,
        i4: &TypedIfSyntax,
    ) {
        let then_block  = self.llvm_context.append_basic_block(function, "then");
        let else_block  = self.llvm_context.append_basic_block(function, "else");
        let merge_block = self.llvm_context.append_basic_block(function, "if_merge");

        let pred = self.compile_expression(builder, ret_var, globals, global_vals, locals, &i4.data.predicate)
            .unwrap()
            .into_int_value();

        builder.build_conditional_branch(pred, then_block, else_block).unwrap();


        let if_scope = self.compile_block(
            builder,
            function,
            ret_var,
            ret_block,
            globals,
            global_vals,
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
                    self.compile_if_statement(builder, function, ret_var, ret_block, globals, global_vals, locals, if_statement);
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
                        global_vals,
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

    /// Return the pointer (address) for any lvalue expression.
    /// Mirrors the cases in `TypedExpr::is_lvalue`.
    pub fn compile_lvalue(
        &mut self,
        builder: &mut Builder<'static>,
        ret_var: Option<PointerValue>,
        globals: &HashMap<String, AnyValueEnum<'static>>,
        global_vals: &GlobalVals,
        locals: &mut AvailableContext<AnyValueEnum<'static>>,
        e: &TypedExpr,
    ) -> Option<PointerValue<'static>> {
        match &e.value {
            TypedExprNode::Identifier(name) => {
                Some(locals.get_identifier(name)?.into_pointer_value())
            }
            TypedExprNode::MemberAccess(ma) => {
                let obj_ptr = self.compile_lvalue(builder, ret_var, globals, global_vals, locals, &ma.object)?;
                let TypeKind::Struct(sid) = self.type_context.get_by_id(ma.object.ty)?.kind.clone() else { return None; };
                let struct_ty = self.type_context.structs[sid as usize].llvm_struct;
                builder.build_struct_gep(struct_ty, obj_ptr, ma.member_index as u32, "field_ptr").ok()
            }
            TypedExprNode::Index(_) => {
                // Index makers return the element pointer directly as an AnyValueEnum.
                self.compile_expression(builder, ret_var, globals, global_vals, locals, e)
                    .map(|v| v.into_pointer_value())
            }
            TypedExprNode::RefRead(inner) => {
                // `inner` has reference type T& — compiling it yields the pointer (the reference value).
                self.compile_expression(builder, ret_var, globals, global_vals, locals, inner)
                    .map(|v| v.into_pointer_value())
            }
            TypedExprNode::UnaryOp(uop) if matches!(uop.op, UnaryOperator::Dereference) => {
                // *ptr — the pointer operand is the address.
                self.compile_expression(builder, ret_var, globals, global_vals, locals, &uop.operand)
                    .map(|v| v.into_pointer_value())
            }
            _ => None,
        }
    }

    pub fn compile_expression(
        &mut self,
        builder: &mut Builder<'static>,
        ret_var: Option<PointerValue>,
        globals: &HashMap<String, AnyValueEnum<'static>>,
        global_vals: &GlobalVals,
        locals: &mut AvailableContext<AnyValueEnum<'static>>,
        e: &TypedExpr,
    ) -> Option<AnyValueEnum<'static>> {
        match &e.value {
            TypedExprNode::NullPtr => {
                Some(self.llvm_context.ptr_type(inkwell::AddressSpace::try_from(0u32).unwrap()).const_null().into())
            }
            TypedExprNode::Literal(lit) => {
                let ctx = self.llvm_context;
                match lit {
                    LiteralValue::Bool(b) => {
                        Some(ctx.custom_width_int_type(1).const_int(*b as u64, false).into())
                    }
                    LiteralValue::Char(c) => {
                        Some(ctx.i32_type().const_int((*c as u32) as u64, false).into())
                    }
                    LiteralValue::String(s) => {
                        let global: GlobalValue<'static> = global_vals.strings[s];
                        let ptr: PointerValue<'static> = global.as_pointer_value();
                        let load: BasicValueEnum<'static> = builder.build_load(
                            self.type_context.create_slice_llvm_structure(), ptr, "string_literal").unwrap();

                        Some(load.into())
                    }
                    LiteralValue::CString(s) => {
                        let global: GlobalValue<'static> = global_vals.cstrings[s];
                        let ptr: PointerValue<'static> = global.as_pointer_value();

                        Some(ptr.into())
                    }
                    LiteralValue::Integer(_) | LiteralValue::Float(_) => {
                        let literal_key = match lit {
                            LiteralValue::Integer(_) => self.type_context.int_literal,
                            LiteralValue::Float(_)   => self.type_context.float_literal,
                            _    => unreachable!(),
                        };

                        //todo: fix panic when doing something weird like 0 as T*
                        self.type_context.get_by_id(e.ty)
                            .and_then(|info| info.ops.from_literal.get(&literal_key))
                            .map(|maker| maker(ctx, lit))
                    }
                }
            }

            TypedExprNode::RefRead(inner) => {
                // Evaluate the inner expr to get the reference pointer, then load through it.
                let ref_ptr = self.compile_expression(builder, ret_var, globals, global_vals, locals, inner).unwrap();
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
                    let val = globals.get(ident).copied()?;
                    // Function pointer: a FunctionValue needs to be a plain pointer value.
                    if matches!(self.type_context.get_by_id(e.ty).map(|i| &i.kind), Some(TypeKind::FnPtr { .. })) {
                        if let AnyValueEnum::FunctionValue(fn_val) = val {
                            return Some(fn_val.as_global_value().as_pointer_value().into());
                        }
                    }
                    Some(val)
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

                        let compiled_expr = self.compile_expression(builder, ret_var, globals, global_vals, locals, expr).unwrap();
                        let basic: BasicValueEnum = compiled_expr.try_into().unwrap();

                        builder.build_store(elem_ptr, basic).unwrap();
                    }
                }

                Some(ptr.into())

            },
            TypedExprNode::Index(index) => {
                let compiled_operand = self.compile_expression(builder, ret_var, globals, global_vals, locals, &index.operand).unwrap();
                let compiled_index = self.compile_expression(builder, ret_var, globals, global_vals, locals, &index.index).unwrap();

                let info = self.type_context.get_by_id(index.operand.ty).unwrap();

                let maker = &info.ops.index[&index.index.ty].1;
                Some(maker(builder, globals, compiled_operand, compiled_index))
            },

            TypedExprNode::BinaryOp(bop) => {
                // --- Assign ---
                if let BinaryOperator::Assign = &bop.op {
                    let rhs_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &bop.rhs).unwrap();
                    let lhs_ptr = self.compile_lvalue(builder, ret_var, globals, global_vals, locals, &bop.lhs)?;
                    let basic: BasicValueEnum<'static> = rhs_val.try_into().ok().unwrap();
                    // For plain identifiers, run drop + typed assign maker.
                    // For everything else (member, index, deref), raw store suffices.
                    if let TypedExprNode::Identifier(name) = &bop.lhs.value {
                        let _ = name;
                        let drop_fn = self.type_context.get_by_id(bop.lhs.ty)
                            .and_then(|info| info.ops.drop.as_ref())
                            .and_then(|drop_name| globals.get(drop_name.as_str()))
                            .copied();
                        if let Some(drop_fn_any) = drop_fn {
                            builder.build_call(drop_fn_any.into_function_value(), &[lhs_ptr.into()], "").unwrap();
                        }
                        let ops = &self.type_context.get_by_id(bop.lhs.ty).unwrap().ops;
                        let maker = ops.assign.get(&bop.rhs.ty).unwrap();
                        maker(builder, lhs_ptr, rhs_val);
                    } else {
                        builder.build_store(lhs_ptr, basic).unwrap();
                    }
                    return Some(rhs_val);
                }

                // --- Short-circuit && ---
                if let BinaryOperator::BoolAnd = &bop.op {
                    let function = builder.get_insert_block().unwrap().get_parent().unwrap();
                    let lhs_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &bop.lhs)?.into_int_value();
                    let lhs_block = builder.get_insert_block().unwrap();
                    let rhs_block   = self.llvm_context.append_basic_block(function, "and_rhs");
                    let merge_block = self.llvm_context.append_basic_block(function, "and_merge");
                    builder.build_conditional_branch(lhs_val, rhs_block, merge_block).unwrap();

                    builder.position_at_end(rhs_block);
                    let rhs_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &bop.rhs)?.into_int_value();
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
                    let lhs_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &bop.lhs)?.into_int_value();
                    let lhs_block = builder.get_insert_block().unwrap();
                    let rhs_block   = self.llvm_context.append_basic_block(function, "or_rhs");
                    let merge_block = self.llvm_context.append_basic_block(function, "or_merge");
                    builder.build_conditional_branch(lhs_val, merge_block, rhs_block).unwrap();

                    builder.position_at_end(rhs_block);
                    let rhs_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &bop.rhs)?.into_int_value();
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
                    &HashMap<String, AnyValueEnum<'static>>,
                    AnyValueEnum<'static>, AnyValueEnum<'static>,
                    &crate::typed_ast::typing::operator::OperatorOverloads,
                    crate::typed_ast::typing::ty::TypeId) -> Option<AnyValueEnum<'static>>>
                    = match &bop.op {
                    BinaryOperator::AddAssign    => Some(|b,g,l,r,ops,rty| ops.add.get(&rty).map(|(_,m)| m(b,g,l,r))),
                    BinaryOperator::SubAssign    => Some(|b,g,l,r,ops,rty| ops.sub.get(&rty).map(|(_,m)| m(b,g,l,r))),
                    BinaryOperator::MulAssign    => Some(|b,g,l,r,ops,rty| ops.mul.get(&rty).map(|(_,m)| m(b,g,l,r))),
                    BinaryOperator::DivAssign    => Some(|b,g,l,r,ops,rty| ops.div.get(&rty).map(|(_,m)| m(b,g,l,r))),
                    BinaryOperator::ModAssign    => Some(|b,g,l,r,ops,rty| ops.rem.get(&rty).map(|(_,m)| m(b,g,l,r))),
                    BinaryOperator::BitAndAssign => Some(|b,g,l,r,ops,rty| ops.bit_and.get(&rty).map(|(_,m)| m(b,g,l,r))),
                    BinaryOperator::BitOrAssign  => Some(|b,g,l,r,ops,rty| ops.bit_or.get(&rty).map(|(_,m)| m(b,g,l,r))),
                    BinaryOperator::BitXorAssign => Some(|b,g,l,r,ops,rty| ops.bit_xor.get(&rty).map(|(_,m)| m(b,g,l,r))),
                    BinaryOperator::ShlAssign    => Some(|b,g,l,r,ops,rty| ops.shl.get(&rty).map(|(_,m)| m(b,g,l,r))),
                    BinaryOperator::ShrAssign    => Some(|b,g,l,r,ops,rty| ops.shr.get(&rty).map(|(_,m)| m(b,g,l,r))),
                    _ => None,
                };
                if let Some(inner_fn) = compound_inner_op {
                    let lhs_ty: BasicTypeEnum<'static> = self.type_context
                        .get_by_id(bop.lhs.ty).unwrap().llvm_type.try_into().ok().unwrap();
                    let field_ptr = self.compile_lvalue(builder, ret_var, globals, global_vals, locals, &bop.lhs)?;
                    let current_val: AnyValueEnum<'static> = builder.build_load(lhs_ty, field_ptr, "ca_load").unwrap().into();
                    let rhs_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &bop.rhs)?;
                    let new_val = {
                        let ops = &self.type_context.get_by_id(bop.lhs.ty).unwrap().ops;
                        inner_fn(builder, globals, current_val, rhs_val, ops, bop.rhs.ty)?
                    };
                    let basic_new: BasicValueEnum<'static> = new_val.try_into().ok().unwrap();
                    builder.build_store(field_ptr, basic_new).unwrap();
                    return Some(new_val);
                }

                // --- Compound &&= / ||= (short-circuit) ---
                if matches!(bop.op, BinaryOperator::BoolAndAssign | BinaryOperator::BoolOrAssign) {
                    let is_and = matches!(bop.op, BinaryOperator::BoolAndAssign);
                    let function = builder.get_insert_block().unwrap().get_parent().unwrap();
                    let alloca = self.compile_lvalue(builder, ret_var, globals, global_vals, locals, &bop.lhs)?;
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
                    let rhs_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &bop.rhs)?.into_int_value();
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
                let lhs_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &bop.lhs).unwrap();
                let rhs_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &bop.rhs).unwrap();

                let result = {
                    let ops = &self.type_context.get_by_id(bop.lhs.ty).unwrap().ops;
                    match &bop.op {
                        BinaryOperator::Add => ops.add.get(&bop.rhs.ty).map(|(_, m)| m(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::Sub => ops.sub.get(&bop.rhs.ty).map(|(_, m)| m(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::Mul => ops.mul.get(&bop.rhs.ty).map(|(_, m)| m(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::Div => ops.div.get(&bop.rhs.ty).map(|(_, m)| m(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::Mod => ops.rem.get(&bop.rhs.ty).map(|(_, m)| m(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::BitAnd => ops.bit_and.get(&bop.rhs.ty).map(|(_, m)| m(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::BitOr  => ops.bit_or.get(&bop.rhs.ty).map(|(_, m)| m(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::BitXor => ops.bit_xor.get(&bop.rhs.ty).map(|(_, m)| m(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::Shl    => ops.shl.get(&bop.rhs.ty).map(|(_, m)| m(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::Shr    => ops.shr.get(&bop.rhs.ty).map(|(_, m)| m(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::Assign => unreachable!("assign handled above"),
                        BinaryOperator::BoolAnd | BinaryOperator::BoolOr => unreachable!("handled above"),
                        BinaryOperator::Eq => ops.cmp.get(&bop.rhs.ty).map(|c| (c.eq)(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::Ne => ops.cmp.get(&bop.rhs.ty).map(|c| (c.ne)(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::Lt => ops.cmp.get(&bop.rhs.ty).map(|c| (c.lt)(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::Gt => ops.cmp.get(&bop.rhs.ty).map(|c| (c.gt)(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::Le => ops.cmp.get(&bop.rhs.ty).map(|c| (c.le)(builder, globals, lhs_val, rhs_val)),
                        BinaryOperator::Ge => ops.cmp.get(&bop.rhs.ty).map(|c| (c.ge)(builder, globals, lhs_val, rhs_val)),
                        _ => unreachable!("compound assign handled above"),
                    }
                };
                result
            }

            TypedExprNode::UnaryOp(uop) => {
                match &uop.op {
                    UnaryOperator::Reference => {
                        // For function identifiers, return the global pointer directly.
                        if let TypedExprNode::Identifier(name) = &uop.operand.value {
                            if let Some(&global) = globals.get(name.as_str()) {
                                return Some(global);
                            }
                        }
                        // All other lvalues: compile_lvalue gives the address.
                        return self.compile_lvalue(builder, ret_var, globals, global_vals, locals, &uop.operand)
                            .map(|ptr| ptr.into());
                    }
                    UnaryOperator::Dereference => {
                        let ptr_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &uop.operand).unwrap();
                        return Some(ptr_val)
                    }
                    UnaryOperator::Neg | UnaryOperator::Not => {}
                }

                let operand_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &uop.operand).unwrap();

                let result = {
                    let ops = &self.type_context.get_by_id(uop.operand.ty).unwrap().ops;
                    match &uop.op {
                        UnaryOperator::Neg => ops.neg.as_ref().map(|(_, maker)| maker(builder, globals, operand_val)),
                        UnaryOperator::Not => ops.not.as_ref().map(|(_, maker)| maker(builder, globals, operand_val)),
                        _ => unreachable!(),
                    }
                };
                // When the operand was a literal pseudo-type (e.g. float_literal/f64) but the
                // outer expression was coerced to a concrete type (e.g. float32), apply the
                // registered conversion so the result has the right LLVM type.
                if self.type_context.is_literal(uop.operand.ty) && e.ty != uop.operand.ty {
                    if let Some(val) = result {
                        if let Some(conv) = self.type_context.get_by_id(uop.operand.ty)
                            .and_then(|info| info.ops.conversion_ops.get(&e.ty))
                        {
                            return Some(conv(builder, val));
                        }
                        return Some(val);
                    }
                }
                result
            }

            TypedExprNode::Cast(cast) => {
                let val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &cast.expr).unwrap();
                let ops = &self.type_context.get_by_id(cast.expr.ty).unwrap().ops;
                ops.conversion_ops.get(&cast.target).map(|maker| maker(builder, val))
            }
            TypedExprNode::CallOp(call) => {
                let caller_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &call.caller).unwrap();

                // Compile each argument.
                let mut args: Vec<BasicMetadataValueEnum<'static>> = Vec::new();
                for arg in &call.arguments {
                    let val = self.compile_expression(builder, ret_var, globals, global_vals, locals, arg).unwrap();
                    let basic: BasicValueEnum<'static> = val.try_into().ok().unwrap();
                    args.push(basic.into());
                }

                let ret_is_void = matches!(
                    self.type_context.get_by_id(e.ty).map(|i| &i.kind),
                    Some(TypeKind::None)
                );

                // Direct named function call.
                if let AnyValueEnum::FunctionValue(fn_val) = caller_val {
                    let call_site = builder.build_call(fn_val, &args, "call").unwrap();
                    if ret_is_void {
                        return None;
                    }
                    return call_site.try_as_basic_value().basic().map(|v| v.into());
                }

                // Indirect call through a function pointer.
                if let AnyValueEnum::PointerValue(fn_ptr) = caller_val {
                    let caller_ty = call.caller.ty;
                    let (param_ids, ret_id) = match self.type_context.get_by_id(caller_ty).map(|i| i.kind.clone()) {
                        Some(TypeKind::FnPtr { params, ret }) => (params, ret),
                        _ => return None,
                    };
                    let param_llvm: Vec<BasicMetadataTypeEnum<'static>> = param_ids.iter()
                        .map(|&id| BasicTypeEnum::try_from(self.type_context.get_by_id(id).unwrap().llvm_type).unwrap().into())
                        .collect();
                    let ret_llvm = self.type_context.get_by_id(ret_id).unwrap().llvm_type;
                    let fn_type = match ret_llvm {
                        AnyTypeEnum::VoidType(t)    => t.fn_type(&param_llvm, false),
                        AnyTypeEnum::IntType(t)     => t.fn_type(&param_llvm, false),
                        AnyTypeEnum::FloatType(t)   => t.fn_type(&param_llvm, false),
                        AnyTypeEnum::PointerType(t) => t.fn_type(&param_llvm, false),
                        AnyTypeEnum::StructType(t)  => t.fn_type(&param_llvm, false),
                        AnyTypeEnum::ArrayType(t)   => t.fn_type(&param_llvm, false),
                        _ => return None,
                    };
                    let call_site = builder.build_indirect_call(fn_type, fn_ptr, &args, "icall").unwrap();
                    if ret_is_void {
                        return None;
                    }
                    return call_site.try_as_basic_value().basic().map(|v| v.into());
                }

                None
            }
            TypedExprNode::SliceConstruct(sc) => {
                let slice_ty = self.type_context.get_by_id(e.ty).unwrap().llvm_type
                    .try_into().ok().unwrap();
                let BasicTypeEnum::StructType(slice_struct_ty) = slice_ty else { return None; };
                let len_val: BasicValueEnum<'static> = self.compile_expression(builder, ret_var, globals, global_vals, locals, &sc.len)?.try_into().ok()?;
                let ptr_val: BasicValueEnum<'static> = self.compile_expression(builder, ret_var, globals, global_vals, locals, &sc.ptr)?.try_into().ok()?;
                let mut agg: BasicValueEnum<'static> = slice_struct_ty.get_undef().into();
                agg = builder.build_insert_value(agg.into_struct_value(), len_val, 0, "sl_len").unwrap().into_struct_value().into();
                agg = builder.build_insert_value(agg.into_struct_value(), ptr_val, 1, "sl_ptr").unwrap().into_struct_value().into();
                Some(agg.into())
            }
            TypedExprNode::StructConstruct(sc) => {
                let struct_ty = self.type_context.structs[sc.struct_id as usize].llvm_struct;
                let mut agg: BasicValueEnum<'static> = struct_ty.get_undef().into();
                for (i, field_expr) in sc.fields.iter().enumerate() {
                    let val = self.compile_expression(builder, ret_var, globals, global_vals, locals, field_expr).unwrap();
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
                let obj_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, &ma.object).unwrap();
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

                let compiled_args: Vec<(AnyValueEnum<'static>, _)> = ci.args.iter()
                    .filter_map(|arg| {
                        let val = self.compile_expression(builder, ret_var, globals, global_vals, locals, arg)?;
                        Some((val, arg.ty))
                    })
                    .collect();

                maker(self, builder, globals, &compiled_args)
            }

            TypedExprNode::SliceLen(slice_expr) => {
                let slice_val = self.compile_expression(builder, ret_var, globals, global_vals, locals, slice_expr)?;
                let sv = BasicValueEnum::try_from(slice_val).ok()?.into_struct_value();
                Some(builder.build_extract_value(sv, 0, "slen").unwrap().into())
            }

            TypedExprNode::SizeOf(ty_id) => {
                use llvm_sys::target::{LLVMABISizeOfType, LLVMGetModuleDataLayout};
                use inkwell::values::AsValueRef;
                let llvm_ty = self.type_context.get_by_id(*ty_id).unwrap().llvm_type;
                let basic_ty: inkwell::types::BasicTypeEnum = llvm_ty.try_into().ok()?;
                let size_bytes = unsafe {
                    let module_ref = llvm_sys::core::LLVMGetGlobalParent(
                        builder.get_insert_block()?.get_parent()?.as_value_ref(),
                    );
                    let td = LLVMGetModuleDataLayout(module_ref);
                    LLVMABISizeOfType(td, basic_ty.as_type_ref())
                };
                let usize_ty = self.type_context.get_by_id(self.type_context.usize).unwrap().llvm_type.into_int_type();
                Some(usize_ty.const_int(size_bytes, false).into())
            }

            TypedExprNode::AlignOf(ty_id) => {
                use llvm_sys::target::{LLVMABIAlignmentOfType, LLVMGetModuleDataLayout};
                use inkwell::values::AsValueRef;
                let llvm_ty = self.type_context.get_by_id(*ty_id).unwrap().llvm_type;
                let basic_ty: inkwell::types::BasicTypeEnum = llvm_ty.try_into().ok()?;
                let align_bytes = unsafe {
                    let module_ref = llvm_sys::core::LLVMGetGlobalParent(
                        builder.get_insert_block()?.get_parent()?.as_value_ref(),
                    );
                    let td = LLVMGetModuleDataLayout(module_ref);
                    LLVMABIAlignmentOfType(td, basic_ty.as_type_ref()) as u64
                };
                let usize_ty = self.type_context.get_by_id(self.type_context.usize).unwrap().llvm_type.into_int_type();
                Some(usize_ty.const_int(align_bytes, false).into())
            }
        }
    }
}
