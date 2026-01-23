use crate::lambda_jit::lambda_il::{Bytecode, BytecodeBuilder, DataLocation, IArg, Instruction, Register, Value};
use crate::lambda_parser::ExprNode;

//compiles the expression to a function
pub fn compile_expr(expr: &ExprNode) -> Result<Bytecode, String> {
    let mut builder = BytecodeBuilder::new();
    let mut label = 0;

    builder = compile_node(expr, builder, &mut label)?;


    builder = builder
        .emit(Instruction::Return);
    builder.build()
}

fn compile_node(expr: &ExprNode, mut bc: BytecodeBuilder, label: &mut i32) -> Result<BytecodeBuilder, String> {
    match expr {
        ExprNode::CallOperation { caller, args } => {
            if let ExprNode::Ident(s) = &**caller {
                todo!()
            } else {
                todo!();
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
        }
        ExprNode::UnaryOperation { op, operand } => { todo!() }
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