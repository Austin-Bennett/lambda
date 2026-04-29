use std::sync::Arc;
use inkwell::types::AsTypeRef;
use inkwell::values::{AnyValueEnum, AsValueRef, FunctionValue};
use llvm_sys::core::{LLVMGetGlobalParent, LLVMGetIntrinsicDeclaration, LLVMLookupIntrinsicID};
use crate::compiler::Compiler;
use crate::compiler::intrinsic::IntrinsicMaker;
use crate::typed_ast::typing::ty::TypeKind;

pub fn add_intrinsics(compiler: &mut Compiler) {
    let ctx = compiler.llvm_context;

    let f32_id = compiler.type_context.float32;
    let f64_id = compiler.type_context.float64;

    // Unary float intrinsics: sqrt, log, floor, ceil, round
    for (name, llvm_name, ret, is_f32) in [
        ("__float32_sqrt",  "llvm.sqrt.f32\0",  f32_id, true),
        ("__float64_sqrt",  "llvm.sqrt.f64\0",  f64_id, false),
        ("__float32_log",   "llvm.log.f32\0",   f32_id, true),
        ("__float64_log",   "llvm.log.f64\0",   f64_id, false),
        ("__float32_floor", "llvm.floor.f32\0", f32_id, true),
        ("__float64_floor", "llvm.floor.f64\0", f64_id, false),
        ("__float32_ceil",  "llvm.ceil.f32\0",  f32_id, true),
        ("__float64_ceil",  "llvm.ceil.f64\0",  f64_id, false),
        ("__float32_round", "llvm.round.f32\0", f32_id, true),
        ("__float64_round", "llvm.round.f64\0", f64_id, false),
    ] {
        let maker: IntrinsicMaker = Arc::new(move |_compiler, builder, _globals, args| {
            let arg = args.first()?.0.into_float_value();

            let module_ref = unsafe {
                LLVMGetGlobalParent(
                    builder.get_insert_block()?.get_parent()?.as_value_ref(),
                )
            };

            let float_ty: inkwell::types::BasicTypeEnum = if is_f32 {
                ctx.f32_type().into()
            } else {
                ctx.f64_type().into()
            };
            let mut type_refs = vec![float_ty.as_type_ref()];

            let id = unsafe { LLVMLookupIntrinsicID(llvm_name.as_ptr() as _, llvm_name.len() - 1) };
            let fn_ref = unsafe {
                LLVMGetIntrinsicDeclaration(module_ref, id, type_refs.as_mut_ptr(), type_refs.len())
            };
            let fn_val: FunctionValue<'static> = unsafe { FunctionValue::new(fn_ref)? };

            let call = builder.build_call(fn_val, &[arg.into()], "intr").ok()?;
            call.try_as_basic_value().basic().map(|v| v.into())
        });

        compiler.register_intrinsic(name, ret, maker);
    }

    // Binary float intrinsic: pow
    for (name, llvm_name, ret, is_f32) in [
        ("__float32_pow", "llvm.pow.f32\0", f32_id, true),
        ("__float64_pow", "llvm.pow.f64\0", f64_id, false),
    ] {
        let maker: IntrinsicMaker = Arc::new(move |_compiler, builder, _globals, args| {
            let base = args.first()?.0.into_float_value();
            let exp  = args.get(1)?.0.into_float_value();

            let module_ref = unsafe {
                LLVMGetGlobalParent(
                    builder.get_insert_block()?.get_parent()?.as_value_ref(),
                )
            };

            let float_ty: inkwell::types::BasicTypeEnum = if is_f32 {
                ctx.f32_type().into()
            } else {
                ctx.f64_type().into()
            };
            let mut type_refs = vec![float_ty.as_type_ref()];

            let id = unsafe { LLVMLookupIntrinsicID(llvm_name.as_ptr() as _, llvm_name.len() - 1) };
            let fn_ref = unsafe {
                LLVMGetIntrinsicDeclaration(module_ref, id, type_refs.as_mut_ptr(), type_refs.len())
            };
            let fn_val: FunctionValue<'static> = unsafe { FunctionValue::new(fn_ref)? };

            let call = builder.build_call(fn_val, &[base.into(), exp.into()], "intr").ok()?;
            call.try_as_basic_value().basic().map(|v| v.into())
        });

        compiler.register_intrinsic(name, ret, maker);
    }
    compiler.register_intrinsic("__drop", compiler.type_context.none, Arc::new(move
        |comp, b, globals, exprs| {

        let (v, ty) = exprs[0];
        let ref_info = comp.type_context.get_by_id(ty).unwrap();
        let TypeKind::Reference(inner_id) = ref_info.kind else {
            panic!("__drop must be called with a reference!");
        };
        let info = comp.type_context.get_by_id(inner_id).unwrap();

        if let Some(drop_mangled) = info.ops.drop.as_ref() {
            let func = globals.get(drop_mangled).unwrap().into_function_value();
            b.build_call(func, &[v.try_into().unwrap()], "manual_drop_call").unwrap();
        }

        None
    }))
}
