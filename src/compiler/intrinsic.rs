use std::sync::Arc;
use std::collections::HashMap;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::values::AnyValueEnum;
use crate::typed_ast::ast::statements::expression::TypedExpr;

pub type IntrinsicMaker = Arc<dyn Fn(
    &mut Builder<'static>,
    &mut Module<'static>,
    &HashMap<String, AnyValueEnum<'static>>,
    &mut HashMap<String, AnyValueEnum<'static>>,
    &[TypedExpr],
    Vec<Option<AnyValueEnum<'static>>>,
) -> Option<AnyValueEnum<'static>>>;
