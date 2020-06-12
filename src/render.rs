/// For quickly rendering data into bytes
use crate::Endian;

pub enum Render {}

impl Render {
    pub fn be_u16(i: u16) -> Renderable {
        Renderable::U16(Endian::Big, i)
    }
    pub fn be_u32(i: u32) -> Renderable {
        Renderable::U32(Endian::Big, i)
    }
    pub fn be_u64(i: u64) -> Renderable {
        Renderable::U64(Endian::Big, i)
    }
    pub fn be_i16(i: i16) -> Renderable {
        Renderable::I16(Endian::Big, i)
    }
    pub fn be_i32(i: i32) -> Renderable {
        Renderable::I32(Endian::Big, i)
    }
    pub fn be_i64(i: i64) -> Renderable {
        Renderable::I64(Endian::Big, i)
    }
}

pub enum Renderable {
    U8(u8),
    U16(Endian, u16),
    U32(Endian, u32),
    U64(Endian, u64),
    I8(i8),
    I16(Endian, i16),
    I32(Endian, i32),
    I64(Endian, i64),
    Seq(Vec<Renderable>),
}

impl From<u8> for Renderable {
    fn from(i: u8) -> Renderable {
        Renderable::U8(i)
    }
}
impl From<u16> for Renderable {
    fn from(i: u16) -> Renderable {
        Renderable::U16(Endian::Little, i)
    }
}
impl From<u32> for Renderable {
    fn from(i: u32) -> Renderable {
        Renderable::U32(Endian::Little, i)
    }
}
impl From<u64> for Renderable {
    fn from(i: u64) -> Renderable {
        Renderable::U64(Endian::Little, i)
    }
}
impl From<i8> for Renderable {
    fn from(i: i8) -> Renderable {
        Renderable::I8(i)
    }
}
impl From<i16> for Renderable {
    fn from(i: i16) -> Renderable {
        Renderable::I16(Endian::Little, i)
    }
}
impl From<i32> for Renderable {
    fn from(i: i32) -> Renderable {
        Renderable::I32(Endian::Little, i)
    }
}
impl From<i64> for Renderable {
    fn from(i: i64) -> Renderable {
        Renderable::I64(Endian::Little, i)
    }
}
impl<T: Into<Renderable>> From<Vec<T>> for Renderable {
    fn from(v: Vec<T>) -> Renderable {
        Renderable::Seq(v.into_iter().map(|t| t.into()).collect())
    }
}
impl<T: Into<Renderable> + Clone> From<&[T]> for Renderable {
    fn from(v: &[T]) -> Renderable {
        v.to_vec().into()
    }
}
impl<A1: Into<Renderable>, A2: Into<Renderable>> From<(A1, A2)> for Renderable {
    fn from(x: (A1, A2)) -> Renderable {
        Renderable::Seq(vec![x.0.into(), x.1.into()])
    }
}
impl<A1: Into<Renderable>, A2: Into<Renderable>, A3: Into<Renderable>> From<(A1, A2, A3)>
    for Renderable
{
    fn from(x: (A1, A2, A3)) -> Renderable {
        Renderable::Seq(vec![x.0.into(), x.1.into(), x.2.into()])
    }
}
impl<A1: Into<Renderable>, A2: Into<Renderable>, A3: Into<Renderable>, A4: Into<Renderable>>
    From<(A1, A2, A3, A4)> for Renderable
{
    fn from(x: (A1, A2, A3, A4)) -> Renderable {
        Renderable::Seq(vec![x.0.into(), x.1.into(), x.2.into(), x.3.into()])
    }
}

pub fn render<R: Into<Renderable>>(r: R) -> Vec<u8> {
    let r = r.into();
    let mut ret = Vec::new();
    rend(&r, &mut ret);
    ret
}

fn rend(r: &Renderable, out: &mut Vec<u8>) {
    match r {
        Renderable::U8(i) => out.push(*i),
        Renderable::U16(endian, i) => match endian {
            Endian::Little => out.extend(&i.to_le_bytes()),
            Endian::Big => out.extend(&i.to_be_bytes()),
        },
        Renderable::U32(endian, i) => match endian {
            Endian::Little => out.extend(&i.to_le_bytes()),
            Endian::Big => out.extend(&i.to_be_bytes()),
        },
        Renderable::U64(endian, i) => match endian {
            Endian::Little => out.extend(&i.to_le_bytes()),
            Endian::Big => out.extend(&i.to_be_bytes()),
        },
        Renderable::I8(i) => out.push(*i as u8),
        Renderable::I16(endian, i) => match endian {
            Endian::Little => out.extend(&i.to_le_bytes()),
            Endian::Big => out.extend(&i.to_be_bytes()),
        },
        Renderable::I32(endian, i) => match endian {
            Endian::Little => out.extend(&i.to_le_bytes()),
            Endian::Big => out.extend(&i.to_be_bytes()),
        },
        Renderable::I64(endian, i) => match endian {
            Endian::Little => out.extend(&i.to_le_bytes()),
            Endian::Big => out.extend(&i.to_be_bytes()),
        },
        Renderable::Seq(parts) => {
            for part in parts {
                rend(part, out);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::render;
    use super::Render;

    #[test]
    fn simple() {
        assert_eq!(&render(15i32), &[15, 0, 0, 0]);
        assert_eq!(&render(-1i32), &[255, 255, 255, 255]);
        assert_eq!(&render(-2i32), &[254, 255, 255, 255]);

        assert_eq!(&render(Render::be_i16(-2)), &[255, 254]);

        assert_eq!(
            &render((0xAABB, Render::be_i16(-8))),
            &[0xBB, 0xAA, 0, 0, 255, 248],
        );
    }
}
