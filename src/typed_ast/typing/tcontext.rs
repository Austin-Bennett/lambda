use std::fmt::Write;
use std::collections::HashMap;
use crate::ast::structure::lstruct;
use crate::ast::ty::Type;
use crate::common::utils::modulepath::ModulePath;
use crate::typed_ast::typing::ty::{StructId, StructInfo, TypeId, TypeInfo, TypeKind};

#[derive(Default)]
pub struct TypeContext {
    types: Vec<TypeInfo>, //stores all known types
    type_lookup: HashMap<Type, TypeId>,

    structs: Vec<StructInfo>,
    struct_lookup: HashMap<ModulePath, StructId>,


    pub none: TypeId,

    pub int_literal: TypeId,
    pub int8: TypeId,
    pub int16: TypeId,
    pub int32: TypeId,
    pub int64: TypeId,

    pub bool: TypeId,

    pub uint8: TypeId,
    pub uint16: TypeId,
    pub uint32: TypeId,
    pub uint64: TypeId,

    pub float32: TypeId,
    pub float64: TypeId,
}


impl TypeContext {

    #[cfg(target_pointer_width = "64")]
    pub const SIZE_POINTER: usize = 8;

    #[cfg(target_pointer_width = "32")]
    pub const SIZE_POINTER: usize = 4;

    pub fn new() -> Self {
        let mut types = Self{
            types: Vec::new(),
            type_lookup: HashMap::new(),
            ..Default::default()
        };



        let (_, id) =types.add(Type::Typename(ModulePath::from_module_path("none")),
            TypeInfo::new(
                TypeKind::None,
                 0,
                 0
                )
        );
        types.none = id;


        let (_, id) = types.add(Type::Typename(ModulePath::from_module_path("int_literal")),
               TypeInfo::new(
                   TypeKind::IntegerLiteral,
                   0,
                   0
               )
        );
        types.int_literal = id;


        let (info, id) = types.add(Type::Typename(ModulePath::from_module_path("int8")),
              TypeInfo::new(
                  TypeKind::Int8,
                  1,
                  1
              )
        );
        info.enable_arithmetic_operators(id);
        info.enable_neg_operator(id);

        types.int8 = id;


        let (info, id) = types.add(Type::Typename(ModulePath::from_module_path("int16")),
              TypeInfo::new(
                  TypeKind::Int16,
                  2,
                  2
              )
        );
        info.enable_arithmetic_operators(id);
        info.enable_neg_operator(id);

        types.int16 = id;


        let (info, id) = types.add(Type::Typename(ModulePath::from_module_path("int32")),
              TypeInfo::new(
                  TypeKind::Int32,
                  4,
                  4
              )
        );
        info.enable_arithmetic_operators(id);
        info.enable_neg_operator(id);

        types.int32 = id;


        let (info, id) = types.add(Type::Typename(ModulePath::from_module_path("int64")),
              TypeInfo::new(
                  TypeKind::Int64,
                  8,
                  8
              )
        );

        info.enable_arithmetic_operators(id);
        info.enable_neg_operator(id);

        types.int64 = id;


        let (_, id) = types.add(Type::Typename(ModulePath::from_module_path("bool")),
              TypeInfo::new(
                  TypeKind::Boolean,
                  1,
                  1
              )
        );

        types.bool = id;


        let (info, id) = types.add(Type::Typename(ModulePath::from_module_path("uint8")),
              TypeInfo::new(
                  TypeKind::UInt8,
                  1,
                  1
              )
        );

        info.enable_arithmetic_operators(id);
        types.uint8 = id;


        let (info, id) = types.add(Type::Typename(ModulePath::from_module_path("uint16")),
            TypeInfo::new(
                TypeKind::UInt16,
                2,
                2
            )
        );

        info.enable_arithmetic_operators(id);
        types.uint16 = id;


        let (info, id) = types.add(Type::Typename(ModulePath::from_module_path("uint32")),
            TypeInfo::new(
                TypeKind::UInt32,
                4,
                4
            )
        );

        info.enable_arithmetic_operators(id);
        types.uint32 = id;


        let (info, id) = types.add(Type::Typename(ModulePath::from_module_path("uint64")),
              TypeInfo::new(
                  TypeKind::UInt64,
                  8,
                  8
              )
        );

        info.enable_arithmetic_operators(id);
        types.uint64 = id;


        let (info, id) = types.add(Type::Typename(ModulePath::from_module_path("float32")),
              TypeInfo::new(
                  TypeKind::Float32,
                  4,
                  4
              )
        );

        info.enable_arithmetic_operators(id);
        info.enable_neg_operator(id);
        types.float32 = id;


        let (info, id) = types.add(Type::Typename(ModulePath::from_module_path("float64")),
              TypeInfo::new(
                  TypeKind::Float64,
                  8,
                  8
              )
        );

        info.enable_arithmetic_operators(id);
        info.enable_neg_operator(id);

        types.float64 = id;



        types
    }


    pub fn get_by_id(&self, id: TypeId) -> Option<&TypeInfo> {
        self.types.get(id as usize)
    }

    pub fn get_by_id_mut(&mut self, id: TypeId) -> Option<&mut TypeInfo> {
        self.types.get_mut(id as usize)
    }

    pub fn get_by_type(&mut self, ty: &Type) -> Option<&TypeInfo> {
        if let Some(ind) = self.type_lookup.get(ty).copied() {

            self.types.get(ind as usize)
        } else {

            if let Some((res, _)) = self.resolve_type(ty) {
                Some(res)
            } else {
                None
            }
        }
    }

    pub fn get_by_type_mut(&mut self, ty: &Type) -> Option<&mut TypeInfo> {
        if let Some(ind) = self.type_lookup.get(ty).copied() {

            self.types.get_mut(ind as usize)
        } else {

            if let Some((res, _)) = self.resolve_type(ty) {
                Some(res)
            } else {
                None
            }
        }
    }

    pub fn get_type_id(&mut self, ty: &Type) -> Option<TypeId> {
        if let Some(t) = self.type_lookup.get(ty) {
            Some(*t)
        } else {
            //try to resolve it
            if let Some((_, id)) = self.resolve_type(ty) {
                Some(id)
            } else {
                None
            }
        }
    }

    //if a type is compound type such as a pointer, or reference
    //then we can resolve it and add it to the context
    pub fn resolve_type(&mut self, ty: &Type) -> Option<(&mut TypeInfo, TypeId)> {
        if let Some(t) = self.type_lookup.get(&ty) {
            let ti = &mut self.types[*t as usize];

            Some((ti, *t))
        } else {

            match ty {
                Type::Typename(tn) => None,
                Type::Reference(r) => {
                    if let Some((_, id)) = self.resolve_type(&r) {
                        let r = ty;
                        let res = self.add(r.clone(), TypeInfo::new(
                            TypeKind::Reference(id),
                            Self::SIZE_POINTER, Self::SIZE_POINTER
                        ));

                        Some(res)
                    } else {
                        None
                    }
                }
                Type::Pointer(p) => {
                    if let Some((_, id)) = self.resolve_type(&p) {
                        let r = ty;
                        let res = self.add(r.clone(), TypeInfo::new(
                            TypeKind::Pointer(id),
                            Self::SIZE_POINTER,  Self::SIZE_POINTER
                        ));

                        Some(res)
                    } else {
                        None
                    }
                }

                /*
                    A contains firstly a length (usize) and a pointer (usize)
                    making it 2 * SIZE_POINTER in length
                */
                Type::Slice(s) => {
                    if let Some((_, id)) = self.resolve_type(&ty) {
                        let r = ty;
                        let res = self.add(r.clone(), TypeInfo::new(
                            TypeKind::Slice(id),
                            2*Self::SIZE_POINTER,  Self::SIZE_POINTER
                        ));

                        Some(res)
                    } else {
                        None
                    }
                }
                Type::Array { ty: aty, size } => {
                    if let Some((ti, id)) = self.resolve_type(&aty) {
                        let r = ty;
                        let ray_size = ti.size * size;
                        let align = ti.align;

                        let res = self.add(r.clone(), TypeInfo::new(
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



    pub fn add(&mut self, ty: Type, info: TypeInfo) -> (&mut TypeInfo, TypeId) {

        if let Some(id) = self.get_type_id(&ty) {
            return (self.get_by_id_mut(id).unwrap(), id);
        }

        let id = self.types.len() as TypeId;
        self.type_lookup.insert(ty, id);
        (self.types.push_mut(info), id)
    }

    pub fn get_struct(&mut self, name: &ModulePath) -> Option<(&mut StructInfo, StructId)> {
        if let Some(id) = self.struct_lookup.get(name) {
            Some((&mut self.structs[*id as usize], *id))
        }  else {
            None
        }
    }

    pub fn add_struct(&mut self, name: ModulePath, info: StructInfo) -> (&mut StructInfo, StructId) {

        //screw you rust
        if let Some(s) = unsafe { &mut *(self as *mut Self) }.get_struct(&name) {
            return s;
        }


        let id = self.structs.len() as TypeId;


        (self.structs.push_mut(info), id)
    }

    pub fn name_of(&self, id: TypeId) -> Option<String> {
        let ty = self.get_by_id(id)?;

        Some(match &ty.kind {
            TypeKind::IntegerLiteral => "int_literal".to_string(),
            TypeKind::Int8 => "int8".to_string(),
            TypeKind::Int16 => "int16".to_string(),
            TypeKind::Int32 => "int32".to_string(),
            TypeKind::Int64 => "int64".to_string(),
            TypeKind::UInt8 => "uint8".to_string(),
            TypeKind::UInt16 => "uint16".to_string(),
            TypeKind::UInt32 => "uint32".to_string(),
            TypeKind::UInt64 => "uint64".to_string(),
            TypeKind::Float32 => "float32".to_string(),
            TypeKind::Float64 => "float64".to_string(),
            TypeKind::Boolean => "bool".to_string(),
            TypeKind::None => "none".to_string(),

            TypeKind::Struct(id) => format!("{}", self.structs[*id as usize].name),
            TypeKind::Pointer(ty) => format!("{}*", self.name_of(*ty)?),
            TypeKind::Reference(ty) => format!("{}&", self.name_of(*ty)?),
            TypeKind::Slice(ty) => format!("{}[]", self.name_of(*ty)?),
            TypeKind::Array { ty, size } => format!("{}[{}]", self.name_of(*ty)?, size),
            TypeKind::Function { ret, params } => {
                let mut res = String::new();
                let _ = write!(res, "{}(", self.name_of(*ret)?);

                let mut first = true;
                for p in params {
                    if !first {
                        let _ =write!(res, ", ");
                    }
                    first = false;

                    let _ = write!(res, "{}", self.name_of(*p)?);
                }
                res += ")";


                res
            }
        })
    }
}