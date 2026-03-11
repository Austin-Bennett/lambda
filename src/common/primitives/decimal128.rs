use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::common::primitives::decimal64::LDecimal64;

//also the representation for float literals
pub struct LDecimal128 {
    bytes: u128,
}

impl LDecimal128 {
    pub const EXP_SIZE: u128 = 6;

    pub fn new(digits: i128, exp: u8) -> Option<Self> {
        if (digits << Self::EXP_SIZE) >> Self::EXP_SIZE != digits { None }
        else {
            Some(Self{
                bytes: (digits << Self::EXP_SIZE) as u128 | exp as u128
            })
        }
    }

    pub fn mantissa(&self) -> i128 {
        (self.bytes >> Self::EXP_SIZE) as i128
    }

    pub fn exponent(&self) -> u8 {
        let inv_size = 8 - Self::EXP_SIZE;
        ((self.bytes as u8) << inv_size) >> inv_size
    }


    pub fn as_bytes(&self) -> u128 {
        self.bytes
    }
}

impl FromStr for LDecimal128 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        //add all the digits, find the exponent position (number of decimal digits)
        let mut exp: u8 = 0;
        let mut digits: Option<i128> = None;

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

                    *d = d.checked_mul(10).ok_or("Digit literal is larger than 5316911983139663491615228241121378304".to_string())?
                        .checked_add((c as u8 - '0' as u8) as i128).ok_or("Digit literal is larger than 5316911983139663491615228241121378304".to_string())?;
                } else {
                    digits = Some((c as u8 - '0' as u8) as i128)
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

                        *d = d.checked_mul(10).ok_or("Digit literal is larger than 5316911983139663491615228241121378304".to_string())?
                            .checked_add((c as u8 - '0' as u8) as i128).ok_or("Digit literal is larger than 5316911983139663491615228241121378304".to_string())?;
                    } else {
                        digits = Some((c as u8 - '0' as u8) as i128)
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
                    bytes: ((digits << Self::EXP_SIZE) as u128) | (exp as u128)
                })
            }
        } else {
            Err(format!("Expected numeric literal, got: {}", s))
        }
    }
}

impl Display for LDecimal128 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let exp = self.exponent();
        let mut mtsa = self.mantissa();

        let neg = mtsa < 0;
        if neg { mtsa = -mtsa; f.write_str("-")?; };
        let factor = 10i128.pow(exp as u32);

        write!(f, "{}", mtsa / factor)?;

        if exp != 0 {
            f.write_str(".")?;
            write!(f, "{:0>width$}", mtsa % factor, width = exp as usize)?;
        }


        Ok(())
    }
}

impl Debug for LDecimal128 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}