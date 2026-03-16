use std::fmt::{Debug, Formatter};

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum SourceDescriptor {
    File
}

impl Debug for SourceDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceDescriptor::File => f.write_str("file")
        }
    }
}

#[derive(Clone, Debug)]
#[derive(Eq, Hash, PartialEq)]
pub struct SourceOwner {
    pub descriptor: SourceDescriptor,
    pub name: String,
}

impl SourceOwner {
    pub fn new(descriptor: SourceDescriptor, name: String) -> Self {
        SourceOwner{
            descriptor,
            name
        }
    }
}