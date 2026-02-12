use std::fs::read_to_string;
use rust_decimal::Decimal;
use crate::lambda_parser::statement::Statement;
use crate::lambda_parser::tokenize;

mod lambda_parser;

fn main() {
    let fdata = read_to_string("test.lm").unwrap();

    let mut tokens = tokenize(fdata);

    let code = Statement::from_tokens(&mut tokens).unwrap();

    println!("{:?}", code);

    println!("{:?}", size_of::<Decimal>())
}