use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Write};
use crate::common::utils::modulepath::ModulePath;
use crate::typed_ast::typing::operator::OperatorOverloads;

pub type TypeId = u32;
pub type StructId = u32;


#[derive(Clone)]
pub struct StructMember {
    pub name: String,
    pub ty: TypeId,
    pub size: usize, //for quick lookup
    pub offset: usize,
}

#[derive(Clone)]
pub struct StructInfo {
    pub name: String,
    pub type_id: TypeId,
    pub members: Vec<StructMember>,
    pub size: usize,
    pub padding: usize,
    pub align: usize,
}

pub struct TypeInfo {
    pub kind: TypeKind,
    pub size: usize,
    pub align: usize,

    pub ops: OperatorOverloads,
}

impl TypeInfo {
    pub fn new(kind: TypeKind, size: usize, align: usize) -> Self {
        Self{
            kind,
            size,
            align,
            ops: OperatorOverloads::new(),
        }
    }

    pub fn enable_neg_operator(&mut self, self_id: TypeId) {
        self.ops.neg = Some(self_id)
    }



    pub fn enable_arithmetic_operators(&mut self, self_id: TypeId
    ) {
        self.ops.add.insert(self_id, self_id);
        self.ops.sub.insert(self_id, self_id);
        self.ops.mul.insert(self_id, self_id);
        self.ops.div.insert(self_id, self_id);

        //for arithmetic types, the call operator is the same as multiplication
        self.ops.call.insert(vec![self_id], self_id);
    }


}

#[macro_export]
macro_rules! enable_operators_with {
    ($info: expr, $id: expr, $($others: expr),*) => {
        $(
            $info.enable_arithmetic_operators_with_other($id, $others);
        )*
    };
}

#[derive(Clone)]
pub enum TypeKind {
    Infer, //used for integer literals and such
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

    Struct(StructId),
    Pointer(TypeId),
    Reference(TypeId),
    Slice(TypeId),
    Array{ ty: TypeId, size: usize }, //size is const-evaluated
    Function { params: Vec<TypeId>, ret: TypeId }
}

//truly only for debug
impl Debug for TypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeKind::Infer => f.write_str("unknown"),
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

            TypeKind::Struct(_) => f.write_str("struct"),
            TypeKind::Pointer(_) => f.write_str("*"),
            TypeKind::Reference(_) => f.write_str("&"),
            TypeKind::Slice(_) => f.write_str("[]"),
            TypeKind::Array { .. } => f.write_str("[N]"),
            TypeKind::Function { .. } => f.write_str("functor"),
        }
    }
}