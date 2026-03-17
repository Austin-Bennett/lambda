#![feature(try_trait_v2)]
#![feature(deref_pure_trait)]

use std::fs;
use std::process::abort;
use anyhow::Result;
use clap::Parser;
use crate::common::source_owner::{SourceDescriptor, SourceOwner};
use crate::compiler::Compiler;
use crate::lexer::token::{FeatureToken, TokenType};

pub mod lexer;
pub mod common;
pub mod compiler;
pub mod ast;

#[cfg(test)]
mod tests {
    use crate::ast::expressions::ExprSyntax;
    use crate::ast::Syntax;
    use crate::common::primitives::decimal128::LDecimal128;
    use crate::common::primitives::decimal64::LDecimal64;
    use crate::common::source_owner::{SourceDescriptor, SourceOwner};
    use crate::common::utils::outcome::Outcome;
    use crate::lexer::token::TokenType;
    use crate::lexer::tokenizer::Tokens;

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


    #[test]
    pub fn test_expressions() {
        let mut tokens = Tokens::tokenize_string(
            SourceOwner{
                descriptor: SourceDescriptor::RustString,
                name: "test_expressions_test_string".to_string()
            },
            "(b b + sqrt(-b - 4a c)) / 2a".to_string()
        )
            .filter(|tk| if let TokenType::User(_) = tk.typ { false } else { true })
            .collect();


        let expr = ExprSyntax::parse(&mut tokens);

        match expr {
            Outcome::Ok(e) => {
                println!("{:?}", e.data)
            }
            Outcome::None => eprintln!("No expression!"),
            Outcome::Err(e) => eprintln!("Compile error: {:?}", e)
        }
    }
}


#[derive(Parser)]
pub struct Arguments {

    #[arg(num_args = 1.., value_name = "FILE")]
    files: Vec<String>
}


fn main() -> Result<()> {
    #[allow(unused_mut)]
    let mut args = Arguments::parse();




    let mut compiler = Compiler::new();

    for file in &args.files {
        match fs::read_to_string(file) {
            Ok(f) => {
                compiler.add_module(
                    SourceOwner::new(
                        SourceDescriptor::File,
                        file.clone()
                    ),
                    f
                )
            },
            Err(e) => eprintln!("{:?}", e)
        }
    }


    compiler.raise_compile_warnings();
    if compiler.raise_compile_errors() {
        abort();
    }
    
    

    for (modp, m) in compiler.as_ref() {
        println!("{}", modp);
        
        for tk in &m.tokens {
            if let TokenType::Feature(FeatureToken::StatementEnd) = tk.typ {
                println!()
            } else if let TokenType::User(_) = tk.typ {
                continue;
            } else {
                print!("{:?} ", tk.typ)
            }
        }
        println!()
    }
    
    Ok(())
}
