use crate::lexer::tokenizer::Token;

pub struct Lexer {
    data: Vec<Token> //treat the back of the vec as the front
}

impl Lexer {
    pub fn new() -> Self {
        Self{ data: Vec::new() }
    }

    pub fn push_front(&mut self, tk: Token) {
        self.data.push(tk);
    }

    pub fn pop_front(&mut self, tk: Token) -> Token {
        if self.data.is_empty() {
            Token::EOF
        } else {
            self.data.pop().unwrap()
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn data(&self) -> &Vec<Token> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &Vec<Token> {
        &mut self.data
    }
}

impl FromIterator<Token> for Lexer {
    fn from_iter<T: IntoIterator<Item=Token>>(iter: T) -> Self {
        let mut vec = Vec::new();

        for i in iter.into_iter() {
            vec.push(i);
        }
        vec.reverse();

        Lexer{ data: vec }
    }
}