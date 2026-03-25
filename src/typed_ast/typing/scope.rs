use std::collections::{HashMap, HashSet, VecDeque};
use crate::common::utils::modulepath::ModulePath;
use crate::typed_ast::typing::ty::{TypeId};

pub struct GlobalContext {
    functions: HashMap<ModulePath, TypeId> //matches functions to their functional type id's
}

pub struct ScopeContext {
    locals: HashMap<ModulePath, TypeId>,
}

pub struct AvailableContext<'a> {
    global_context: &'a GlobalContext,
    scopes: VecDeque<ScopeContext>
}

impl<'a> AvailableContext<'a> {
    pub fn new(global_context: &'a GlobalContext) -> Self {
        Self{
            global_context,
            scopes: VecDeque::new(),
        }
    }
    
    pub fn get_identifier_type(&self, id: &ModulePath) -> Option<TypeId> {
        //walk through all scopes looking for the id
        if let Some(ty) = self.global_context.functions.get(id) {
            Some(*ty)
        } else {
            for scope in &self.scopes {
                if let Some(ty) = scope.locals.get(id) {
                    return Some(*ty);
                }
            }
            
            None
        }
    }
}