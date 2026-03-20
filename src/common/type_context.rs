use std::collections::HashMap;
use std::fmt::format;
use crate::ast::statements::expressions::Expr;
use crate::common::utils::modulepath::ModulePath;
use crate::ast::ty;
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};


//the only things fully typed are the typename, expressions in things
//like arrays are not typed yet
pub enum PartialType {
    Typename(TypeName),
    Reference(Box<PartialType>),
    Pointer(Box<PartialType>),
    Slice(Box<PartialType>),
    Array{ty: Box<PartialType>, size: Expr},
}

impl PartialType {
    pub fn from_untyped(context: &TypeContext, untyped: ty::Type, smap: SourceMap, compiler: &mut Compiler) -> Option<Self> {
        match untyped {
            ty::Type::Typename(ty) => {
                Some(Self::Typename(
                    match context.get_type_by_name(&ty) {
                        Some(t) => t.clone(),
                        None => {
                            compiler.emit_compile_message(
                                CompileMessage::new(
                                    smap,
                                    format!("Unknown typename: {}", ty.path),
                                    CompileMessageType::Error
                                )
                            );

                            return None;
                        }
                    }
                ))
            }
            ty::Type::Reference(r) => {
                Some(
                    Self::Reference(
                        Box::new(
                            Self::from_untyped(context, *r, smap, compiler)?
                        )
                    )
                )
            }
            ty::Type::Pointer(p) => {
                Some(
                    Self::Pointer(
                        Box::new(
                            Self::from_untyped(context, *p, smap, compiler)?
                        )
                    )
                )
            }
            ty::Type::Slice(s) => {
                Some(
                    Self::Slice(
                        Box::new(
                            Self::from_untyped(context, *s, smap, compiler)?
                        )
                    )
                )
            }
            ty::Type::Array { ty, size } => {
                Some(
                    Self::Array{
                        ty: Box::new(Self::from_untyped(context, *ty, smap, compiler)?),
                        size
                    }
                )
            }
        }
    }
}


#[derive(Clone)]
//todo: Generic args?
pub struct TypeName {
    pub modulepath: ModulePath,
    //if known, this is Some
    pub size: Option<usize>
}


#[derive(Clone)]
pub struct TypeContext {
    types: Vec<TypeName>, //the index of a type in this array is a typeid
    
    //for quick lookup of a type by its module path
    typeids: HashMap<ModulePath, u64>,
}

impl TypeContext {

    pub fn new() -> Self {
        Self{
            types: Vec::new(),
            typeids: HashMap::new(),
        }
    }

    pub fn add_type(&mut self, ty: TypeName) -> u64 {
        let res = self.types.len() as u64;
        
        self.typeids.insert(ty.modulepath.clone(), res);
        self.types.push(ty);
        
        res
    }

    pub fn get_type_by_id(&self, id: u64) -> Option<&TypeName> {
        self.types.get(id as usize)
    }

    pub fn get_type_by_name(&self, name: &ModulePath) -> Option<&TypeName> {
        self.types.get(*(self.typeids.get(name)?) as usize)
    }

}