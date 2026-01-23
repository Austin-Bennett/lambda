use crate::lambda_jit::lambda_il::{Bytecode, BytecodeBuilder, DataLocation, IArg, Instruction, Register, Value};
use crate::lambda_parser::ExprNode;

//compiles the expression to a function
pub fn compile_expr(expr: &ExprNode) -> Result<Bytecode, String> {
    let mut builder = BytecodeBuilder::new()
        .emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Bottom))))
        .emit(Instruction::Store(DataLocation::Register(Register::Bottom), IArg::Data(DataLocation::Register(Register::Stack))));
    let mut label = 0;

    builder = compile_node(expr, builder, &mut label)?;


    builder = builder
        .emit(Instruction::Pop(DataLocation::Register(Register::Bottom)))
        .emit(Instruction::Return);
    builder.build()
}

fn compile_node(expr: &ExprNode, mut bc: BytecodeBuilder, label: &mut i32) -> Result<BytecodeBuilder, String> {
    match expr {
        ExprNode::Argument(u) => {
            bc = bc
                .emit(Instruction::Store(DataLocation::Register(Register::Ret), IArg::Data(DataLocation::StackRegOffset(Register::Bottom, -3 - (*u as isize)))))
        }
        ExprNode::CallOperation { caller, args } => {
            if let ExprNode::Ident(s) = &**caller {
                //push all the arguments
                for i in args {
                    bc = compile_node(i, bc, label)?
                        .emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Ret))));
                }
                //now call the dynamic function
                bc = bc.emit(Instruction::CallDynamic(s.clone()))
                //we need to pop all the arguments off the stack
                    .emit(Instruction::PopN(args.len()));
            } else {
                //multiplication, args must be of length 1
                if args.len() != 1 {
                    return Err("Can only multiply expression with 1 argument when using call syntax".to_string())
                }
                //compile the rhs
                bc = compile_node(&args[0], bc, label)?;
                //push rhs
                bc = bc.emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Ret))));

                //compile the lhs (the caller)
                bc = compile_node(caller.as_ref(), bc, label)?
                //multiply
                    .emit(Instruction::Mul(DataLocation::Register(Register::Ret), IArg::Data(DataLocation::StackRegOffset(Register::Stack, -1))))
                //pop
                    .emit(Instruction::PopN(1));
            }
        }
        ExprNode::BinaryOperation { op, operands } => {
            let lhs = &operands.0;

            let rhs = &operands.1;

            bc = compile_node(rhs, bc, label)?
                .emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Ret))));

            bc = compile_node(lhs, bc, label)?;

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
                        IArg::Value(Value::Num(0.0))))
                        .jump_less(l.clone())
                        .emit(Instruction::Store(DataLocation::Register(Register::Ret),
                        IArg::Value(Value::Num(1.0))))
                        .decl_label(l)
                        ;


                }
                "==" => {
                    let l = label.to_string();
                    *label += 1;
                    bc = bc
                        .emit(Instruction::Store(DataLocation::Register(Register::Ret),
                                                 IArg::Value(Value::Num(0.0))))
                        .jump_zero(l.clone())
                        .emit(Instruction::Store(DataLocation::Register(Register::Ret),
                                                 IArg::Value(Value::Num(1.0))))
                        .decl_label(l)
                    ;
                }

                o => return Err(format!("Unknown operator: {}", o))
            }
            //once we've done the operation we need to pop from the stack
            bc = bc.emit(Instruction::PopN(1))
        }
        ExprNode::UnaryOperation { op, operand } => {
            bc = compile_node(operand.as_ref(), bc, label)?;
            match *op {
                "-" => {
                    bc = bc
                        .emit(Instruction::Mul(DataLocation::Register(Register::Ret), IArg::Value(Value::Num(-1.0))));
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
                .emit(Instruction::Store(DataLocation::Register(Register::Ret), IArg::Value(Value::Num(*n))))
        }
        ExprNode::Void => {}
        ExprNode::Error(e) => return Err(e.clone())
    }

    Ok(bc)
}