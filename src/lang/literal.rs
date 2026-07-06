

pub enum Literal {
    Int(IntLiteral),
    Float(f64),
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IntLiteral {
    pub(crate) negative: bool,
    pub(crate) val: u64,
}

impl IntLiteral {
    pub fn as_signed(&self) -> Option<i64> {
        if self.negative {
            let min_magnitude = i64::MIN.unsigned_abs(); // 9223372036854775808
            match self.val.cmp(&min_magnitude) {
                std::cmp::Ordering::Greater => None,
                std::cmp::Ordering::Equal => Some(i64::MIN),
                std::cmp::Ordering::Less => Some(-(self.val as i64)),
            }
        } else {
            i64::try_from(self.val).ok()
        }
    }

    pub fn as_signed_lossy(&self) -> i64 {
        if self.negative {
            let min_magnitude = i64::MIN.unsigned_abs();
            if self.val >= min_magnitude {
                i64::MIN
            } else {
                -(self.val as i64)
            }
        } else {
            if self.val > i64::MAX as u64 {
                i64::MAX
            } else {
                self.val as i64
            }
        }
    }

    pub fn as_unsigned(&self) -> u64 {
        if self.negative {
            self.val.wrapping_neg()
        } else {
            self.val
        }
    }
}