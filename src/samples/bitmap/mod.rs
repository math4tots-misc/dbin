//! BMP file parser
use crate::Pattern;

#[derive(Debug)]
pub enum Key {
    FileSize,
    PixelOffset,
    DibHeaderSize,
    WidthInPixels,
    HeightInPixels,
}

impl From<Key> for i64 {
    fn from(k: Key) -> i64 { k as i64 }
}

// TODO
// pub fn pattern() -> Pattern {
//     use crate::prelude::*;
//     file_header()
// }

pub fn file_header() -> Pattern {
    use crate::prelude::*;

    all_of((
        magic(&[0x42, 0x4D]),
        U32.store(Key::FileSize),
        U16, // reserved
        U16, // reserved
        U32.store(Key::PixelOffset),
    ))
}

pub fn dib_header() -> Pattern {
    use crate::prelude::*;

    all_of((
        U32.store(Key::DibHeaderSize),
        U32.store(Key::WidthInPixels),
        U32.store(Key::HeightInPixels),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Data;

    pub const BMP_BYTES: &'static [u8] = include_bytes!("TRU256.BMP");

    #[test]
    pub fn file_header_with_sample() {
        let pat = file_header();
        let data = pat.parse(BMP_BYTES).unwrap();
        assert_eq!(data, Data::fseq(vec![
            Data::fbytes(vec![0x42, 0x4D]),
            Data::Int(49206), // size of the file
            Data::Int(0),
            Data::Int(0),
            Data::Int(54),
        ]));
    }

    #[test]
    pub fn dib_header_with_sample() {
        let pat: Pattern = crate::prelude::all_of((
            file_header(),
            dib_header(),
        )).to_map(vec![
            Key::FileSize,
            Key::PixelOffset,
            Key::DibHeaderSize,
            Key::WidthInPixels,
            Key::HeightInPixels,
        ]);
        let data = pat.parse(BMP_BYTES).unwrap();
        assert_eq!(data, Data::fseq(vec![
            vec!["FileSize".into(), Data::Int(49206)].into(),
            vec!["PixelOffset".into(), Data::Int(54)].into(),
            vec!["DibHeaderSize".into(), Data::Int(40)].into(),
            vec!["WidthInPixels".into(), Data::Int(256)].into(),
            vec!["HeightInPixels".into(), Data::Int(64)].into(),
        ]));
    }
}
