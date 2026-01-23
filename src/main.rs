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
mod lambda;

mod tests {
    use crate::lambda_jit::il_env;
    use crate::lambda_jit::il_env::EnvError;
    use crate::lambda_jit::lambda_il::{BytecodeBuilder, DataLocation, IArg, Instruction, Register, Value};
    use crate::lambda_jit::lambda_il::Instruction::*;
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

        //bytecode that calculates fibonacci(30)
        let code = BytecodeBuilder::new()
            .decl_label("fib")
            .emit(Push(IArg::Data(DataLocation::Register(Register::Bottom))))
            .emit(Store(DataLocation::Register(Register::Bottom), IArg::Data(DataLocation::Register(Register::Stack))))
            .emit(Store(DataLocation::Register(Register::Ret), IArg::Value(Value::Num(1.0))))
            .emit(Cmp(IArg::Data(DataLocation::StackRegOffset(Register::Bottom, -3)), IArg::Value(Value::Num(2.0))))
            .jump_less("end_early")
            //arg value is at bottom - 3
            .emit(Push(IArg::Data(DataLocation::StackRegOffset(Register::Bottom, -3)))) //save the instruction to stack
            .emit(Sub(DataLocation::StackRegOffset(Register::Bottom, 0), IArg::Value(Value::Num(1.0))))
            .call("fib")
            //clean up argument
            .emit(PopN(1))
            //save the return result to the stack at bottom + 0
            .emit(Push(IArg::Data(DataLocation::Register(Register::Ret))))
            //push the arg back onto the stack
            .emit(Push(IArg::Data(DataLocation::StackRegOffset(Register::Bottom, -3))))
            .emit(Sub(DataLocation::StackRegOffset(Register::Bottom, 1), IArg::Value(Value::Num(2.0))))
            .call("fib")
            .emit(Add(DataLocation::StackRegOffset(Register::Bottom, 0), IArg::Data(DataLocation::Register(Register::Ret))))
            .emit(Store(DataLocation::Register(Register::Ret), IArg::Data(DataLocation::StackRegOffset(Register::Bottom, 0))))
            .emit(PopN(2))
            .decl_label("end_early")
            .emit(Pop(DataLocation::Register(Register::Bottom)))
            .emit(Return)
            .decl_label("main")
            .emit(Push(IArg::Value(Value::Num(30.0))))
            .call("fib")
            .emit(PopN(1))
            .emit(Return)
            .build().unwrap();

        let mut env = il_env::Env::new();
        let res;
        let t = time! {
            res = env.execute(&code, false);
        };

        match res {
            Ok(v) => {
                println!("Result: {:?} [{:?}]", v, t)
            }
            Err(e) => {
                println!("Error: {:?} [{:?}]", e, t);
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