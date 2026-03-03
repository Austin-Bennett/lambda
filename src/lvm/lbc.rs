use crate::lvm::lbc::DataSize::QWord;
use crate::lvm::lenv::LPTR;


#[derive(Copy, Clone)]
pub enum Register {
    Ret,
    Aux,
    Cmp,
    Stack,
    Bottom,
    Pc,
}


#[repr(usize)]
#[derive(Copy, Clone)]
pub enum DataSize {
    Byte = 1,
    Word = 2,
    DWord = 4,
    QWord = 8
}





#[derive(Copy, Clone)]
pub enum Instruction {
    
    //push a 64-bit sized value onto the stack
    Push(Register),
    
    //pop a 64-bit sized value
    Pop(Register),
    Exit,
    
    //64-bit moves
    Mov(Register, u64),
    MovPtrReg(u64, Register),
    MovRegPtr(Register, u64),
    
    //64-bit signed comparison
    Cmp(Register, Register),
    
    //call operation, pushes the current PC onto the stack and jumps to the specified address
    Call(u64),
    //return, pops the return address into PC
    Return,
    
    //jumps
    Jump(u64),
    JumpLess(u64),
    JumpGreater(u64),
    JumpLessEq(u64),
    JumpGreaterEq(u64),
    JumpEq(u64),
    JumpNEq(u64),
    
    //64-bit signed integer operations
    //first arg is treated as a pointer
    Add(Register, Register),
    Sub(Register, Register),
    Mul(Register, Register),
    Div(Register, Register),
    
}