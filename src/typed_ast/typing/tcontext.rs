use std::collections::HashMap;
use crate::common::utils::modulepath::ModulePath;
use crate::typed_ast::canonical_type::CanonicalType;
use crate::typed_ast::typing::ty::{TypeId, TypeInfo, TypeKind};

pub struct TypeContext {
    types: Vec<TypeInfo>, //stores all known types
    lookup: HashMap<CanonicalType, TypeId>,

}

impl TypeContext {
    pub fn new() -> Self {
        let mut types = Self{
            types: Vec::new(),
            lookup: HashMap::new(),
        };



        types.add(CanonicalType::Typename(ModulePath::from_module_path("none")),
            TypeInfo{
                kind: TypeKind::None,
                size: 0,
                align: 0
            }
        );



        types.add(CanonicalType::Typename(ModulePath::from_module_path("int8")),
            TypeInfo{
                kind: TypeKind::Int8,
                size: 1,
                align: 1
            }
        );


        types.add(CanonicalType::Typename(ModulePath::from_module_path("int16")),
            TypeInfo{
                kind: TypeKind::Int16,
                size: 2,
                align: 2
            }
        );


        types.add(CanonicalType::Typename(ModulePath::from_module_path("int32")),
            TypeInfo{
                kind: TypeKind::Int32,
                size: 4,
                align: 4
            }
        );


        types.add(CanonicalType::Typename(ModulePath::from_module_path("int64")),
            TypeInfo{
                kind: TypeKind::Int64,
                size: 8,
                align: 8
            }
        );


        types.add(CanonicalType::Typename(ModulePath::from_module_path("bool")),
            TypeInfo{
                kind: TypeKind::Boolean,
                size: 1,
                align: 1
            }
        );


        types.add(CanonicalType::Typename(ModulePath::from_module_path("uint8")),
            TypeInfo{
                kind: TypeKind::UInt8,
                size: 1,
                align: 1
            }
        );


        types.add(CanonicalType::Typename(ModulePath::from_module_path("uint16")),
            TypeInfo{
                kind: TypeKind::UInt16,
                size: 2,
                align: 2
            }
        );


        types.add(CanonicalType::Typename(ModulePath::from_module_path("uint32")),
            TypeInfo{
                kind: TypeKind::UInt32,
                size: 4,
                align: 4
            }
        );


        types.add(CanonicalType::Typename(ModulePath::from_module_path("uint64")),
            TypeInfo{
                kind: TypeKind::UInt64,
                size: 8,
                align: 8
            }
        );


        types.add(CanonicalType::Typename(ModulePath::from_module_path("float32")),
            TypeInfo{
                kind: TypeKind::Float32,
                size: 4,
                align: 4
            }
        );


        types.add(CanonicalType::Typename(ModulePath::from_module_path("float64")),
            TypeInfo{
                kind: TypeKind::Float64,
                size: 8,
                align: 8
            }
        );



        types
    }


    pub fn add(&mut self, ty: CanonicalType, info: TypeInfo) {
        self.lookup.insert(ty, self.types.len() as TypeId);
        self.types.push(info);
    }


}