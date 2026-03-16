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

