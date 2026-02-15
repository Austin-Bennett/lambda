use std::fs::read_to_string;
use rust_decimal::Decimal;
use crate::lambda_parser::statement::Statement;
use crate::lambda_parser::tokenize;

mod lambda_parser;
mod macros;
mod lvm;

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