use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Hash)]
pub struct IntegerLiteral {
    pub negative: bool,
    pub value: u128,
}

impl Debug for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", if self.negative { "-" } else { "" }, self.value)
    }
}

impl IntegerLiteral {
    //fails if there are no digits, no literal, or bigger than a u128
    pub fn from_string(s: impl AsRef<str>) -> Option<Self> {
        let mut chars = s.as_ref().chars().peekable();

        let is_negative = *chars.peek()? == '-';

        if is_negative { chars.next(); }

        let mut v = None;


        while let Some(c) = chars.next() {
            if !c.is_numeric() {
                break;
            } else {
                if v.is_none() {
                    v = Some(c as u128 - '0' as u128);
                } else {
                    //returns if the integer overflows
                    v = Some(v.unwrap().checked_mul(10)?.checked_add(c as u128 - '0' as u128)?)
                }
            }
        }

        Some(Self{
            negative: is_negative,
            value: v?,
        })
    }

    //converts to u8, combining the sign into the first significant bit
    pub fn as_u8_lossy(&self) -> u8 {
        self.value as u8 | if self.negative { 1 << 7 } else { 0 }
    }

    //fails if negative or too large
    pub fn as_u8(&self) -> anyhow::Result<u8> {
        if self.negative { 
            Err(anyhow::Error::msg("negative values are not allowed for unsigned integers!"))
        }
        else if self.value & 0xFF != self.value { 
            Err(anyhow::Error::msg(format!("{} is too big to fit inside a 8-bit unsigned integer!", self.value)))
        }
        else { Ok(self.value as u8) }
    }

    //converts to u16, ignoring extra bits and combining the sign into the first significant bit
    pub fn as_u16_lossy(&self) -> u16 {
        self.value as u16 | if self.negative { 1 << 15 } else { 0 }
    }

    //fails if negative or too large
    pub fn as_u16(&self) -> anyhow::Result<u16> {
        if self.negative {
            Err(anyhow::Error::msg("negative values are not allowed for unsigned integers!"))
        }
        else if self.value & 0xFF_FF != self.value {
            Err(anyhow::Error::msg(format!("{} is too big to fit inside a 16-bit unsigned integer!", self.value)))
        }
        else { Ok(self.value as u16) }
    }

    //converts to u32, ignoring extra bits and combining the sign into the first significant bit
    pub fn as_u32_lossy(&self) -> u32 {
        self.value as u32 | if self.negative { 1 << 31 } else { 0 }
    }

    //fails if negative or too large
    pub fn as_u32(&self) -> anyhow::Result<u32> {
        if self.negative {
            Err(anyhow::Error::msg("negative values are not allowed for unsigned integers!"))
        }
        else if self.value & 0xFF_FF_FF_FF != self.value {
            Err(anyhow::Error::msg(format!("{} is too big to fit inside a 32-bit unsigned integer!", self.value)))
        }
        else { Ok(self.value as u32) }
    }

    //converts to u64 ignoring extra bits and combining the sign into the first significant bit
    pub fn as_u64_lossy(&self) -> u64 {
        self.value as u64 | if self.negative { 1 << 63 } else { 0 }
    }

    //fails if negative or too big
    pub fn as_u64(&self) -> anyhow::Result<u64> {
        if self.negative {
            Err(anyhow::Error::msg("negative values are not allowed for unsigned integers!"))
        }
        else if self.value & 0xFF_FF != self.value {
            Err(anyhow::Error::msg(format!("{} is too big to fit inside a 64-bit unsigned integer!", self.value)))
        }
        else { Ok(self.value as u64) }
    }

    //converts to u128 combining the sign into the first significant bit
    pub fn as_u128_lossy(&self) -> u128 {

        self.value | if self.negative { 1 << 127 } else { 0 }
    }

    //fails if negative
    pub fn as_u128(&self) -> anyhow::Result<u128> {
        if self.negative { Err(anyhow::Error::msg("negative values are not allowed for unsigned integers!")) }
        else { Ok(self.value) }
    }


    //converts to i8, ignoring extra bits
    pub fn as_i8_lossy(&self) -> i8 {
        let v = self.value as i8;
        if self.negative {
            -v
        } else {
            v
        }
    }

    //fails if too large
    pub fn as_i8(&self) -> anyhow::Result<i8> {
        if self.value & 0x7F != self.value { 
            Err(anyhow::Error::msg(format!("value of magnitude: {} cannot fit in signed 8-bit integer", self.value)))
        }
        else {
            let v = self.value as i8;
            Ok(if self.negative {
                -v
            } else {
                v
            })
        }
    }

    //converts to i16, ignoring extra bits
    pub fn as_i16_lossy(&self) -> i16 {
        let v = self.value as i16;
        if self.negative {
            -v
        } else {
            v
        }
    }

    //fails if too large
    pub fn as_i16(&self) -> anyhow::Result<i16> {
        if self.value & 0x7F_FF != self.value {
            Err(anyhow::Error::msg(format!("value of magnitude: {} cannot fit in signed 16-bit integer", self.value)))
        }
        else {
            let v = self.value as i16;
            Ok(if self.negative {
                -v
            } else {
                v
            })
        }
    }

    //converts to i32, ignoring extra bits
    pub fn as_i32_lossy(&self) -> i32 {
        let v = self.value as i32;
        if self.negative {
            -v
        } else {
            v
        }
    }

    //fails if too large
    pub fn as_i32(&self) -> anyhow::Result<i32> {
        if self.value & 0x7F_FF_FF_FF != self.value {
            Err(anyhow::Error::msg(format!("value of magnitude: {} cannot fit in signed 32-bit integer", self.value)))
        }
        else {
            let v = self.value as i32;
            Ok(if self.negative {
                -v
            } else {
                v
            })
        }
    }

    //converts to i64, ignoring extra bits
    pub fn as_i64_lossy(&self) -> i64 {
        let v = self.value as i64;
        if self.negative {
            -v
        } else {
            v
        }
    }

    //fails if too large
    pub fn as_i64(&self) -> anyhow::Result<i64> {
        if self.value & 0x7F_FF_FF_FF_FF_FF_FF_FF != self.value {
            Err(anyhow::Error::msg(format!("value of magnitude: {} cannot fit in signed 64-bit integer", self.value)))
        }
        else {
            let v = self.value as i64;
            Ok(if self.negative {
                -v
            } else {
                v
            })
        }
    }

    //converts to i128, ignoring extra bits
    pub fn as_i128_lossy(&self) -> i128 {
        let v = self.value as i128;
        if self.negative {
            -v
        } else {
            v
        }
    }

    //fails if too large
    pub fn as_i128(&self) -> anyhow::Result<i128> {
        if self.value & 0x7F_FF_FF_FF_FF_FF_FF_FF_FF_FF_FF_FF_FF_FF_FF_FF != self.value {
            Err(anyhow::Error::msg(format!("value of magnitude: {} cannot fit in signed 128-bit integer", self.value)))
        }
        else {
            let v = self.value as i128;
            Ok(if self.negative {
                -v
            } else {
                v
            })
        }
    }
}




#[derive(Clone, Debug)]
pub enum LiteralValue {
    Integer(IntegerLiteral),
    Float(f64),
    Bool(bool),
}

impl std::hash::Hash for LiteralValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            LiteralValue::Integer(i) => { 0u8.hash(state); i.hash(state); }
            LiteralValue::Float(f)   => { 1u8.hash(state); f.to_bits().hash(state); }
            LiteralValue::Bool(b)    => { 2u8.hash(state); b.hash(state); }
        }
    }
}
