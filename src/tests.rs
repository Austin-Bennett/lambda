use std::collections::VecDeque;
use crate::ast::block::BlockSyntax;
#[cfg(test)]

use crate::ast::expressions::ExprSyntax;
use crate::ast::items::function::FunctionSyntax;
use crate::ast::items::ItemSyntax;
use crate::ast::items::structure::StructureSyntax;
use crate::ast::statement::StatementSyntax;
use crate::ast::Syntax;
use crate::common::primitives::decimal128::LDecimal128;
use crate::common::primitives::decimal64::LDecimal64;
use crate::common::source_owner::{SourceDescriptor, SourceOwner};
use crate::common::utils::outcome::Outcome;
use crate::lexer::token::{Token, TokenType};
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




