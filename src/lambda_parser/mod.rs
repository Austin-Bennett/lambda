pub mod tokenization;
pub mod expression;
pub mod statement;


use std::cmp::{Ordering, PartialOrd};
use std::str::FromStr;
use lazy_static::lazy_static;
pub use expression::*;
pub use tokenization::*;



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