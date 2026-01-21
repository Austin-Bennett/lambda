use crate::lambda_parser::ExprNode;
use std::collections::HashMap;

pub struct Env {
    pub(crate) scopes: Vec<HashMap<String, Variable>>
}

impl Env {
    pub fn push_scope(&mut self, map: HashMap<String, Variable>) {
        self.scopes.push(map);
    }

    pub fn pop_scope(&mut self) -> Option<HashMap<String, Variable>> {
        self.scopes.pop()
    }

    pub fn undeclare(&mut self, name: impl AsRef<str>) -> bool {
        for map in self.scopes.iter_mut().rev() {
            if let Some(_) = map.remove(name.as_ref()) {
                return true;
            }
        }
        false
    }

    pub fn set(&mut self, name: impl AsRef<str>, val: Variable) {
        for map in self.scopes.iter_mut().rev() {
            if let Some(v) = map.get_mut(name.as_ref()) {
                *v = val;
                return;
            }
        }

        //default, insert into this map
        self.insert(name, val)
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<&Variable> {
        for map in self.scopes.iter().rev() {
            if let Some(v) = map.get(name.as_ref()) {
                return Some(v);
            }
        }

        None
    }

    #[allow(unused)]
    pub fn get_mut(&mut self, name: impl AsRef<str>) -> Option<&mut Variable> {
        for map in self.scopes.iter_mut().rev() {
            if let Some(v) = map.get_mut(name.as_ref()) {
                return Some(v);
            }
        }

        None
    }

    //auto inserts at the top level
    pub fn insert(&mut self, name: impl AsRef<str>, val: Variable) {
        if let Some(map) = self.scopes.last_mut() {
            map.insert(name.as_ref().to_string(), val);
        }
    }
}


#[derive(Clone)]
pub enum Variable {
    Number(f64),
    Expression(ExprNode),
    Function{inputs: Vec<String>, body: ExprNode},
    NativeFunction(&'static dyn Fn(&mut Env, Vec<ExprNode>) -> Result<ExprNode, String>),
}

impl Variable {

    #[allow(unused)]
    pub fn new() -> Self {
        Self::Number(0.0)
    }


    #[allow(unused)]
    pub fn num(n: f64) -> Self {
        Self::Number(n)
    }

}

