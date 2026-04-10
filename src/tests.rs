use std::fs;
use inkwell::debug_info::DWARFSourceLanguage::D;
use lme::desc::LME;
use crate::ast::structure::StructureSyntax;
#[cfg(test)]

use crate::ast::ty::TypeSyntax;
use crate::ast::Syntax;
use crate::codegen::exec_builder::ExecBuilder;
use crate::codegen::instructions::{Data, Instruction, EffectiveType, Register, Size};
use crate::common::source_owner::{SourceDescriptor, SourceOwner};
use crate::compiler::Compiler;
use crate::lexer::token::TokenType;
use crate::lexer::tokenizer::Tokens;

pub fn make_tokens(s: &str) -> Tokens {
    Tokens::tokenize_string(
        SourceOwner::new(
            SourceDescriptor::RustString,
            "test_string".to_string(),
        ),
        s.to_string()
    )
}


#[test]
pub fn test_type_parse() {
    let mut compiler = Compiler::new();

    let mut tokens = make_tokens("T&*[][N]").collect();

    let Some(ty) = TypeSyntax::parse(
        &mut tokens,
        &mut compiler,
    ) else {
        compiler.raise_compile_warnings(false);
        compiler.raise_compile_errors(false);
        panic!()
    };


    compiler.raise_compile_warnings(false);
    if compiler.raise_compile_errors(false) {
       panic!()
    }

    println!("{:?}", ty.data)
}


#[test]
pub fn test_struct_make() {
    let mut compiler = Compiler::new();


    let tokens = make_tokens("\
        struct test {\
            a: int32,\
            b: int32,\
            c: int64,\
        }\
    ");

    let mut tks = tokens.filter(|t|
        if let TokenType::User(u) = &t.typ
        { false } else { true }).collect();

    let Some(structure) = StructureSyntax::parse(&mut tks, &mut compiler) else {
        compiler.raise_compile_errors(false);
        panic!();
    };

    let Some(s) = compiler.create_structure_from_ast(&structure.data, &structure.smap) else {
        compiler.raise_compile_errors(false);
        panic!();
    };

    println!("struct {}", s.name);
    
    println!("size: {}\npadding: {}\nalign: {}", s.size, s.padding, s.align);
    
    for m in s.members {
        println!("{} offset {}", m.name, m.offset);
    }
}

#[test]
pub fn test_write_lme_format() {
    let mut builder = ExecBuilder::new();

    builder
        .decl_label("fib".to_string())
        .emit(Instruction::Push(Data::Register(Register::Bottom)))
        .emit(Instruction::Move(Data::Register(Register::Stack), Register::Bottom))

        .emit(Instruction::MvAux(Data::Value(1)))
        .emit(Instruction::Leab(Data::Value((-24i64) as u64), Size::QWord))
        .emit(Instruction::Cmp(EffectiveType::Unsigned))
        .jump_le("fib_else")

        .emit(Instruction::Sub(EffectiveType::Unsigned))
        .emit(Instruction::Push(Data::Register(Register::Ret)))
        .call("fib")
        .emit(Instruction::PopN(Data::Value(1)))
        .emit(Instruction::Push(Data::Register(Register::Ret)))
        .emit(Instruction::Leab(Data::Value((-24i64) as u64), Size::QWord))
        .emit(Instruction::MvAux(Data::Value(2)))
        .emit(Instruction::Sub(EffectiveType::Unsigned))
        .emit(Instruction::Push(Data::Register(Register::Ret)))
        .call("fib")
        .emit(Instruction::PopN(Data::Value(1)))
        .emit(Instruction::Pop(Register::Aux))
        .emit(Instruction::Add(EffectiveType::Unsigned))
        .jump("fib_end")

        .decl_label("fib_else")
        .emit(Instruction::MvRet(Data::Value(1)))

        .decl_label("fib_end")

        .emit(Instruction::Pop(Register::Bottom))
        .emit(Instruction::Return)
        .decl_label("main".to_string())
        .decl_entry()
        .emit(Instruction::Push(Data::Value(2)))
        .call("fib")
        .emit(Instruction::PopN(Data::Value(1)))
        .emit(Instruction::Return)
    ;

    // builder
    //     .decl_label("main")
    //     .decl_entry()
    //     .emit(Instruction::MvRet(Data::Value(2)))
    //     .emit(Instruction::MvAux(Data::Value(2)))
    //     .emit(Instruction::Add(EffectiveType::Unsigned))
    //     .emit(Instruction::Return);

    let lme = builder.build().unwrap();
    let bytes = lme.to_bytes().unwrap();

    fs::write("test.lme", bytes).unwrap();
}

#[test]
pub fn test_read_lme_format() {

    let bytes = fs::read("test.lme").unwrap();

    let mut lme = LME::from_memory(bytes).unwrap();

    println!("{}", lme.code_entry);
    for func in &lme.functions {
        println!("{}: {}", func.name, func.loc);
    }

    println!("{:?}", lme.code);
}