use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Write};
use inkwell::types::{AnyType, AnyTypeEnum, BasicTypeEnum, StructType};
use inkwell::values::BasicValueEnum;
use crate::common::utils::modulepath::ModulePath;
use crate::compiler::Compiler;
use crate::lexer::literal::IntegerLiteral;
use crate::typed_ast::typing::operator::OperatorOverloads;

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
    pub size: usize,

    pub ops: OperatorOverloads,

    pub llvm_type: AnyTypeEnum<'static>
}

impl TypeInfo {
    pub fn new(kind: TypeKind, size: usize, llvm_type: AnyTypeEnum<'static>) -> Self {
        Self{
            kind,
            size,
            ops: OperatorOverloads::new(),
            llvm_type,
        }
    }

    pub fn enable_from_int_literal(&mut self, width: u32, signed: bool) {
        self.ops.from_int_literal = Some(
            Box::new(
                move |compiler: &Compiler, info: &TypeInfo, literal: IntegerLiteral| -> BasicValueEnum {
                    compiler.llvm_context.custom_width_int_type(width).const_int(literal.as_u64_lossy(), signed).into()
                }
            )
        )
    }

    pub fn enable_neg_operator(&mut self, self_id: TypeId) {
        self.ops.neg = Some(self_id)
    }

    pub fn enable_arithmetic_operators(&mut self, self_id: TypeId) {
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
    Int(u32),
    UInt(u32),

    Float(u32),

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
        }
    }
}