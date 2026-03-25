use crate::ast::structure::StructureSyntax;
#[cfg(test)]

use crate::ast::ty::TypeSyntax;
use crate::ast::Syntax;
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