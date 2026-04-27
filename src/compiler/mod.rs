use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::{fs, mem};
use std::ops::Deref;
use std::path::PathBuf;
use inkwell::AddressSpace;
use inkwell::IntPredicate;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::BasicValueEnum;
use crate::typed_ast::typing::operator::{BinaryOperatorMaker, ComparisonMakers};
use crate::ast::items::function::FunctionSyntax;
use crate::ast::items::modify::{ModifySyntax, SelfMode};
use crate::ast::items::structure::StructureSyntax;
use crate::common::source_owner::{SourceDescriptor, SourceOwner};
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::lexer::token::{ExpressionToken, StatementToken, Token, TokenType};
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
use crate::typed_ast::ast::items::function::{Function as TypedFunction, FunctionSignature};
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

    // Generic templates (items with type_parameters.len() > 0)
    pub generic_fns:      HashMap<String, FunctionSyntax>,
    pub generic_structs:  HashMap<String, StructureSyntax>,
    pub generic_modifies: HashMap<String, Vec<ModifySyntax>>,

    // Monomorphization caches: (template_name, concrete_type_args) -> result
    pub mono_struct_cache: HashMap<(String, Vec<TypeId>), TypeId>,
    pub mono_fn_cache:     HashMap<(String, Vec<TypeId>), String>,

    // Active type-parameter substitutions during monomorphization ("T" -> TypeId)
    pub type_param_subst: HashMap<String, TypeId>,

    // The type currently being type-checked as a method body, used for private-access enforcement
    pub current_self_type: Option<TypeId>,

    // Generic function instantiations queued for type-checking: (template, mangled_name, subst)
    pending_generic_fns: Vec<(FunctionSyntax, String, HashMap<String, TypeId>)>,
    // Already type-checked monomorphic functions waiting to be added to the output module
    pub pending_mono_fns: Vec<TypedFunction>,
    // Generic modify block bodies queued for type-checking: (type_id, modify, subst, mangled_struct_name)
    pending_mono_methods: Vec<(TypeId, ModifySyntax, HashMap<String, TypeId>, String)>,
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
            str_literal_reg:     Registry::new(),
            generic_fns:         HashMap::new(),
            generic_structs:     HashMap::new(),
            generic_modifies:    HashMap::new(),
            mono_struct_cache:   HashMap::new(),
            mono_fn_cache:       HashMap::new(),
            type_param_subst:    HashMap::new(),
            current_self_type:    None,
            pending_generic_fns:  Vec::new(),
            pending_mono_fns:     Vec::new(),
            pending_mono_methods: Vec::new(),
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
        // generic structs go into generic_structs registry instead
        for (_, m) in &modules {
            for item in &m.ast {
                if let Item::Struct(s) = item {
                    if !s.data.type_parameters.is_empty() {
                        self.generic_structs.insert(s.data.name.clone(), s.clone());
                    } else if let Some(info) = self.create_structure_from_ast(&s.data, &s.smap) {
                        self.type_context.add_struct(info.name.clone(), info);
                    }
                }
            }
        }

        // collect generic modify blocks
        for (_, m) in &modules {
            for item in &m.ast {
                if let Item::Modify(modify) = item {
                    if !modify.data.type_parameters.is_empty() {
                        let type_name = match &modify.data.ty {
                            Type::Typename(n) => n.clone(),
                            Type::Generic { name, .. } => name.clone(),
                            _ => continue,
                        };
                        self.generic_modifies.entry(type_name).or_default().push(modify.clone());
                    }
                }
            }
        }

        //loop through all functions, create their identifiers ahead of time
        //add to the global scope
        for (_, m) in &modules {
            for item in &m.ast {
                if let Item::Func(func) = item {
                    // generic functions go into the registry, not the scope
                    if !func.data.type_parameters.is_empty() {
                        self.generic_fns.insert(func.data.name.clone(), func.clone());
                        continue;
                    }
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

        // pre-register all methods from modify blocks (skip generic modify blocks)
        for (_, m) in &modules {
            for item in &m.ast {
                if let Item::Modify(modify) = item {
                    if !modify.data.type_parameters.is_empty() {
                        continue;
                    }
                    let type_name = modify.data.ty.mangle_name();
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
                        match method.self_mode {
                            SelfMode::ByRef => params.push(self.type_context.reference_to(type_id)),
                            SelfMode::Value => params.push(type_id),
                            SelfMode::None => {}
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
                        match op.self_mode {
                            SelfMode::ByRef => params.push(self.type_context.reference_to(type_id)),
                            SelfMode::Value => params.push(type_id),
                            SelfMode::None => {}
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
                        let rhs_ty = if op.self_mode.has_self() && params.len() > 1 {
                            params[1]
                        } else if !op.self_mode.has_self() && !params.is_empty() {
                            params[0]
                        } else {
                            self.type_context.none
                        };

                        let bool_llvm = self.type_context.types[self.type_context.bool as usize]
                            .llvm_type.into_int_type();

                        match op.op_name.as_str() {
                            "add" | "sub" | "mul" | "div" => {
                                if op.self_mode == SelfMode::ByRef {
                                    self.emit_compile_message(CompileMessage::new(
                                        modify.smap.clone(),
                                        format!("operator '{}' must take self by value, not by reference", op.op_name),
                                        CompileMessageType::Error,
                                    ));
                                }
                                let mangled = op_mangled.clone();
                                let closure: BinaryOperatorMaker = Box::new(move |b, g, l, r| {
                                    let fn_val = g[&mangled].into_function_value();
                                    let res = b.build_call(fn_val, &[BasicValueEnum::try_from(l).unwrap().into(), BasicValueEnum::try_from(r).unwrap().into()], "user_binop").unwrap();
                                    res.try_as_basic_value().basic().unwrap().into()
                                });
                                let type_info = self.type_context.get_by_id_mut(type_id).unwrap();
                                match op.op_name.as_str() {
                                    "add" => { type_info.ops.add.insert(rhs_ty, (ret, closure)); }
                                    "sub" => { type_info.ops.sub.insert(rhs_ty, (ret, closure)); }
                                    "mul" => { type_info.ops.mul.insert(rhs_ty, (ret, closure)); }
                                    "div" => { type_info.ops.div.insert(rhs_ty, (ret, closure)); }
                                    _     => unreachable!(),
                                }
                            }
                            "cmp" => {
                                if op.self_mode == SelfMode::ByRef {
                                    self.emit_compile_message(CompileMessage::new(
                                        modify.smap.clone(),
                                        format!("operator 'cmp' must take self by value, not by reference"),
                                        CompileMessageType::Error,
                                    ));
                                }
                                let mk = |pred: IntPredicate, mname: String| -> BinaryOperatorMaker {
                                    Box::new(move |b, g, l, r| {
                                        let fn_val = g[&mname].into_function_value();
                                        let cmp_result = b.build_call(fn_val, &[BasicValueEnum::try_from(l).unwrap().into(), BasicValueEnum::try_from(r).unwrap().into()], "user_cmp")
                                            .unwrap().try_as_basic_value().basic().unwrap().into_int_value();
                                        let zero = cmp_result.get_type().const_int(0, false);
                                        b.build_int_z_extend(
                                            b.build_int_compare(pred, cmp_result, zero, "ucmp").unwrap(),
                                            bool_llvm, "bcmp"
                                        ).unwrap().into()
                                    })
                                };
                                let makers = ComparisonMakers {
                                    eq: mk(IntPredicate::EQ,  op_mangled.clone()),
                                    ne: mk(IntPredicate::NE,  op_mangled.clone()),
                                    lt: mk(IntPredicate::SLT, op_mangled.clone()),
                                    gt: mk(IntPredicate::SGT, op_mangled.clone()),
                                    le: mk(IntPredicate::SLE, op_mangled.clone()),
                                    ge: mk(IntPredicate::SGE, op_mangled.clone()),
                                };
                                self.type_context.get_by_id_mut(type_id).unwrap().ops.cmp.insert(rhs_ty, makers);
                            }
                            "assign" => {
                                self.type_context.get_by_id_mut(type_id).unwrap()
                                    .ops.user_assign.insert(rhs_ty, (ret, op_mangled));
                            }
                            "drop" => {
                                if op.self_mode != SelfMode::ByRef {
                                    self.emit_compile_message(CompileMessage::new(
                                        modify.smap.clone(),
                                        format!("operator 'drop' must take self by reference (self&), not by value"),
                                        CompileMessageType::Error,
                                    ));
                                }
                                self.type_context.get_by_id_mut(type_id).unwrap().ops.drop = Some(op_mangled);
                            }
                            other => {
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

        // Process queued generic instantiations (loop because instantiating one item
        // may queue further instantiations from generic calls or method bodies)
        loop {
            let fns = mem::take(&mut self.pending_generic_fns);
            let methods = mem::take(&mut self.pending_mono_methods);
            if fns.is_empty() && methods.is_empty() { break; }

            for (template, mangled, subst) in fns {
                self.type_param_subst = subst;
                let mut fn_context = context.clone();
                if let Some(tf) = TypedFunction::from_parts(
                    &mangled,
                    SelfMode::None,
                    None,
                    &template.data.parameters,
                    template.data.ty.as_ref(),
                    template.data.body.as_ref(),
                    template.data.is_extern,
                    template.smap.clone(),
                    self,
                    &mut fn_context,
                ) {
                    self.pending_mono_fns.push(tf);
                }
                self.type_param_subst.clear();
            }

            for (type_id, modify, subst, mangled_struct_name) in methods {
                self.type_param_subst = subst;
                let mut fn_context = context.clone();
                for method in &modify.data.methods {
                    let mangled = format!("{}_{}", mangled_struct_name, method.name);
                    if let Some(func) = TypedFunction::from_method(method, type_id, &mangled, self, &mut fn_context) {
                        self.pending_mono_fns.push(func);
                    }
                }
                for op in &modify.data.operators {
                    let op_mangled = format!("{}__op_{}", mangled_struct_name, op.op_name);
                    if let Some(func) = TypedFunction::from_operator(op, type_id, &op_mangled, self, &mut fn_context) {
                        self.pending_mono_fns.push(func);
                    }
                }
                self.type_param_subst.clear();
            }
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
        // Type-parameter substitution takes priority over the cache
        if let Type::Typename(name) = ty {
            if let Some(&id) = self.type_param_subst.get(name) {
                return Some(id);
            }
        }
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
                Type::Generic { name, params } => {
                    self.monomorphize_struct(name.clone(), params.clone())
                }
            }
        }
    }
    

    /// Build the mangled name for a generic instantiation, e.g. "Pair_int32_float32"
    pub fn mangle_name(&self, base: &str, args: &[TypeId]) -> String {
        let mut s = base.to_string();
        for &id in args {
            s.push('_');
            s.push_str(&self.type_context.name_of(id).unwrap_or_else(|| format!("t{}", id)));
        }
        s
    }

    /// Monomorphize a generic struct with concrete type arguments.
    /// Returns the TypeId of the instantiated struct.
    pub fn monomorphize_struct(&mut self, name: String, params: Vec<Type>) -> Option<TypeId> {
        let arg_ids: Vec<TypeId> = params.iter()
            .map(|p| self.resolve_type(p))
            .collect::<Option<_>>()?;

        let key = (name.clone(), arg_ids.clone());
        if let Some(&tid) = self.mono_struct_cache.get(&key) {
            return Some(tid);
        }

        let template = self.generic_structs.get(&name)?.clone();

        let subst: HashMap<String, TypeId> = template.data.type_parameters.iter()
            .cloned()
            .zip(arg_ids.iter().copied())
            .collect();

        let mangled = self.mangle_name(&name, &arg_ids);

        // Substitute member types and create the concrete struct info
        let old_subst = mem::replace(&mut self.type_param_subst, subst);
        let mut members = Vec::new();
        let mut ok = true;
        for m in &template.data.members {
            let Some(ty) = m.ty.as_ref() else { continue; };
            let Some(id) = self.resolve_type(ty) else {
                ok = false;
                break;
            };
            members.push(StructMember { name: m.name.clone(), ty: id, public: m.public });
        }
        self.type_param_subst = old_subst;
        if !ok { return None; }

        let struct_member_types: Vec<BasicTypeEnum> = members.iter()
            .map(|m| self.type_context.get_by_id(m.ty).unwrap().llvm_type.try_into().unwrap())
            .collect();

        let info = StructInfo {
            name: mangled.clone(),
            members,
            type_id: 0,
            llvm_struct: self.llvm_context.struct_type(&struct_member_types, false),
        };
        let struct_id = self.type_context.add_struct(mangled.clone(), info);
        let tid = self.type_context.get_struct_type(struct_id)?;

        self.mono_struct_cache.insert(key, tid);

        // Instantiate any generic modify blocks that match this concrete struct
        self.instantiate_generic_modifies_for(&name, tid, &arg_ids, &mangled);

        Some(tid)
    }

    /// Match a generic modify block's type pattern against concrete type args.
    /// Returns a substitution map if the pattern matches, None otherwise.
    fn match_modify_pattern(
        &mut self,
        modify: &ModifySyntax,
        concrete_args: &[TypeId],
    ) -> Option<HashMap<String, TypeId>> {
        let Type::Generic { params, .. } = &modify.data.ty else {
            return None;
        };

        if params.len() != concrete_args.len() {
            return None;
        }

        let type_params = modify.data.type_parameters.clone();
        let mut subst: HashMap<String, TypeId> = HashMap::new();

        for (param_ty, &concrete_id) in params.iter().zip(concrete_args.iter()) {
            match param_ty {
                Type::Typename(n) if type_params.contains(n) => {
                    if let Some(&existing) = subst.get(n) {
                        if existing != concrete_id { return None; }
                    } else {
                        subst.insert(n.clone(), concrete_id);
                    }
                }
                concrete_pattern => {
                    let expected_id = self.resolve_type(concrete_pattern)?;
                    if expected_id != concrete_id { return None; }
                }
            }
        }

        Some(subst)
    }

    /// Register method/operator signatures for a concrete instantiation of a generic modify block,
    /// and queue the bodies for type-checking.
    fn apply_generic_modify_block(
        &mut self,
        type_id: TypeId,
        modify: ModifySyntax,
        subst: HashMap<String, TypeId>,
        mangled_struct_name: &str,
    ) {
        let old_subst = mem::replace(&mut self.type_param_subst, subst.clone());

        for method in &modify.data.methods {
            let mangled = format!("{}_{}", mangled_struct_name, method.name);

            if self.type_context.get_by_id(type_id)
                .map_or(false, |i| i.methods.contains_key(&method.name))
            {
                continue;
            }

            let ret = match &method.ret {
                Some(ty) => self.resolve_type(ty).unwrap_or(self.type_context.none),
                None => self.type_context.none,
            };

            let mut params = Vec::new();
            match method.self_mode {
                SelfMode::ByRef => params.push(self.type_context.reference_to(type_id)),
                SelfMode::Value => params.push(type_id),
                SelfMode::None => {}
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

            self.type_context.get_by_id_mut(type_id)
                .unwrap()
                .methods
                .insert(method.name.clone(), (mangled, fn_type_id, method.public));
        }

        for op in &modify.data.operators {
            let op_mangled = format!("{}__op_{}", mangled_struct_name, op.op_name);

            let ret = match &op.ret {
                Some(ty) => self.resolve_type(ty).unwrap_or(self.type_context.none),
                None => self.type_context.none,
            };

            let mut params = Vec::new();
            match op.self_mode {
                SelfMode::ByRef => params.push(self.type_context.reference_to(type_id)),
                SelfMode::Value => params.push(type_id),
                SelfMode::None => {}
            }
            for p in &op.params {
                if let Some(pty) = p.data.ty.as_ref() {
                    if let Some(pid) = self.resolve_type(pty) {
                        params.push(pid);
                    }
                }
            }

            self.type_context.add_functional_type(&FunctionSignature {
                name: op_mangled.clone(),
                ret,
                params: params.clone(),
            });

            let rhs_ty = if op.self_mode.has_self() && params.len() > 1 {
                params[1]
            } else if !op.self_mode.has_self() && !params.is_empty() {
                params[0]
            } else {
                self.type_context.none
            };

            let bool_llvm = self.type_context.types[self.type_context.bool as usize]
                .llvm_type.into_int_type();

            match op.op_name.as_str() {
                "add" | "sub" | "mul" | "div" => {
                    if op.self_mode == SelfMode::ByRef {
                        self.emit_compile_message(CompileMessage::new(
                            modify.smap.clone(),
                            format!("operator '{}' must take self by value, not by reference", op.op_name),
                            CompileMessageType::Error,
                        ));
                    }
                    let mangled = op_mangled.clone();
                    let closure: BinaryOperatorMaker = Box::new(move |b, g, l, r| {
                        let fn_val = g[&mangled].into_function_value();
                        let res = b.build_call(fn_val, &[BasicValueEnum::try_from(l).unwrap().into(), BasicValueEnum::try_from(r).unwrap().into()], "user_binop").unwrap();
                        res.try_as_basic_value().basic().unwrap().into()
                    });
                    let type_info = self.type_context.get_by_id_mut(type_id).unwrap();
                    match op.op_name.as_str() {
                        "add" => { type_info.ops.add.insert(rhs_ty, (ret, closure)); }
                        "sub" => { type_info.ops.sub.insert(rhs_ty, (ret, closure)); }
                        "mul" => { type_info.ops.mul.insert(rhs_ty, (ret, closure)); }
                        "div" => { type_info.ops.div.insert(rhs_ty, (ret, closure)); }
                        _     => unreachable!(),
                    }
                }
                "cmp" => {
                    if op.self_mode == SelfMode::ByRef {
                        self.emit_compile_message(CompileMessage::new(
                            modify.smap.clone(),
                            format!("operator 'cmp' must take self by value, not by reference"),
                            CompileMessageType::Error,
                        ));
                    }
                    let mk = |pred: IntPredicate, mname: String| -> BinaryOperatorMaker {
                        Box::new(move |b, g, l, r| {
                            let fn_val = g[&mname].into_function_value();
                            let cmp_result = b.build_call(fn_val, &[BasicValueEnum::try_from(l).unwrap().into(), BasicValueEnum::try_from(r).unwrap().into()], "user_cmp")
                                .unwrap().try_as_basic_value().basic().unwrap().into_int_value();
                            let zero = cmp_result.get_type().const_int(0, false);
                            b.build_int_z_extend(
                                b.build_int_compare(pred, cmp_result, zero, "ucmp").unwrap(),
                                bool_llvm, "bcmp"
                            ).unwrap().into()
                        })
                    };
                    let makers = ComparisonMakers {
                        eq: mk(IntPredicate::EQ,  op_mangled.clone()),
                        ne: mk(IntPredicate::NE,  op_mangled.clone()),
                        lt: mk(IntPredicate::SLT, op_mangled.clone()),
                        gt: mk(IntPredicate::SGT, op_mangled.clone()),
                        le: mk(IntPredicate::SLE, op_mangled.clone()),
                        ge: mk(IntPredicate::SGE, op_mangled.clone()),
                    };
                    self.type_context.get_by_id_mut(type_id).unwrap().ops.cmp.insert(rhs_ty, makers);
                }
                "assign" => {
                    self.type_context.get_by_id_mut(type_id).unwrap()
                        .ops.user_assign.insert(rhs_ty, (ret, op_mangled));
                }
                "drop" => {
                    if op.self_mode != SelfMode::ByRef {
                        self.emit_compile_message(CompileMessage::new(
                            modify.smap.clone(),
                            format!("operator 'drop' must take self by reference (self&), not by value"),
                            CompileMessageType::Error,
                        ));
                    }
                    self.type_context.get_by_id_mut(type_id).unwrap().ops.drop = Some(op_mangled);
                }
                _ => {}
            }
        }

        self.type_param_subst = old_subst;

        self.pending_mono_methods.push((type_id, modify, subst, mangled_struct_name.to_string()));
    }

    /// Instantiate all matching generic modify blocks for a newly-created struct type.
    fn instantiate_generic_modifies_for(
        &mut self,
        struct_name: &str,
        type_id: TypeId,
        arg_ids: &[TypeId],
        mangled_name: &str,
    ) {
        let modifies = match self.generic_modifies.get(struct_name) {
            Some(v) => v.clone(),
            None => return,
        };

        for modify in modifies {
            if let Some(subst) = self.match_modify_pattern(&modify, arg_ids) {
                self.apply_generic_modify_block(type_id, modify, subst, mangled_name);
            }
        }
    }

    /// Ensure a generic function is instantiated with the given type arguments.
    /// Returns (mangled_name, fn_type_id) on success.
    pub fn ensure_generic_fn(&mut self, name: &str, type_args: Vec<TypeId>) -> Option<(String, TypeId)> {
        let key = (name.to_string(), type_args.clone());
        if let Some(mangled) = self.mono_fn_cache.get(&key) {
            let fn_type_id = self.type_context.resolve_type(&Type::Typename(mangled.clone()))?;
            return Some((mangled.clone(), fn_type_id));
        }

        let template = self.generic_fns.get(name)?.clone();

        let subst: HashMap<String, TypeId> = template.data.type_parameters.iter()
            .cloned()
            .zip(type_args.iter().copied())
            .collect();

        let mangled = self.mangle_name(name, &type_args);

        // Compute the concrete return type and parameter types for the signature
        let old_subst = mem::replace(&mut self.type_param_subst, subst.clone());

        let ret = match &template.data.ty {
            Some(ty) => self.resolve_type(ty)?,
            None => self.type_context.none,
        };

        let params: Vec<TypeId> = template.data.parameters.iter()
            .filter_map(|p| p.data.ty.as_ref())
            .map(|ty| self.resolve_type(ty))
            .collect::<Option<_>>()?;

        self.type_param_subst = old_subst;

        let fn_type_id = self.type_context.add_functional_type(&FunctionSignature {
            name: mangled.clone(),
            ret,
            params,
        });

        // Queue body for type-checking (done after all modules are processed)
        self.pending_generic_fns.push((template, mangled.clone(), subst));
        self.mono_fn_cache.insert(key, mangled.clone());

        Some((mangled, fn_type_id))
    }

    pub fn emit_compile_message(&mut self, msg: CompileMessage) {
        match msg.ty {
            CompileMessageType::Error => self.errors.push(msg),
            CompileMessageType::Warning => self.warnings.push(msg),
            CompileMessageType::Info => self.warnings.push(msg),
        }
    }
    
    fn print_message(&self, msg: &CompileMessage, ty: &str) {

        let source_string = self.source_map.get(&msg.source.owner).cloned().unwrap_or(format!("UNKNOWN SOURCE: {:?}", msg.source.owner));
        
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
        let mut iter = tokens.peekable();
        while let Some(tk) = iter.next() {
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
                                Ok(tks) => {
                                    self.source_map.insert(tks.get_owner().clone(),
                                        fs::read_to_string(p).unwrap()
                                    );
                                    self.add_module_impl(tks, added)
                                },
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

                TokenType::Statement(StatementToken::UsecKW) => {
                    // Skip whitespace between `usec` and the path string literal
                    while matches!(iter.peek().map(|t| &t.typ), Some(TokenType::User(_))) {
                        iter.next();
                    }
                    // The header path follows as a StringLiteral token
                    let is_str = matches!(
                        iter.peek().map(|t| &t.typ),
                        Some(TokenType::Expression(ExpressionToken::StringLiteral(_)))
                    );
                    let _path_str = if is_str {
                        iter.next().and_then(|t| {
                            if let TokenType::Expression(ExpressionToken::StringLiteral(p)) = t.typ {
                                Some(p)
                            } else {
                                None
                            }
                        })
                    } else {
                        None
                    };

                    todo!("C header parsing")
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

