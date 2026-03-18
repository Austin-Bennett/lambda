use std::fmt::{Debug, Formatter};
use crate::common::source_owner::SourceOwner;

#[derive(Clone)]
pub struct SourceMap {
    pub owner: SourceOwner,
    pub offset: usize,
    pub line: usize,
    pub char: usize,
    pub len: usize,
}

impl Debug for SourceMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} offset: {}, ", self.owner, self.offset)?;
        write!(f, "line: {:?}, char: {:?}, len: {:?}", self.line, self.char, self.len)
    }
}

impl AsRef<SourceMap> for SourceMap {
    fn as_ref(&self) -> &SourceMap {
        self
    }
}

impl SourceMap {

    pub fn end(&self) -> usize {
        self.offset + self.len
    }

    pub fn extend(&mut self, other: impl AsRef<SourceMap>) {
        let other = other.as_ref();
        if self.end() < other.end() {
            self.len = other.end() - self.offset;
        } else if self.offset > other.offset {
            self.offset = other.offset;
        }
    }
    
}
