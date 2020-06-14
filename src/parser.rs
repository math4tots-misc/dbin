use crate::err;
use crate::Context;
use crate::Data;
use crate::Expr;
use crate::Scope;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum ParseError {
    Other(String),
}

pub enum Pattern {
    Exact(Vec<u8>), // expect an exact sequence of bytes

    // integral types
    U8,
    I8,
    LeU16,
    LeU32,
    LeU64,
    BeU16,
    BeU32,
    BeU64,
    LeI16,
    LeI32,
    LeI64,
    BeI16,
    BeI32,
    BeI64,

    // float types
    LeF32,
    LeF64,
    BeF32,
    BeF64,

    // null terminated string
    CStr,

    // Array, with variable length
    Array(Box<Pattern>, Expr),

    AnyOf(Vec<Pattern>),
    AllOf(Vec<Pattern>), // results in Seq of patterns

    // pseudo patterns
    // these change the parsed results and parse state,
    // but do not directly modify what sequence of bytes
    // they match
    Store(Box<Pattern>, i64), // stores the resulting Data into the current scope
    Map(
        Box<Pattern>,
        Box<dyn Fn(&Scope, Data) -> Result<Data, ParseError>>,
    ),
}

impl Pattern {
    pub fn map<F: Fn(&Scope, Data) -> Result<Data, ParseError> + 'static>(self, f: F) -> Pattern {
        Pattern::Map(Box::new(self), Box::new(f))
    }
    pub fn mapval<D: Into<Data>>(self, d: D) -> Pattern {
        let d = d.into();
        self.map(move |_, _| Ok(d.clone()))
    }
    pub fn store<K: Into<i64>>(self, key: K) -> Pattern {
        Pattern::Store(self.into(), key.into())
    }
    pub fn parse(&self, bytes: &[u8]) -> Result<Data, ParseError> {
        let mut ctx = Context::new(bytes);
        self.parse_ctx(&mut ctx)
    }
    fn parse_ctx(&self, ctx: &mut Context) -> Result<Data, ParseError> {
        match self {
            Pattern::Exact(bytes) => {
                let peek = ctx.peek(bytes.len())?;
                if bytes.as_slice() == peek {
                    Ok(ctx.read(bytes.len())?.into())
                } else {
                    Err(ParseError::Other(format!(
                        "Expected {:?} but got {:?}",
                        bytes, peek
                    )))
                }
            }
            Pattern::U8 => Ok((uint(true, ctx.read(1)?) as i64).into()),
            Pattern::I8 => Ok((sint(true, ctx.read(1)?) as i64).into()),
            Pattern::LeU16 => Ok((uint(true, ctx.read(2)?) as i64).into()),
            Pattern::LeU32 => Ok((uint(true, ctx.read(4)?) as i64).into()),
            Pattern::LeU64 => Ok((uint(true, ctx.read(8)?) as i64).into()),
            Pattern::BeU16 => Ok((uint(false, ctx.read(2)?) as i64).into()),
            Pattern::BeU32 => Ok((uint(false, ctx.read(4)?) as i64).into()),
            Pattern::BeU64 => Ok((uint(false, ctx.read(8)?) as i64).into()),
            Pattern::LeI16 => Ok((sint(true, ctx.read(2)?) as i64).into()),
            Pattern::LeI32 => Ok((sint(true, ctx.read(4)?) as i64).into()),
            Pattern::LeI64 => Ok((sint(true, ctx.read(8)?) as i64).into()),
            Pattern::BeI16 => Ok((sint(false, ctx.read(2)?) as i64).into()),
            Pattern::BeI32 => Ok((sint(false, ctx.read(4)?) as i64).into()),
            Pattern::BeI64 => Ok((sint(false, ctx.read(8)?) as i64).into()),
            Pattern::LeF32 => Ok((f32::from_bits(uint(true, ctx.read(4)?) as u32) as f64).into()),
            Pattern::LeF64 => Ok((f64::from_bits(uint(true, ctx.read(8)?) as u64)).into()),
            Pattern::BeF32 => Ok((f32::from_bits(uint(false, ctx.read(4)?) as u32) as f64).into()),
            Pattern::BeF64 => Ok((f64::from_bits(uint(false, ctx.read(8)?) as u64)).into()),
            Pattern::CStr => {
                let mut bytes = Vec::new();
                while ctx.peek(1)?[0] != 0 {
                    bytes.push(ctx.read(1)?[0]);
                }
                match std::str::from_utf8(&bytes) {
                    Ok(s) => Ok(s.into()),
                    Err(error) => err(format!("{:?}", error)),
                }
            }
            Pattern::Array(pat, expr) => {
                let len = match expr.eval(ctx.scope())? {
                    Data::Int(i) => i as usize,
                    x => return err(format!("Got non-int for array len ({:?})", x)),
                };
                let mut ret = Vec::new();
                for _ in 0..len {
                    ret.push(pat.parse_ctx(ctx)?);
                }
                Ok(ret.into())
            }
            Pattern::AnyOf(pats) => {
                let pos = ctx.save();
                let mut last = err("Empty 'any-of'");
                for pat in pats {
                    last = pat.parse_ctx(ctx);
                    if last.is_ok() {
                        return last;
                    } else {
                        ctx.restore(pos);
                    }
                }
                last
            }
            Pattern::AllOf(pats) => {
                let mut ret = Vec::new();
                for pat in pats {
                    ret.push(pat.parse_ctx(ctx)?);
                }
                Ok(ret.into())
            }
            Pattern::Store(pat, key) => {
                let val = pat.parse_ctx(ctx)?;
                ctx.scope_mut().set(*key, val.clone());
                Ok(val)
            }
            Pattern::Map(pat, f) => {
                let val = pat.parse_ctx(ctx)?;
                Ok(f(ctx.scope(), val)?)
            }
        }
    }

    /// convenience method that
    /// returns a new Pattern mapped by adding the given value
    /// to the resulting value
    ///   - numeric types can be added to each other,
    ///       with two integral types, the result is an intgral value
    ///       otherwise, you get a Float value
    ///   - string types can be added to each other
    ///       to create a concatenated string
    pub fn add<D: Into<Data>>(self, rhs: D) -> Pattern {
        let rhs = rhs.into();
        self.map(move |_, lhs| {
            let rhs = rhs.clone();
            match (lhs, rhs) {
                (Data::Int(a), Data::Int(b)) => Ok((a + b).into()),
                (Data::Float(a), Data::Float(b)) => Ok((a + b).into()),
                (Data::Float(a), Data::Int(b)) => Ok((a + b as f64).into()),
                (Data::Int(a), Data::Float(b)) => Ok((a as f64 + b).into()),
                (Data::String(a), Data::String(b)) => Ok(format!("{}{}", a, b).into()),
                (a, b) => err(format!("Could not add given values ({:?}, {:?})", a, b,)),
            }
        })
    }

    /// convenience method that accepts a list of keys
    /// and returns a Pattern that when parsed will return a
    /// map of list of (name, value) pairs, where the names
    /// are generated from Debug of the keys and value comes from
    /// lookup up the Scope
    pub fn to_map<K: Into<i64> + Debug>(self, keys: Vec<K>) -> Pattern {
        let pairs: Vec<_> = keys.into_iter().map(|k| {
            let s: Data = format!("{:?}", k).into();
            let k: i64 = k.into();
            (k, s)
        }).collect();
        self.map(move |scope, _| {
            let mut ret = Vec::new();
            for (key, keystr) in pairs.clone() {
                let val: Data = scope.get_or_error(key)?.clone();
                let key: Data = keystr.into();
                let pair: Data = vec![key, val].into();
                ret.push(pair);
            }
            Ok(ret.into())
        })
    }
}

fn uint(little_endian: bool, bytes: &[u8]) -> u64 {
    let mut ret: u64 = 0;
    if little_endian {
        for byte in bytes.iter().rev() {
            ret <<= 8;
            ret += (*byte) as u64;
        }
    } else {
        for byte in bytes {
            ret <<= 8;
            ret += (*byte) as u64;
        }
    }
    ret
}

fn sint(little_endian: bool, bytes: &[u8]) -> i64 {
    let mut bytes = bytes.to_vec();
    let byte = if little_endian {
        *bytes.last_mut().unwrap() as i8
    } else {
        bytes[0] as i8
    };
    let minus = if byte < 0 {
        for byte in &mut bytes {
            *byte = !*byte;
        }
        true
    } else {
        false
    };
    let ui = uint(little_endian, &bytes);
    if minus {
        -(ui.wrapping_add(1) as i64)
    } else {
        ui as i64
    }
}
