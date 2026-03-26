use std::collections::{HashMap, HashSet, VecDeque};
use crate::common::utils::modulepath::ModulePath;
use crate::typed_ast::typing::ty::{TypeId};




pub struct ScopeContext {
    locals: HashMap<String, TypeId>,
}

impl ScopeContext {
    pub fn new() -> Self {
        Self{
            locals: HashMap::new(),
        }
    }
}


pub struct AvailableContext {
    scopes: Vec<ScopeContext>
}

impl<'a> AvailableContext {
    pub fn new() -> Self {
        Self{
            scopes: Vec::new(),
        }
    }
    
    pub fn get_identifier_type(&self, id: &String) -> Option<TypeId> {
        //walk through all scopes looking for the id

        for scope in &self.scopes {
            if let Some(ty) = scope.locals.get(id) {
                return Some(*ty);
            }
        }

        None

    }

    pub fn push_new_scope(&mut self) {
        self.scopes.push(ScopeContext::new());
    }

    pub fn pop_last_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare_identifier_in_scope(&mut self, ident: String, id: TypeId) {

        let scope = self.scopes.last_mut().unwrap();

        scope.locals.insert(ident, id);
    }
}