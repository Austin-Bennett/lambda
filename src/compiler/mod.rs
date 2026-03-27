use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::mem;
use crate::common::source_owner::SourceOwner;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::lexer::token::{StatementToken, Token, TokenType};
use crate::lexer::tokenizer::Tokens;
pub use compile_message::CompileMessage;

pub mod modules;
pub mod compile_message;

use modules::*;
use crate::ast::Item;
use crate::ast::statements::vardecl::VarDecl;
use crate::ast::structure::lstruct;
use crate::ast::ty::Type;
use crate::typed_ast::ast::items::function::FunctionSignature;
use crate::typed_ast::ast::statements::vardecl::TypedVarDecl;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::{StructId, StructInfo, StructMember, TypeId, TypeInfo, TypeKind};

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
    untyped_modules: HashMap<ModulePath, LModule>,
    typed_modules: HashMap<ModulePath, LTypedModule>,

    _findset: HashSet<String>,

    pub type_context: TypeContext,
}




impl Compiler {
    pub fn new() -> Self {
        Self{
            errors:          Vec::new(),
            warnings:        Vec::new(),
            source_map:      HashMap::new(),
            untyped_modules: HashMap::new(),
            typed_modules:   HashMap::new(),
            type_context:    TypeContext::new(),
            _findset:        HashSet::new(),
        }
    }

    pub fn get_untyped_modules(&self) -> &HashMap<ModulePath, LModule> {
        &self.untyped_modules
    }

    pub fn get_typed_modules(&self) -> &HashMap<ModulePath, LTypedModule> { &self.typed_modules }
    
    //adds primitive types to the type context
    pub fn create_typed_ast(&mut self) {
        //take ownership of the modules
        let modules = mem::take(&mut self.untyped_modules);

        let mut context = AvailableContext::new();
        context.push_new_scope();

        //loop through all functions, create their identifiers ahead of time
        //add to the global scope
        for (p, m) in &modules {
            for item in &m.ast {
                if let Item::Func(func) = item {
                    //create its signature
                    let ret = match &func.data.ty {
                        Some(ty) => {
                            let Some((_, id)) = self.resolve_type(ty) else {

                                self.emit_compile_message(
                                    CompileMessage::new(
                                        func.smap.clone(),
                                        format!("Could not resolve type {:?}", ty),
                                        CompileMessageType::Error,
                                    )
                                );

                                continue;
                            };

                            id
                        },
                        None => self.type_context.none
                    };

                    let mut params = Vec::new();

                    for p in &func.data.parameters {
                        let (_, id) = match self.resolve_type(&p.data.ty) {
                            Some(v ) => v,
                            None => {
                                self.emit_compile_message(
                                    CompileMessage::new(
                                        p.smap.clone(),
                                        format!("Could not resolve type: {:?}", p.data.ty),
                                        CompileMessageType::Error,
                                    )
                                );

                                continue;
                            }
                        };

                        params.push(id);
                    }

                    //register this type
                    //this is similar to what rust does as well, they did it to help
                    //the optimization process, I did it because im lazy
                    let (info, id) = self.type_context.add(
                        Type::Typename(func.data.to_typename()),
                        TypeInfo::new(
                            TypeKind::Function { ret, params: params.clone() },
                            TypeContext::SIZE_POINTER, TypeContext::SIZE_POINTER //size of a pointer because it is a pointer...?
                        )
                    );
                    
                    info.ops.call.insert(params, ret);
                    
                    context.declare_identifier_in_scope(func.data.name.clone(), id);
                }
            }
        }

        for (p, m) in &modules {

            let m = LTypedModule::from_ast(m, self, &mut context);
            self.typed_modules.insert(p.clone(), m);
        }

        //return ownership
        self.untyped_modules = modules;
    }
    
    pub fn get_structure(&mut self, name: &String) -> Option<(&mut StructInfo, StructId)> {
        if let Some(s) = unsafe { &mut * (&raw mut self.type_context) }.get_struct(name) {
            Some(s)
        } else if self._findset.contains(name) {
            None
        } else {
            self._findset.insert(name.clone());
            //let this = self as *mut Self;
            let mut res = None;
            {
                'outer: for (_, m) in &self.untyped_modules {
                    for item in &m.ast {
                        if let Item::Struct(s) = item && s.data.name == *name {
                            res = Some(s);
                            break 'outer;
                        }
                    }
                }
            }

            //we have to clone to obey the borrow checker
            let Some(structure) = res.cloned() else { return None; };

            let Some(structure) = self.create_structure_from_ast(&structure.data, &structure.smap) else { return None; };


            Some(self.type_context.add_struct(structure.name.clone(), structure))
        }
    }

    pub fn create_structure_from_ast(&mut self, ast: &lstruct::Structure, smap: &SourceMap) -> Option<StructInfo> {

        let mut align = 1;
        let mut raw_offset = 0;
        let mut members = Vec::new();
        let mut raw_size = 0;

        for VarDecl{ name, ty, value: _ } in &ast.members {
            //get the type
            let Some((info, id)) = self.resolve_type(ty) else {
                self.emit_compile_message(
                    CompileMessage::new(
                        smap.clone(),
                        format!("Unknown type: {:?}", ty),
                        CompileMessageType::Error,
                    )
                );
                return None;
            };

            raw_size += info.size;

            align = align.max(info.align);
            members.push(
                StructMember{
                    name: name.clone(),
                    size: info.size,
                    ty: id,
                    offset: raw_offset,
                }
            );

            raw_offset += info.size;
        }


        let mut accum_offset = 0;
        let mut size = 0;

        //align members
        for m in &mut members {
            if (m.offset + accum_offset) % align != 0 {
                //we need to increase this members offset
                accum_offset += align - ((m.offset + accum_offset) % align);
            }
            m.offset += accum_offset;
            size = m.offset + m.size;
        }

        Some(StructInfo{
            name: ast.name.clone(),
            members,
            size,
            padding: size - raw_size,
            align,
            type_id: 0, //to be set
        })
    }

    pub fn resolve_typename(&mut self, path: &String) -> Option<(&mut TypeInfo, TypeId)> {
        if let Some(res) = unsafe { &mut *(&raw mut self.type_context) }.resolve_type(&Type::Typename(path.clone())) {
            Some(res)
        } else {
            //look for a structure in any of the modules
            self._findset.clear();
            if let Some((info, id)) = self.get_structure(path) {
                let tid = info.type_id;

                Some((self.type_context.get_by_id_mut(tid)?, tid))
            } else {
                None
            }
        }
    }

    pub fn resolve_type(&mut self, ty: &Type) -> Option<(&mut TypeInfo, TypeId)> {
        if let Some(res) = unsafe{ (&mut *( &raw mut self.type_context )) .resolve_type(ty) } {
            Some(res)
        } else {
            match ty {
                Type::Typename(name) => {
                    if let Some(res) = self.resolve_typename(name) {
                        Some(res)
                    } else {
                        None
                    }
                }
                Type::Reference(t) => {
                    if let Some((ti, id)) = self.resolve_type(t) {
                        Some(self.type_context.add(
                            Type::Reference(t.clone()),
                            TypeInfo::new(
                                TypeKind::Reference(id),
                                TypeContext::SIZE_POINTER, TypeContext::SIZE_POINTER
                            )
                        ))
                    } else {
                        None
                    }
                }
                Type::Pointer(t) => {
                    if let Some((ti, id)) = self.resolve_type(t) {
                        Some(self.type_context.add(
                            Type::Pointer(t.clone()),
                            TypeInfo::new(
                                TypeKind::Pointer(id),
                                TypeContext::SIZE_POINTER, TypeContext::SIZE_POINTER
                            )
                        ))
                    } else {
                        None
                    }
                }
                Type::Slice(t) => {
                    if let Some((ti, id)) = self.resolve_type(t) {
                        Some(self.type_context.add(
                            Type::Slice(t.clone()),
                            TypeInfo::new(
                                TypeKind::Slice(id),
                                TypeContext::SIZE_POINTER * 2, TypeContext::SIZE_POINTER
                            )
                        ))
                    } else {
                        None
                    }
                }
                Type::Array { ty: t, size } => {
                    if let Some((ti, id)) = self.resolve_type(t) {
                        let r = ty;
                        let ray_size = ti.size * size;
                        let align = ti.align;

                        let res = self.type_context.add(r.clone(), TypeInfo::new(
                            TypeKind::Array { ty: id, size: *size },
                            ray_size, align
                        ));

                        Some(res)
                    } else {
                        None
                    }
                }
            }
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

        if self.untyped_modules.contains_key(&modp) || added.contains(&modp) {
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
                    if !self.untyped_modules.contains_key(&mod_path) && !added.contains(&mod_path) {
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
        self.untyped_modules.insert(
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

