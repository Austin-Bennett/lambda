use lazy_static::lazy_static;
use radix_trie::Trie;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
    Let,
    Fn,
    Return,
}

impl Keyword {
    pub fn len(&self) -> usize {
        match self {
            Keyword::Let => "let".len(),
            Keyword::Fn => "fn".len(),
            Keyword::Return => "return".len(),
        }
    }
}


