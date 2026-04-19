use std::fmt::Write;
use std::collections::HashMap;
use inkwell::AddressSpace;
use inkwell::types::{AnyType, AnyTypeEnum, BasicMetadataTypeEnum, BasicType, BasicTypeEnum, StructType};
use crate::ast::structure::lstruct;
use crate::ast::ty::Type;
use crate::common::utils::modulepath::ModulePath;
use crate::typed_ast::ast::items::function::FunctionSignature;
use crate::typed_ast::ast::statements::expression::BinaryOperator;
use crate::typed_ast::typing::operator::OperatorOverloads;
use crate::typed_ast::typing::ty::{StructId, StructInfo, TypeId, TypeInfo, TypeKind};

pub struct TypeContext {
    pub llvm_context: &'static inkwell::context::Context,
    pub types: Vec<TypeInfo>, //stores all known types
    pub type_lookup: HashMap<Type, TypeId>,

    pub structs: Vec<StructInfo>,
    pub struct_lookup: HashMap<String, StructId>,

    pub none: TypeId,
    
    pub infer: TypeId,

    pub int8: TypeId,
    pub int16: TypeId,
    pub int32: TypeId,
    pub int64: TypeId,

    pub bool: TypeId,

    pub uint8: TypeId,
    pub uint16: TypeId,
    pub uint32: TypeId,
    pub uint64: TypeId,

    pub usize: TypeId,

    pub float32: TypeId,
    pub float64: TypeId,
}



impl TypeContext {

    #[cfg(target_pointer_width = "64")]
    pub const SIZE_POINTER: usize = 8;

    #[cfg(target_pointer_width = "32")]
    pub const SIZE_POINTER: usize = 4;

    pub fn new(llvm_context: &'static inkwell::context::Context) -> Self {
        let mut types = Self{
            llvm_context,
            types: Vec::new(),
            type_lookup: HashMap::new(),
            structs: vec![],
            struct_lookup: Default::default(),
            none: 0,
            infer: 0,
            int8: 0,
            int16: 0,
            int32: 0,
            int64: 0,
            bool: 0,
            uint8: 0,
            uint16: 0,
            uint32: 0,
            uint64: 0,
            usize: 0,
            float32: 0,
            float64: 0,
        };

        types.setup_types();


        
        types
    }
    
    fn enable_arithmetic_neg(&mut self, id: TypeId) {
        self.types[id as usize].enable_arithmetic_operators(id);
        self.types[id as usize].enable_neg_operator(id);
    }

    fn setup_types(&mut self) {
        


        let id = self.add(Type::Typename("none".into()),
            TypeInfo::new(
                TypeKind::None,
                0, self.llvm_context.void_type().into()
            )
        );
        self.none = id;
        

        let id = self.add(Type::Typename("#infer".into()),
              TypeInfo::new(
                  TypeKind::Infer,
                  0, self.llvm_context.void_type().into()
              )
        );
        self.infer = id;
        self.enable_arithmetic_neg(id);
        
        
        //SIGNED INTEGERS
        

        let id = self.add(Type::Typename("int8".into()),
                          TypeInfo::new(
                              TypeKind::Int(8),
                              1, self.llvm_context.i8_type().into()
                          )
        );
        self.int8 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("int16".into()),
                          TypeInfo::new(
                              TypeKind::Int(16),
                              2, self.llvm_context.i16_type().into()
                          )
        );
        self.int16 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("int32".into()),
                          TypeInfo::new(
                              TypeKind::Int(32),
                              4, self.llvm_context.i32_type().into()
                          )
        );
        self.int32 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("int64".into()),
                          TypeInfo::new(
                              TypeKind::Int(64),
                              8, self.llvm_context.i64_type().into()
                          )
        );
        self.int64 = id;
        self.enable_arithmetic_neg(id);


        
        //UNSIGNED INTEGER

        let id = self.add(Type::Typename("uint8".into()),
                          TypeInfo::new(
                              TypeKind::UInt(8),
                              1, self.llvm_context.i8_type().into()
                          )
        );
        self.uint8 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("int16".into()),
                          TypeInfo::new(
                              TypeKind::UInt(16),
                              2, self.llvm_context.i16_type().into()
                          )
        );
        self.uint16 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("int32".into()),
                          TypeInfo::new(
                              TypeKind::UInt(32),
                              4, self.llvm_context.i32_type().into()
                          )
        );
        self.uint32 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("int64".into()),
                          TypeInfo::new(
                              TypeKind::UInt(64),
                              8, self.llvm_context.i64_type().into()
                          )
        );
        self.uint64 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("bool".into()),
                          TypeInfo::new(
                              TypeKind::Boolean,
                              1, self.llvm_context.i8_type().into()
                          )
        );
        self.bool = id;
        
        
        
        //FLOATING NUMBERS

        let id = self.add(Type::Typename("float32".into()),
                          TypeInfo::new(
                              TypeKind::Float(32),
                              4, self.llvm_context.f32_type().into()
                          )
        );
        self.float32 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("float64".into()),
                          TypeInfo::new(
                              TypeKind::Float(64),
                              8, self.llvm_context.f64_type().into()
                          )
        );
        self.float64 = id;
        self.enable_arithmetic_neg(id);

    }

    pub fn is_int(&self, id: TypeId) -> bool {
        id == self.uint8 ||
        id == self.uint16 ||
        id == self.uint32 ||
        id == self.uint64 ||

        id == self.int8 ||
        id == self.int16 ||
        id == self.int32 ||
        id == self.int64
    }


    pub fn get_by_id(&self, id: TypeId) -> Option<&TypeInfo>
    {
        self.types.get(id as usize)
    }
    
    pub fn get_by_id_mut(&mut self, id: TypeId) -> Option<& mut TypeInfo> {
        self.types.get_mut(id as usize)
    }

    pub fn create_slice_llvm_structure(&self) -> StructType<'static> {
        self.llvm_context.struct_type(
            &[
                self.get_by_id(self.usize).unwrap().llvm_type.try_into().unwrap(),
                self.llvm_context.ptr_type(AddressSpace::try_from(0u32).unwrap()).into(),
            ],
            true
        )
    }

    //if a type is compound type such as a pointer, or reference
    //then we can resolve it and add it to the context
    pub fn resolve_type(&mut self, ty: &Type) -> Option<TypeId> {
        if let Some(t) = self.type_lookup.get(&ty) {
            
            Some(*t)
        } else {

            match ty {
                Type::Typename(tn) => None,
                Type::Reference(r) => {
                    if let Some(id) = self.resolve_type(&r) {

                        let r = ty;
                        let res = self.add(r.clone(), TypeInfo::new(
                            TypeKind::Reference(id),
                            Self::SIZE_POINTER, self.llvm_context.ptr_type(AddressSpace::try_from(0u32).unwrap()).into()
                        ));

                        Some(res)
                    } else {
                        None
                    }
                }
                Type::Pointer(p) => {
                    if let Some(id) = self.resolve_type(&p) {
                        let r = ty;
                        let res = self.add(r.clone(), TypeInfo::new(
                            TypeKind::Pointer(id),
                            Self::SIZE_POINTER, self.llvm_context.ptr_type(AddressSpace::try_from(0u32).unwrap()).into()
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
                    if let Some(id) = self.resolve_type(&s) {
                        let r = ty;
                        let usize_type = self.get_by_id(self.usize).unwrap();
                        let res = self.add(r.clone(), TypeInfo::new(
                            TypeKind::Slice(id),
                            2*Self::SIZE_POINTER, self.llvm_context.struct_type(
                                &[
                                    usize_type.llvm_type.try_into().unwrap(),
                                    self.llvm_context.ptr_type(AddressSpace::try_from(0u32).unwrap()).as_basic_type_enum()
                                ], true
                            ).into()
                        ));

                        Some(res)
                    } else {
                        None
                    }
                }
                Type::Array { ty: aty, size } => {
                    if let Some(id) = self.resolve_type(&aty) {

                        let r = ty;
                        
                        let ti = &self.types[id as usize];
                        let ray_size = ti.size * size;
                        let ray_type: BasicTypeEnum = ti.llvm_type.try_into().unwrap();

                        let res = self.add(r.clone(), TypeInfo::new(
                            TypeKind::Array { ty: id, size: *size },
                            ray_size, ray_type.array_type(*size as u32).into()
                        ));

                        Some(res)
                    } else {
                        None
                    }
                }
            }

        }
    }

    

    pub fn add(&mut self, ty: Type, info: TypeInfo) -> TypeId {
        if let Some(id) = unsafe{ &mut * (&raw mut *self) }.resolve_type(&ty) {
            return id;
        }

        let id = self.types.len() as TypeId;
        self.type_lookup.insert(ty, id);
        self.types.push(info);

        id
    }



    pub fn add_functional_type(&mut self, sig: &FunctionSignature) -> TypeId {
        if let Some(ty) = self.type_lookup.get(&Type::Typename(sig.name.clone()))  {
            return *ty;
        }

        //its type is a pointer to this function
        let mut info = TypeInfo{
            kind: TypeKind::Function { params: sig.params.clone(), ret: sig.ret },
            size: Self::SIZE_POINTER,
            ops: OperatorOverloads::new(),
            llvm_type: self.llvm_context.ptr_type(AddressSpace::try_from(0u32).unwrap()).into(),
        };

        info.ops.call.insert(sig.params.clone(), sig.ret);

        self.add(
            Type::Typename(sig.name.clone()),
            info
        )
    }


    pub fn get_struct(&self, name: &String) -> Option<StructId> {
        if let Some(id) = self.struct_lookup.get(name) {
            Some(*id)
        }  else {
            None
        }
    }

    pub fn get_struct_by_id(&self, id: StructId) -> Option<&StructInfo> {
        if let Some(info) = self.structs.get(id as usize) {
            Some(info)
        } else {
            None
        }
    }

    pub fn get_struct_type(&self, id: StructId) -> Option<TypeId> {
        if let Some(info) = self.get_struct_by_id(id) {
            Some(info.type_id)
        } else {
            None
        }
    }

    pub fn add_struct(&mut self, name: String, info: StructInfo) -> StructId {

        //screw you rust
        if let Some(id) = unsafe { &mut *(self as *mut Self) }.get_struct(&name) {
            return id;
        }


        let id = self.structs.len() as StructId;
        
        
        
        
        
        let (tid) = self.add(Type::Typename(name), TypeInfo::new(
            TypeKind::Struct(id),
            info.llvm_struct
                .size_of()
                .map(|i| i.get_zero_extended_constant().unwrap())
                .unwrap_or(0)
                as usize,
            info.llvm_struct.into()
        ));
        self.structs.push(info);
        
        let info = self.structs.get_mut(id as usize).unwrap();
        info.type_id = tid;

        id
    }

    pub fn name_of(&self, id: TypeId) -> Option<String> {
        let kind = &self.get_by_id(id)?.kind;

        Some(match kind {
            TypeKind::Infer => "unknown".to_string(),
            TypeKind::Int(w) => format!("int{}", w),
            TypeKind::UInt(w) => format!("uint{}", w),
            TypeKind::Float(w) => format!("float{}", w),
            
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