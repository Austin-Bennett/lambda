extern crate core;

use crate::lvm::lexecutable::LExecutableBuilder;
use crate::lambda_parser::statement::Statement;
use crate::lambda_parser::tokenize;
use crate::tests::*;
use std::fs::read_to_string;
use crate::lvm::lbc::*;
use crate::lvm::lenv::LEnv;

mod lambda_parser;
mod lvm;
mod macros;
mod utils;
pub mod compiler;

mod tests {
    use std::{alloc::Layout, ptr, ptr::{slice_from_raw_parts, slice_from_raw_parts_mut}};
    use std::fs::read_to_string;
    use crate::lambda_parser::statement::{Block, Statement};
    use crate::lambda_parser::tokenize;
    use crate::lvm::lheap::LHeap;
    use crate::time;

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



        let (hptr, ptr) = heap.alloc(Layout::new::<Block>());
        let ptr = ptr as *mut Block;

        let fdata = read_to_string("test.lm").unwrap();
        let mut tokens;
        let token_gen_time = time! {
            tokens = tokenize(fdata);
        };




        let tree_building_time = time! {
            unsafe{ ptr::write(ptr, Statement::from_tokens(&mut tokens).unwrap()); }
        };
        let code = unsafe{
            &mut *ptr
        };

        println!("{:?}", code);

        println!(
            "tokenization: {:?}, tree building: {:?}",
            token_gen_time, tree_building_time
        );

        println!( "{}", heap.free(hptr) );
    }
}

fn main() {

    let executable = {
        let mut builder = LExecutableBuilder::new();
        
        builder
            .add(Instruction::Mov(Register::Ret, 2))
            .add(Instruction::Mov(Register::Aux, 2))
            //.add(Instruction::Add(Register::Ret, Register::Aux))
            .add(Instruction::Exit);
        
        builder.build()
    };
    
    let mut env = LEnv::new();

    let t = time! {
        env.exec(&executable);
    };


    println!("{} ({:?})", env.state.r_ret, t)

}
