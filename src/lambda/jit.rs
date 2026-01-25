use std::ops::Deref;
use crate::lambda_jit::il_env::Env;
use crate::lambda_jit::lambda_il::{Bytecode, BytecodeBuilder, DataLocation, IArg, Instruction, Register, Value};
use crate::lambda_parser::ExprNode;

//compiles the expression to a function, does not handle the '=' operator since that
//is a dynamic feature technically
pub fn compile_expr(expr: &ExprNode, env: &mut Env, is_func: bool) -> Result<Bytecode, String> {

    let mut builder = BytecodeBuilder::new();

    if is_func {
        builder = builder
        .emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Bottom))))
            .emit(Instruction::Store(DataLocation::Register(Register::Bottom), IArg::Data(DataLocation::Register(Register::Stack))));
    }
    let mut label = 0;

    builder = compile_node(expr, builder, &mut label, env, is_func)?;

    if is_func {
        builder = builder
            .emit(Instruction::Pop(DataLocation::Register(Register::Bottom)))
            .emit(Instruction::Return);
    }
    builder.build()
}

fn compile_node(expr: &ExprNode, mut bc: BytecodeBuilder, label: &mut i32, env: &mut Env, is_func: bool) -> Result<BytecodeBuilder, String> {
    match expr {
        ExprNode::Argument(u) => {
            bc = bc
                .emit(Instruction::Store(DataLocation::Register(Register::Ret), IArg::Data(DataLocation::StackRegOffset(Register::Bottom, -3 - (*u as isize)))))
        }
        ExprNode::CallOperation { caller, args } => {
            if let ExprNode::Ident(s) = &**caller {

                match s.as_str() {
                    //special cases
                    "if" => {
                        //expect 3 arguments
                        if args.len() != 3 {
                            return Err("Expected if statement to have 3 parameter!".to_string());
                        }

                        let fl = label.to_string();
                        *label += 1;
                        let el = label.to_string();
                        *label += 1;

                        //compile the first to be used as the determinant
                        //store the result in compare
                        //use that to determine where to jump
                        bc = compile_node(&args[0], bc, label, env, is_func)?
                            .emit(Instruction::Store(DataLocation::Register(Register::Cmp), IArg::Data(DataLocation::Register(Register::Ret))))
                            .jump_zero(&fl);

                        bc = compile_node(&args[1], bc, label, env, is_func)?
                            .jump(&el)
                            .decl_label(&fl);

                        bc = compile_node(&args[2], bc, label, env, is_func)?;

                        bc = bc.decl_label(&el);
                    }

                    //default:
                    s => {
                        //push all the arguments
                        for i in args {
                            bc = compile_node(i, bc, label, env, is_func)?
                                .emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Ret))));
                        }

                        //now call the dynamic function
                        bc = bc
                            .emit(Instruction::CallDynamic(s.to_string()))
                            //we need to pop all the arguments off the stack
                            .emit(Instruction::PopN(args.len()));
                    }
                }


            } else {
                //multiplication, args must be of length 1
                if args.len() != 1 {
                    return Err("Can only multiply expression with 1 argument when using call syntax".to_string())
                }
                //compile the rhs
                bc = compile_node(&args[0], bc, label, env, is_func)?;
                //push rhs
                bc = bc.emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Ret))));

                //compile the lhs (the caller)
                bc = compile_node(caller.as_ref(), bc, label, env, is_func)?
                //multiply
                    .emit(Instruction::Mul(DataLocation::Register(Register::Ret), IArg::Data(DataLocation::StackRegOffset(Register::Stack, -1))))
                //pop
                    .emit(Instruction::PopN(1));
            }
        }
        ExprNode::BinaryOperation { op, operands } => {
            let lhs = &operands.0;

            let rhs = &operands.1;

            bc = compile_node(rhs, bc, label, env, is_func)?
                .emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Ret))));

            bc = compile_node(lhs, bc, label, env, is_func)?;

            //emit the compare instruction if this is a comparison
            if *op == "<" || *op == "==" {
                bc = bc
                    .emit(Instruction::Cmp(IArg::Data(DataLocation::Register(Register::Ret)),
                        IArg::Data(DataLocation::StackRegOffset(Register::Stack, -1))));
            }

            match *op {
                "+" => {
                    bc = bc
                        .emit(Instruction::Add(DataLocation::Register(Register::Ret),
                                               IArg::Data(DataLocation::StackRegOffset(Register::Stack, -1))));
                },
                "-" => {
                    bc = bc
                        .emit(Instruction::Sub(DataLocation::Register(Register::Ret),
                                               IArg::Data(DataLocation::StackRegOffset(Register::Stack, -1))));
                },
                "*" => {
                    bc = bc
                        .emit(Instruction::Mul(DataLocation::Register(Register::Ret),
                                               IArg::Data(DataLocation::StackRegOffset(Register::Stack, -1))));
                },
                "/" => {
                    bc = bc
                        .emit(Instruction::Div(DataLocation::Register(Register::Ret),
                                               IArg::Data(DataLocation::StackRegOffset(Register::Stack, -1))));
                },

                "<" => {
                    let l = label.to_string();
                    *label += 1;
                    bc = bc
                        .emit(Instruction::Store(DataLocation::Register(Register::Ret),
                                                IArg::Value(Value::numf64(1.0))))
                        .jump_less(l.clone())
                        .emit(Instruction::Store(DataLocation::Register(Register::Ret),
                                                IArg::Value(Value::numf64(0.0))))
                        .decl_label(l)
                    ;
                }
                "==" => {
                    let l = label.to_string();
                    *label += 1;
                    bc = bc
                        .emit(Instruction::Store(DataLocation::Register(Register::Ret),
                                                 IArg::Value(Value::numf64(1.0))))
                        .jump_zero(l.clone())
                        .emit(Instruction::Store(DataLocation::Register(Register::Ret),
                                                 IArg::Value(Value::numf64(0.0))))
                        .decl_label(l)
                    ;
                }

                o => return Err(format!("Unknown operator: {}", o))
            }
            //once we've done the operation we need to pop from the stack
            bc = bc.emit(Instruction::PopN(1))
        }
        ExprNode::UnaryOperation { op, operand } => {
            bc = compile_node(operand.as_ref(), bc, label, env, is_func)?;
            match *op {
                "-" => {
                    bc = bc
                        .emit(Instruction::Mul(DataLocation::Register(Register::Ret), IArg::Value(Value::numf64(-1.0))));
                }
                "+" => {},
                s => return Err(format!("Unknown operator: {}", s))
            }
        }
        ExprNode::Ident(id) => {
            bc = bc
                .emit(Instruction::Store(DataLocation::Register(Register::Ret), IArg::Data(DataLocation::Dynamic(id.clone()))));
        }
        ExprNode::Num(n) => {
            bc = bc
                .emit(Instruction::Store(DataLocation::Register(Register::Ret), IArg::Value(Value::Num(n.clone()))))
        }
        ExprNode::Void => {}
        ExprNode::Error(e) => return Err(e.clone()),
        ExprNode::FunctionDecl { ident, expr } => {
            let compilation = compile_expr(&*expr, env, true)?;
            env.add_dynamic_mod(ident.clone(), compilation);
        }
    }

    Ok(bc)
}

impl Env {
    //replaces dynamic calls with static calls
    pub fn link(&self, main: Bytecode) -> Result<Bytecode, String> {
        let mut builder = BytecodeBuilder::new();
        
        //first add all dynamic functions inside us
        for (i, bc) in &self.dynamic_functions {
            builder = builder
                .decl_label(&i)
                .add_code(bc.deref().clone());
        }
        
        builder = builder.decl_label("main");
        
        
        //link with main
        for i in main.code {
            if let Instruction::CallDynamic(s) = i {
                builder = builder.call(s);
            } else {
                builder = builder.emit(i);
            }
        }
        
        builder.build()
    }
}