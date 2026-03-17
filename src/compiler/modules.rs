use crate::common::utils::modulepath::ModulePath;
use crate::lexer::token::Token;

pub struct LModule {
    pub tokens: Vec<Token>,
    pub dependencies: Vec<ModulePath>, //module dependencies
}


impl LModule {
    
}