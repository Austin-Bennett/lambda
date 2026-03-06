extern crate core;

use std::alloc::Layout;
use crate::lvm::lexecutable::{LExecutableBuilder, PseudoInstruction};
use crate::lambda_parser::statement::{Block, Statement};
use crate::lambda_parser::tokenize;
use std::fs::read_to_string;
use std::{fs, ptr};
use crate::compiler::Compiler;
use crate::lvm::lbc::*;
use crate::lvm::lenv::LEnv;
use crate::lvm::lheap::LHeap;

mod lambda_parser;
mod lvm;
mod macros;
mod utils;
pub mod compiler;




//#[inline(always)]
pub fn test_heap() {
    let mut heap = LHeap::new();

    let (h_pointer, ptr);
    let time = time!{
        (h_pointer, ptr)= heap.alloc(Layout::array::<i32>(10).unwrap());
    };

    println!("Alloc Time: {:?}", time);

    let ptr = ptr as *mut i32;

    println!("allocation: {:?}", ptr);

    let free;
    let time = time!{
        free = heap.free(h_pointer);
    };

    println!("Free Time: {:?}", time);


    println!("Free: {}", free);
    println!("Double free: {}", heap.free(h_pointer));
}

pub fn test_compiler() {
    let mut compiler = Compiler::new();

    let file_data = fs::read_to_string("test.lm").unwrap();
    let mut tokens = tokenize(file_data);
    let block = Statement::from_tokens(&mut tokens).unwrap();

    let program = compiler.compile_block_executable(&block);


    println!("ENTRY: {}", program.entry);
    for (i, is) in program.instructions.iter().enumerate() {
        println!("{}: {:?}", i, is)
    }

    let mut env = LEnv::new();

    let time = time! {
        env.exec(&program, 0);
    };

    println!("res: {} [{:?}]", f64::from_bits( env.state.r_ret ), time);
}

fn main() {
    test_compiler()
}
