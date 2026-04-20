use crate::ast::ty::Type;
use crate::typed_ast::ast::items::function::FunctionSignature;
use crate::typed_ast::typing::operator::{AssignmentMaker, BinaryOperatorMaker, ConversionMaker, OperatorOverloads, UnaryOperatorMaker};
use inkwell::values::BasicValueEnum;
use crate::typed_ast::typing::ty::{StructId, StructInfo, TypeId, TypeInfo, TypeKind};
use inkwell::builder::Builder;
use inkwell::types::{BasicType, BasicTypeEnum, StructType};
use inkwell::AddressSpace;
use std::collections::HashMap;
use std::fmt::Write;

pub struct TypeContext {
    pub llvm_context: &'static inkwell::context::Context,
    pub types: Vec<TypeInfo>, //stores all known types
    pub type_lookup: HashMap<Type, TypeId>,
    pub type_ids: HashMap<TypeId, Type>, // reverse map

    pub structs: Vec<StructInfo>,
    pub struct_lookup: HashMap<String, StructId>,

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
            type_ids: HashMap::new(),
            structs: vec![],
            struct_lookup: Default::default(),
            none: 0,
            int_literal: 0,
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
    
    /// Auto-generate LLVM codegen callbacks for arithmetic (+, -, *, /) and negation
    /// for the type identified by `id`.  The correct instruction family (integer vs float,
    /// signed vs unsigned division) is selected from the type's `TypeKind`.
    fn enable_arithmetic_neg(&mut self, id: TypeId) {
        let kind = self.types[id as usize].kind.clone();

        match kind {
            // Signed integers and the compiler-internal int_literal type
            TypeKind::Int(_) | TypeKind::IntLiteral => {
                let add: BinaryOperatorMaker = Box::new(|b, l, r| {
                    b.build_int_add(l.into_int_value(), r.into_int_value(), "iadd")
                        .unwrap()
                        .into()
                });
                let sub: BinaryOperatorMaker = Box::new(|b, l, r| {
                    b.build_int_sub(l.into_int_value(), r.into_int_value(), "isub")
                        .unwrap()
                        .into()
                });
                let mul: BinaryOperatorMaker = Box::new(|b, l, r| {
                    b.build_int_mul(l.into_int_value(), r.into_int_value(), "imul")
                        .unwrap()
                        .into()
                });
                let div: BinaryOperatorMaker = Box::new(|b, l, r| {
                    b.build_int_signed_div(l.into_int_value(), r.into_int_value(), "idiv")
                        .unwrap()
                        .into()
                });
                let neg: UnaryOperatorMaker = Box::new(|b, v| {
                    b.build_int_neg(v.into_int_value(), "ineg").unwrap().into()
                });
                self.types[id as usize].enable_arithmetic_operators(id, add, sub, mul, div);
                self.types[id as usize].enable_neg_operator(id, neg);
            }

            // Unsigned integers – division is unsigned
            TypeKind::UInt(_) => {
                let add: BinaryOperatorMaker = Box::new(|b, l, r| {
                    b.build_int_add(l.into_int_value(), r.into_int_value(), "uadd")
                        .unwrap()
                        .into()
                });
                let sub: BinaryOperatorMaker = Box::new(|b, l, r| {
                    b.build_int_sub(l.into_int_value(), r.into_int_value(), "usub")
                        .unwrap()
                        .into()
                });
                let mul: BinaryOperatorMaker = Box::new(|b, l, r| {
                    b.build_int_mul(l.into_int_value(), r.into_int_value(), "umul")
                        .unwrap()
                        .into()
                });
                let div: BinaryOperatorMaker = Box::new(|b, l, r| {
                    b.build_int_unsigned_div(l.into_int_value(), r.into_int_value(), "udiv")
                        .unwrap()
                        .into()
                });
                let neg: UnaryOperatorMaker = Box::new(|b, v| {
                    b.build_int_neg(v.into_int_value(), "uneg").unwrap().into()
                });
                self.types[id as usize].enable_arithmetic_operators(id, add, sub, mul, div);
                self.types[id as usize].enable_neg_operator(id, neg);
            }

            // Floating-point types
            TypeKind::Float(_) => {
                let add: BinaryOperatorMaker = Box::new(|b, l, r| {
                    b.build_float_add(l.into_float_value(), r.into_float_value(), "fadd")
                        .unwrap()
                        .into()
                });
                let sub: BinaryOperatorMaker = Box::new(|b, l, r| {
                    b.build_float_sub(l.into_float_value(), r.into_float_value(), "fsub")
                        .unwrap()
                        .into()
                });
                let mul: BinaryOperatorMaker = Box::new(|b, l, r| {
                    b.build_float_mul(l.into_float_value(), r.into_float_value(), "fmul")
                        .unwrap()
                        .into()
                });
                let div: BinaryOperatorMaker = Box::new(|b, l, r| {
                    b.build_float_div(l.into_float_value(), r.into_float_value(), "fdiv")
                        .unwrap()
                        .into()
                });
                let neg: UnaryOperatorMaker = Box::new(|b, v| {
                    b.build_float_neg(v.into_float_value(), "fneg").unwrap().into()
                });
                self.types[id as usize].enable_arithmetic_operators(id, add, sub, mul, div);
                self.types[id as usize].enable_neg_operator(id, neg);
            }

            _ => {
                // Non-numeric types that somehow ended up here – no-op.
            }
        }
    }

    pub fn enable_assignment(&mut self, id: TypeId) {
        let maker: AssignmentMaker = Box::new(|b, ptr, val| {
            b.build_store(ptr, BasicValueEnum::try_from(val).unwrap()).unwrap();
        });
        self.types[id as usize].ops.assign.insert(id, maker);
    }

    fn enable_conversions_to(&mut self, from_id: TypeId, to_id: TypeId) {
        let from_kind = self.types[from_id as usize].kind.clone();
        let to_kind   = self.types[to_id as usize].kind.clone();
        let to_llvm: BasicTypeEnum = match self.types[to_id as usize].llvm_type.try_into() {
            Ok(t) => t,
            Err(_) => return,
        };

        let maker: ConversionMaker = match (&from_kind, &to_kind) {
            (
                TypeKind::Int(_) | TypeKind::UInt(_) | TypeKind::IntLiteral,
                TypeKind::Int(_) | TypeKind::UInt(_),
            ) => {
                let src_bits: u32 = match &from_kind {
                    TypeKind::Int(w) | TypeKind::UInt(w) => *w,
                    TypeKind::IntLiteral => 32,
                    _ => unreachable!(),
                };
                let dst_bits: u32 = match &to_kind {
                    TypeKind::Int(w) | TypeKind::UInt(w) => *w,
                    _ => unreachable!(),
                };
                let dst_int_ty = to_llvm.into_int_type();
                if dst_bits > src_bits {
                    match &from_kind {
                        TypeKind::UInt(_) => Box::new(move |b: &Builder<'static>, v: inkwell::values::AnyValueEnum<'static>| {
                            b.build_int_z_extend(v.into_int_value(), dst_int_ty, "zext").unwrap().into()
                        }),
                        _ => Box::new(move |b: &Builder<'static>, v: inkwell::values::AnyValueEnum<'static>| {
                            b.build_int_s_extend(v.into_int_value(), dst_int_ty, "sext").unwrap().into()
                        }),
                    }
                } else if dst_bits < src_bits {
                    Box::new(move |b: &Builder<'static>, v: inkwell::values::AnyValueEnum<'static>| {
                        b.build_int_truncate(v.into_int_value(), dst_int_ty, "trunc").unwrap().into()
                    })
                } else {
                    Box::new(|_b, v| v) // same width, different sign — no-op in LLVM
                }
            }
            (TypeKind::Int(_) | TypeKind::IntLiteral, TypeKind::Float(_)) => {
                let dst_float_ty = to_llvm.into_float_type();
                Box::new(move |b: &Builder<'static>, v: inkwell::values::AnyValueEnum<'static>| {
                    b.build_signed_int_to_float(v.into_int_value(), dst_float_ty, "sitofp").unwrap().into()
                })
            }
            (TypeKind::UInt(_), TypeKind::Float(_)) => {
                let dst_float_ty = to_llvm.into_float_type();
                Box::new(move |b: &Builder<'static>, v: inkwell::values::AnyValueEnum<'static>| {
                    b.build_unsigned_int_to_float(v.into_int_value(), dst_float_ty, "uitofp").unwrap().into()
                })
            }
            (TypeKind::Float(_), TypeKind::Int(_)) => {
                let dst_int_ty = to_llvm.into_int_type();
                Box::new(move |b: &Builder<'static>, v: inkwell::values::AnyValueEnum<'static>| {
                    b.build_float_to_signed_int(v.into_float_value(), dst_int_ty, "fptosi").unwrap().into()
                })
            }
            (TypeKind::Float(_), TypeKind::UInt(_)) => {
                let dst_int_ty = to_llvm.into_int_type();
                Box::new(move |b: &Builder<'static>, v: inkwell::values::AnyValueEnum<'static>| {
                    b.build_float_to_unsigned_int(v.into_float_value(), dst_int_ty, "fptoui").unwrap().into()
                })
            }
            (TypeKind::Float(sw), TypeKind::Float(dw)) => {
                let dst_float_ty = to_llvm.into_float_type();
                if dw > sw {
                    Box::new(move |b: &Builder<'static>, v: inkwell::values::AnyValueEnum<'static>| {
                        b.build_float_ext(v.into_float_value(), dst_float_ty, "fpext").unwrap().into()
                    })
                } else {
                    Box::new(move |b: &Builder<'static>, v: inkwell::values::AnyValueEnum<'static>| {
                        b.build_float_trunc(v.into_float_value(), dst_float_ty, "fptrunc").unwrap().into()
                    })
                }
            }
            _ => return,
        };

        self.types[from_id as usize].ops.conversion_ops.insert(to_id, maker);
    }

    fn setup_types(&mut self) {
        


        let id = self.add(Type::Typename("none".into()),
            TypeInfo::new(
                TypeKind::None,
                0, self.llvm_context.void_type().into()
            )
        );
        self.none = id;
        

        let id = self.add(Type::Typename("#int_literal".into()),
              TypeInfo::new(
                  TypeKind::IntLiteral,
                  4, self.llvm_context.i32_type().into()
              )
        );
        self.int_literal = id;
        self.enable_arithmetic_neg(id);

        
        
        //SIGNED INTEGERS
        

        let id = self.add(Type::Typename("int8".into()),
                          TypeInfo::new(
                              TypeKind::Int(8),
                              1, self.llvm_context.i8_type().into()
                          ).enable_from_int_literal(8, true)
        );
        self.int8 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("int16".into()),
                          TypeInfo::new(
                              TypeKind::Int(16),
                              2, self.llvm_context.i16_type().into()
                          ).enable_from_int_literal(16, true)
        );
        self.int16 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("int32".into()),
                          TypeInfo::new(
                              TypeKind::Int(32),
                              4, self.llvm_context.i32_type().into()
                          ).enable_from_int_literal(32, true)
        );
        self.int32 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("int64".into()),
                          TypeInfo::new(
                              TypeKind::Int(64),
                              8, self.llvm_context.i64_type().into()
                          ).enable_from_int_literal(64, true)
        );
        self.int64 = id;
        self.enable_arithmetic_neg(id);


        
        //UNSIGNED INTEGER

        let id = self.add(Type::Typename("uint8".into()),
                          TypeInfo::new(
                              TypeKind::UInt(8),
                              1, self.llvm_context.i8_type().into()
                          ).enable_from_int_literal(8, false)
        );
        self.uint8 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("uint16".into()),
                          TypeInfo::new(
                              TypeKind::UInt(16),
                              2, self.llvm_context.i16_type().into()
                          ).enable_from_int_literal(16, false)
        );
        self.uint16 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("uint32".into()),
                          TypeInfo::new(
                              TypeKind::UInt(32),
                              4, self.llvm_context.i32_type().into()
                          ).enable_from_int_literal(32, false)
        );
        self.uint32 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("uint64".into()),
                          TypeInfo::new(
                              TypeKind::UInt(64),
                              8, self.llvm_context.i64_type().into()
                          ).enable_from_int_literal(64, false)
        );


        self.uint64 = id;
        self.enable_arithmetic_neg(id);

        let id = self.add(Type::Typename("usize".into()),
        TypeInfo::new(
            TypeKind::UInt(Self::SIZE_POINTER as u32 * 8),
            Self::SIZE_POINTER, self.llvm_context.custom_width_int_type(Self::SIZE_POINTER as u32 * 8).into(),
        ).enable_from_int_literal(Self::SIZE_POINTER as u32 * 8, false));

        self.usize = id;
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

        let numeric = [
            self.int_literal,
            self.int8, self.int16, self.int32, self.int64,
            self.uint8, self.uint16, self.uint32, self.uint64, self.usize,
            self.float32, self.float64,
        ];
        for &from_id in &numeric {
            for &to_id in &numeric {
                if from_id != to_id {
                    self.enable_conversions_to(from_id, to_id);
                }
            }
            self.enable_assignment(from_id);
        }
        self.enable_assignment(self.bool);
    }

    pub fn is_int(&self, id: TypeId) -> bool {
        id == self.uint8 ||
        id == self.uint16 ||
        id == self.uint32 ||
        id == self.uint64 ||
        id == self.usize ||

        id == self.int8 ||
        id == self.int16 ||
        id == self.int32 ||
        id == self.int64 ||
        id == self.int_literal
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

    

    fn enable_ptr_conversions(&mut self, ptr_id: TypeId) {
        let usize_id = self.usize;
        let usize_int_ty = match BasicTypeEnum::try_from(self.types[usize_id as usize].llvm_type) {
            Ok(t) => t.into_int_type(),
            Err(_) => return,
        };
        let ptr_llvm_ty = self.llvm_context.ptr_type(AddressSpace::try_from(0u32).unwrap());

        // ptr → usize
        self.types[ptr_id as usize].ops.conversion_ops.insert(usize_id, Box::new(move |b, v| {
            b.build_ptr_to_int(v.into_pointer_value(), usize_int_ty, "ptrtoint").unwrap().into()
        }));

        // usize → ptr
        self.types[usize_id as usize].ops.conversion_ops.insert(ptr_id, Box::new(move |b, v| {
            b.build_int_to_ptr(v.into_int_value(), ptr_llvm_ty, "inttoptr").unwrap().into()
        }));

        // cross-ptr no-op conversions with all existing pointer/reference types
        let existing: Vec<TypeId> = self.types.iter().enumerate()
            .filter_map(|(i, info)| {
                let id = i as TypeId;
                if id == ptr_id { return None; }
                match &info.kind {
                    TypeKind::Pointer(_) | TypeKind::Reference(_) => Some(id),
                    _ => None,
                }
            })
            .collect();

        for &other_id in &existing {
            self.types[ptr_id as usize].ops.conversion_ops.insert(other_id, Box::new(|_b, v| v));
            self.types[other_id as usize].ops.conversion_ops.insert(ptr_id, Box::new(|_b, v| v));
        }
    }

    pub fn add(&mut self, ty: Type, info: TypeInfo) -> TypeId {
        if let Some(id) = self.type_lookup.get(&ty) {
            return *id;
        }

        let id = self.types.len() as TypeId;
        self.type_lookup.insert(ty.clone(), id);
        self.type_ids.insert(id, ty);
        self.types.push(info);

        if matches!(self.types[id as usize].kind, TypeKind::Pointer(_) | TypeKind::Reference(_))
            && self.usize != 0
        {
            self.enable_ptr_conversions(id);
        }

        id
    }

    pub fn pointer_to(&mut self, inner_id: TypeId) -> TypeId {
        let inner_ty = self.type_ids[&inner_id].clone();
        let ptr_ty = Type::Pointer(Box::new(inner_ty));
        if let Some(&id) = self.type_lookup.get(&ptr_ty) {
            return id;
        }
        self.add(ptr_ty, TypeInfo::new(
            TypeKind::Pointer(inner_id),
            Self::SIZE_POINTER,
            self.llvm_context.ptr_type(AddressSpace::try_from(0u32).unwrap()).into(),
        ))
    }

    pub fn reference_to(&mut self, inner_id: TypeId) -> TypeId {
        let inner_ty = self.type_ids[&inner_id].clone();
        let ref_ty = Type::Reference(Box::new(inner_ty));
        if let Some(&id) = self.type_lookup.get(&ref_ty) {
            return id;
        }
        self.add(ref_ty, TypeInfo::new(
            TypeKind::Reference(inner_id),
            Self::SIZE_POINTER,
            self.llvm_context.ptr_type(AddressSpace::try_from(0u32).unwrap()).into(),
        ))
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
            TypeKind::IntLiteral => "integer".to_string(),
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