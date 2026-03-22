use std::fmt::{Debug, Formatter, Write};

pub type TypeId = u32;

pub struct TypeInfo {
    pub kind: TypeKind,
    pub size: usize,
    pub align: usize,
}



pub enum TypeKind {
    Int8,
    Int16,
    Int32,
    Int64,

    UInt8,
    UInt16,
    UInt32,
    UInt64,

    Float32,
    Float64,

    Boolean,

    None, //void or ()


    Pointer(TypeId),
    Reference(TypeId),
    Slice(TypeId),
    Array{ ty: TypeId, size: u64 }, //size is const-evaluated
    Function { params: Vec<TypeId>, ret: TypeId }
}

//truly only for debug
impl Debug for TypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeKind::Int8 => f.write_str("int8"),
            TypeKind::Int16 => f.write_str("int16"),
            TypeKind::Int32 => f.write_str("int32"),
            TypeKind::Int64 => f.write_str("int64"),
            TypeKind::UInt8 => f.write_str("uint8"),
            TypeKind::UInt16 => f.write_str("uint16"),
            TypeKind::UInt32 => f.write_str("uint32"),
            TypeKind::UInt64 => f.write_str("uint64"),
            TypeKind::Float32 => f.write_str("float32"),
            TypeKind::Float64 => f.write_str("float64"),
            TypeKind::Boolean => f.write_str("bool"),
            TypeKind::None => f.write_str("none"),
            TypeKind::Pointer(_) => f.write_str("*"),
            TypeKind::Reference(_) => f.write_str("&"),
            TypeKind::Slice(_) => f.write_str("[]"),
            TypeKind::Array { .. } => f.write_str("[N]"),
            TypeKind::Function { .. } => f.write_str("functor"),
        }
    }
}