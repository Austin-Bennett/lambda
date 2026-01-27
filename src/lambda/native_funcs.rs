use crate::lambda_jit::il_env::Env;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::io::stdout;
use rust_decimal::{Decimal, MathematicalOps};
use crate::lambda_jit::lambda_il::Value;

pub fn constants(env: &mut Env) {
    env.dynamic_values.insert("pi".to_string(), Value::Num(Decimal::PI));
}
pub fn l_clear(env: &mut Env) {
    let _ = execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));
}

pub fn l_sqrt(env: &mut Env) {
    let arg = env.stack[env.reg_bottom-3];

    env.reg_ret = Value::Num(arg.to_number().sqrt().unwrap_or(Decimal::new(0, 0)));
}