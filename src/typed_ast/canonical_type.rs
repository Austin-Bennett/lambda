use std::hash::{Hash, Hasher};
use std::mem;
use crate::ast::ty::Type;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::compiler::Compiler;
use crate::consteval::{eval_expr_const_uint, ConstContext};

//the only real difference is there is no expressions
//as they have been const evaluated away
#[derive(Eq)]
pub enum CanonicalType {
    Typename(ModulePath),
    Reference(Box<CanonicalType>),
    Pointer(Box<CanonicalType>),
    Slice(Box<CanonicalType>),
    Array{ty: Box<CanonicalType>, size: usize},
}

impl CanonicalType {
    pub fn from_ast(ty: &Type, smap: &SourceMap, const_context: &ConstContext, compiler: &mut Compiler) -> Option<Self> {
        Some(match ty {
            Type::Typename(ty) => Self::Typename(ty.clone()),
            Type::Reference(r) => Self::Reference(Box::new( Self::from_ast( &r, smap, const_context, compiler )? )),
            Type::Pointer(r) => Self::Pointer(Box::new( Self::from_ast( &r, smap, const_context, compiler )? )),
            Type::Slice(s) => Self::Slice(Box::new( Self::from_ast( &s, smap, const_context, compiler )? )),
            Type::Array { ty, size } => Self::Array {
                ty: Box::new( Self::from_ast( &ty, smap, const_context, compiler )? ),
                size: eval_expr_const_uint(size, smap, const_context, compiler)? as usize,
            }
        })
    }
}

impl Hash for CanonicalType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            CanonicalType::Typename(ty) => { ty.hash(state) }
            CanonicalType::Reference(r) => { state.write(&[0x1]); r.hash(state) }
            CanonicalType::Pointer(p) => { state.write(&[0x2]); p.hash(state) }
            CanonicalType::Slice(s) => { state.write(&[0x3]); s.hash(state) }
            CanonicalType::Array { ty, size } => { state.write(&[0x4]); ty.hash(state); size.hash(state); }
        }
    }
}

impl PartialEq for CanonicalType {
    fn eq(&self, other: &Self) -> bool {
        if mem::discriminant(self) != mem::discriminant(other) { return false; }
        match self {
            CanonicalType::Typename(ty) => {
                let CanonicalType::Typename(oty) = other else { unsafe{ std::hint::unreachable_unchecked() } };
                ty == oty
            }
            CanonicalType::Reference(r) => {
                let CanonicalType::Reference(oty) = other else { unsafe{ std::hint::unreachable_unchecked() } };
                r.eq(oty)
            }
            CanonicalType::Pointer(p) => {
                let CanonicalType::Pointer(op) = other else { unsafe{ std::hint::unreachable_unchecked() } };
                p.eq(op)
            }
            CanonicalType::Slice(s) => {
                let CanonicalType::Slice(os) = other else { unsafe{ std::hint::unreachable_unchecked() } };
                s.eq(os)
            }
            CanonicalType::Array { ty, size } => {
                let CanonicalType::Array { ty: oty, size: osize } = other else { unsafe{ std::hint::unreachable_unchecked() } };
                ty.eq(oty) && size.eq(osize)
            }
        }
    }
}
