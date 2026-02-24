use crate::lambda_parser::statement::Statement;
use crate::lambda_parser::tokenize;
use std::fs::read_to_string;

mod lambda_parser;
mod lvm;
mod macros;
mod utils;

mod tests {
    use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

    use crate::lvm::lheap::LHeap;

    #[test]
    pub fn test_heap() {
        let mut heap = LHeap::new()
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

    println!(
        "tokenization: {:?}, tree building: {:?}",
        token_gen_time, tree_building_time
    );
}
