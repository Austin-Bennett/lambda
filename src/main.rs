#![feature(likely_unlikely)]
#![feature(stmt_expr_attributes)]

use crate::lambda_parser::ExprNode;
use crate::tests::{test_interpreter};
use std::env;
use std::io::Write;
use crate::lambda_jit::il_env::Env;
use crate::lambda_jit::lambda_il::{BytecodeBuilder, DataLocation, IArg, Instruction, Register, Value};

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
    use crate::lambda_jit::lambda_il::{BytecodeBuilder, DataLocation, IArg, Instruction, Register, Value};
    use std::io;




    pub fn test_interpreter() -> () {

        let mut env = Env::new();

        env.add_native_function("clear", l_clear);
        env.add_native_function("sqrt", l_sqrt);

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
                compiled = match compile_expr(&expr, &mut env, false) {
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
            //println!("Time: [Total: {:?}][Eval: {:?}][Tree: {:?}][Compiling: {:?}]", time + tree_time + ctime, time, tree_time, ctime);
            match result {
                Ok(v) => println!("Result: {:?}", v),
                Err(e) => println!("Failed: {:?}", e),
            }
        }
    }


}

pub fn fib(n: f64) -> f64 {
    if n <= 1.1 { return 1.0 }
    let (mut a, mut b) = (1.0, 1.0);
    let mut iter = 2.0;

    while iter < n {
        let res = a + b;
        a = b;
        b = res;
        iter += 1.0;
    }

    b
}

pub fn main() {
    let arg = &env::args().collect::<Vec<_>>()[1];

    match arg.as_str() {
        "interpreter" => {
            let _ = test_interpreter();
        }

        s => eprintln!("Unknown test: {}", s),
    }
    // let mut env = Env::new();
    //
    // let recur_bc = {
    //     let mut builder = BytecodeBuilder::new();
    //
    //     builder
    //         .decl_label("fib")
    //         .emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Bottom))))
    //         .emit(Instruction::Store(DataLocation::Register(Register::Bottom), IArg::Data(DataLocation::Register(Register::Stack))))
    //         .emit(Instruction::Cmp(IArg::Data(DataLocation::StackBottomOffset(-3)), IArg::Value(Value::numf64(1.0))))
    //         .jump_greater("recur")
    //         .emit(Instruction::Store(DataLocation::Register(Register::Ret), IArg::Value(Value::numf64(1.0))))
    //         .jump("end")
    //         .decl_label("recur")
    //         //fib - 1
    //         .emit(Instruction::Push(IArg::Data(DataLocation::StackBottomOffset(-3))))
    //         .emit(Instruction::Sub(DataLocation::StackStackOffset(-1), IArg::Value(Value::numf64(1.0))))
    //         .call("fib")
    //         .emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Ret))))
    //         //fib - 2
    //         .emit(Instruction::Push(IArg::Data(DataLocation::StackBottomOffset(-3))))
    //         .emit(Instruction::Sub(DataLocation::StackStackOffset(-1), IArg::Value(Value::numf64(2.0))))
    //         .call("fib")
    //         .emit(Instruction::Add(DataLocation::Register(Register::Ret), IArg::Data(DataLocation::StackStackOffset(-2))))
    //         .emit(Instruction::PopN(3))
    //         //return
    //         .decl_label("end")
    //         .emit(Instruction::Pop(DataLocation::Register(Register::Bottom)))
    //         .emit(Instruction::Return)
    //
    //         //main
    //         .decl_label("main")
    //         .emit(Instruction::Push(IArg::Value(Value::numf64(30.0))))
    //         .call("fib")
    //         .emit(Instruction::PopN(1))
    //         .emit(Instruction::Return);
    //
    //
    //     builder.build().unwrap()
    // };
    //
    // let iter_bc = {
    //     let mut builder = BytecodeBuilder::new();
    //
    //     builder
    //         .decl_label("fib")
    //         .emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Bottom))))
    //         .emit(Instruction::Store(DataLocation::Register(Register::Bottom), IArg::Data(DataLocation::Register(Register::Stack))))
    //         .emit(Instruction::Cmp(IArg::Data(DataLocation::StackBottomOffset(-3)), IArg::Value(Value::numf64(1.0))))
    //         .emit(Instruction::Store(DataLocation::Register(Register::R2), IArg::Value(Value::numf64(1.0))))
    //         .jump_less_eq("end")
    //         //declare a and b at r1 and r2
    //         .emit(Instruction::Store(DataLocation::Register(Register::R1), IArg::Value(Value::numf64(1.0))))
    //         //declare iter at r3
    //         .emit(Instruction::Store(DataLocation::Register(Register::R3), IArg::Value(Value::numf64(2.0))))
    //         //loop
    //         .decl_label("loop")
    //         //if iter >= n goto end
    //         .emit(Instruction::Cmp(IArg::Data(DataLocation::Register(Register::R3)), IArg::Data(DataLocation::StackBottomOffset(-3))))
    //         .jump_greater_eq("end")
    //
    //         //r4 = a + b
    //         .emit(Instruction::Store(DataLocation::Register(Register::R4), IArg::Data(DataLocation::Register(Register::R1))))
    //         .emit(Instruction::Add(DataLocation::Register(Register::R4), IArg::Data(DataLocation::Register(Register::R2))))
    //         //a = b
    //         .emit(Instruction::Store(DataLocation::Register(Register::R1), IArg::Data(DataLocation::Register(Register::R2))))
    //         //b = r4
    //         .emit(Instruction::Store(DataLocation::Register(Register::R2), IArg::Data(DataLocation::Register(Register::R4))))
    //         //r3 += 1
    //         .emit(Instruction::Add(DataLocation::Register(Register::R3), IArg::Value(Value::numf64(1.0))))
    //         .jump("loop")
    //
    //         .decl_label("end")
    //         //return b
    //         .emit(Instruction::Pop(DataLocation::Register(Register::Bottom)))
    //         .emit(Instruction::Store(DataLocation::Register(Register::Ret), IArg::Data(DataLocation::Register(Register::R2))))
    //         .emit(Instruction::Return)
    //         //main
    //         .decl_label("main")
    //         .emit(Instruction::Push(IArg::Value(Value::numf64(30.0))))
    //         .call("fib")
    //         .emit(Instruction::PopN(1))
    //         .emit(Instruction::Return);
    //
    //     builder.build().unwrap()
    // };
    //
    // let res;
    // let t = time! { res = env.execute(&iter_bc).unwrap() };
    //
    // println!("Result: {:?} [{:?}]", res, t);
    //
    // // let res; let t = time!{ res = fib(30.0); };
    // //
    // // println!("Result: {:?} [{:?}]", res, t);
}