#![feature(generic_const_exprs)]
use anyhow::Result;
use crate::lexer::token::{FeatureToken, Token, TokenType};
use crate::lexer::tokenizer::Tokens;

pub mod lexer;
mod common;

mod tests {
    use crate::common::primitives::decimal128::LDecimal128;
    use crate::common::primitives::decimal64::LDecimal64;

    #[test]
    pub fn test_decimalf64() {
        let pi = LDecimal64::new(3141592653589793238, 0);
        println!("{}", pi);

        let pi2 = LDecimal64::new(3141592653589793238, -100);
        println!("{}", pi2);


        let pi2 = LDecimal64::new(0, -18);
        println!("{}", pi2);


        let pi = LDecimal64::new(-3141592653589793238, 100);
        println!("{}", pi);

        let pi2 = LDecimal64::new(-3141592653589793238, -100);
        println!("{}", pi2);


        let pi2 = LDecimal64::new(-3141592653589793238, -18);
        println!("{}", pi2);
    }

    #[test]
    pub fn test_decimal128() {
        let pi = LDecimal128::new(31415926535897932384626433832795028841, 0);
        println!("{}", pi);

        let pi2 = LDecimal128::new(31415926535897932384626433832795028841, -100);
        println!("{}", pi2);

        let pi2 = LDecimal128::new(31415926535897932384626433832795028841, -37);
        println!("{}", pi2);

        let pi2 = LDecimal128::new(0, -37);
        println!("{}", pi2);



        let pi = LDecimal128::new(-31415926535897932384626433832795028841, 100);
        println!("{}", pi);

        let pi2 = LDecimal128::new(-31415926535897932384626433832795028841, -100);
        println!("{}", pi2);


        let pi2 = LDecimal128::new(-31415926535897932384626433832795028841, -37);
        println!("{}", pi2);
    }
}

fn main() -> Result<()> {
    let tokens = Tokens::tokenize("test.lm")?.collect::<Vec<Token>>();

    for tk in &tokens {
        if let TokenType::Feature(FeatureToken::StatementEnd) = tk.typ {
            println!()
        } else {
            print!("{:?} ", tk.typ);
        }
    }
    
    Ok(())
}
