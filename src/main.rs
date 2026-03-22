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
pub mod typed_ast;
pub mod consteval;

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


    compiler.raise_compile_warnings(true);
    if compiler.raise_compile_errors(true) {
        abort();
    }
    
    

    for (modp, m) in compiler.as_ref() {
        println!("{}", modp);
        
        for i in &m.ast {
            println!("{:?}", i);
        }
        println!();
    }
    
    Ok(())
}
