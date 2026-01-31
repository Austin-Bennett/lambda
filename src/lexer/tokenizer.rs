use std::collections::VecDeque;
use rust_decimal::Decimal;
use crate::lexer::lang_parser::Parser;
use crate::lexer::operators::{OpParser, Operator};
use crate::lexer::parsers::{get_comment_length, get_long_comment_length, get_whitespace_len, IdentParser, NumberParser};

#[derive(Debug)]
pub enum Token {
    EOF, //end of file, used as a placeholder
    Newline,
    Op(Operator),
    Ident(String),
    Keyword(&'static str),
    NumberLiteral(String),
    StringLiteral(String),
}

pub struct TokenFileInfo {
    char_pos: usize,
    line: usize,
}

const PARSERS: &[&dyn Parser] = &[
    &OpParser,
    &NumberParser,
    &IdentParser,
];

pub fn tokenize(file: &str) -> Vec<(Token, TokenFileInfo)> {
    let mut pos = 0;
    let mut line = 0;
    let mut chars_to_line = 0;
    let mut res = Vec::new();

    'outer: while pos < file.len() {
        if file[pos..].starts_with('\n') {
            res.push((Token::Newline, TokenFileInfo{ char_pos: pos - chars_to_line, line }));
            line += 1;
            pos += '\n'.len_utf8();
            chars_to_line = pos;
        } else if let i = get_whitespace_len(&file[pos..]) && i > 0 {
            pos += i;
        } else if let i = get_comment_length(&file[pos..]) && i > 0 {
            pos += i;
        } else if let (i, lines, pos_to_line) = get_long_comment_length(&file[pos..]) && i > 0 {
            if lines > 0 {
                res.push((Token::Newline, TokenFileInfo{ char_pos: pos - chars_to_line, line }));
            }

            chars_to_line = pos + pos_to_line;

            pos += i;
            line += lines;
        } else {
            //try to parse
            for p in PARSERS {
                if let Some((tk, len)) = p.parse(&file[pos..]) {
                    res.push((
                        tk,
                        TokenFileInfo{ char_pos: pos - chars_to_line, line }
                    ));
                    pos += len;
                    continue 'outer;
                }
            }
        }
    }

    res
}