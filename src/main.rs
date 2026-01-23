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
        let code = BytecodeBuilder::new()
            .decl_label("add")
            .emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Bottom))))
            .emit(Instruction::Store(DataLocation::Register(Register::Bottom), IArg::Data(DataLocation::Register(Register::Stack))))
            //bottom points to the previous stack bottom + 1, so bottom is at bottom -1, return addr is at bottom -2, y at bottom-3, and x at bottom-4
            .emit(Instruction::Store(DataLocation::Register(Register::Ret), IArg::Data(DataLocation::StackRegOffset(Register::Bottom, -4))))
            .emit(Instruction::Add(DataLocation::Register(Register::Ret), IArg::Data(DataLocation::StackRegOffset(Register::Bottom, -3))))
            .emit(Instruction::Pop(DataLocation::Register(Register::Bottom)))
            .emit(Instruction::Return)
            .decl_label("main")
            .emit(Instruction::Push(IArg::Value(Value::Num(2.0))))
            .emit(Instruction::Push(IArg::Value(Value::Num(2.0))))
            .call("add")
            .emit(Instruction::PopN(2))
            .emit(Instruction::Return)
            .build().unwrap();

        let mut env = il_env::Env::new();

        match env.execute(&code, true) {
            Ok(v) => {
                println!("Result: {:?}", v)
            }
            Err(e) => {
                println!("Error: {:?}", e);
                println!("STACK: {}, PC: {}, BOTTOM: {}, RET: {:?}", env.reg_stack, env.reg_pc, env.reg_bottom, env.reg_ret);
                for i in 0..env.stack.len() {
                    print!("STACK[{}]: {:?}", i, env.stack[i]);
                    if i == env.reg_stack {
                        println!(" <- STACK");
                    } else if i == env.reg_bottom {
                        println!(" <- BOTTOM");
                    } else {
                        println!();
                    }
                }
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