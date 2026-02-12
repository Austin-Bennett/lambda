pub mod tokenization;
pub mod expression;
pub mod statement;

use std::cmp::{Ordering, PartialOrd};
use std::str::FromStr;
use lazy_static::lazy_static;
use rust_decimal::Decimal;
pub use expression::*;
pub use tokenization::*;

lazy_static!{
    pub static ref EPSILON: Decimal = Decimal::from_str("0.0000000000000000000000000001").unwrap();
}

pub trait FloatHelpers {
    fn equates(&self, other: Self) -> bool;
    fn is_zero(&self) -> bool;
}



impl FloatHelpers for Decimal {
    fn equates(&self, other: Self) -> bool {
        (self - other).abs() <= *EPSILON
    }

    fn is_zero(&self) -> bool {
        self.abs() <= *EPSILON
    }
}

#[macro_export]
macro_rules! time {
    {$($body:tt)*} => {{

        let start = std::time::Instant::now();

        {
            $($body)*
        }

        std::time::Instant::now() - start
    }};
}