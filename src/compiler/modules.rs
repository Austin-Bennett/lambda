use std::collections::VecDeque;
use crate::ast;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::common::utils::progress::Progress;
use crate::common::utils::Todo;
use crate::compiler::CompileMessage;
use crate::lexer::token::Token;

pub struct LModule {
    pub smap: SourceMap,
    pub ast: Progress<Vec<ast::Item>, Todo, Todo>,
    pub dependencies: Vec<ModulePath>, //module dependencies
}


impl LModule {
    
}

impl ast::Syntax for LModule {
    fn parse(tokens: &mut VecDeque<Token>) -> Outcome<Self, CompileMessage>
    where
        Self: Sized
    {
        todo!()
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}