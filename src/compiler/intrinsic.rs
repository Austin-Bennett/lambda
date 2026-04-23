use std::sync::Arc;
use std::collections::HashMap;
use inkwell::builder::Builder;
use inkwell::values::AnyValueEnum;
use crate::typed_ast::ast::statements::expression::TypedExpr;
use crate::typed_ast::typing::scope::AvailableContext;

pub type IntrinsicMaker = Arc<dyn Fn(
    &mut Builder<'static>,
    &HashMap<String, AnyValueEnum<'static>>,
    &mut AvailableContext<AnyValueEnum<'static>>,
    &[TypedExpr],
    Vec<Option<AnyValueEnum<'static>>>,
) -> Option<AnyValueEnum<'static>>>;
