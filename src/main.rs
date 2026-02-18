use std::fs::read_to_string;
use rust_decimal::Decimal;
use crate::lambda_parser::statement::Statement;
use crate::lambda_parser::tokenize;

mod lambda_parser;
mod macros;
mod lvm;

mod tests {
    use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
    use crate::lvm::lheap::LHeap;

    #[test]
    pub fn test_heap() {
        
        let mut heap = LHeap::new();
        
        //allocate some memory
        let allocation = heap.alloc(100); //100 bytes (25 32-bit integers)
        let ray =  unsafe{ &mut *slice_from_raw_parts_mut(heap.get_ptr(allocation) as *mut i32, 25) };
        
        
        ray.fill(0);
        println!("{:?}", ray);
        
        println!("{}", heap);

        ray[0] = 0xFF94;

        println!("{:?}", ray);
        println!("{}", heap);

        println!("{:?}", heap.free(allocation));
        println!("{:?}", heap.free(allocation)); //test double free


    }
}

fn main() {
    let fdata = read_to_string("test.lm").unwrap();
    let mut tokens;
    let token_gen_time = time! {
        tokens = tokenize(fdata);
    };

    let code;
    let tree_building_time = time! {
        code = Statement::from_tokens(&mut tokens).unwrap();
    };


    println!("{:?}", code);

    println!("tokenization: {:?}, tree building: {:?}", token_gen_time, tree_building_time);
}