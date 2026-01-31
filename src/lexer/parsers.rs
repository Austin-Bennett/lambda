use crate::lexer::lang_parser::Parser;
use crate::lexer::tokenizer::Token;
use crate::lexer::tokenizer::Token::NumberLiteral;

pub fn get_whitespace_len(file: &str) -> usize {
    let mut i = 0;
    while i < file.len() && let Some(c) = file[i..].chars().next() {
        if c.is_whitespace() && c != '\n' {
            i += c.len_utf8();
        } else {
            break;
        }
    }
    i
}

pub fn get_comment_length(file: &str) -> usize {

    if file.starts_with("//") {
        file.find('\n').unwrap_or(file.len())
    } else {
        0
    }
}

//len, lines, pos_to_line
pub fn get_long_comment_length(file: &str) -> (usize, usize, usize) {

    if file.starts_with("/*") {
        let mut i = "/*".len();
        let mut lines = 0;
        let mut pos_to_line = 0;

        while i < file.len() && !file[i..].starts_with("*/") {
            let c = file[i..].chars().next().unwrap();
            if c == '\n' {
                lines += 1;
            }

            i += c.len_utf8();
            if c == '\n' {
                pos_to_line = i;
            }
        }

        if i < file.len() {
            i += "*/".len();
        }

        (i, lines, pos_to_line)

    } else {
        (0, 0, 0)
    }
}

pub struct IdentParser;
pub struct KeywordParser;
pub struct NumberParser;
pub struct StringParser;

pub fn handle_escaped_char(s: &str) -> char {
    return s.chars().next().unwrap();
}

impl Parser for StringParser {
    fn parse(&self, s: &str) -> Option<(Token, usize)> {
        let mut len = 0;
        if s.starts_with('\"') {

            len += '\"'.len_utf8();
            let res = String::new();


            while len < s.len() {
                let c = s[len..].chars().next().unwrap();
                let chr = if c == '\"' {
                    len += '\"'.len_utf8();
                    break;
                } else if c == '\\' {
                    len += '\\'.len_utf8();
                    handle_escaped_char(&s[len..])
                } else {
                    c
                };
            }

            Some((Token::StringLiteral(res), len))

        } else {
            None
        }
    }
}

impl Parser for IdentParser {
    fn parse(&self, s: &str) -> Option<(Token, usize)> {

        let mut chars = s.chars();
        if let Some(c) = chars.next() {
            if c != '_' && !c.is_alphabetic() {
                None
            } else {
                let mut res = String::new();
                res.push(c);
                while let Some(c) =chars.next() {
                    if c.is_whitespace() || (!c.is_alphanumeric() && c != '_') {
                        break;
                    }
                    res.push(c);
                }
                let l = res.len();
                Some((Token::Ident(res), l))
            }
        } else {
            None
        }

    }
}

impl Parser for NumberParser {
    fn parse(&self, s: &str) -> Option<(Token, usize)> {
        //must start with a number

        let mut chars = s.chars().peekable();
        if let Some(c) = chars.next() {
            if !c.is_numeric() {
                None
            } else {
                let mut res = String::new();
                res.push(c);
                let mut found_decimal = false;
                while let Some(c) = chars.peek() {
                    if *c == '.' && !found_decimal {
                        res.push('.');
                        found_decimal = true;
                        chars.next();
                    } else if c.is_numeric() {
                        res.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if let Some('%') = chars.next() {
                    res.push('%');
                }
                let len = res.len();
                Some((NumberLiteral(res), len))
            }
        } else {
            None
        }
    }
}

impl Parser for KeywordParser {
    fn parse(&self, s: &str) -> Option<(Token, usize)> {
        if s.starts_with("if") {
            Some((Token::Keyword("if"), "if".len()))
        } else {
            None
        }
    }
}