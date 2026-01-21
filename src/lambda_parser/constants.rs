use std::f64::consts::{E, PI};
use crate::lambda_parser::{Env, Variable};

pub fn init(map: &mut Env) {
    map.insert("pi", Variable::Number(PI));
    map.insert("e", Variable::Number(E));
}