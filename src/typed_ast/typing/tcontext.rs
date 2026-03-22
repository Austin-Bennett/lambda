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
        let mut types = Vec::new();
        let mut lookup = HashMap::new();

        lookup.insert(CanonicalType::Typename(ModulePath::from_module_path("none")), types.len() as TypeId);
        types.push(
            TypeInfo{
                kind: TypeKind::None,
                size: 0,
                align: 0
            }
        );


        lookup.insert(CanonicalType::Typename(ModulePath::from_module_path("int8")), types.len() as TypeId);
        types.push(
            TypeInfo{
                kind: TypeKind::Int8,
                size: 1,
                align: 1
            }
        );

        lookup.insert(CanonicalType::Typename(ModulePath::from_module_path("int16")), types.len() as TypeId);
        types.push(
            TypeInfo{
                kind: TypeKind::Int16,
                size: 2,
                align: 2
            }
        );

        lookup.insert(CanonicalType::Typename(ModulePath::from_module_path("int32")), types.len() as TypeId);
        types.push(
            TypeInfo{
                kind: TypeKind::Int32,
                size: 4,
                align: 4
            }
        );

        lookup.insert(CanonicalType::Typename(ModulePath::from_module_path("int32")), types.len() as TypeId);
        types.push(
            TypeInfo{
                kind: TypeKind::Int64,
                size: 8,
                align: 8
            }
        );

        lookup.insert(CanonicalType::Typename(ModulePath::from_module_path("int64")), types.len() as TypeId);
        types.push(
            TypeInfo{
                kind: TypeKind::Boolean,
                size: 1,
                align: 1
            }
        );

        lookup.insert(CanonicalType::Typename(ModulePath::from_module_path("uint8")), types.len() as TypeId);
        types.push(
            TypeInfo{
                kind: TypeKind::UInt8,
                size: 1,
                align: 1
            }
        );

        lookup.insert(CanonicalType::Typename(ModulePath::from_module_path("uint16")), types.len() as TypeId);
        types.push(
            TypeInfo{
                kind: TypeKind::UInt16,
                size: 2,
                align: 2
            }
        );

        lookup.insert(CanonicalType::Typename(ModulePath::from_module_path("uint32")), types.len() as TypeId);
        types.push(
            TypeInfo{
                kind: TypeKind::UInt32,
                size: 4,
                align: 4
            }
        );

        lookup.insert(CanonicalType::Typename(ModulePath::from_module_path("uint64")), types.len() as TypeId);
        types.push(
            TypeInfo{
                kind: TypeKind::UInt64,
                size: 8,
                align: 8
            }
        );

        lookup.insert(CanonicalType::Typename(ModulePath::from_module_path("float32")), types.len() as TypeId);
        types.push(
            TypeInfo{
                kind: TypeKind::Float32,
                size: 4,
                align: 4
            }
        );

        lookup.insert(CanonicalType::Typename(ModulePath::from_module_path("float64")), types.len() as TypeId);
        types.push(
            TypeInfo{
                kind: TypeKind::Float64,
                size: 8,
                align: 8
            }
        );



        Self{
            types,
            lookup,
        }
    }
}