use std::fmt::{Debug, Display, Formatter, LowerExp, Write};
use std::ops::Add;
use std::str::FromStr;

//64 bit decimal
#[derive(Copy, Clone)]
pub struct LDecimal64 {
    //top 56 are the digits last 8 is the exponent which is implicitly negative
    //so 3.14159 = 314159 (digits) * 10^-5 (exponent)
    bytes: u64
}

impl LDecimal64 {
    pub const EXP_SIZE: u64 = 5;
    //top 8 bits are ignored
    pub fn new(value: i64, exponent: u8) -> Option<Self> {
        if ((value << Self::EXP_SIZE) >> Self::EXP_SIZE) != value { return None; }

        Some(Self {
            bytes: ((value as u64) << Self::EXP_SIZE) | (exponent as u64)
        })
    }

    pub fn mantissa(&self) -> i64 {
        //mantissa is the highest 56 bits
        (self.bytes >> Self::EXP_SIZE) as i64
    }


    //note that the exponent is returned positive, but implicitly negative
    pub fn exponent(&self) -> u8 {
        //exponent is the lowest 8 bits
        let inv_offset = 8 - Self::EXP_SIZE;
        ((self.bytes as u8) << inv_offset) >> inv_offset
    }

    pub fn as_bytes(&self) -> u64 {
        self.bytes
    }
}

impl FromStr for LDecimal64 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        //add all the digits, find the exponent position (number of decimal digits)
        let mut exp: u8 = 0;
        let mut digits: Option<i64> = None;

        let mut chars = s.chars().peekable();
        let neg = if let Some('-') = chars.peek() {
            chars.next();
            true
        } else {
            false
        };

        while let Some(c) = chars.peek() {
            if !c.is_numeric() {
                break;
            } else {
                let c = chars.next().unwrap();
                if let Some(d) = &mut digits {

                    *d = d.checked_mul(10).ok_or("Digit literal is larger than 576460752303423488".to_string())?
                        .checked_add((c as u8 - '0' as u8) as i64).ok_or("Digit literal is larger than 576460752303423488".to_string())?;
                } else {
                    digits = Some((c as u8 - '0' as u8) as i64)
                }
            }
        }

        if let Some('.') = chars.peek() {
            chars.next();
            while let Some(c) = chars.peek() {
                if !c.is_numeric() {
                    return Err(format!("Non-numerical char: {} found in numeric literal", c));
                } else {
                    let c = chars.next().unwrap();
                    exp += 1;
                    if let Some(d) = &mut digits {

                        *d = d.checked_mul(10).ok_or("Digit literal is larger than 576460752303423488".to_string())?
                            .checked_add((c as u8 - '0' as u8) as i64).ok_or("Digit literal is larger than 576460752303423488".to_string())?;
                    } else {
                        digits = Some((c as u8 - '0' as u8) as i64)
                    }
                }
            }
        }


        let inv_offset = 8 - Self::EXP_SIZE;
        if let Some(digits) = digits {
            if (digits << Self::EXP_SIZE) >> Self::EXP_SIZE != digits {
                Err("Too many digits to store in a 56-bit mantissa".to_string())
            } else if ((exp << inv_offset) >> inv_offset) != exp {
                Err(format!("Too many decimals in numeric literal {}, note: max number of digits after decimal allowed is {}", s, 2u64.pow(Self::EXP_SIZE as u32)-1))
            } else {
                Ok(Self{
                    bytes: ((digits << Self::EXP_SIZE) as u64) | (exp as u64)
                })
            }
        } else {
            Err(format!("Expected numeric literal, got: {}", s))
        }
    }
}

impl Display for LDecimal64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let exp = self.exponent();
        let mut mtsa = self.mantissa();

        let neg = mtsa < 0;
        if neg { mtsa = -mtsa; f.write_str("-")?; };
        let factor = 10i64.pow(exp as u32);

        write!(f, "{}", mtsa / factor)?;

        if exp != 0 {
            f.write_str(".")?;
            write!(f, "{:0>width$}", mtsa % factor, width = exp as usize)?;
        }


        Ok(())
    }
}

impl Debug for LDecimal64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}



//todo: arithmetic operations