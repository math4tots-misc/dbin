use std::collections::HashMap;

pub struct ReadContext<'a> {
    /// Simple key value store to aid in the process of parsing
    map_stack: Vec<HashMap<ContextValue, ContextValue>>,

    /// index into bytes
    pos: usize,

    /// actual bytes that are to be read
    /// we can't just use io::Read here because we may sometimes
    /// need to rewind
    bytes: &'a [u8],
}

impl<'a> ReadContext<'a> {
    pub fn peek(&self, n: usize) -> Result<&'a [u8], String> {
        match self.bytes.get(self.pos..self.pos + n) {
            Some(s) => Ok(s),
            None => Err("Tried to peek beyond end".to_owned()),
        }
    }
    pub fn read(&mut self, n: usize) -> Result<&'a [u8], String> {
        self.pos += n;
        match self.bytes.get(self.pos - n..self.pos) {
            Some(s) => Ok(s),
            None => Err("Tried to read beyond end".to_owned()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextValue {
    Int(i64),
    String(String),
}

impl From<i64> for ContextValue {
    fn from(i: i64) -> ContextValue {
        ContextValue::Int(i)
    }
}

impl From<String> for ContextValue {
    fn from(s: String) -> ContextValue {
        ContextValue::String(s)
    }
}

impl From<&str> for ContextValue {
    fn from(s: &str) -> ContextValue {
        ContextValue::String(s.to_owned())
    }
}
