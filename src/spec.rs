use crate::ReadContext;

/// Describes how a series of bytes may be structured
/// The `Data` type parameter determines the type of
/// the data when parsed
pub trait DataSpec {
    type Data;
    fn read(&self, ctx: &mut ReadContext) -> Result<Self::Data, String>;
}

pub enum Endian {
    Little,
    Big,
}

/// unsigned int, made of given number of bytes
/// the output type is u64, so byte_count must be <= 8
pub struct UInt {
    endian: Endian,
    byte_count: usize,
}

impl UInt {
    pub fn new(endian: Endian, byte_count: usize) -> UInt {
        assert!(byte_count <= 8 && byte_count > 0);
        UInt { endian, byte_count }
    }
}

impl DataSpec for UInt {
    type Data = u64;
    fn read(&self, ctx: &mut ReadContext) -> Result<Self::Data, String> {
        let mut bytes = ctx.read(self.byte_count)?.to_vec();
        if let Endian::Big = self.endian {
            bytes.reverse();
        }
        for _ in self.byte_count..8 {
            bytes.push(0);
        }
        let mut ret: u64 = 0;
        for digit in bytes.into_iter().rev() {
            ret <<= 8;
            ret += digit as u64;
        }
        Ok(ret)
    }
}

/// signed int, made of given number of bytes
/// signed ints are a bit more restrictive than the unsigned
/// counterparts -- byte_count must be one of 1, 2, 4, 8
pub struct SInt {
    endian: Endian,
    byte_count: usize,
}

impl SInt {
    pub fn new(endian: Endian, byte_count: usize) -> SInt {
        match byte_count {
            1 | 2 | 4 | 8 => (),
            _ => panic!("SInt byte count must be one of 1, 2, 4, 8"),
        }
        SInt { endian, byte_count }
    }
}

impl DataSpec for SInt {
    type Data = i64;
    fn read(&self, ctx: &mut ReadContext) -> Result<Self::Data, String> {
        match self.byte_count {
            1 => Ok(ctx.read(1)?[0] as i8 as i64),
            2 => {
                let bytes = ctx.read(2)?;
                let bytes = [bytes[0], bytes[1]];
                Ok(match self.endian {
                    Endian::Little => i16::from_le_bytes(bytes),
                    Endian::Big => i16::from_be_bytes(bytes),
                } as i64)
            }
            4 => {
                let bytes = ctx.read(4)?;
                let bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
                Ok(match self.endian {
                    Endian::Little => i32::from_le_bytes(bytes),
                    Endian::Big => i32::from_be_bytes(bytes),
                } as i64)
            }
            8 => {
                let bytes = ctx.read(8)?;
                let bytes = [
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ];
                Ok(match self.endian {
                    Endian::Little => i64::from_le_bytes(bytes),
                    Endian::Big => i64::from_be_bytes(bytes),
                })
            }
            _ => panic!("FUBAR, SInt invalid byte_count: {}", self.byte_count),
        }
    }
}

pub struct Struct<K> {
    fields: Vec<(K, DataSpec)>,
}
