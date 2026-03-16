
#[derive(Clone)]
#[derive(Debug)]
pub enum BindingPower {
    Binary(u8, u8),
    Unary,
}

#[derive(Clone, Debug)]
pub struct Operator {
    pub bp: BindingPower,
    pub tk: &'static str,
}

impl Operator {
    pub const MIN_BINARY_BP: BindingPower = BindingPower::Binary(0, 0);
    pub const ADDITIVE_BP: BindingPower = BindingPower::Binary(1, 1);
    pub const MULTIPLICATIVE_BP: BindingPower = BindingPower::Binary(2, 2);
}