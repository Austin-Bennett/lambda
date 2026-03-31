use std::io;
use std::io::{BufWriter, Write};

pub trait LMEByteEncoder {
    fn encode<T: Write>(&self, writer: &mut BufWriter<T>) -> io::Result<()>;
}