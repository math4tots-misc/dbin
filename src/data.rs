use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    Int(i64),
    Float(f64),
    Bytes(Rc<Vec<u8>>),
    String(Rc<str>),
    Seq(Rc<Vec<Data>>),
}

impl Data {
    pub fn seq(vec: Vec<Data>) -> Data {
        Data::Seq(vec.into())
    }
    pub fn i64(&self) -> Option<i64> {
        if let Data::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    pub fn f64(&self) -> Option<f64> {
        if let Data::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    pub fn bytes(&self) -> Option<&Vec<u8>> {
        if let Data::Bytes(bytes) = self {
            Some(bytes)
        } else {
            None
        }
    }
    pub fn str(&self) -> Option<&Rc<str>> {
        if let Data::String(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn list(&self) -> Option<&Rc<Vec<Data>>> {
        if let Data::Seq(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn u8(&self) -> Option<u8> {
        self.i64().map(|i| i as u64 as u8)
    }
    pub fn u16(&self) -> Option<u16> {
        self.i64().map(|i| i as u64 as u16)
    }
    pub fn u32(&self) -> Option<u32> {
        self.i64().map(|i| i as u64 as u32)
    }
    pub fn u64(&self) -> Option<u64> {
        self.i64().map(|i| i as u64 as u64)
    }
    pub fn i8(&self) -> Option<i8> {
        self.i64().map(|i| i as i8)
    }
    pub fn i16(&self) -> Option<i16> {
        self.i64().map(|i| i as i16)
    }
    pub fn i32(&self) -> Option<i32> {
        self.i64().map(|i| i as i32)
    }
    pub fn f32(&self) -> Option<f32> {
        self.f64().map(|i| i as f32)
    }
}

impl From<i64> for Data {
    fn from(x: i64) -> Data {
        Data::Int(x)
    }
}

impl From<f64> for Data {
    fn from(x: f64) -> Data {
        Data::Float(x)
    }
}

impl From<Rc<Vec<u8>>> for Data {
    fn from(x: Rc<Vec<u8>>) -> Data {
        Data::Bytes(x)
    }
}

impl From<&Rc<Vec<u8>>> for Data {
    fn from(x: &Rc<Vec<u8>>) -> Data {
        Data::Bytes(x.clone())
    }
}

impl From<&[u8]> for Data {
    fn from(x: &[u8]) -> Data {
        Data::Bytes(x.to_vec().into())
    }
}

impl From<Rc<str>> for Data {
    fn from(x: Rc<str>) -> Data {
        Data::String(x)
    }
}

impl From<&Rc<str>> for Data {
    fn from(x: &Rc<str>) -> Data {
        Data::String(x.clone())
    }
}

impl From<String> for Data {
    fn from(x: String) -> Data {
        Data::String(x.into())
    }
}

impl From<&str> for Data {
    fn from(x: &str) -> Data {
        Data::String(x.to_owned().into())
    }
}

impl From<Rc<Vec<Data>>> for Data {
    fn from(x: Rc<Vec<Data>>) -> Data {
        Data::Seq(x)
    }
}

impl From<&Rc<Vec<Data>>> for Data {
    fn from(x: &Rc<Vec<Data>>) -> Data {
        Data::Seq(x.clone())
    }
}

impl From<Vec<Data>> for Data {
    fn from(x: Vec<Data>) -> Data {
        Data::Seq(x.into())
    }
}

impl From<&[Data]> for Data {
    fn from(x: &[Data]) -> Data {
        Data::Seq(x.to_vec().into())
    }
}
