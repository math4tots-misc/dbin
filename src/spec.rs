use crate::Expr;
use crate::Key;

/// Dead simple, quick and dirty binary data parser combinator
pub enum Spec {
    Int {
        little_endian: bool,
        signed: bool,
        nbytes: usize,

        /// Expected value for the parse to succeed
        /// (e.g. for magic)
        expected: Option<i64>,
    },
    Array {
        size: Expr,
        member: Box<Spec>,
    },
    Enum(Vec<(Key, Spec)>),
    Struct(Vec<Spec>),
    Scope {
        args: Vec<(Key, Expr)>,
        body: Box<Spec>,
    },
    Store(Key, Box<Spec>),
}

impl Spec {
    pub fn args() -> SpecArgs {
        SpecArgs { args: vec![] }
    }
    pub fn pairs() -> SpecPairs {
        SpecPairs { pairs: vec![] }
    }
    pub fn le_magic_u64(value: u64) -> Spec {
        Self::magic_u64(true, value)
    }
    pub fn be_magic_u64(value: u64) -> Spec {
        Self::magic_u64(false, value)
    }
    pub fn magic_u64(little_endian: bool, value: u64) -> Spec {
        Self::magic(little_endian, 8, value as i64)
    }
    pub fn magic(little_endian: bool, nbytes: usize, value: i64) -> Spec {
        Spec::Int {
            little_endian,
            signed: true,
            nbytes,
            expected: Some(value),
        }
    }
    pub fn le_magic(nbytes: usize, value: i64) -> Spec {
        Self::magic(true, nbytes, value)
    }
    pub fn be_magic(nbytes: usize, value: i64) -> Spec {
        Self::magic(false, nbytes, value)
    }
    pub fn int(little_endian: bool, signed: bool, nbytes: usize) -> Spec {
        Spec::Int {
            little_endian,
            signed,
            nbytes,
            expected: None,
        }
    }
    pub fn le_int(signed: bool, nbytes: usize) -> Spec {
        Self::int(true, signed, nbytes)
    }
    pub fn be_int(signed: bool, nbytes: usize) -> Spec {
        Self::int(true, signed, nbytes)
    }
    pub fn le_sint(nbytes: usize) -> Spec {
        Self::le_int(true, nbytes)
    }
    pub fn le_uint(nbytes: usize) -> Spec {
        Self::le_int(false, nbytes)
    }
    pub fn be_sint(nbytes: usize) -> Spec {
        Self::be_int(true, nbytes)
    }
    pub fn be_uint(nbytes: usize) -> Spec {
        Self::be_int(false, nbytes)
    }
    pub fn arr<E: Into<Expr>>(p: Spec, e: E) -> Spec {
        Spec::Array {
            size: e.into(),
            member: p.into(),
        }
    }

    /// Enum -- will try each of the alternatives in order,
    /// and return the first successful variant
    /// The variant will be tagged in the resulting Data
    /// (as a Data::Enum)
    pub fn en<K: Into<Key>>(vec: Vec<(K, Spec)>) -> Spec {
        Spec::Enum(vec.into_iter().map(|(k, p)| (k.into(), p)).collect())
    }

    /// Struct -- all alternatives must succeed in order
    pub fn st(vec: Vec<Spec>) -> Spec {
        Spec::Struct(vec)
    }

    pub fn scope<K: Into<Key>, E: Into<Expr>>(args: Vec<(K, E)>, p: Spec) -> Spec {
        Spec::Scope {
            args: args
                .into_iter()
                .map(|(k, e)| (k.into(), e.into()))
                .collect(),
            body: p.into(),
        }
    }

    /// Stores the result of a parse into a key for later use
    /// (e.g. as length of an array)
    pub fn store<K: Into<Key>>(key: K, p: Spec) -> Spec {
        Spec::Store(key.into(), p.into())
    }
}

pub struct SpecArgs {
    args: Vec<(Key, Expr)>,
}

impl SpecArgs {
    pub fn arg<K: Into<Key>, E: Into<Expr>>(mut self, k: K, e: E) -> Self {
        self.args.push((k.into(), e.into()));
        self
    }
    pub fn get(self) -> Vec<(Key, Expr)> {
        self.args
    }
}

pub struct SpecPairs {
    pairs: Vec<(Key, Spec)>,
}

impl SpecPairs {
    pub fn arg<K: Into<Key>>(mut self, k: K, p: Spec) -> Self {
        self.pairs.push((k.into(), p));
        self
    }
    pub fn get(self) -> Vec<(Key, Spec)> {
        self.pairs
    }
}
