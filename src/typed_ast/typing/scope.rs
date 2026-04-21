use std::collections::HashMap;
use crate::typed_ast::typing::ty::TypeId;




pub struct ScopeContext<I> {
    locals: HashMap<String, I>,
}

impl<I> ScopeContext<I> {
    pub fn new() -> Self {
        Self{
            locals: HashMap::new(),
        }
    }
}


pub struct AvailableContext<I> {
    scopes: Vec<ScopeContext<I>>
}

impl<'a, I> AvailableContext<I> {
    pub fn new() -> Self {
        Self{
            scopes: Vec::new(),
        }
    }
    
    pub fn get_identifier(&self, id: &String) -> Option<&I> {
        //walk through all scopes looking for the id

        for scope in &self.scopes {
            if let Some(ty) = scope.locals.get(id) {
                return Some(ty);
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

    pub fn declare_identifier_in_scope(&mut self, ident: String, d: I) {

        let scope = self.scopes.last_mut().unwrap();

        scope.locals.insert(ident, d);
    }
}