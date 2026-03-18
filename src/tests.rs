use crate::ast::block::BlockSyntax;
#[cfg(test)]

use crate::ast::expressions::ExprSyntax;
use crate::ast::statement::StatementSyntax;
use crate::ast::Syntax;
use crate::common::primitives::decimal128::LDecimal128;
use crate::common::primitives::decimal64::LDecimal64;
use crate::common::source_owner::{SourceDescriptor, SourceOwner};
use crate::common::utils::outcome::Outcome;
use crate::lexer::token::TokenType;
use crate::lexer::tokenizer::Tokens;

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
pub fn test_expressions() {
    let mut tokens = Tokens::tokenize_string(
        SourceOwner{
            descriptor: SourceDescriptor::RustString,
            name: "test_expressions_test_string".to_string()
        },
        "(b b + sqrt(-b - 4a c)) / 2a".to_string()
    )
        .filter(|tk| if let TokenType::User(_) = tk.typ { false } else { true })
        .collect();


    let expr = ExprSyntax::parse(&mut tokens);

    match expr {
        Outcome::Ok(e) => {
            println!("{:?}", e.data)
        }
        Outcome::None => eprintln!("No expression!"),
        Outcome::Err(e) => eprintln!("Compile error: {:?}", e)
    }
}


#[test]
pub fn test_statement_expr() {
    let mut tokens = Tokens::tokenize_string(
        SourceOwner{
            descriptor: SourceDescriptor::RustString,
            name: "test_expressions_test_string".to_string()
        },
        "(b b + sqrt(-b - 4a c)) / 2a".to_string()
    )
        .filter(|tk| if let TokenType::User(_) = tk.typ { false } else { true })
        .collect();


    let expr = StatementSyntax::parse(&mut tokens);

    match expr {
        Outcome::Ok(e) => {
            println!("{:?}", e.data)
        }
        Outcome::None => eprintln!("No statement!"),
        Outcome::Err(e) => eprintln!("Compile error: {:?}", e)
    }
}

#[test]
pub fn test_statement_var_decl_set() {
    let mut tokens = Tokens::tokenize_string(
        SourceOwner{
            descriptor: SourceDescriptor::RustString,
            name: "test_expressions_test_string".to_string()
        },
        "a: i64 = (b b + sqrt(-b - 4a c)) / 2a".to_string()
    )
        .filter(|tk| if let TokenType::User(_) = tk.typ { false } else { true })
        .collect();


    let expr = StatementSyntax::parse(&mut tokens);

    match expr {
        Outcome::Ok(e) => {
            println!("{:?}", e.data)
        }
        Outcome::None => eprintln!("No statement!"),
        Outcome::Err(e) => eprintln!("Compile error: {:?}", e)
    }
}

#[test]
pub fn test_statement_var_decl_unset() {
    let mut tokens = Tokens::tokenize_string(
        SourceOwner{
            descriptor: SourceDescriptor::RustString,
            name: "test_expressions_test_string".to_string()
        },
        "a: i64".to_string()
    )
        .filter(|tk| if let TokenType::User(_) = tk.typ { false } else { true })
        .collect();


    let expr = StatementSyntax::parse(&mut tokens);

    match expr {
        Outcome::Ok(e) => {
            println!("{:?}", e.data)
        }
        Outcome::None => eprintln!("No statement!"),
        Outcome::Err(e) => eprintln!("Compile error: {:?}", e)
    }
}

#[test]
pub fn test_block() {
    let mut tokens = Tokens::tokenize_string(
        SourceOwner{
            descriptor: SourceDescriptor::RustString,
            name: "test_expressions_test_string".to_string()
        },
        "{\n\
            (b b + sqrt(-b - 4a c)) / 2a\n\
            a: i64 = (b b + sqrt(-b - 4a c)) / 2a\n\
            a: i64\n\
        }".to_string()
    )
        .filter(|tk| if let TokenType::User(_) = tk.typ { false } else { true })
        .collect();


    let expr = BlockSyntax::parse(&mut tokens);

    match expr {
        Outcome::Ok(e) => {
            println!("{:?}", e)
        }
        Outcome::None => eprintln!("No block!"),
        Outcome::Err(e) => eprintln!("Compile error: {:?}", e)
    }
}