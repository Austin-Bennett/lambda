use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use crate::common::source_owner::SourceOwner;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::lexer::token::{StatementToken, Token, TokenType};
use crate::lexer::tokenizer::Tokens;
pub use compile_message::CompileMessage;

pub mod modules;
mod compile_message;

use modules::*;

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
}


impl Compiler {
    pub fn new() -> Self {
        Self{
            errors: Vec::new(),
            warnings: Vec::new(),
            source_map: HashMap::new(),
            modules: HashMap::new(),
        }
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

    fn raise_messages(&self, messages: &Vec<CompileMessage>, ty: &str) -> bool {
        if !messages.is_empty() {

            for e in messages {
                self.print_message(e, ty)
            }

            true
        } else {
            false
        }
    }


    //returns true if there were compile errors after printing them
    pub fn raise_compile_errors(&self) -> bool {
        self.raise_messages(&self.errors, "error")
    }

    pub fn raise_compile_warnings(&self) -> bool {
        self.raise_messages(&self.warnings, "warning")
    }
}

impl Compiler {
    fn add_module_impl(&mut self, tokens: Tokens, added: &mut HashSet<ModulePath>) {
        let modp = ModulePath::from_path(&tokens.get_owner().name);

        if self.modules.contains_key(&modp) || added.contains(&modp) {
            return;
        }

        added.insert(modp.clone());

        let mut tks: Vec<Token> = Vec::new();
        let mut dependencies = Vec::new();
        //iterate through the tokens and find module dependencies
        for tk in tokens {


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


                _ => tks.push(tk)
            }



        }

        self.modules.insert(modp, LModule {
            tokens: tks,
            dependencies
        });
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