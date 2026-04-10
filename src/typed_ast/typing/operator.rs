
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use crate::codegen::compilation_primitives::{BinaryMethodCompiler, MethodCompiler};
use crate::typed_ast::typing::ty::TypeId;

//todo: figure out how were gonna compile ts






//each operator overload maps its rhs (or call parameters) to its result type id
pub struct OperatorOverloads {
    pub neg: Option<(TypeId, MethodCompiler)>,

    pub add: HashMap<TypeId, (TypeId, BinaryMethodCompiler, bool)>,
    pub sub: HashMap<TypeId, (TypeId, BinaryMethodCompiler, bool)>,
    pub mul: HashMap<TypeId, (TypeId, BinaryMethodCompiler, bool)>,
    pub div: HashMap<TypeId, (TypeId, BinaryMethodCompiler, bool)>,

    //second is whether it is constructed directly on the stack or moved into RET
    pub copy: Option<(MethodCompiler, bool)>,

    //todo: compilation
    pub call: HashMap<Vec<TypeId>, TypeId>,

    //types this type can implicitly convert to
    pub implicit_conversion: HashMap<TypeId, (MethodCompiler, bool)>,
}

impl OperatorOverloads {
    pub fn new() -> Self {
        Self{
            neg: None,
            
            add: HashMap::new(),
            sub: HashMap::new(),
            mul: HashMap::new(),
            div: HashMap::new(),
            
            copy: None,
            
            call: HashMap::new(),

            implicit_conversion: HashMap::new(),
        }
    }
    
    
}