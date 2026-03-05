use crate::lvm::lbc::DataSize::QWord;
use crate::lvm::lenv::LPTR;


#[derive(Copy, Clone)]
#[derive(Debug)]
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





#[derive(Copy, Clone, Debug)]
pub enum Instruction {

    StackAlloc(u64),
    StackFree(u64),

    //push a 64-bit sized value onto the stack
    Push(Register),
    
    //pop a 64-bit sized value
    Pop(Register),

    //pops N 64 bit values off the stack
    PopN(u64),
    Exit,
    
    //64-bit moves
    Mov(Register, u64),
    MovR(Register, Register),
    MovPtrReg(u64, Register),
    MovRegPtr(Register, u64),
    MovBottom(Register, i64), //moves the stack value from bottom offset by the specified value into the specified register
    
    //64-bit signed comparison
    Cmp,
    
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
    //performs on Ret and Aux
    Add,
    Sub,
    Mul,
    Div,

    //negates the sign of the RET register
    Negate,
    
}
