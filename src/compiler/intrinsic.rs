use std::sync::Arc;
use std::collections::HashMap;
use inkwell::builder::Builder;
use inkwell::values::AnyValueEnum;
use crate::compiler::Compiler;
use crate::typed_ast::typing::ty::TypeId;

pub type IntrinsicMaker = Arc<dyn Fn(
    &mut Compiler,
    &mut Builder<'static>,
    &HashMap<String, AnyValueEnum<'static>>,
    &[(AnyValueEnum<'static>, TypeId)],
) -> Option<AnyValueEnum<'static>>>;
