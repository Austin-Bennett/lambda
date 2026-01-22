use crate::lambda_parser::native_funcs::lambda_native;
use crate::lambda_parser::{constants, Env, ExprNode, Variable};
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::mem::swap;
use crate::tests::test_bytecode;

mod lambda_parser;
mod macros;
mod lambda_jit;


mod tests {
    use crate::lambda_jit::il_env;
    use crate::lambda_jit::il_env::EnvError;
    use crate::lambda_jit::lambda_il::{BytecodeBuilder, DataLocation, IArg, Instruction, Register, Value};
    use super::*;

    pub fn test_interpreter() -> std::io::Result<()> {
        print!("> ");
        std::io::stdout().flush()?;
        let mut inp = String::new();
        std::io::stdin().read_line(&mut inp)?;
        inp = inp.trim().to_string();

        let mut varmap = Env { scopes: vec![HashMap::new()] };

        lambda_native::init(&mut varmap);
        constants::init(&mut varmap);

        loop {
            match ExprNode::from_str(&inp) {
                Ok(n) => {
                    let solved;
                    let time = time! { solved = n.solve(&mut varmap) };
                    match solved {
                        Ok(n) => {
                            if let ExprNode::Void = n {} else {
                                println!("{:?} [{:?}]", n, time);
                                varmap.insert("ans", Variable::Expression(n));
                            }
                        }
                        Err(s) => println!("Error: {}", s)
                    }
                }
                Err(e) => {
                    println!("Error: {}", e)
                }
            }

            print!("> ");
            std::io::stdout().flush()?;
            inp.clear();
            std::io::stdin().read_line(&mut inp)?;
            inp = inp.trim().to_string();
        }
    }

    pub fn test_bytecode() {
        let bc = BytecodeBuilder::new()
            .exec(Instruction::Store(DataLocation::Register(Register::Ret), IArg::Value(Value::Num(3.0))))
            .exec(Instruction::ADD(DataLocation::Register(Register::Ret), IArg::Value(Value::Num(3.0))))
            .exec(Instruction::Return)
            .build();

        let mut env = il_env::Env::new();
        
        match env.execute(&bc) {
            Ok(v) => {
                println!("{:?}", v);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

pub fn main() {
    let arg = &env::args().collect::<Vec<_>>()[1];

    match arg.as_str() {
        "interpreter" => {
            let _ = tests::test_interpreter();
        }
        "bytecode" => {
            let _ = test_bytecode();
        }

        s => eprintln!("Unknown test: {}", s),
    }
}