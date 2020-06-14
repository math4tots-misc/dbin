use crate::Data;
use crate::ParseError;
use crate::Scope;

pub struct Expr(Box<dyn Fn(&Scope) -> Result<Data, ParseError>>);

impl Expr {
    pub fn new<F: Fn(&Scope) -> Result<Data, ParseError> + 'static>(f: F) -> Expr {
        Expr(Box::new(f))
    }
    pub fn eval(&self, scope: &Scope) -> Result<Data, ParseError> {
        (self.0)(scope)
    }
}

impl<T: Into<Data>> From<T> for Expr {
    fn from(t: T) -> Expr {
        let t = t.into();
        Expr(Box::new(move |_| Ok(t.clone())))
    }
}
