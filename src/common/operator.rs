
#[derive(Copy, Clone, Hash)]
#[derive(Debug)]
pub enum BindingPower {
    Binary(u8, u8),
    BinaryOrUnary(u8, u8),
    Unary,
    /// Post-fix operators (member access, call, index) — bind tighter than any prefix.
    PostFix,
}

impl BindingPower {
    pub const fn effective_lbp(&self) -> u8 {
        match self {
            BindingPower::Binary(l, _r) => *l,
            BindingPower::BinaryOrUnary(l, _r) => *l,
            BindingPower::Unary => 0,
            BindingPower::PostFix => 100,
        }
    }

    pub const fn effective_rbp(&self) -> u8 {
        match self {
            BindingPower::Binary(_l, r) => *r,
            BindingPower::BinaryOrUnary(_l, r) => *r,
            BindingPower::Unary => 255,
            BindingPower::PostFix => 101,
        }
    }

    pub const fn is_binary(&self) -> bool {
        match self {
            BindingPower::Binary(_, _) => true,
            BindingPower::BinaryOrUnary(_, _) => true,
            BindingPower::Unary => false,
            BindingPower::PostFix => false,
        }
    }

    pub const fn effective_unary_rbp(&self) -> u8 {
        match self {
            BindingPower::Unary => 255,
            BindingPower::BinaryOrUnary(_, _) => 255,
            BindingPower::Binary(_, _) => 0,
            BindingPower::PostFix => 0,
        }
    }

    pub const fn is_unary(&self) -> bool {
        match self {
            BindingPower::Binary(_, _) => false,
            BindingPower::BinaryOrUnary(_, _) => true,
            BindingPower::Unary => true,
            BindingPower::PostFix => false,
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

// Precedence scale (Rust-like, from lowest to highest):
// assign(0) < ||(2) < &&(4) < cmp(6) < |(8) < ^(10) < &(12) < shifts(14) < add(16) < mul(18)
// lbp > rbp → right-assoc; lbp < rbp → left-assoc (same-prec doesn't bind to its own RHS)
impl Operator {
    pub const MIN_BINARY_BP:    BindingPower = BindingPower::Binary(0,  1);
    pub const BOOL_OR_BP:       BindingPower = BindingPower::Binary(2,  1);
    pub const BOOL_AND_BP:      BindingPower = BindingPower::Binary(4,  3);
    pub const COMPARISON_BP:    BindingPower = BindingPower::Binary(6,  7);
    pub const BIT_OR_BP:        BindingPower = BindingPower::Binary(8,  7);
    pub const BIT_XOR_BP:       BindingPower = BindingPower::Binary(10, 9);
    pub const BIT_AND_BP:       BindingPower = BindingPower::Binary(12, 11);
    pub const SHIFT_BP:         BindingPower = BindingPower::Binary(14, 13);
    pub const ADDITIVE_BP:      BindingPower = BindingPower::Binary(16, 15);
    pub const MULTIPLICATIVE_BP:BindingPower = BindingPower::Binary(18, 17);

    // assignment (same low BP for all variants)
    pub const ASSIGN:       Self = Operator { tk: "=",   bp: Operator::MIN_BINARY_BP };
    pub const ADD_ASSIGN:   Self = Operator { tk: "+=",  bp: Operator::MIN_BINARY_BP };
    pub const SUB_ASSIGN:   Self = Operator { tk: "-=",  bp: Operator::MIN_BINARY_BP };
    pub const MUL_ASSIGN:   Self = Operator { tk: "*=",  bp: Operator::MIN_BINARY_BP };
    pub const DIV_ASSIGN:   Self = Operator { tk: "/=",  bp: Operator::MIN_BINARY_BP };
    pub const BIT_AND_ASSIGN: Self = Operator { tk: "&=", bp: Operator::MIN_BINARY_BP };
    pub const BIT_OR_ASSIGN:  Self = Operator { tk: "|=", bp: Operator::MIN_BINARY_BP };
    pub const BIT_XOR_ASSIGN: Self = Operator { tk: "^=", bp: Operator::MIN_BINARY_BP };
    pub const SHL_ASSIGN:   Self = Operator { tk: "<<=", bp: Operator::MIN_BINARY_BP };
    pub const SHR_ASSIGN:   Self = Operator { tk: ">>=", bp: Operator::MIN_BINARY_BP };
    pub const BOOL_AND_ASSIGN: Self = Operator { tk: "&&=", bp: Operator::MIN_BINARY_BP };
    pub const BOOL_OR_ASSIGN:  Self = Operator { tk: "||=", bp: Operator::MIN_BINARY_BP };

    // boolean short-circuit
    pub const BOOL_OR:  Self = Operator { tk: "||", bp: Operator::BOOL_OR_BP };
    pub const BOOL_AND: Self = Operator { tk: "&&", bp: Operator::BOOL_AND_BP };

    // comparison
    pub const EQ: Self = Operator { tk: "==", bp: Operator::COMPARISON_BP };
    pub const NE: Self = Operator { tk: "!=", bp: Operator::COMPARISON_BP };
    pub const LT: Self = Operator { tk: "<",  bp: Operator::COMPARISON_BP };
    pub const GT: Self = Operator { tk: ">",  bp: Operator::COMPARISON_BP };
    pub const LE: Self = Operator { tk: "<=", bp: Operator::COMPARISON_BP };
    pub const GE: Self = Operator { tk: ">=", bp: Operator::COMPARISON_BP };

    // bitwise binary
    pub const BIT_OR:  Self = Operator { tk: "|",  bp: Operator::BIT_OR_BP };
    pub const BIT_XOR: Self = Operator { tk: "^",  bp: Operator::BIT_XOR_BP };
    pub const BIT_AND: Self = Operator { tk: "&",  bp: Operator::BIT_AND_BP.to_binary_or_unary() };
    pub const SHL:     Self = Operator { tk: "<<", bp: Operator::SHIFT_BP };
    pub const SHR:     Self = Operator { tk: ">>", bp: Operator::SHIFT_BP };

    // arithmetic
    pub const ADD: Self = Operator { tk: "+", bp: Operator::ADDITIVE_BP };
    pub const SUB: Self = Operator { tk: "-", bp: Operator::ADDITIVE_BP.to_binary_or_unary() };
    pub const MUL: Self = Operator { tk: "*", bp: Operator::MULTIPLICATIVE_BP.to_binary_or_unary() };
    pub const DIV: Self = Operator { tk: "/", bp: Operator::MULTIPLICATIVE_BP };

    // unary-only
    pub const NOT: Self = Operator { tk: "!", bp: BindingPower::Unary };
}