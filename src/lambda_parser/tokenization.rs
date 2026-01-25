use lazy_static::lazy_static;
use rust_decimal::Decimal;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
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



lazy_static!(
    pub static ref operators: Vec<Operator> = {
        let mut res = vec![
            Operator::new("+", BindingPower::BinaryOrUnary(1.0, 1.1)),
            Operator::new("-", BindingPower::BinaryOrUnary(1.0, 1.1)),
            Operator::new("*", BindingPower::Binary(2.0, 2.1)),
            Operator::new("/", BindingPower::Binary(2.0, 2.1)),
            //Operator::new("**", BindingPower::Binary(3.0, 3.1)),

            Operator::new("<", BindingPower::Binary(1.0, 1.1)),
            Operator::new(">", BindingPower::Binary(1.0, 1.1)),
            Operator::new("<=", BindingPower::Binary(1.0, 1.1)),
            Operator::new(">=", BindingPower::Binary(1.0, 1.1)),
            Operator::new("!=", BindingPower::Binary(1.0, 1.1)),
            Operator::new("==", BindingPower::Binary(1.0, 1.1)),

            Operator::new(",", BindingPower::Seperator),
            Operator::new("=", BindingPower::Binary(-10.0, -9.9)),
        ];
        res.sort_by(|f, s|  if f.token.len() > s.token.len() { Ordering::Less } else { Ordering::Greater } );
        res
    };
);

pub fn get_operator(st: &str) -> Option<(usize, Operator)> {
    for op in (&operators).iter() {
        if st.starts_with(op.token) {
            return Some((op.token.len(), *op))
        }
    }
    None
}

pub fn get_number(st: &str) -> Option<(usize, Decimal)> {


    //consume all numbers, and 1 optional decimal
    let mut found_dec = false;
    let mut prcnt = false;

    let mut res: String = String::new();

    for c in st.chars() {
        if c.is_numeric() {
            res.push(c);
        } else if c == '.' && !found_dec {
            found_dec = true;
            res.push('.');
        } else {
            if c == '%' {
                prcnt = true;
            }
            break;
        }
    }


    if res.len() > 0 {
        let val = Decimal::from_str(&res).unwrap();
        Some((res.len(), if prcnt { val / Decimal::new(100, 1) } else { val }))
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
pub enum Token {
    Ident(String),
    Num(Decimal),
    Operator(Operator),
    ParenthesesGroup(VecDeque<Token>)
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
        }
    }
}

impl Token {
    //todo
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