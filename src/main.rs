#![feature(adt_const_params)]
#![feature(pattern)]

use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use clap::Parser;
use crate::context::Context;



pub mod lexer;
pub mod context;
pub mod utils;
pub mod lang;

#[cfg(test)]
pub mod tests;

#[derive(Parser)]
pub struct CompilerArguments {
    #[arg(num_args = 0.., required = true, value_name = "FILE")]
    file: Vec<PathBuf>
}



fn main() {
    let args = CompilerArguments::parse();

    let context = Arc::new(RwLock::new(
        Context::new(args)
    ));
    
    


}
