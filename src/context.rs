use crate::Key;
use std::collections::HashMap;

pub struct Context<'a> {
    /// Simple key value store to aid in the process of parsing
    map_stack: Vec<HashMap<Key, Key>>,

    /// index into bytes
    pos: usize,

    /// actual bytes that are to be read
    /// we can't just use io::Read here because we may sometimes
    /// need to rewind
    bytes: &'a [u8],
}

impl<'a> Context<'a> {
    pub fn new(bytes: &'a [u8]) -> Context<'a> {
        Context {
            map_stack: vec![HashMap::new()],
            pos: 0,
            bytes,
        }
    }
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
    pub fn push_stack(&mut self, map: HashMap<Key, Key>) {
        self.map_stack.push(map);
    }
    pub fn pop_stack(&mut self) {
        self.map_stack.pop().unwrap();
    }
    pub fn getvar<K: Into<Key>>(&self, k: K) -> Option<&Key> {
        self.map_stack.last().unwrap().get(&k.into())
    }
    pub fn store(&self) -> usize {
        self.pos
    }
    pub fn restore(&mut self, pos: usize) {
        self.pos = pos;
    }
    pub fn setvar<K: Into<Key>, V: Into<Key>>(&mut self, k: K, v: V) {
        self.map_stack
            .last_mut()
            .unwrap()
            .insert(k.into(), v.into());
    }
}
