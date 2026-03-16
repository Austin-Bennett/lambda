use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use crate::common::utils::modulepath::ModulePath;
use crate::lexer::token::{StatementToken, Token, TokenType};
use crate::lexer::tokenizer::Tokens;

pub struct LModule {
    //filtered to not include user tokens
    pub tokens: Vec<Token>,
    pub dependencies: Vec<ModulePath>, //module dependencies
}


impl LModule {

}




