use crate::err;
pub use crate::Expr;
pub use crate::Pattern;
use crate::PatternVec;

pub fn magic(bytes: &[u8]) -> Pattern {
    Pattern::Exact(bytes.to_vec())
}

pub fn magic_u16(x: u16) -> Pattern {
    le_magic_u16(x)
}

pub fn magic_u32(x: u32) -> Pattern {
    le_magic_u32(x)
}

pub fn magic_u64(x: u64) -> Pattern {
    le_magic_u64(x)
}

pub fn le_magic_u16(x: u16) -> Pattern {
    magic(&x.to_le_bytes()).mapval(x as i64)
}

pub fn le_magic_u32(x: u32) -> Pattern {
    magic(&x.to_le_bytes()).mapval(x as i64)
}

pub fn le_magic_u64(x: u64) -> Pattern {
    magic(&x.to_le_bytes()).mapval(x as i64)
}

pub fn be_magic_u16(x: u16) -> Pattern {
    magic(&x.to_be_bytes()).mapval(x as i64)
}

pub fn be_magic_u32(x: u32) -> Pattern {
    magic(&x.to_be_bytes()).mapval(x as i64)
}

pub fn be_magic_u64(x: u64) -> Pattern {
    magic(&x.to_be_bytes()).mapval(x as i64)
}

pub const U8: Pattern = Pattern::U8;
pub const I8: Pattern = Pattern::I8;

pub const LE_U16: Pattern = Pattern::LeU16;
pub const LE_U32: Pattern = Pattern::LeU32;
pub const LE_U64: Pattern = Pattern::LeU64;
pub const BE_U16: Pattern = Pattern::BeU16;
pub const BE_U32: Pattern = Pattern::BeU32;
pub const BE_U64: Pattern = Pattern::BeU64;

pub const LE_I16: Pattern = Pattern::LeI16;
pub const LE_I32: Pattern = Pattern::LeI32;
pub const LE_I64: Pattern = Pattern::LeI64;
pub const BE_I16: Pattern = Pattern::BeI16;
pub const BE_I32: Pattern = Pattern::BeI32;
pub const BE_I64: Pattern = Pattern::BeI64;

// By default, if no endianness is specified,
// assume little endian
pub const U16: Pattern = Pattern::LeU16;
pub const U32: Pattern = Pattern::LeU32;
pub const U64: Pattern = Pattern::LeU64;
pub const I16: Pattern = Pattern::LeI16;
pub const I32: Pattern = Pattern::LeI32;
pub const I64: Pattern = Pattern::LeI64;

pub const CSTR: Pattern = Pattern::CStr;

pub fn any_of<PV: Into<PatternVec>>(pv: PV) -> Pattern {
    Pattern::AnyOf(pv.into().get())
}

pub fn all_of<PV: Into<PatternVec>>(pv: PV) -> Pattern {
    Pattern::AllOf(pv.into().get())
}

pub fn array_of<E: Into<Expr>>(p: Pattern, e: E) -> Pattern {
    Pattern::Array(p.into(), e.into())
}

/// Convenience method -- returns the expression from
/// retrieving a value from the scope
pub fn getvar(key: i64) -> Expr {
    Expr::new(move |scope| match scope.get(key) {
        Some(val) => Ok(val.clone()),
        None => err(format!("Key {:?} not found", key)),
    })
}
