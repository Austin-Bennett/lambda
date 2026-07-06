use lazy_static::lazy_static;
use radix_trie::{Trie, TrieCommon};
use crate::lang::keyword::Keyword;
use crate::lang::literal::IntLiteral;
use crate::lang::operator::{BindingPower};
use crate::lexer::{Token, TokenParser};
use crate::utils::iter_tools::ToLambdaIterator;

lazy_static!{
    pub static ref OPERATORS: Trie<&'static str, BindingPower> = {
        let mut res = Trie::new();

        res.insert("+", BindingPower::binary(BindingPower::ADDITIVE_BP));
        res.insert("-", BindingPower::binary_or_unary(BindingPower::ADDITIVE_BP));
        res.insert("*", BindingPower::binary(BindingPower::MULTIPLICATIVE_BP));
        res.insert("/", BindingPower::binary(BindingPower::MULTIPLICATIVE_BP));

        res
    };

    pub static ref KEYWORDS: Trie<&'static str, Keyword> = {
        let mut res = Trie::new();

        res.insert("let", Keyword::Let);
        res.insert("fn", Keyword::Fn);
        res.insert("return", Keyword::Return);

        res
    };
}

pub struct OperatorParser;
impl TokenParser for OperatorParser {
    fn parse(&self, str: &str) -> Option<(Token, usize)> {
        if let Some(op) = OPERATORS.get_ancestor(str) {
            if let Some(key ) = op.key() && let Some(val) = op.value() {
                Some((Token::Op(key, *val), key.len()))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct KeywordParser;
impl TokenParser for KeywordParser {
    fn parse(&self, str: &str) -> Option<(Token, usize)> {
        let op = KEYWORDS.get_ancestor_value(str)?;
        let len = op.len();

        // get_ancestor_value matches by prefix, so "let" would also match the
        // start of "letter". Only accept the match if it isn't immediately
        // followed by another identifier character.
        if str[len..].starts_with(|c: char| IdentifierParser::is_identifier_char(c)) {
            return None;
        }

        Some((Token::Keyword(*op), len))
    }
}

pub struct IntLiteralParser;

impl TokenParser for IntLiteralParser {
    fn parse(&self, str: &str) -> Option<(Token, usize)> {
        let mut chars = str.chars().lmb_iter();

        let mut len = 0;
        let negative = if let Some(_) = chars.next_if_eq(&'-') {
            len += 1;
            true
        } else {
            false
        };

        let mut val = 0u64;

        let mut error = false;

        let Some(_) = chars
            .peek_if(|c| c.is_ascii_digit()) else {
            return None;
        };

        while let Some(digit) = chars
            .next_if(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as u64) {


            if !error {
                val = match val
                    .checked_mul(10)
                    .and_then(|v| v.checked_add(digit)) {
                    Some(v) => v,
                    None => {
                        error = true;
                        val
                    }
                }
            }
            len += 1;
        }

        if error {
            Some((Token::Error(Some("Literal is too large to fit in any numeric type!".into())), len))
        } else {
            Some((Token::IntegerLiteral(IntLiteral{ val, negative }), len))
        }
    }
}

pub struct FloatLiteralParser;

impl TokenParser for FloatLiteralParser {
    fn parse(&self, str: &str) -> Option<(Token, usize)> {
        let mut len = 0;

        let mut integer = 0.0;
        let mut frac = 0.0;
        let mut frac_len = 0;

        let mut chars = str.chars().lmb_iter();

        let negative = if let Some(_) = chars.next_if_eq(&'-') {
            len += 1;
            true
        } else {
            false
        };

        let mut found_decimal = false;;

        let Some(_) = chars.peek_if(|c| c.is_ascii_digit()) else {
            return None;
        };


        while let Some(digit) = chars.next_if(|c| c.is_ascii_digit() || *c == '.') {
            if digit == '.' && !found_decimal {
                found_decimal = true;
            } else if digit == '.' {
                return None;
            } else {
                let digit = digit.to_digit(10).unwrap() as f64;

                if !found_decimal {
                    integer = integer * 10.0 + digit;
                } else {
                    frac = frac * 10.0 + digit;
                    frac_len += 1;
                }
            }
            len += 1;
        }

        if !found_decimal {
            None
        } else {
            let val = integer + frac / 10f64.powi(frac_len);

            Some((Token::FloatLiteral(if negative { -val } else { val }), len))
        }
    }
}

pub struct IdentifierParser;

impl IdentifierParser {
    pub fn is_identifier_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    pub fn is_starting_identifier_char(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }
}

impl TokenParser for IdentifierParser {
    fn parse(&self, str: &str) -> Option<(Token, usize)> {
        let mut chars = str.chars().lmb_iter();

        let Some(_) = chars.peek_if(|c| Self::is_starting_identifier_char(*c)) else { return None; };

        let mut ident = String::new();

        while let Some(c) = chars.next_if(|c| Self::is_identifier_char(*c)) {
            ident.push(c);
        }
        let len = ident.len();
        Some((Token::Identifier(ident), len))
    }
}

pub struct SingleCharParser<const C: char>(pub Token);

impl<const C: char> TokenParser for SingleCharParser<C> {
    fn parse(&self, str: &str) -> Option<(Token, usize)> {
        if str.starts_with(C) {
            Some((self.0.clone(), C.len_utf8()))
        } else {
            None
        }
    }
}