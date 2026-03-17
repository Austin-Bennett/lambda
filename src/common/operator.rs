
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum BindingPower {
    Binary(u8, u8),
    Unary,
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
    pub const SUB: Self = Operator{ tk: "-", bp: Operator::ADDITIVE_BP };
    pub const MUL: Self = Operator{ tk: "*", bp: Operator::MULTIPLICATIVE_BP };
    pub const DIV: Self = Operator{ tk: "/", bp: Operator::MULTIPLICATIVE_BP };
}