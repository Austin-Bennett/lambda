use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::mem;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::id;
use inkwell::AddressSpace;
use inkwell::types::{BasicType, BasicTypeEnum};
use crate::common::source_owner::SourceOwner;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::lexer::token::{StatementToken, Token, TokenType};
use crate::lexer::tokenizer::Tokens;
pub use compile_message::CompileMessage;

pub mod modules;
pub mod compile_message;
pub mod codegen;
pub mod intrinsic;

use modules::*;
use crate::ast::Item;
use crate::ast::statements::vardecl::VarDecl;
use crate::ast::structure::lstruct;
use crate::ast::ty::Type;
use crate::typed_ast::ast::items::function::FunctionSignature;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::{StructId, StructInfo, StructMember, TypeId, TypeInfo, TypeKind};

pub enum CompileMessageType {
    Error,
    Warning,
    Info,
}


pub struct Registry<T: Hash + Eq + Clone>  {
    pub items: Vec<T>,
    pub item_map: HashMap<T, u32>,
}

impl<T: Hash + Eq + Clone> Registry<T> {
    pub fn new() -> Self {
        Self{
            items: Vec::new(),
            item_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: T) -> u32 {
        if let Some(v) = self.item_map.get(&item) {
            return *v;
        }
        let id = self.items.len();
        self.item_map.insert(item.clone(), id as u32);
        self.items.push(item);

        id as u32
    }
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

    pub module_search_paths: Vec<PathBuf>,
    pub intrinsics: HashMap<String, intrinsic::IntrinsicMaker>,

    pub llvm_context: &'static inkwell::context::Context,
    pub type_context: TypeContext,

    pub str_literal_reg: Registry<String>,
}




impl Compiler {
    pub fn new(llvm_context: &'static inkwell::context::Context) -> Self {
        Self{
            errors:              Vec::new(),
            warnings:            Vec::new(),
            source_map:          HashMap::new(),
            untyped_modules:     HashMap::new(),
            typed_modules:       HashMap::new(),
            type_context:        TypeContext::new(llvm_context),
            llvm_context,
            _findset:            HashSet::new(),
            module_search_paths: vec![PathBuf::from("./")],
            intrinsics:          HashMap::new(),
            str_literal_reg: Registry::new(),
        }
    }

    pub fn register_intrinsic(&mut self, name: &str, ret: TypeId, maker: intrinsic::IntrinsicMaker) {
        self.type_context.register_intrinsic(name, ret);
        self.intrinsics.insert(name.to_string(), maker);
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

        // pre-register all struct types so they are available during type-checking
        for (_, m) in &modules {
            for item in &m.ast {
                if let Item::Struct(s) = item {
                    if let Some(info) = self.create_structure_from_ast(&s.data, &s.smap) {
                        self.type_context.add_struct(info.name.clone(), info);
                    }
                }
            }
        }

        //loop through all functions, create their identifiers ahead of time
        //add to the global scope
        for (_, m) in &modules {
            for item in &m.ast {
                if let Item::Func(func) = item {
                    //create its signature
                    let ret = match &func.data.ty {
                        Some(ty) => {
                            let Some(id) = self.resolve_type(ty) else {

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
                        let Some(pty) = p.data.ty.as_ref() else { continue; };
                        let id = match self.resolve_type(pty) {
                            Some(v) => v,
                            None => {
                                self.emit_compile_message(
                                    CompileMessage::new(
                                        p.smap.clone(),
                                        format!("Could not resolve type: {:?}", pty),
                                        CompileMessageType::Error,
                                    )
                                );
                                continue;
                            }
                        };
                        params.push(id);
                    }

                    
                    //get this functions type
                    let id = self.type_context.add_functional_type(
                        
                        &FunctionSignature{
                            name: func.data.name.clone(),
                            ret,
                            params
                        }
                    );

                    
                    context.declare_identifier_in_scope(func.data.name.clone(), id);
                }
            }
        }

        // pre-register all methods from modify blocks
        for (_, m) in &modules {
            for item in &m.ast {
                if let Item::Modify(modify) = item {
                    let crate::ast::ty::Type::Typename(type_name) = &modify.data.ty else {
                        self.emit_compile_message(CompileMessage::new(
                            modify.smap.clone(),
                            "modify blocks only support named types (e.g. Point, int32)".into(),
                            CompileMessageType::Error,
                        ));
                        continue;
                    };
                    let type_name = type_name.clone();
                    let Some(type_id) = self.resolve_type(&modify.data.ty) else {
                        self.emit_compile_message(CompileMessage::new(
                            modify.smap.clone(),
                            format!("Unknown type in modify block: {}", type_name),
                            CompileMessageType::Error,
                        ));
                        continue;
                    };

                    for method in &modify.data.methods {
                        let mangled = format!("{}_{}", type_name, method.name);

                        if self.type_context.get_by_id(type_id)
                            .map_or(false, |i| i.methods.contains_key(&method.name))
                        {
                            self.emit_compile_message(CompileMessage::new(
                                modify.smap.clone(),
                                format!("duplicate method '{}' on type '{}'", method.name, type_name),
                                CompileMessageType::Error,
                            ));
                            continue;
                        }

                        let ret = match &method.ret {
                            Some(ty) => self.resolve_type(ty).unwrap_or(self.type_context.none),
                            None => self.type_context.none,
                        };

                        let mut params = Vec::new();
                        if method.has_self {
                            let self_ref_ty = self.type_context.reference_to(type_id);
                            params.push(self_ref_ty);
                        }
                        for p in &method.params {
                            if let Some(pty) = p.data.ty.as_ref() {
                                if let Some(pid) = self.resolve_type(pty) {
                                    params.push(pid);
                                }
                            }
                        }

                        let fn_type_id = self.type_context.add_functional_type(&FunctionSignature {
                            name: mangled.clone(),
                            ret,
                            params,
                        });

                        context.declare_identifier_in_scope(mangled.clone(), fn_type_id);

                        self.type_context.get_by_id_mut(type_id)
                            .unwrap()
                            .methods
                            .insert(method.name.clone(), (mangled, fn_type_id, method.public));
                    }

                    for op in &modify.data.operators {
                        let op_mangled = format!("{}__op_{}", type_name, op.op_name);

                        let ret = match &op.ret {
                            Some(ty) => self.resolve_type(ty).unwrap_or(self.type_context.none),
                            None => self.type_context.none,
                        };

                        let mut params = Vec::new();
                        if op.has_self {
                            let self_ref_ty = self.type_context.reference_to(type_id);
                            params.push(self_ref_ty);
                        }
                        for p in &op.params {
                            if let Some(pty) = p.data.ty.as_ref() {
                                if let Some(pid) = self.resolve_type(pty) {
                                    params.push(pid);
                                }
                            }
                        }

                        let fn_type_id = self.type_context.add_functional_type(&FunctionSignature {
                            name: op_mangled.clone(),
                            ret,
                            params: params.clone(),
                        });

                        context.declare_identifier_in_scope(op_mangled.clone(), fn_type_id);

                        // Determine rhs type (first non-self param)
                        let rhs_ty = if op.has_self && params.len() > 1 {
                            params[1]
                        } else if !op.has_self && !params.is_empty() {
                            params[0]
                        } else {
                            self.type_context.none
                        };

                        let type_info = self.type_context.get_by_id_mut(type_id).unwrap();
                        match op.op_name.as_str() {
                            "add"    => { type_info.ops.user_add.insert(rhs_ty, (ret, op_mangled)); }
                            "sub"    => { type_info.ops.user_sub.insert(rhs_ty, (ret, op_mangled)); }
                            "mul"    => { type_info.ops.user_mul.insert(rhs_ty, (ret, op_mangled)); }
                            "div"    => { type_info.ops.user_div.insert(rhs_ty, (ret, op_mangled)); }
                            "cmp"    => { type_info.ops.user_cmp.insert(rhs_ty, op_mangled); }
                            "assign" => { type_info.ops.user_assign.insert(rhs_ty, (ret, op_mangled)); }
                            "drop"   => { type_info.ops.drop = Some(op_mangled); }
                            other    => {
                                self.emit_compile_message(CompileMessage::new(
                                    modify.smap.clone(),
                                    format!("unknown operator '{}'; valid operators: add, sub, mul, div, cmp, assign, drop", other),
                                    CompileMessageType::Error,
                                ));
                            }
                        }
                    }
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


    
    pub fn get_structure(&mut self, name: &String) -> Option<StructId> {
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

            let Some(structure) = self.create_structure_from_ast(&structure.data, &structure.smap) else {
                return None;
            };
            let res = self.type_context.add_struct(structure.name.clone(), structure);

            Some(res)
        }
    }

    pub fn create_structure_from_ast(&mut self, ast: &lstruct::Structure, smap: &SourceMap) -> Option<StructInfo> {

        let mut members = Vec::new();

        for VarDecl{ name, ty, value: _, public } in &ast.members {
            let Some(ety) = ty.as_ref() else { continue; };
            //get the type
            let Some(id) = self.resolve_type(ety) else {
                self.emit_compile_message(
                    CompileMessage::new(
                        smap.clone(),
                        format!("Unknown type: {:?}", ety),
                        CompileMessageType::Error,
                    )
                );
                return None;
            };

            members.push(StructMember{
                name: name.clone(),
                ty: id,
                public: *public,
            });
        }


        let struct_member_types: Vec<BasicTypeEnum> = members.iter()
            .map(
                |v: &StructMember|
                    self.type_context.get_by_id(v.ty).unwrap().llvm_type.try_into().unwrap()
            )
            .collect();


        Some(StructInfo{
            name: ast.name.clone(),
            members,
            type_id: 0, //to be set
            llvm_struct: self.llvm_context.struct_type(&struct_member_types, false)
        })
    }

    pub fn resolve_typename(&mut self, path: &String) -> Option<TypeId> {
        if let Some(res) = unsafe { &mut *(&raw mut self.type_context) }.resolve_type(&Type::Typename(path.clone())) {
            Some(res)
        } else {
            //look for a structure in any of the modules
            self._findset.clear();
            if let Some(id) = self.get_structure(path) {

                self.type_context.get_struct_type(id)
            } else {
                None
            }
        }
    }

    pub fn resolve_type(&mut self, ty: &Type) -> Option<TypeId> {
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
                    if let Some(id) = self.resolve_type(t) {
                        Some(self.type_context.add(
                            Type::Reference(t.clone()),
                            TypeInfo::new(
                                TypeKind::Reference(id),
                                self.llvm_context
                                    .ptr_type(AddressSpace::try_from(0u32).unwrap())
                                    .into()
                            )
                        ))
                    } else {
                        None
                    }
                }
                Type::Pointer(t) => {
                    if let Some(id) = self.resolve_type(t) {
                        Some(self.type_context.add(
                            Type::Pointer(t.clone()),
                            TypeInfo::new(
                                TypeKind::Pointer(id),
                                self.llvm_context
                                    .ptr_type(AddressSpace::try_from(0u32).unwrap())
                                    .into()
                            )
                        ))
                    } else {
                        None
                    }
                }
                Type::Slice(t) => {
                    if let Some(id) = self.resolve_type(t) {
                        let slice_struct =
                            self.type_context.create_slice_llvm_structure().into();
                        Some(self.type_context.add(
                            Type::Slice(t.clone()),
                            TypeInfo::new(
                                TypeKind::Slice(id),
                                slice_struct
                            )
                        ))
                    } else {
                        None
                    }
                }
                Type::Array { ty: t, size } => {
                    if let Some(id) = self.resolve_type(t) {
                        let ray_typ;

                        let ti = &self.type_context.types[id as usize];
                        let typ: BasicTypeEnum = ti.llvm_type.try_into().unwrap();
                        ray_typ = typ.array_type(*size as u32);


                        let res = self.type_context.add(t.deref().clone(), TypeInfo::new(
                            TypeKind::Array { ty: id, size: *size },
                            ray_typ.into()
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
                        let rel = mod_path.to_path();
                        let resolved = if rel.is_absolute() {
                            Some(rel.clone())
                        } else {
                            self.module_search_paths.iter()
                                .map(|sp| sp.join(&rel))
                                .find(|p| p.exists())
                        };
                        match resolved {
                            Some(p) => match Tokens::tokenize(&p) {
                                Ok(tks) => self.add_module_impl(tks, added),
                                Err(e) => self.emit_compile_message(CompileMessage::new(
                                    tk.smap.clone(),
                                    format!("Failed to load module {:?}: {}", p, e),
                                    CompileMessageType::Error,
                                )),
                            },
                            None => self.emit_compile_message(CompileMessage::new(
                                tk.smap.clone(),
                                format!("Failed to find module {:?} in any search path", rel),
                                CompileMessageType::Error,
                            )),
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

