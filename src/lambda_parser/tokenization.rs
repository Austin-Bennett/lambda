use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter, Write};
use std::str::FromStr;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum BindingPower {
    Seperator,
    #[allow(dead_code)]
    Unary,
    Binary(f32, f32),
    BinaryOrUnary(f32, f32), //binary bp if being parsed as a binary operator
}

pub const MINBP: f32 = f32::NEG_INFINITY;

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub struct Operator {
    pub token: &'static str,
    pub bp: BindingPower
}

impl Operator {
    pub fn new(tk: &'static str, bp: BindingPower) -> Self {
        Self{
            token: tk,
            bp
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[derive(PartialEq)]
pub enum Keyword {
    If,
    Else,
}

lazy_static!(
    pub static ref operators: Vec<Operator> = {
        let mut res = vec![
            Operator::new("+", BindingPower::BinaryOrUnary(1.0, 1.1)),
            Operator::new("-", BindingPower::BinaryOrUnary(1.0, 1.1)),
            Operator::new("*", BindingPower::Binary(2.0, 2.1)),
            Operator::new("/", BindingPower::Binary(2.0, 2.1)),
            Operator::new("**", BindingPower::Binary(3.0, 3.1)),

            Operator::new("<", BindingPower::Binary(1.0, 1.1)),
            Operator::new(">", BindingPower::Binary(1.0, 1.1)),
            Operator::new("<=", BindingPower::Binary(1.0, 1.1)),
            Operator::new(">=", BindingPower::Binary(1.0, 1.1)),
            Operator::new("!=", BindingPower::Binary(1.0, 1.1)),
            Operator::new("==", BindingPower::Binary(1.0, 1.1)),

            Operator::new(",", BindingPower::Seperator),
            Operator::new("=", BindingPower::Binary(-10.0, -9.9)),
        ];

        res.sort_by(|f, s|
            if f.token.len() > s.token.len() { Ordering::Less } else { Ordering::Greater } );
        res
    };

    pub static ref keywords: Vec<(&'static str, Keyword)> = {
        let mut res = vec![
            ("if", Keyword::If),
            ("else", Keyword::Else),
        ];

        res.sort_by(|f, s|
            if f.0.len() > s.0.len() { Ordering::Less } else { Ordering::Greater } );
        res
    };
);

pub fn get_keyword(st: &str) -> Option<(usize, Keyword)> {
    for k in (&keywords).iter() {
        if st.starts_with(k.0) {
            return Some((k.0.len(), k.1))
        }
    }
    None
}

pub fn get_operator(st: &str) -> Option<(usize, Operator)> {
    for op in (&operators).iter() {
        if st.starts_with(op.token) {
            return Some((op.token.len(), *op))
        }
    }
    None
}

#[derive(Clone)]
#[derive(PartialEq)]
pub struct NumericalLiteral {
    pub literal: String,
    pub flags: u8,
}

impl NumericalLiteral {
    const F_PERCENTAGE: u8 = 0b1;
    const F_INTEGER: u8 = 0b10;
}

impl Display for NumericalLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = self.literal.clone();
        if self.flags & Self::F_PERCENTAGE > 0 {
            res.push('%');
        }
        write!(f, "{}", res)
    }
}

pub fn get_number(st: &str) -> Option<(usize, NumericalLiteral)> {


    //consume all numbers, and 1 optional decimal
    let mut found_dec = false;
    let mut prcnt = false;

    let mut res: String = String::new();
    let mut len = 0;

    for c in st.chars() {
        if c.is_numeric() {
            res.push(c);
            len += 1;
        } else if c == '.' && !found_dec {
            found_dec = true;
            res.push('.');
            len += 1;
        } else {
            if c == '%' {
                prcnt = true;
                len += 1;
            }
            break;
        }
    }


    if res.len() > 0 {
        Some((len, NumericalLiteral{
            literal: res,
            flags: 0
                | if prcnt { NumericalLiteral::F_PERCENTAGE } else { 0 }
                | if !found_dec { NumericalLiteral::F_INTEGER } else { 0 }
        }))
    } else {
        None
    }
}

pub fn get_identifier(st: &str) -> Option<(usize, String)> {
    //read while the chars are alphabetical
    let mut res: String = String::new();

    for c in st.chars() {
        if c.is_alphabetic() { //todo, allow some symbols like $ or #?
            res.push(c);
        } else {
            break;
        }
    }



    if res.len() > 0 {
        Some((res.len(), res))
    } else {
        None
    }
}



#[derive(Clone)]
#[derive(PartialEq)]
pub enum Token {
    Ident(String),
    Num(NumericalLiteral),
    Operator(Operator),
    Keyword(Keyword),
    ParenthesesGroup(VecDeque<Token>),
    OpenBrace,
    CloseBrace,
    Newline,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(s) => f.write_str(s.as_str()),
            Token::Num(n) => f.write_str(n.to_string().as_str()),
            Token::Operator(o) => f.write_str(o.token),
            Token::ParenthesesGroup(tks) => {
                tks.fmt(f)
            }
            Token::CloseBrace => f.write_str("}"),
            Token::OpenBrace => f.write_str("{"),
            Token::Keyword(k) => k.fmt(f),
            Token::Newline => f.write_str("\\n"),
        }
    }
}

impl Token {
    pub fn is_expression(&self) -> bool {
        match self {
            Token::Ident(_) => true,
            Token::Num(_) => true,
            Token::Operator(_) => true,
            Token::Keyword(_) => false,
            Token::ParenthesesGroup(_) => true,
            Token::OpenBrace => false,
            Token::CloseBrace => false,
            Token::Newline => false,
        }
    }
}

pub fn tokenize(chars: impl AsRef<str>) -> VecDeque<Token> {
    let mut res = VecDeque::new();
    let mut pgroups = Vec::new();
    let str = chars.as_ref();
    let mut i = 0;

    while i < str.len() {
        let tk = if let Some((len, op)) = get_operator(&str[i..]) {
            i += len;
            Token::Operator(op)
        } else if str[i..].chars().next().unwrap_or('\0') == '\n' {
            i += 1;
            Token::Newline
        } else if str[i..].chars().next().unwrap_or('\0') == '{' {
            i += 1;
            Token::OpenBrace
        } else if str[i..].chars().next().unwrap_or('\0') == '}' {
            i += 1;
            Token::CloseBrace
        } else if str[i..].chars().next().unwrap_or('\0') == '(' {
            i += 1;

            pgroups.push(VecDeque::new());

            continue;
        } else if str[i..].chars().next().unwrap_or('\0') == ')' {
            i += 1;

            if let Some(grp) = pgroups.pop() {
                Token::ParenthesesGroup(grp)
            } else {
                continue;
            }
        } else if let Some((len, n)) = get_keyword(&str[i..]) {
            i += len;
            Token::Keyword(n)
        } else if let Some((len, n)) = get_number(&str[i..]) {
            i += len;
            Token::Num(n)
        } else if let Some((len, s)) = get_identifier(&str[i..]) {
            i += len;
            Token::Ident(s)
        } else {
            //probably white space, just move forwards like nothing happened
            i += 1;
            continue;
        };

        if let Some(v) = pgroups.last_mut() {
            v.push_back(tk);
        } else {
            res.push_back(tk);
        }
    }

    res
}

pub trait TokenHelpers {
    fn next_is_expr(&self) -> bool;

}

impl TokenHelpers for VecDeque<Token> {
    fn next_is_expr(&self) -> bool {

        if let Some(b) = self.front() {
            b.is_expression()
        } else {
            false
        }
    }


}

#[macro_export]
macro_rules! match_tokens {
    ($tokens:expr, $($pat:pat),* $(,)?) => {
        { match_tokens!(@inner $tokens, 0usize, $($pat),*) }
    };

    // recursive case
    (@inner $tokens:expr, $idx:expr, $head:pat, $($tail:pat),*) => {
        if let Some($head) = $tokens.get($idx) {
            match_tokens!(@inner $tokens, $idx + 1usize, $($tail),*)
        } else {
            false
        }
    };

    // base case
    (@inner $tokens:expr, $idx:expr, $last:pat) => {
        if let Some($last) = $tokens.get($idx) {
            true
        } else {
            false
        }
    };
}