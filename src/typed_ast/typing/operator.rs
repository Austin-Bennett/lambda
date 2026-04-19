
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use inkwell::values::BasicValueEnum;
use crate::compiler::Compiler;
use crate::lexer::literal::IntegerLiteral;
use crate::typed_ast::typing::ty::{TypeId, TypeInfo};

//todo: figure out how were gonna compile ts



pub type IntLiteralMaker = Box<dyn Fn(&Compiler, &TypeInfo, IntegerLiteral) -> BasicValueEnum<'static>>;


//each operator overload maps its rhs (or call parameters) to its result type id
pub struct OperatorOverloads {
    pub neg: Option<TypeId>,

    pub add: HashMap<TypeId, TypeId>,
    pub sub: HashMap<TypeId, TypeId>,
    pub mul: HashMap<TypeId, TypeId>,
    pub div: HashMap<TypeId, TypeId>,

    //second is whether it is constructed directly on the stack or moved into RET
    pub copy: (),
    pub from_int_literal: Option<IntLiteralMaker>,
    
    pub call: HashMap<Vec<TypeId>, TypeId>,

    //types this type can implicitly convert to
    pub implicit_conversion: HashMap<TypeId, ()>,
}

impl OperatorOverloads {
    pub fn new() -> Self {
        Self{
            neg: None,
            
            add: HashMap::new(),
            sub: HashMap::new(),
            mul: HashMap::new(),
            div: HashMap::new(),
            
            copy: (),
            from_int_literal: None,
            
            call: HashMap::new(),

            implicit_conversion: HashMap::new(),
        }
    }
    
    
}