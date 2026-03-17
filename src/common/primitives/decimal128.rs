use std::fmt::{Debug, Display, Formatter};

//also the representation for float literals
#[derive(Copy, Clone)]
pub struct LDecimal128 {
    mantissa: i128,
    exponent: i16,
}

impl LDecimal128 {
    pub const MAX_DIGITS: i16 = 38;
    pub fn new(value: i128, exponent: i16) -> Self {


        Self {
            mantissa: value,
            exponent
        }
    }

    pub fn mantissa(&self) -> i128 {
        self.mantissa
    }


    pub fn exponent(&self) -> i16 {
        self.exponent
    }

}



impl Display for LDecimal128 {
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
            // > 10^38
            write!(f, "{:0<width$}", mtsa, width = exp as usize)
        } else if exp < -Self::MAX_DIGITS {
            // < 1
            write!(f, "0.{:0>width$}", mtsa, width = -exp as usize)
        } else {
            let factor = 10i128.pow(exp.abs() as u32);
            write!(f, "{}.{:0<width$}", mtsa / factor, mtsa % factor, width = -exp as usize)
        }
    }
}

impl Debug for LDecimal128 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
