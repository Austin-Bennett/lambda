use crate::lambda_jit::lambda_il::{Bytecode, BytecodeBuilder, DataLocation, IArg, Instruction, Register};
use crate::lambda_parser::ExprNode;

//compiles the expression to a function
pub fn compile_expr(expr: ExprNode) -> Result<Bytecode, String> {
    let mut builder = BytecodeBuilder::new();
    //setup stack
    builder = builder
        .emit(Instruction::Push(IArg::Data(DataLocation::Register(Register::Bottom))))
        .emit(Instruction::Store(DataLocation::Register(Register::Bottom), IArg::Data(DataLocation::Register(Register::Stack))));





    builder = builder
        .emit(Instruction::Pop(DataLocation::Register(Register::Bottom)))
        .emit(Instruction::Return);
    builder.build()
}