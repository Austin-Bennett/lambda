use lazy_static::lazy_static;
use radix_trie::{Trie, TrieCommon};

//acts as the operators 'type'
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BindingPower {
    BinaryOrUnary(u8, u8),
    Binary(u8, u8),
    Unary,
    PostFix,
    Unknown,
}

impl BindingPower {
    pub const ADDITIVE_BP: (u8, u8) = (5, 6);
    pub const MULTIPLICATIVE_BP: (u8, u8) = (7, 8);
    
    pub fn binary_or_unary(bp: (u8, u8)) -> Self {
        Self::BinaryOrUnary(bp.0, bp.1)
    }
    
    pub fn binary(bp: (u8, u8)) -> Self {
        Self::Binary(bp.0, bp.1)
    }
    
    pub fn is_unary(&self) -> bool {
        match self { 
            BindingPower::BinaryOrUnary(_, _) | BindingPower::Unary => true,
            _ => false
        }
    }
    
    pub fn is_binary(&self) -> Option<(u8, u8)> {
        match self {
            BindingPower::BinaryOrUnary(lbp, rbp) | BindingPower::Binary(lbp, rbp) => Some((*lbp, *rbp)),
            _ => None
        }
    }
}