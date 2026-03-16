use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use crate::common::source_owner::SourceOwner;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::lexer::modules::{LModule};
use crate::lexer::token::{StatementToken, Token, TokenType};
use crate::lexer::tokenizer::Tokens;

pub struct CompileMessage {
    source: SourceMap,
    message: String,
}

pub enum CompilerError<T> {
    Ok(T),
    None,
    Err(CompileMessage)
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

    pub fn emit_compile_warning(&mut self, source: SourceMap, message: String) {
        self.warnings.push(CompileMessage {
            source,
            message
        });
    }

    pub fn emit_compile_error(&mut self, source: SourceMap, message: String) {
        self.errors.push(CompileMessage {
            source,
            message
        });
    }

    fn raise_messages(&self, messages: &Vec<CompileMessage>, ty: &str) -> bool {
        if !messages.is_empty() {

            for e in messages {
                let source_string = &self.source_map[&e.source.owner];
                //first print the source
                println!("Compile {} in {:?} {} at line {} character {}:",
                         ty,
                         e.source.owner.descriptor,
                         e.source.owner.name,
                         e.source.line + 1,
                         e.source.char + 1,
                );
                let end = e.source.offset + e.source.len;
                println!("\"{}\"\n", &source_string[e.source.offset..end]);
                println!("{}", e.message);
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
                                self.emit_compile_error(tk.smap.clone(), format!("Failed to find module {:?} due to error: {}", &path, e))
                            }
                        }
                    }
                    dependencies.push(mod_path);
                },

                TokenType::User(_) => {},

                TokenType::CompileWarning(message) => {
                    self.emit_compile_warning(tk.smap.clone(), message.clone());
                }

                TokenType::CompileError(message) => {
                    self.emit_compile_error(tk.smap.clone(), message.clone());
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