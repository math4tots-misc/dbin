use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Int(i64),
    String(Rc<str>),
}

impl From<i64> for Key {
    fn from(i: i64) -> Key {
        Key::Int(i)
    }
}

impl From<Rc<str>> for Key {
    fn from(s: Rc<str>) -> Key {
        Key::String(s)
    }
}

impl From<&Rc<str>> for Key {
    fn from(s: &Rc<str>) -> Key {
        Key::String(s.clone())
    }
}

impl From<String> for Key {
    fn from(s: String) -> Key {
        Key::String(s.into())
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Key {
        Key::String(s.to_owned().into())
    }
}
