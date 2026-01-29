use crate::lexer::lang_parser::Parser;
use crate::lexer::tokenizer::Token;

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
    let mut i = 0;
    if file.starts_with("/*") {
        i += "/*".len();
        while i < file.len() {
            if file[i..].starts_with("*/") {
                i += "*/".len();
                break;
            } else {
                i += file[i..].chars().next().unwrap().len_utf8();
            }
        }
    } else if file.starts_with("//") {
        i += "//".len();
        while i < file.len() && let Some(c) = file[i..].chars().next() {
            if c == '\n' {
                break;
            } else {
                i += c.len_utf8();
            }
        }
    }

    i
}

pub struct IdentParser;
