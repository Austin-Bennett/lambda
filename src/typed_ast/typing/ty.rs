use crate::typed_ast::typing::operator::{BinaryOperatorMaker, OperatorOverloads, UnaryOperatorMaker};
use inkwell::types::{AnyTypeEnum, StructType};
use std::fmt::{Debug, Formatter};

pub type TypeId = u32;
pub type StructId = u32;


#[derive(Clone)]
pub struct StructMember {
    pub name: String,
    pub ty: TypeId,
}

#[derive(Clone)]
pub struct StructInfo {
    pub name: String,
    pub type_id: TypeId,
    pub members: Vec<StructMember>,
    pub llvm_struct: StructType<'static>,
}

pub struct TypeInfo {
    pub kind: TypeKind,

    pub ops: OperatorOverloads,

    pub llvm_type: AnyTypeEnum<'static>,
}

impl TypeInfo {
    pub fn new(kind: TypeKind, llvm_type: AnyTypeEnum<'static>) -> Self {
        Self {
            kind,
            ops: OperatorOverloads::new(),
            llvm_type,
        }
    }

    pub fn enable_index_operator(&mut self, index: TypeId, result: TypeId, maker: BinaryOperatorMaker) {
        self.ops.index.insert(
            index, (result, maker)
        );
    }

    /// Register add/sub/mul/div overloads against `self_id` with the given codegen callbacks.
    /// Also registers `call(self_id) -> self_id` (multiplication-as-call for numeric types).
    pub fn enable_arithmetic_operators(
        &mut self,
        self_id: TypeId,
        add: BinaryOperatorMaker,
        sub: BinaryOperatorMaker,
        mul: BinaryOperatorMaker,
        div: BinaryOperatorMaker,
    ) {
        self.ops.add.insert(self_id, (self_id, add));
        self.ops.sub.insert(self_id, (self_id, sub));
        self.ops.mul.insert(self_id, (self_id, mul));
        self.ops.div.insert(self_id, (self_id, div));

        // For arithmetic types, `x(y)` is sugar for multiplication.
        self.ops.call.insert(vec![self_id], self_id);
    }

    pub fn enable_neg_operator(&mut self, self_id: TypeId, maker: UnaryOperatorMaker) {
        self.ops.neg = Some((self_id, maker));
    }
}


#[derive(Clone)]
pub enum TypeKind {
    IntLiteral,
    FloatLiteral,
    Int(u32),
    UInt(u32),

    Float(u32),

    Boolean,

    None, // void / ()

    Struct(StructId),
    Pointer(TypeId),
    Reference(TypeId),
    Slice(TypeId),
    Array { ty: TypeId, size: usize }, // size is const-evaluated
    Function { params: Vec<TypeId>, ret: TypeId },
    Intrinsic { ret: TypeId },
}

// Truly only for debug
impl Debug for TypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeKind::IntLiteral => write!(f, "integer"),
            TypeKind::FloatLiteral => write!(f, "float"),
            TypeKind::Int(w) => write!(f, "int{}", w),
            TypeKind::UInt(w) => write!(f, "uint{}", w),
            TypeKind::Float(w) => write!(f, "float{}", w),
            TypeKind::Boolean => f.write_str("bool"),
            TypeKind::None => f.write_str("none"),

            TypeKind::Struct(_) => f.write_str("struct"),
            TypeKind::Pointer(_) => f.write_str("*"),
            TypeKind::Reference(_) => f.write_str("&"),
            TypeKind::Slice(_) => f.write_str("[]"),
            TypeKind::Array { .. } => f.write_str("[N]"),
            TypeKind::Function { .. } => f.write_str("functor"),
            TypeKind::Intrinsic { .. } => f.write_str("intrinsic"),
        }
    }
}
