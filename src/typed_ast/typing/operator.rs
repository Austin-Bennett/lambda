use crate::lexer::literal::IntegerLiteral;
use crate::typed_ast::typing::ty::TypeId;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{AnyValueEnum, PointerValue};
use std::collections::HashMap;

// Produces an LLVM constant from an integer literal token.
// The context must be `'static` because all generated values are `'static`.
pub type IntLiteralMaker = Box<dyn Fn(&'static Context, IntegerLiteral) -> AnyValueEnum<'static>>;

// Binary operator: (builder, lhs_value, rhs_value) -> result_value
pub type BinaryOperatorMaker = Box<dyn Fn(&Builder<'static>, AnyValueEnum<'static>, AnyValueEnum<'static>) -> AnyValueEnum<'static>>;

// Unary operator: (builder, operand_value) -> result_value
pub type UnaryOperatorMaker = Box<dyn Fn(&Builder<'static>, AnyValueEnum<'static>) -> AnyValueEnum<'static>>;

// Conversion operator: (builder, value) -> converted_value
pub type ConversionMaker = Box<dyn Fn(&Builder<'static>, AnyValueEnum<'static>) -> AnyValueEnum<'static>>;

// Assignment operator: (builder, destination_ptr, value)
pub type AssignmentMaker = Box<dyn Fn(&Builder<'static>, PointerValue<'static>, AnyValueEnum<'static>)>;


// Each operator overload maps its rhs type (or call parameter list) to
// (result TypeId, codegen callback).
pub struct OperatorOverloads {
    // neg: result type + codegen callback
    pub neg: Option<(TypeId, UnaryOperatorMaker)>,

    // binary arithmetic: rhs TypeId -> (result TypeId, codegen callback)
    pub add: HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,
    pub sub: HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,
    pub mul: HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,
    pub div: HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,

    // assign: rhs TypeId -> store callback
    pub assign: HashMap<TypeId, AssignmentMaker>,
    
    
    
    pub from_int_literal: Option<IntLiteralMaker>,

    // call: parameter type list -> result TypeId
    // (codegen for call is handled separately in compile_expression)
    pub call: HashMap<Vec<TypeId>, TypeId>,
    
    pub index: HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,

    // explicit `as` casts: target TypeId -> codegen callback
    pub conversion_ops: HashMap<TypeId, ConversionMaker>,
}

impl OperatorOverloads {
    pub fn new() -> Self {
        Self {
            neg: None,

            add: HashMap::new(),
            sub: HashMap::new(),
            mul: HashMap::new(),
            div: HashMap::new(),

            assign: HashMap::new(),
            from_int_literal: None,

            call: HashMap::new(),
            
            index: HashMap::new(),

            conversion_ops: HashMap::new(),
        }
    }
}
