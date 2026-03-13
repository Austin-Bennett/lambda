use std::path::PathBuf;

#[derive(Debug)]
pub struct SourceMap {
    pub owner: String, //either a file, or explicitly specified
    pub offset: usize, //char
    pub len: usize,
}

