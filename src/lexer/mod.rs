pub mod parsers;

use std::arch::x86_64::_tile_release;
use std::str::pattern::Pattern;
use crate::lang::keyword::Keyword;
use crate::lang::literal::IntLiteral;
use crate::lang::operator::BindingPower;
use crate::lang::source_map::SourceMap;
use crate::lexer::parsers::{FloatLiteralParser, IdentifierParser, IntLiteralParser, KeywordParser, OperatorParser};
use crate::utils::iter_tools::ToLambdaIterator;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Error(Option<String>),
    Identifier(String),
    Op(&'static str, BindingPower),
    IntegerLiteral(IntLiteral),
    FloatLiteral(f64),
    Keyword(Keyword),
}

pub struct SourceToken(Token, SourceMap);

pub trait TokenParser {
    fn parse(&self, str: &str) -> Option<(Token, usize)>;
}


pub struct Tokens {
    source: String,
    offset: u32,
    line: u32,
    char: u32,
}

impl Tokens {
    pub const PARSERS: &[&dyn TokenParser] = &[
        &KeywordParser,
        &OperatorParser,
        &IdentifierParser,
        &FloatLiteralParser,
        &IntLiteralParser,
    ];


    pub fn new(source: String) -> Self {
        Self{
            source,
            offset: 0,
            line: 0,
            char: 0,
        }
    }

    fn increment_by_char(offset: &mut u32, char: &mut u32, line: &mut u32, c: char) {
        *offset += c.len_utf8() as u32;
        *char += 1;
        if c == '\n' {
            *line += 1;
            *char = 0;
        }
    }

    fn skip_whitespace(&mut self) {
        let mut chars = self.source[self.offset as usize..].chars().lmb_iter();

        while let Some(c) = chars.next_if(|v| v.is_whitespace()) {
            Self::increment_by_char(&mut self.offset, &mut self.char, &mut self.line, c);
        }
    }

    fn skip_comment(&mut self) {
        let slice = &self.source[self.offset as usize..];

        if slice.starts_with("//") {
            //skip to the next newline
            let len = slice.find('\n');

            if let Some(len) = len {
                self.offset += len as u32 + '\n'.len_utf8() as u32;
                self.line += 1;
                self.char = 0;
            } else {
                self.offset += slice.len() as u32;
                self.char += slice.chars().count() as u32;
            }
        } else if slice.starts_with("/*") {
            //skip until the /*
        }
    }

    fn skip_to(&mut self, next: &str) -> bool {
        let mut slice = &self.source[self.offset as usize..];
        loop {
            let nl = slice.find('\n');
            let end = slice.find(next);

            match (nl, end) {
                (Some(nl), Some(end)) => {
                    if nl < end {
                        self.offset += (nl + '\n'.len_utf8()) as u32

                    } else {
                        self.offset += end as u32;
                        self.char += slice[0..end].chars().count() as u32;

                        break true;
                    }
                },
                (None, Some(end)) => {
                    self.offset += end as u32;
                    self.char += slice[0..end].chars().count() as u32;

                    break true;
                },
                (Some(_), None) | (None, None) => {
                    break false;
                }
            }
        }

    }
}

impl Iterator for Tokens {
    type Item = SourceToken;

    fn next(&mut self) -> Option<Self::Item> {



        todo!()
    }
}