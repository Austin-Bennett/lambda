use std::ops::{Deref, Mul};
use crate::lambda_jit::il_env::Env;
use crate::lambda_jit::lambda_il::{Bytecode, BytecodeBuilder, DataLocation, IArg, Instruction, Register, Value};
use crate::lambda_jit::lambda_il::DataLocation::Dynamic;
use crate::lambda_parser::ExprNode;

//compiles the expression to a function, does not handle the '=' operator since that
//is a dynamic feature technically
pub fn compile_expr(expr: &ExprNode, env: &mut Env, is_func: bool) -> Result<Bytecode, String> {

    let mut builder = BytecodeBuilder::new();

    if is_func {
        builder
        .emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Bottom))))
            .emit(Instruction::Store(DataLocation::Register(Register::Bottom), IArg::Data(DataLocation::Register(Register::Stack))));
    }
    let mut label = 0;


    let v = compile_node(expr, &mut builder, &mut label, env, &DataLocation::Register(Register::Ret))?;
    if let Some(v) = v {
        builder.emit(Instruction::Store(DataLocation::Register(Register::Ret), IArg::Value(v)));
    }

    if is_func {
        builder
            .emit(Instruction::Pop(DataLocation::Register(Register::Bottom)))
            .emit(Instruction::Return);
    }
    builder.build()
}






fn compile_node<'a>(expr: &ExprNode,
                    bc: &'a mut BytecodeBuilder,
                    label: &mut i32,
                    env: &mut Env,
                    output: &DataLocation
) -> Result<Option<Value>, String> {
    match expr {
        ExprNode::InlineInstruction(i) => {
            bc.emit(i.clone());
        }
        ExprNode::Argument(u) => {
            bc
                .emit(Instruction::Store(output.clone(), IArg::Data(DataLocation::StackBottomOffset(-3 - (*u as isize)))));
        }
        ExprNode::CallOperation { caller, args } => {
            if let ExprNode::Ident(s) = &**caller {

                match s.as_str() {
                    //special cases
                    "if" => {
                        //expect 3 arguments
                        if args.len() != 3 {
                            return Err("Expected if statement to have 3 parameters!".to_string());
                        }

                        let fl = label.to_string();
                        *label += 1;
                        let el = label.to_string();
                        *label += 1;

                        //compile the first to be used as the determinant
                        //store the result in compare
                        //use that to determine where to jump
                        let v = compile_node(&args[0], bc, label, env, &DataLocation::Register(Register::Cmp))?;


                        if let Some(v) = v {
                            bc.emit(Instruction::Store(DataLocation::Register(Register::Cmp), IArg::Value(v)));
                        }

                        bc.jump_zero(&fl);

                        let v = compile_node(&args[1], bc, label, env, output)?;

                        if let Some(v) = v {
                            bc.emit(Instruction::Store(output.clone(), IArg::Value(v)));
                        }

                            bc.jump(&el)
                            .decl_label(&fl);

                        let v = compile_node(&args[2], bc, label, env, output)?;

                        if let Some(v) = v {
                            bc.emit(Instruction::Store(output.clone(), IArg::Value(v)));
                        }

                        bc.decl_label(&el);
                    },
                    "sum" => {
                        if args.len() != 3 {
                            return Err("Expected if statement to have 3 parameters!".to_string());
                        }

                        //first result is the start
                        if let Some(v) = compile_node(&args[0], bc, label, env, &DataLocation::Register(Register::R1))? {
                            bc.emit(Instruction::Store(Dynamic("n".to_string()), IArg::Value(v)));
                        } else {
                            bc.emit(Instruction::Store(Dynamic("n".to_string()),
                                                       IArg::Data(DataLocation::Register(Register::R1))));
                        }

                        bc.emit(Instruction::Push(IArg::Value(Value::numf64(0.0)))); //stack-2

                        if let Some(v) = compile_node(&args[1], bc, label, env, &DataLocation::Register(Register::R1))? {
                            bc.emit(Instruction::Push(IArg::Value(v)));
                        } else {
                            bc.emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::R1))));//stack-1
                        }



                        let l_start = label.to_string();
                        *label += 1;
                        let l_end = label.to_string();
                        *label += 1;



                        //the loop
                        bc
                            .decl_label(l_start.clone())
                            .emit(Instruction::Cmp(IArg::Data(DataLocation::Dynamic("n".to_string())),
                                        IArg::Data(DataLocation::StackStackOffset(-1))))
                            .jump_greater(l_end.clone());

                        //bc.emit(Instruction::DebugPrint(DataLocation::Dynamic("n".to_string())));

                        if let Some(v) = compile_node(&args[2], bc, label, env, &DataLocation::Register(Register::R1))? {
                            bc.emit(Instruction::Add(DataLocation::StackStackOffset(-2), IArg::Value(v)));
                        } else {
                            bc.emit(Instruction::Add(DataLocation::StackStackOffset(-2),
                                                     IArg::Data(DataLocation::Register(Register::R1))));
                        }

                        bc
                            .emit(Instruction::Add(DataLocation::Dynamic("n".to_string()),
                                                   IArg::Value(Value::numf64(1.0))))
                            .jump(l_start)
                            .decl_label(l_end)
                            .emit(Instruction::Store(DataLocation::Register(Register::Ret),
                                                     IArg::Data(DataLocation::StackStackOffset(-2))))
                            .emit(Instruction::PopN(2));


                    }

                    //default:
                    s => {
                        //push all the arguments
                        for i in args {
                            let v = compile_node(i, bc, label, env, output)?;
                            //push to stack
                            if let Some(v) = v{
                                bc.emit(Instruction::Push(IArg::Value(v)));
                            } else {
                                bc.emit(Instruction::Push(IArg::Data(output.clone())));
                            }
                        }

                        //now call the dynamic function
                        bc
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
                let v = compile_node(&args[0], bc, label, env, output)?;

                //allocate the rhs
                let (rhs, pop) = if let Some(v) = v {
                    (IArg::Value(v), false)
                } else {
                    bc.emit(Instruction::Push(IArg::Data(output.clone())));
                    (IArg::Data(DataLocation::StackStackOffset(-1)), true)
                };

                //compile the lhs (the caller)
                let v = compile_node(caller.as_ref(), bc, label, env, output)?;


                //multiply
                if let Some(v) = v {
                    bc.emit(Instruction::Store(output.clone(), IArg::Value(v)));
                }

                //multiply
                bc.emit(Instruction::Mul(output.clone(), rhs));
                if pop { bc.emit(Instruction::PopN(1)); }

            }
        }
        ExprNode::BinaryOperation { op, operands } => {
            let lhs = &operands.0;

            let rhs = &operands.1;



            let v =  compile_node(rhs, bc, label, env, output)?;



            let (rhs, pop) = if let Some(v) = v {
                (IArg::Value(v), false)
            } else {
                //push to stack
                bc.emit(Instruction::Push(IArg::Data(output.clone())));
                (IArg::Data(DataLocation::StackStackOffset(-1)), true)
            };

            if let ExprNode::Ident(s) = lhs && *op == "=" {
                bc.emit(Instruction::Store(DataLocation::Dynamic(s.clone()), rhs));
                if pop {
                    bc.emit(Instruction::PopN(1));
                }
                return Ok(None)
            }

            let v =  compile_node(lhs, bc, label, env, output)?;

            if let Some(v) = v {
                bc.emit(Instruction::Store(output.clone(), IArg::Value(v)));
            }

            //emit the compare instruction if this is a comparison
            if *op == "<" || *op == "<=" || *op == ">" || *op == ">=" || *op == "==" || *op == "!=" {
                bc
                    .emit(Instruction::Cmp(IArg::Data(output.clone()),
                        rhs.clone()));
            }

            match *op {
                "+" => {
                    bc
                        .emit(Instruction::Add(output.clone(),
                                               rhs));
                },
                "-" => {
                    bc
                        .emit(Instruction::Sub(output.clone(),
                                               rhs));
                },
                "*" => {
                    bc
                        .emit(Instruction::Mul(output.clone(),
                                               rhs));
                },
                "**" => {
                    bc.emit(Instruction::Pow(output.clone(), rhs));
                }
                "/" => {
                    bc
                        .emit(Instruction::Div(output.clone(),
                                               rhs));
                },

                "<" => {
                    let less = label.to_string();
                    *label += 1;
                    let end = label.to_string();
                    *label += 1;
                    bc
                        .jump_less(&less)
                        .emit(Instruction::Store(output.clone(), IArg::Value(Value::numf64(0.0))))
                        .jump(&end)
                        .decl_label(less)
                        .emit(Instruction::Store(output.clone(), IArg::Value(Value::numf64(1.0))))
                        .decl_label(end)
                    ;
                }
                ">" => {
                    let greater = label.to_string();
                    *label += 1;
                    let end = label.to_string();
                    *label += 1;
                    bc
                        .jump_greater(&greater)
                        .emit(Instruction::Store(output.clone(), IArg::Value(Value::numf64(0.0))))
                        .jump(&end)
                        .decl_label(greater)
                        .emit(Instruction::Store(output.clone(), IArg::Value(Value::numf64(1.0))))
                        .decl_label(end)
                    ;
                }
                "<=" => {
                    let less = label.to_string();
                    *label += 1;
                    let end = label.to_string();
                    *label += 1;
                    bc
                        .jump_less_eq(&less)
                        .emit(Instruction::Store(output.clone(), IArg::Value(Value::numf64(0.0))))
                        .jump(&end)
                        .decl_label(less)
                        .emit(Instruction::Store(output.clone(), IArg::Value(Value::numf64(1.0))))
                        .decl_label(end)
                    ;
                }
                ">=" => {
                    let greater = label.to_string();
                    *label += 1;
                    let end = label.to_string();
                    *label += 1;
                    bc
                        .jump_greater_eq(&greater)
                        .emit(Instruction::Store(output.clone(), IArg::Value(Value::numf64(0.0))))
                        .jump(&end)
                        .decl_label(greater)
                        .emit(Instruction::Store(output.clone(), IArg::Value(Value::numf64(1.0))))
                        .decl_label(end)
                    ;
                }
                "==" => {
                    let eq = label.to_string();
                    *label += 1;
                    let end = label.to_string();
                    *label += 1;
                    bc
                        .jump_zero(&eq)
                        .emit(Instruction::Store(output.clone(), IArg::Value(Value::numf64(0.0))))
                        .jump(&end)
                        .decl_label(eq)
                        .emit(Instruction::Store(output.clone(), IArg::Value(Value::numf64(1.0))))
                        .decl_label(end)
                    ;
                }
                "!=" => {
                    let neq = label.to_string();
                    *label += 1;
                    let end = label.to_string();
                    *label += 1;
                    bc
                        .jump_not_zero(&neq)
                        .emit(Instruction::Store(output.clone(), IArg::Value(Value::numf64(0.0))))
                        .jump(&end)
                        .decl_label(neq)
                        .emit(Instruction::Store(output.clone(), IArg::Value(Value::numf64(1.0))))
                        .decl_label(end)
                    ;
                }


                o => return Err(format!("Unknown operator: {}", o))
            }
            if pop {
                bc.emit(Instruction::PopN(1));
            }
        }
        ExprNode::UnaryOperation { op, operand } => {
            let v = compile_node(operand.as_ref(), bc, label, env, output)?;
            if let Some(mut v) = v {
                return match *op {
                    "-" => {
                        v = v.mul(Value::numf64(-1.0));
                        Ok(Some(v))
                    }
                    "+" => { Ok(Some(v)) },
                    s => Err(format!("Unknown unary operator: {}", s))
                }
            }
            match *op {
                "-" => {
                    bc
                        .emit(Instruction::Mul(output.clone(), IArg::Value(Value::numf64(-1.0))));
                }
                "+" => {},
                s => return Err(format!("Unknown unary operator: {}", s))
            }
        }
        ExprNode::Ident(id) => {
            bc
                .emit(Instruction::Store(output.clone(), IArg::Data(DataLocation::Dynamic(id.clone()))));
        }
        ExprNode::Num(n) => {
            return Ok( Some(Value::Num(*n)))
        }
        ExprNode::Void => {}
        ExprNode::Error(e) => return Err(e.clone()),
        ExprNode::FunctionDecl { ident, expr } => {
            let compilation = compile_expr(&*expr, env, true)?;
            env.add_dynamic_mod(ident.clone(), compilation);
        }
    }

    Ok(None)
}

impl Env {
    //replaces dynamic calls with static calls
    pub fn link(&self, main: Bytecode) -> Result<Bytecode, String> {
        let mut builder = BytecodeBuilder::new();

        let mut dyns = Vec::new();
        //figure out what dynamic functions we need
        for i in &main.code {
            if let Instruction::CallDynamic(s) = i {
                dyns.push(s);
            }
        }
        

        for i in dyns {

            if let Some(bc) = self.dynamic_functions.get(i)
            {
                builder
                    .decl_label(&i)
                    .add_code(bc.deref().clone(), true);
            }
        }
        
        builder.decl_label("main");
        
        
        //link with main
        for i in main.code {
            if let Instruction::CallDynamic(s) = i {
                builder.call(s);
            } else {
                builder.emit(i);
            }
        }
        
        
        
        builder.build()
    }
}