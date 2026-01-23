pub mod tokenization;
pub mod expression;
pub mod variables;
pub mod constants;

pub use expression::*;
pub use tokenization::*;
pub use variables::*;

const EPSILON: f64 = 1e-15;

pub trait FloatHelpers {
    fn equates(self, other: Self) -> bool;
}

impl FloatHelpers for f64 {
    fn equates(self, other: Self) -> bool {
        (self - other).abs() < EPSILON
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