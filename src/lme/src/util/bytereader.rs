use std::ops::Deref;

pub struct LEByteReader<'a> {
    bytes: &'a [u8],
    head: usize,
}

impl<'a> Deref for LEByteReader<'a> {
    type Target = [u8];

    fn deref(&self) -> &'a Self::Target {
        &self.bytes[self.head..]
    }
}

impl<'a> LEByteReader<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self{
            bytes: slice,
            head: 0
        }
    }

    pub fn next_byte(&mut self) -> Option<u8> {
        if self.head >= self.bytes.len() {
            None
        } else {
            let res = self.bytes[self.head];
            self.head += 1;
            Some(res)
        }
    }

    pub fn next_word(&mut self) -> Option<u16> {
        if self.head + 2 >= self.bytes.len() { return None; }

        let v = Some(u16::from_le_bytes(self.bytes[self.head..self.head+2].try_into().ok()?));

        self.head += 2;

        v
    }

    pub fn next_dword(&mut self) -> Option<u32> {
        if self.head + 4 >= self.bytes.len() { return None; }

        let v = Some(u32::from_le_bytes(self.bytes[self.head..self.head+4].try_into().ok()?));

        self.head += 4;

        v
    }

    pub fn next_qword(&mut self) -> Option<u64> {
        if self.head + 8 >= self.bytes.len() { return None; }

        let v = Some(u64::from_le_bytes(self.bytes[self.head..self.head+8].try_into().ok()?));

        self.head += 8;

        v
    }

    pub fn next_str(&mut self, len: usize) -> Option<&'a str> {
        if self.head + len >= self.bytes.len() {
            return None;
        }

        let s = str::from_utf8(&self.bytes[self.head..self.head+len]).ok();

        self.head += len;

        s
    }


    pub fn peek_byte(&mut self) -> Option<u8> {
        if self.head >= self.bytes.len() {
            None
        } else {
            Some(self.bytes[self.head])
        }
    }

    pub fn peek_word(&mut self) -> Option<u16> {
        if self.head + 2 >= self.bytes.len() { return None; }


        Some(u16::from_le_bytes(self.bytes[self.head..self.head+2].try_into().ok()?))
    }

    pub fn peek_dword(&mut self) -> Option<u32> {
        if self.head + 4 >= self.bytes.len() { return None; }


        Some(u32::from_le_bytes(self.bytes[self.head..self.head+4].try_into().ok()?))
    }

    pub fn peek_qword(&mut self) -> Option<u64> {
        if self.head + 8 >= self.bytes.len() { return None; }

        Some(u64::from_le_bytes(self.bytes[self.head..self.head+8].try_into().ok()?))
    }


}