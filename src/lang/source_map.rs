use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use crate::utils::registry::RegistryID;



#[derive(Clone)]
pub struct Source {
    label: Option<String>,
    source_str: String,
}

impl Source {

    pub fn new(label: String, contents: String) -> Self {
        Self{
            label: Some(label),
            source_str: contents
        }
    }

    pub fn get_source(&self) -> &str {
        &self.source_str
    }

    pub fn get_label(&self) -> Option<&str> {
        self.label.as_ref().map(|l| l.as_str())
    }
}

impl Debug for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Source {:?}:\n{}", self.label.as_ref().map(|v| v.as_str()).unwrap_or("<unknown>"), self.source_str)
    }
}

impl<T: Into<String>> From<T> for Source {
    fn from(value: T) -> Self {
        Self{
            label: None,
            source_str: value.into()
        }
    }
}

#[derive(Copy, Clone)]
pub struct SourceMap {
    pub source: RegistryID,
    pub line: u32,
    pub char: u32,
    pub offset: u32,
    pub len: u32,
}

impl Debug for SourceMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, char {}, len {}", self.line, self.char, self.len)
    }
}

impl SourceMap {
    pub fn extend(&mut self, other: &SourceMap) -> bool {
        //only extend if other.offset > self.offset and other.offset + other.len > self.offset+self.len and they share the same ID

        if self.source == other.source && other.offset >= self.offset && other.offset + other.len > self.offset + self.len {
            self.len = (other.offset + other.len) - self.offset;

            true
        } else {
            false
        }
    }
}