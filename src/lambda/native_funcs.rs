use crate::lambda_jit::il_env::Env;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::io::stdout;


pub fn l_clear(env: &mut Env) {
    let _ = execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));
}