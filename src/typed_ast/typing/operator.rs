
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use crate::codegen::compilation_primitives::{BinaryMethodCompiler, MethodCompiler};
use crate::typed_ast::typing::ty::TypeId;

//todo: figure out how were gonna compile ts






//each operator overload maps its rhs (or call parameters) to its result type id
pub struct OperatorOverloads {
    pub neg: Option<TypeId>,

    pub add: HashMap<TypeId, TypeId>,
    pub sub: HashMap<TypeId, TypeId>,
    pub mul: HashMap<TypeId, TypeId>,
    pub div: HashMap<TypeId, TypeId>,

    //second is whether it is constructed directly on the stack or moved into RET
    pub copy: (),

    
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
            
            call: HashMap::new(),

            implicit_conversion: HashMap::new(),
        }
    }
    
    
}