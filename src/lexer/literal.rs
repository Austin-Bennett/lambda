use anyhow::Result;
use crate::common::primitives::decimal128::LDecimal128;

#[derive(Copy, Clone)]
pub struct IntegerLiteral {
    pub negative: bool,
    pub value: u128,
}


impl IntegerLiteral {
    //fails if there are no digits, no literal, or bigger than a u64
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
    pub fn as_u8(&self) -> Option<u8> {
        if self.negative { None }
        else if self.value & 0xFF != self.value { None }
        else { Some(self.value as u8) }
    }

    //converts to u16, ignoring extra bits and combining the sign into the first significant bit
    pub fn as_u16_lossy(&self) -> u16 {
        self.value as u16 | if self.negative { 1 << 15 } else { 0 }
    }

    //fails if negative or too large
    pub fn as_u16(&self) -> Option<u16> {
        if self.negative { None }
        else if self.value & 0xFFFF != self.value { None }
        else { Some(self.value as u16) }
    }

    //converts to u32, ignoring extra bits and combining the sign into the first significant bit
    pub fn as_u32_lossy(&self) -> u32 {
        self.value as u32 | if self.negative { 1 << 31 } else { 0 }
    }

    //fails if negative or too large
    pub fn as_u32(&self) -> Option<u32> {
        if self.negative { None }
        else if self.value & 0xFFFFFF != self.value { None }
        else { Some(self.value as u32) }
    }

    //converts to u64 ignoring extra bits and combining the sign into the first significant bit
    pub fn as_u64_lossy(&self) -> u64 {

        self.value as u64 | if self.negative { 1 << 63 } else { 0 }
    }

    //fails if negative or too big
    pub fn as_u64(&self) -> Option<u64> {
        if self.negative { None }
        else if self.value & 0xFFFFFFFF_FFFFFFFF != self.value { None }
        else { Some(self.value as u64) }
    }

    //converts to u128 combining the sign into the first significant bit
    pub fn as_u128_lossy(&self) -> u128 {

        self.value | if self.negative { 1 << 127 } else { 0 }
    }

    //fails if negative
    pub fn as_u128(&self) -> Option<u128> {
        if self.negative { None }
        else { Some(self.value as u128) }
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
    pub fn as_i8(&self) -> Option<i8> {
        if self.value & 0x7F != self.value { None }
        else {
            let v = self.value as i8;
            Some(if self.negative {
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
    pub fn as_i16(&self) -> Option<i16> {
        if self.value & 0x7F_FF != self.value { None }
        else {
            let v = self.value as i16;
            Some(if self.negative {
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
    pub fn as_i32(&self) -> Option<i32> {
        if self.value & 0x7F_FF_FF_FF != self.value { None }
        else {
            let v = self.value as i32;
            Some(if self.negative {
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
    pub fn as_i64(&self) -> Option<i64> {
        if self.value & 0x7F_FF_FF_FF_FF_FF_FF_FF != self.value { None }
        else {
            let v = self.value as i64;
            Some(if self.negative {
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
    pub fn as_i128(&self) -> Option<i128> {
        if self.value & 0x7F_FF_FF_FF_FF_FF_FF_FF_FF_FF_FF_FF_FF_FF_FF_FF != self.value { None }
        else {
            let v = self.value as i128;
            Some(if self.negative {
                -v
            } else {
                v
            })
        }
    }
}



pub struct ScientificLiteral {
    pub base: LDecimal128,
    pub exp: LDecimal128,
}

impl ScientificLiteral {
    pub fn from_string(s: impl AsRef<str>) -> Option<Self> {
        //parse up until the e or E
        let e = s.as_ref().find('e').unwrap_or(s.as_ref().find('E')?);

        let base: LDecimal128 = s.as_ref()[0..e].parse().ok()?;

        let exp: LDecimal128 = s.as_ref()[e+1..].parse().ok()?;

        Some( ScientificLiteral{
            base,
            exp
        } )
    }
}