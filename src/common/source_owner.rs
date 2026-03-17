use std::fmt::{Debug, Formatter};

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum SourceDescriptor {
    RustString,
    File,
}

impl Debug for SourceDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceDescriptor::File => f.write_str("file"),
            SourceDescriptor::RustString => f.write_str("rust string"),
        }
    }
}

#[derive(Clone)]
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

impl Debug for SourceOwner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.descriptor, self.name)
    }
}