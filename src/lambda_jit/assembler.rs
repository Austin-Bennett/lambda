use std::cmp::PartialEq;
use std::collections::VecDeque;
use crate::lambda_jit::lambda_il::{Bytecode, BytecodeBuilder};

#[derive(PartialEq)]
pub enum TGroupType {
    Bracket,
    Parentheses
}

pub enum Token {
    Atom(String),
    Group(TGroupType, VecDeque<Token>),
    COMMA,
    PLUS,
    MINUS,
    NEWLINE,
}

pub fn group(code: impl AsRef<str>) -> VecDeque<String> {
    let mut res = VecDeque::new();
    let mut last = 0;
    let mut split_last = false;
    let r = code.as_ref();

    for (i, c) in r.chars().enumerate() {

        if c.is_whitespace() && !split_last {
            res.push_back(r[last..i].to_string());
            last = i+c.len_utf8();
            split_last = true;
            continue;
        }
        if c == '[' || c == ']' || c == '(' ||
            c == ')' || c == ',' || c == '+' || c == '-' || c == '\n' {
            res.push_back(r[last..i].to_string());
            res.push_back(c.to_string());
            split_last = true;

            last = i+c.len_utf8();
            continue;
        }
        split_last = false;
    }

    if last < r.len() {
        res.push_back(r[last..].to_string());
    }

    res
}


pub fn tokenize(code: impl AsRef<str>) -> Result<VecDeque<Token>, String> {
    let raw = group(code);
    let mut res = VecDeque::new();
    let mut group_stack = Vec::new();

    for g in raw {
        let tk = match g.as_str() {
            "[" => {
                group_stack.push((TGroupType::Bracket, VecDeque::new()));
                continue;
            }
            "(" => {
                group_stack.push((TGroupType::Parentheses, VecDeque::new()));
                continue;
            }
            "]" | ")" => {
                let grp = match group_stack.pop() {
                    None => return Err(format!("Unmatched \"{}\"", g)),
                    Some(g) => g
                };
                if g == "]" && grp.0 != TGroupType::Bracket ||
                    g == ")" && grp.0 != TGroupType::Parentheses {
                    return Err(format!("Expected matching group operator for {}", g));
                }
                Token::Group(grp.0, grp.1)
            }
            "," => Token::COMMA,
            "+" => Token::PLUS,
            "-" => Token::MINUS,
            "\n" => Token::NEWLINE,
            s => Token::Atom(s.to_string())
        };

        if !group_stack.is_empty() {
            group_stack.last_mut().unwrap().1.push_back(tk);
        } else {
            res.push_back(tk);
        }
    }

    Ok(res)
}

pub fn assemble(code: impl AsRef<str>) -> Result<Bytecode, String> {
    let tokens = tokenize(code)?;
    let mut builder = BytecodeBuilder::new();



    Ok(builder.build()?)
}