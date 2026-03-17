
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum BindingPower {
    Binary(u8, u8),
    BinaryOrUnary(u8, u8),
    Unary,
}

impl BindingPower {
    pub const fn effective_lbp(&self) -> u8 {
        match self {
            BindingPower::Binary(l, r) => *l,
            BindingPower::BinaryOrUnary(l, r) => *l,
            //unary operator has no precedence to the left
            BindingPower::Unary => 0,
        }
    }

    pub const fn effective_rbp(&self) -> u8 {
        match self {
            BindingPower::Binary(l, r) => *r,
            BindingPower::BinaryOrUnary(l, r) => *r,
            //unary operator has max precedence to the right
            BindingPower::Unary => 255,
        }
    }

    pub const fn is_binary(&self) -> bool {
        match self {
            BindingPower::Binary(_, _) => true,
            BindingPower::BinaryOrUnary(_, _) => true,
            BindingPower::Unary => false,
        }
    }

    pub const fn is_unary(&self) -> bool {
        match self {
            BindingPower::Binary(_, _) => false,
            BindingPower::BinaryOrUnary(_, _) => true,
            BindingPower::Unary => true,
        }
    }

    pub const fn to_binary_or_unary(self) -> BindingPower {
        BindingPower::BinaryOrUnary(self.effective_lbp(), self.effective_rbp())
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Operator {
    pub bp: BindingPower,
    pub tk: &'static str,
}

impl Operator {
    pub const MIN_BINARY_BP: BindingPower = BindingPower::Binary(0, 0);
    pub const ADDITIVE_BP: BindingPower = BindingPower::Binary(1, 1);
    pub const MULTIPLICATIVE_BP: BindingPower = BindingPower::Binary(2, 2);




    pub const ADD: Self = Operator{ tk: "+", bp: Operator::ADDITIVE_BP };
    pub const SUB: Self = Operator{ tk: "-", bp: Self::ADDITIVE_BP.to_binary_or_unary() };
    pub const MUL: Self = Operator{ tk: "*", bp: Operator::MULTIPLICATIVE_BP };
    pub const DIV: Self = Operator{ tk: "/", bp: Operator::MULTIPLICATIVE_BP };
}