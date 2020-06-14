use crate::err;
use crate::Data;
use crate::ParseError;
use std::collections::HashMap;

pub struct Context<'a> {
    scope_stack: Vec<Scope>,
    pos: usize,
    bytes: &'a [u8],
}

impl<'a> Context<'a> {
    pub(crate) fn new(bytes: &[u8]) -> Context {
        Context {
            scope_stack: vec![Scope(HashMap::new())],
            pos: 0,
            bytes,
        }
    }
    pub fn scope(&self) -> &Scope {
        self.scope_stack.last().unwrap()
    }
    pub fn scope_mut(&mut self) -> &mut Scope {
        self.scope_stack.last_mut().unwrap()
    }
    pub fn peek(&self, n: usize) -> Result<&'a [u8], ParseError> {
        match self.bytes.get(self.pos..self.pos + n) {
            Some(s) => Ok(s),
            None => err("Tried to peek beyond end"),
        }
    }
    pub fn read(&mut self, n: usize) -> Result<&'a [u8], ParseError> {
        self.pos += n;
        match self.bytes.get(self.pos - n..self.pos) {
            Some(s) => Ok(s),
            None => err("Tried to read beyond end"),
        }
    }
    pub fn push_stack(&mut self) {
        let map = self.scope().0.clone();
        self.scope_stack.push(Scope(map));
    }
    pub fn pop_stack(&mut self) {
        self.scope_stack.pop().unwrap();
    }
    pub fn save(&self) -> usize {
        self.pos
    }
    pub fn restore(&mut self, pos: usize) {
        self.pos = pos;
    }
}

pub struct Scope(HashMap<i64, Data>);

impl Scope {
    pub fn get(&self, key: i64) -> Option<&Data> {
        self.0.get(&key)
    }
    pub fn set(&mut self, key: i64, value: Data) {
        self.0.insert(key, value);
    }
}
