#![feature(likely_unlikely)]
#![feature(stmt_expr_attributes)]

use std::io::{stdin, stdout, Write};
use std::sync::{Arc, Mutex};
use std::sync::atomic::spin_loop_hint;
use crate::lambda::jit::compile_expr;
use crate::lambda::native_funcs::{constants, l_clear, l_quit, l_sqrt};
use crate::lambda_jit::il_env::Env;
use crate::lambda_jit::lambda_il::Value::Void;
use crate::lambda_parser::ExprNode;

mod lambda_parser;
mod macros;
mod lambda_jit;
mod lambda;

struct SendPtr<T>(*mut T);

impl<T> SendPtr<T> {
    pub unsafe fn set(&mut self, val: T) {
        unsafe { *self.0 = val }
    }

    pub unsafe fn get(&mut self) -> &T {
        unsafe { &*self.0 }
    }

    pub unsafe fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0 }
    }
}

unsafe impl<T> Send for SendPtr<T> {}

pub fn main() {
    let mut env = Env::new();

    constants(&mut env);
    env.add_native_function("quit", l_quit);
    env.add_native_function("clear", l_clear);
    env.add_native_function("sqrt", l_sqrt);

    let mut rptr = SendPtr(&mut env.run as *mut bool);
    ctrlc::set_handler(move || {
        println!("Interrupted");
        //not thread safe at all, but who honestly cares, this shouldn't do too much damage
        unsafe{ rptr.set(false); }

    }).expect("Failed to set ctrlc handler");


    let mut s = String::new();

    loop {
        env.reg_ret = Void;
        s.clear();
        print!("> ");
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut s);
        s = s.trim().to_string();
        if s.is_empty() {
            continue;
        }

        let expr = match ExprNode::from_str(& s) {
            Ok(v) => v,
            Err(e) => {
                println!("Parse Error: {}", e);
                continue;
            }
        };

        let bc = {
            match compile_expr(&expr, &mut env, false) {
                Ok(v) => {
                    match env.link(v) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("Linker error: {}", e);
                            continue;
                        }
                    }
                },
                Err(e) => {
                    println!("Compile Error: {}", e);
                    continue;
                }
            }
        };

        println!("Compiled [entry: {:?}]:\n{:?}", bc.entry, bc);

        match env.execute(&bc) {
            Ok(v) => println!("Result: {:?}", v),
            Err(e) => println!("Runtime Error: {}", e)
        }

    }
}