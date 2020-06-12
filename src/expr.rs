use crate::Key;
use std::rc::Rc;

pub enum Expr {
    Int(i64),
    String(Rc<str>),
    Var(Key),
    Add(Vec<Expr>),
}

impl Expr {
    pub fn var<K: Into<Key>>(key: K) -> Expr {
        Expr::Var(key.into())
    }

    pub fn add<E: Into<Expr>>(self, other: E) -> Expr {
        let other = other.into();
        match (self, other) {
            (Expr::Add(mut exprs), Expr::Add(others)) => {
                exprs.extend(others);
                Expr::Add(exprs)
            }
            (Expr::Add(mut exprs), other) => {
                exprs.push(other);
                Expr::Add(exprs)
            }
            (a, b) => Expr::Add(vec![a, b]),
        }
    }
}

impl From<i64> for Expr {
    fn from(i: i64) -> Expr {
        Expr::Int(i)
    }
}

impl From<Rc<str>> for Expr {
    fn from(s: Rc<str>) -> Expr {
        Expr::String(s)
    }
}

impl From<&Rc<str>> for Expr {
    fn from(s: &Rc<str>) -> Expr {
        Expr::String(s.clone())
    }
}

impl From<String> for Expr {
    fn from(s: String) -> Expr {
        Expr::String(s.into())
    }
}

impl From<&str> for Expr {
    fn from(s: &str) -> Expr {
        Expr::String(s.to_owned().into())
    }
}
