use crate::Key;
use std::rc::Rc;

/// The result of a parse
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Data {
    Int(i64),
    String(Rc<str>),
    Array(Vec<Data>),
    Enum(Key, Box<Data>),
    Struct(Vec<Data>),
}
