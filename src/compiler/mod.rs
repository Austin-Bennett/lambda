use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use crate::common::source_owner::{SourceDescriptor, SourceOwner};
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::lexer::token::{StatementToken, Token, TokenType};
use crate::lexer::tokenizer::Tokens;
pub use compile_message::CompileMessage;

pub mod modules;
mod compile_message;

use modules::*;
use crate::ast::ty::Type;
use crate::common::type_context::{TypeContext, TypeName};
use crate::common::utils::outcome::Outcome;
use crate::common::utils::progress::Progress;

pub enum CompileMessageType {
    Error,
    Warning,
    Info,
}





//keeps track of compilation context
pub struct Compiler {
    errors: Vec<CompileMessage>,
    warnings: Vec<CompileMessage>,
    //maps paths to their sources
    source_map: HashMap<SourceOwner, String>,
    modules: HashMap<ModulePath, LModule>,
    type_context: TypeContext,
}




impl Compiler {
    pub fn new() -> Self {
        Self{
            errors: Vec::new(),
            warnings: Vec::new(),
            source_map: HashMap::new(),
            modules: HashMap::new(),
            type_context: TypeContext::new(),
        }
    }
    
    pub fn resolve_types(&mut self) {
        
    }
    
    pub fn resolve_typename(&mut self, type_name: ModulePath) -> Option<&TypeName> {
        match self.type_context.get_type_by_name(&type_name) {
            None => {}
            Some(t) => {
                return Some(t);
            }
        }
        
        
        //otherwise, search for the structure name in any file, if not found,
        //return None
        for (_, m) in &self.modules {
            match &m.ast { 
                //anything besides Todo should already by in the contex
                Progress::Todo(p) => {
                    for i in p {
                        
                    }
                }
                _ => {}
            }
        }
        
        
        None
    }

    pub fn emit_compile_message(&mut self, msg: CompileMessage) {
        match msg.ty {
            CompileMessageType::Error => self.errors.push(msg),
            CompileMessageType::Warning => self.warnings.push(msg),
            CompileMessageType::Info => self.warnings.push(msg),
        }
    }
    
    fn print_message(&self, msg: &CompileMessage, ty: &str) {
        let source_string = &self.source_map[&msg.source.owner];
        
        //first print the source
        println!("Compiler {} at {:?} {} at line {} character {}:",
                 ty,
                 msg.source.owner.descriptor,
                 msg.source.owner.name,
                 msg.source.line + 1,
                 msg.source.char + 1,
        );
        let end = msg.source.offset + msg.source.len;
        println!("\"{}\"\n", &source_string[msg.source.offset..end]);
        println!("{}\n", msg.message);
        for i in &msg.notes {
            self.print_message(&i, "note")
        }
    }

    fn print_message_no_source(&self, msg: &CompileMessage, ty: &str) {

        //first print the source
        println!("Compiler {} at {:?} {} at line {} character {}:",
                 ty,
                 msg.source.owner.descriptor,
                 msg.source.owner.name,
                 msg.source.line + 1,
                 msg.source.char + 1,
        );
        let end = msg.source.offset + msg.source.len;
        println!("{}\n", msg.message);
        for i in &msg.notes {
            self.print_message(&i, "note")
        }
    }

    fn raise_messages(&self, messages: &Vec<CompileMessage>, ty: &str, with_source: bool) -> bool {
        if !messages.is_empty() {

            for e in messages {
                if with_source { self.print_message(e, ty) }
                else { self.print_message_no_source(e, ty) }
            }

            true
        } else {
            false
        }
    }



    //returns true if there were compile errors after printing them
    pub fn raise_compile_errors(&self, with_source: bool) -> bool {
        self.raise_messages(&self.errors, "error", with_source)
    }

    pub fn raise_compile_warnings(&self, with_source: bool) -> bool {
        self.raise_messages(&self.warnings, "warning", with_source)
    }
}

impl Compiler {
    fn add_module_impl(&mut self, tokens: Tokens, added: &mut HashSet<ModulePath>) {
        let modp = ModulePath::from_path(&tokens.get_owner().name);

        if self.modules.contains_key(&modp) || added.contains(&modp) {
            return;
        }

        let mut module_smap = SourceMap{
            owner: tokens.get_owner().clone(),
            offset: 0,
            line: 0,
            char: 0,
            len: 0
        };

        added.insert(modp.clone());

        let mut tks: VecDeque<Token> = VecDeque::new();
        let mut dependencies = Vec::new();
        //iterate through the tokens and find module dependencies
        for tk in tokens {
            module_smap.extend(&tk.smap);

            match &tk.typ {
                TokenType::Statement(StatementToken::UseKW(s)) => {
                    let mod_path = ModulePath::from_module_path(s);
                    if !self.modules.contains_key(&mod_path) && !added.contains(&mod_path) {
                        let path = mod_path.to_path();
                        match Tokens::tokenize(&path) {
                            Ok(tks) => {
                                self.add_module_impl(tks, added)
                            }
                            Err(e) => {
                                self.emit_compile_message(CompileMessage::new(
                                    tk.smap.clone(), 
                                    format!("Failed to find module {:?} due to error: {}", &path, e),
                                    CompileMessageType::Error
                                ))
                            }
                        }
                    }
                    dependencies.push(mod_path);
                },

                TokenType::User(_) => {},

                TokenType::CompileWarning(message) => {
                    self.emit_compile_message(CompileMessage::new(
                        tk.smap.clone(),
                        message.clone(),
                        CompileMessageType::Warning,
                    ))
                }

                TokenType::CompileError(message) => {
                    self.emit_compile_message(CompileMessage::new(
                        tk.smap.clone(),
                        message.clone(),
                        CompileMessageType::Error,
                    ))
                }


                _ => tks.push_back(tk)
            }



        }

        let module = LModule::parse_untyped(modp.clone(), tks, module_smap, dependencies, self);
        self.modules.insert(
            modp,
            module
        );
    }

    pub fn add_module(&mut self, owner: SourceOwner, source: String) {

        self.source_map.insert(owner.clone(), source.clone());

        let tokens = Tokens::tokenize_string(owner, source);
        let mut hmap = HashSet::new();

        self.add_module_impl(tokens, &mut hmap)
    }
}

impl AsRef<HashMap<ModulePath, LModule>> for Compiler {

    fn as_ref(&self) -> &HashMap<ModulePath, LModule>  {
        &self.modules
    }
}

impl AsMut<HashMap<ModulePath, LModule>> for Compiler {

    fn as_mut(&mut self) -> &mut HashMap<ModulePath, LModule> {
        &mut self.modules
    }
}