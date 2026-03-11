use anyhow::Result;

pub mod lexer;
mod common;

mod tests {
    use crate::common::primitives::decimal128::LDecimal128;
    use crate::common::primitives::decimal64::LDecimal64;

    #[test]
    pub fn test_decimalf64() {
        let pi = LDecimal64::new(31415926535897932, 16).unwrap();
        println!("{}", pi);

        let pi2 = LDecimal64::new(30015926535897932, 16).unwrap();
        println!("{}", pi2);
    }

    #[test]
    pub fn test_decimal128() {
        let pi = LDecimal128::new(314159265358979323846264338327950288, 35).unwrap();
        println!("{}", pi);

        let pi2 = LDecimal128::new(300159265358979323846264338327950288, 35).unwrap();
        println!("{}", pi2);
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");
    
    
    Ok(())
}
