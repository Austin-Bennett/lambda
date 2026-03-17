use std::path::PathBuf;
use crate::common::source_owner::SourceOwner;

#[derive(Debug, Clone)]
pub struct SourceMap {
    pub owner: SourceOwner,
    pub offset: usize,
    pub line: usize,
    pub char: usize,
    pub len: usize,
}


impl SourceMap {

    pub fn end(&self) -> usize {
        self.offset + self.len
    }

    pub fn extend(&mut self, other: SourceMap) {
        if self.end() < other.end() {
            self.len = other.end() - self.offset;
        } else if self.offset > other.offset {
            self.offset = other.offset;
        }
    }
}
