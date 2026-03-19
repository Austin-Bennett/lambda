use std::collections::HashMap;
use crate::common::utils::modulepath::ModulePath;

//todo: Generic args?
pub struct Type {
    pub modulepath: ModulePath,
    //if known, this is Some
    pub size: Option<usize>
}


pub struct TypeContext {
    types: Vec<Type>, //the index of a type in this array is a typeid
    
    //for quick lookup of a type by its module path
    typeids: HashMap<ModulePath, u64>,
}

impl TypeContext {

    pub fn new() -> Self {
        Self{
            types: Vec::new(),
            typeids: HashMap::new(),
        }
    }

    pub fn add_type(&mut self, ty: Type) -> u64 {
        let res = self.types.len() as u64;
        
        self.typeids.insert(ty.modulepath.clone(), res);
        self.types.push(ty);
        
        res
    }

}