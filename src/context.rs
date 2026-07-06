use crate::CompilerArguments;
use crate::lang::source_map::{Source, SourceMap};
use crate::utils::registry::Registry;

pub enum CompileMessageKind {
    Warning,
    TokenError,
}
pub struct CompileMessage {
    kind: CompileMessageKind,
    message: Option<String>,
    source: SourceMap,
}


pub struct Context {
    compiler_arguments: CompilerArguments,
    errors: Vec<CompileMessage>,
    sources: Registry<Source>,
    
}

impl Context {
    pub fn new(args: CompilerArguments) -> Self {
        Self{
            compiler_arguments: args,
            errors: Vec::new(),
            sources: Registry::new()
        }
    }
    
    pub fn emit_warning(&mut self, source: SourceMap, message: Option<impl Into<String>>) {
        self.errors.push(CompileMessage{
            kind: CompileMessageKind::Warning,
            message: message.map(|v| v.into()),
            source
        })
    }
    
    pub fn emit_token_error(&mut self, source: SourceMap, message: Option<impl Into<String>>) {
        self.errors.push(CompileMessage{
            kind: CompileMessageKind::TokenError,
            message: message.map(|v| v.into()),
            source
        });
    }
}