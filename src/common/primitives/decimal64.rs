use std::fmt::{Debug, Display, Formatter, LowerExp, Write};
use std::ops::Add;
use std::str::FromStr;

//64 bit decimal
#[derive(Copy, Clone)]
pub struct LDecimal64 {
    mantissa: i64,
    exponent: i8,
}

impl LDecimal64 {
    pub const MAX_DIGITS: i8 = 18;
    pub fn new(value: i64, exponent: i8) -> Self {


        Self {
            mantissa: value,
            exponent
        }
    }

    pub fn mantissa(&self) -> i64 {
        self.mantissa
    }


    pub fn exponent(&self) -> i8 {
        self.exponent
    }

}



impl Display for LDecimal64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let exp = self.exponent();
        let mut mtsa = self.mantissa();

        if mtsa == 0 {
            return write!(f, "0");
        }


        let neg = mtsa < 0;
        if neg { mtsa = -mtsa; f.write_str("-")?; };

        //max digits for a 64-bit signed number
        if exp >= 0 {
            // > 10^18
            write!(f, "{:0<width$}", mtsa, width = exp as usize)
        } else if exp < -Self::MAX_DIGITS {
            // < 1
            write!(f, "0.{:0>width$}", mtsa, width = -exp as usize)
        } else {
            let factor = 10i64.pow(exp.abs() as u32);
            write!(f, "{}.{:0<width$}", mtsa / factor, mtsa % factor, width = -exp as usize)
        }
    }
}

impl Debug for LDecimal64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}



//todo: arithmetic operations