use crate::ast::Syntax;
#[cfg(test)]

use crate::ast::ty::TypeSyntax;
use crate::common::primitives::decimal128::LDecimal128;
use crate::common::primitives::decimal64::LDecimal64;
use crate::common::source_owner::{SourceDescriptor, SourceOwner};
use crate::compiler::Compiler;
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
pub fn test_decimalf64() {
    let pi = LDecimal64::new(3141592653589793238, 0);
    println!("{}", pi);

    let pi2 = LDecimal64::new(3141592653589793238, -100);
    println!("{}", pi2);


    let pi2 = LDecimal64::new(0, -18);
    println!("{}", pi2);


    let pi = LDecimal64::new(-3141592653589793238, 100);
    println!("{}", pi);

    let pi2 = LDecimal64::new(-3141592653589793238, -100);
    println!("{}", pi2);


    let pi2 = LDecimal64::new(-3141592653589793238, -18);
    println!("{}", pi2);
}

#[test]
pub fn test_decimal128() {
    let pi = LDecimal128::new(31415926535897932384626433832795028841, 0);
    println!("{}", pi);

    let pi2 = LDecimal128::new(31415926535897932384626433832795028841, -100);
    println!("{}", pi2);

    let pi2 = LDecimal128::new(31415926535897932384626433832795028841, -37);
    println!("{}", pi2);

    let pi2 = LDecimal128::new(0, -37);
    println!("{}", pi2);



    let pi = LDecimal128::new(-31415926535897932384626433832795028841, 100);
    println!("{}", pi);

    let pi2 = LDecimal128::new(-31415926535897932384626433832795028841, -100);
    println!("{}", pi2);


    let pi2 = LDecimal128::new(-31415926535897932384626433832795028841, -37);
    println!("{}", pi2);
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