use crate::Context;
use crate::Data;
use crate::Expr;
use crate::Key;
use crate::Spec;
use std::collections::HashMap;

impl Spec {
    pub fn parse(&self, bytes: &[u8]) -> Result<Data, String> {
        let mut ctx = Context::new(bytes);
        parse(&mut ctx, self)
    }
}

fn parse(ctx: &mut Context, spec: &Spec) -> Result<Data, String> {
    match spec {
        Spec::Int {
            little_endian,
            signed,
            nbytes,
            expected,
        } => {
            let bytes = ctx.read(*nbytes)?;
            let i = getint(*little_endian, *signed, bytes);
            match expected {
                Some(expected) => {
                    if i == *expected {
                        Ok(Data::Int(i))
                    } else {
                        Err(format!("Expected {} but got {}", expected, i))
                    }
                }
                None => Ok(Data::Int(i)),
            }
        }
        Spec::Array { size, member } => {
            let size = evalsize(ctx, size)?;
            let mut ret = Vec::new();
            for _ in 0..size {
                ret.push(parse(ctx, member)?);
            }
            Ok(Data::Array(ret))
        }
        Spec::Enum(pairs) => {
            let mut ret = Err("Empty enum".to_owned());
            let save_point = ctx.store();
            for (key, spec) in pairs {
                ret = parse(ctx, spec);
                if let Ok(data) = ret {
                    return Ok(Data::Enum(key.clone(), data.into()));
                } else {
                    ctx.restore(save_point);
                }
            }
            ret
        }
        Spec::Struct(pairs) => {
            let mut ret = Vec::new();
            for spec in pairs {
                let data = parse(ctx, spec)?;
                ret.push(data);
            }
            Ok(Data::Struct(ret))
        }
        Spec::Scope { args, body } => {
            let mut map = HashMap::new();
            for (key, valexpr) in args {
                let val = eval(ctx, valexpr)?;
                map.insert(key.clone(), val);
            }
            ctx.push_stack(map);
            let ret = parse(ctx, body);
            ctx.pop_stack();
            ret
        }
        Spec::Store(key, body) => {
            let data = parse(ctx, body)?;
            let val = match &data {
                Data::Int(i) => Key::Int(*i),
                Data::String(s) => Key::String(s.clone()),
                _ => {
                    return Err(format!("Could not convert {:?} into key value", data));
                }
            };
            ctx.setvar(key.clone(), val);
            Ok(data)
        }
    }
}

fn eval(ctx: &mut Context, expr: &Expr) -> Result<Key, String> {
    match expr {
        Expr::Int(i) => Ok(Key::Int(*i)),
        Expr::String(s) => Ok(Key::String(s.clone())),
        Expr::Var(name) => match ctx.getvar(name.clone()) {
            Some(val) => Ok(val.clone()),
            None => Err(format!("Variable {:?} not found", name)),
        },
        Expr::Add(parts) => {
            let mut ret = 0;
            for part in parts {
                ret += evalint(ctx, part)?;
            }
            Ok(Key::Int(ret))
        }
    }
}

fn evalint(ctx: &mut Context, expr: &Expr) -> Result<i64, String> {
    let key = eval(ctx, expr)?;
    match key {
        Key::Int(i) => Ok(i),
        Key::String(s) => Err(format!("Expected int but got {:?}", s)),
    }
}

fn evalsize(ctx: &mut Context, expr: &Expr) -> Result<usize, String> {
    let key = eval(ctx, expr)?;
    match key {
        Key::Int(i) => Ok(i as usize),
        Key::String(s) => Err(format!("Expected size but got {:?}", s)),
    }
}

fn getint(little_endian: bool, signed: bool, bytes: &[u8]) -> i64 {
    if signed {
        sint(little_endian, bytes)
    } else {
        uint(little_endian, bytes) as i64
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

#[cfg(test)]
mod tests {

    fn le_sint(bytes: &[u8]) -> i64 {
        super::sint(true, bytes)
    }

    fn le_uint(bytes: &[u8]) -> u64 {
        super::uint(true, bytes)
    }

    fn be_sint(bytes: &[u8]) -> i64 {
        super::sint(false, bytes)
    }

    fn be_uint(bytes: &[u8]) -> u64 {
        super::uint(false, bytes)
    }

    #[test]
    fn int_helpers() {
        assert_eq!(le_uint(&890u64.to_le_bytes()), 890);
        assert_eq!(le_uint(&678u64.to_le_bytes()), 678);
        assert_eq!(le_sint(&123i64.to_le_bytes()), 123);
        assert_eq!(le_sint(&(-456i64).to_le_bytes()), -456);
        assert_eq!(le_sint(&(-0i64).to_le_bytes()), -0);
        assert_eq!(le_sint(&(-1i64).to_le_bytes()), -1);
        assert_eq!(be_uint(&890u64.to_be_bytes()), 890);
        assert_eq!(be_uint(&678u64.to_be_bytes()), 678);
        assert_eq!(be_sint(&123i64.to_be_bytes()), 123);
        assert_eq!(be_sint(&(-456i64).to_be_bytes()), -456);
        assert_eq!(be_sint(&(-0i64).to_be_bytes()), -0);
        assert_eq!(be_sint(&(-1i64).to_be_bytes()), -1);

        assert_eq!(le_uint(&890u32.to_le_bytes()), 890);
        assert_eq!(le_uint(&678u32.to_le_bytes()), 678);
        assert_eq!(le_sint(&123i32.to_le_bytes()), 123);
        assert_eq!(le_sint(&(-456i32).to_le_bytes()), -456);
        assert_eq!(le_sint(&(-0i32).to_le_bytes()), -0);
        assert_eq!(le_sint(&(-1i32).to_le_bytes()), -1);
        assert_eq!(be_uint(&890u32.to_be_bytes()), 890);
        assert_eq!(be_uint(&678u32.to_be_bytes()), 678);
        assert_eq!(be_sint(&123i32.to_be_bytes()), 123);
        assert_eq!(be_sint(&(-456i32).to_be_bytes()), -456);
        assert_eq!(be_sint(&(-0i32).to_be_bytes()), -0);
        assert_eq!(be_sint(&(-1i32).to_be_bytes()), -1);
    }
}
