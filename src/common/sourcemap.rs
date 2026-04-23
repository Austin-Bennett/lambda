use std::fmt::{Debug, Formatter};
use crate::common::source_owner::{SourceDescriptor, SourceOwner};

#[derive(Clone, Hash)]
pub struct SourceMap {
    pub owner: SourceOwner,
    pub offset: usize,
    pub line: usize,
    pub char: usize,
    pub len: usize,
}

impl Default for SourceMap {
    fn default() -> Self {
        Self{
            owner: SourceOwner::new(SourceDescriptor::RustString, "default".to_string()),
            offset: 0,
            line: 0,
            char: 0,
            len: 0
        }
    }
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
    
    pub fn end_source(&self) -> SourceMap {
        SourceMap{
            owner: self.owner.clone(),
            offset: self.end(),
            //the line may be wrong, that's just how it is
            //this is only really used for niche things that wont show as errors anywhere
            line: self.line,
            char: self.char + self.offset,
            len: 0
        }
    }

    pub fn extend(&mut self, other: impl AsRef<SourceMap>) {
        let other = other.as_ref();
        if other.len == 0 { return; }
        if self.len == 0 {
            self.offset = other.offset;
            self.len = other.len;
            return;
        }
        let new_start = self.offset.min(other.offset);
        let new_end = self.end().max(other.end());
        self.offset = new_start;
        self.len = new_end - new_start;
    }
    
}
