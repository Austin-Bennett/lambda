
#[derive(Copy, Clone, Hash)]
#[derive(Debug)]
pub enum BindingPower {
    Binary(u8, u8),
    BinaryOrUnary(u8, u8),
    Unary,
}

impl BindingPower {
    pub const fn effective_lbp(&self) -> u8 {
        match self {
            BindingPower::Binary(l, _r) => *l,
            BindingPower::BinaryOrUnary(l, _r) => *l,
            //unary operator has no precedence to the left
            BindingPower::Unary => 0,
        }
    }

    pub const fn effective_rbp(&self) -> u8 {
        match self {
            BindingPower::Binary(_l, r) => *r,
            BindingPower::BinaryOrUnary(_l, r) => *r,
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
#[derive(Copy, Clone, Debug, Hash)]
pub struct Operator {
    pub bp: BindingPower,
    pub tk: &'static str,
}

impl Operator {
    pub const MIN_BINARY_BP: BindingPower = BindingPower::Binary(0, 1);
    pub const ADDITIVE_BP: BindingPower = BindingPower::Binary(4, 3);
    pub const MULTIPLICATIVE_BP: BindingPower = BindingPower::Binary(6, 5);
    pub const BITWISE_AND_BP: BindingPower = BindingPower::Binary(2, 1);





    pub const ASSIGN: Self = Operator{ tk: "=", bp: Operator::MIN_BINARY_BP };

    pub const ADD: Self = Operator{ tk: "+", bp: Operator::ADDITIVE_BP };
    pub const SUB: Self = Operator{ tk: "-", bp: Self::ADDITIVE_BP.to_binary_or_unary() };
    pub const MUL: Self = Operator{ tk: "*", bp: Operator::MULTIPLICATIVE_BP.to_binary_or_unary() };
    pub const DIV: Self = Operator{ tk: "/", bp: Operator::MULTIPLICATIVE_BP };

    pub const BITWISE_AND: Self = Operator{ tk: "&", bp: Operator::BITWISE_AND_BP.to_binary_or_unary() };
}