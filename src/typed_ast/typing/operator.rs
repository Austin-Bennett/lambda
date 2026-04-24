use crate::lexer::literal::LiteralValue;
use crate::typed_ast::typing::ty::TypeId;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{AnyValueEnum, PointerValue};
use std::collections::HashMap;

// Produces an LLVM constant from a literal value.
// Key in `from_literal` is the literal pseudo-type's TypeId (e.g. int_literal, float_literal).
pub type LiteralMaker = Box<dyn Fn(&'static Context, &LiteralValue) -> AnyValueEnum<'static>>;

// Binary operator: (builder, lhs_value, rhs_value) -> result_value
pub type BinaryOperatorMaker = Box<dyn Fn(&Builder<'static>, AnyValueEnum<'static>, AnyValueEnum<'static>) -> AnyValueEnum<'static>>;

// Unary operator: (builder, operand_value) -> result_value
pub type UnaryOperatorMaker = Box<dyn Fn(&Builder<'static>, AnyValueEnum<'static>) -> AnyValueEnum<'static>>;

// Conversion operator: (builder, value) -> converted_value
pub type ConversionMaker = Box<dyn Fn(&Builder<'static>, AnyValueEnum<'static>) -> AnyValueEnum<'static>>;

// Assignment operator: (builder, destination_ptr, value)
pub type AssignmentMaker = Box<dyn Fn(&Builder<'static>, PointerValue<'static>, AnyValueEnum<'static>)>;


pub struct ComparisonMakers {
    pub eq: BinaryOperatorMaker,
    pub ne: BinaryOperatorMaker,
    pub lt: BinaryOperatorMaker,
    pub gt: BinaryOperatorMaker,
    pub le: BinaryOperatorMaker,
    pub ge: BinaryOperatorMaker,
}

// Each operator overload maps its rhs type (or call parameter list) to
// (result TypeId, codegen callback).
pub struct OperatorOverloads {
    // neg: result type + codegen callback
    pub neg: Option<(TypeId, UnaryOperatorMaker)>,
    // not: bitwise/boolean NOT, result type + callback
    pub not: Option<(TypeId, UnaryOperatorMaker)>,

    // binary arithmetic: rhs TypeId -> (result TypeId, codegen callback)
    pub add: HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,
    pub sub: HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,
    pub mul: HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,
    pub div: HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,

    // bitwise binary: rhs TypeId -> (result TypeId, codegen callback)
    pub bit_and: HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,
    pub bit_or:  HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,
    pub bit_xor: HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,
    pub shl:     HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,
    pub shr:     HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,

    // User-defined operator functions: rhs TypeId -> (result TypeId, mangled function name)
    // These take priority over the built-in codegen callbacks above.
    pub user_add: HashMap<TypeId, (TypeId, String)>,
    pub user_sub: HashMap<TypeId, (TypeId, String)>,
    pub user_mul: HashMap<TypeId, (TypeId, String)>,
    pub user_div: HashMap<TypeId, (TypeId, String)>,
    // cmp result is always int8; rhs TypeId -> mangled name
    pub user_cmp: HashMap<TypeId, String>,
    // assign: rhs TypeId -> (result TypeId, mangled name)
    pub user_assign: HashMap<TypeId, (TypeId, String)>,
    // drop: mangled name of the drop function (no rhs, no return)
    pub drop: Option<String>,

    // assign: rhs TypeId -> store callback
    pub assign: HashMap<TypeId, AssignmentMaker>,

    // literal_pseudo_type_id -> codegen callback; keyed by the literal's TypeId
    pub from_literal: HashMap<TypeId, LiteralMaker>,

    // call: parameter type list -> result TypeId
    // (codegen for call is handled separately in compile_expression)
    pub call: HashMap<Vec<TypeId>, TypeId>,

    pub index: HashMap<TypeId, (TypeId, BinaryOperatorMaker)>,

    // explicit `as` casts: target TypeId -> codegen callback
    pub conversion_ops: HashMap<TypeId, ConversionMaker>,

    // comparison: rhs TypeId -> all six comparison makers; result type is always bool
    pub cmp: HashMap<TypeId, ComparisonMakers>,
}

impl OperatorOverloads {
    pub fn new() -> Self {
        Self {
            neg: None,
            not: None,

            add: HashMap::new(),
            sub: HashMap::new(),
            mul: HashMap::new(),
            div: HashMap::new(),

            bit_and: HashMap::new(),
            bit_or:  HashMap::new(),
            bit_xor: HashMap::new(),
            shl:     HashMap::new(),
            shr:     HashMap::new(),

            user_add: HashMap::new(),
            user_sub: HashMap::new(),
            user_mul: HashMap::new(),
            user_div: HashMap::new(),
            user_cmp: HashMap::new(),
            user_assign: HashMap::new(),
            drop: None,

            assign: HashMap::new(),
            from_literal: HashMap::new(),

            call: HashMap::new(),

            index: HashMap::new(),

            conversion_ops: HashMap::new(),

            cmp: HashMap::new(),
        }
    }
}
