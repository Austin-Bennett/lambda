use crate::lambda_parser::ExprNode;
use crate::tests::{test_bytecode, test_interpreter};
use std::env;
use std::io::Write;

mod lambda_parser;
mod macros;
mod lambda_jit;
mod lambda;

mod tests {
    use super::*;
    use crate::lambda::jit::compile_expr;
    use crate::lambda::native_funcs::*;
    use crate::lambda_jit::il_env;
    use crate::lambda_jit::il_env::Env;
    use crate::lambda_jit::lambda_il::Instruction::*;
    use crate::lambda_jit::lambda_il::{BytecodeBuilder, DataLocation, IArg, Register, Value};
    use std::io;




    pub fn test_interpreter() -> ! {

        let mut env = Env::new();

        env.add_native_function("clear", l_clear);
        let _ =compile_expr(&ExprNode::from_str("f(n) = if(n < 2, 1, f(n-1)+f(n-2))").unwrap(), &mut env, false);


        let mut s = String::new();

        loop {
            env.reg_ret = Value::Void;
            s.clear();
            print!("> ");
            let _ = io::stdout().flush();
            let _ = io::stdin().read_line(&mut s);

            if s.starts_with("inspect") {
                println!("{:?}", env.dynamic_functions);
                println!("{:?}", env.dynamic_values);
                continue;
            }


            let tree_time;
            let expr = match {
                let res;
                tree_time = time!{ res = ExprNode::from_str(&s); };
                res
            } {
                Ok(e) => e,
                Err(e) => {
                    println!("Failed to compile expression: {}", e);
                    continue;
                }
            };


            let compiled;

            let ctime = time! {
                compiled = match compile_expr(&expr, &mut env, true) {
                    Ok(c) => match env.link(c) {
                        Ok(c) => c,
                        Err(e) => {
                            println!("Failed to link expression: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        println!("Failed to compile expression: {}", e);
                        continue;
                    }
                };
            };



            //println!("Compiled [entry={}]:\n{:?}", compiled.entry, compiled);

            let result;
            let time = time!{
                result = env.execute(&compiled);
            };

            match result {
                Ok(v) => println!("Result: {:?} [Eval: {:?}][Tree: {:?}][Compiling: {:?}]", v, time, tree_time, ctime),
                Err(e) => println!("Failed: {:?} [Eval: {:?}][Tree: {:?}][Compiling: {:?}]", e, time, tree_time, ctime),
            }
        }
    }


}


pub fn main() {
    let arg = &env::args().collect::<Vec<_>>()[1];

    match arg.as_str() {
        "interpreter" => {
            let _ = test_interpreter();
        }

        s => eprintln!("Unknown test: {}", s),
    }
}