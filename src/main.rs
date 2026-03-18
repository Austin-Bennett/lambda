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
pub mod tests;




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
