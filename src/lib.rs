mod context;
mod data;
mod expr;
mod parser;
pub mod prelude;
mod pvec;
mod render;
pub mod samples;

pub use context::Context;
pub use context::Scope;
pub use data::Data;
pub use expr::Expr;
pub use parser::ParseError;
pub use parser::Pattern;
pub use pvec::PatternVec;
pub use render::render;
pub use render::Render;
pub use render::Renderable;

fn err<T, S: Into<String>>(s: S) -> Result<T, ParseError> {
    Err(ParseError::Other(s.into()))
}

pub enum Endian {
    Little,
    Big,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let bytes = render((1234u64, 50000u16));
        let parser = {
            use prelude::*;

            all_of((magic_u64(1234), U8, U8))
        };

        let data = parser.parse(&bytes).unwrap();

        assert_eq!(
            data,
            Data::fseq(vec![Data::Int(1234), Data::Int(80), Data::Int(195),])
        );
        assert_eq!(50000, (195 << 8) + 80);
    }

    #[test]
    fn alternatives() {
        let bytes = render((1234u64, 50000u16));
        let parser = {
            use prelude::*;
            any_of((
                all_of((all_of(()).mapval("big-endian"), be_magic_u64(1234), BE_U16)),
                all_of((
                    all_of(()).mapval("little-endian"),
                    le_magic_u64(1234),
                    U8,
                    U8,
                )),
            ))
        };

        let data = parser.parse(&bytes).unwrap();

        assert_eq!(
            data,
            Data::fseq(vec![
                "little-endian".into(),
                Data::Int(1234),
                Data::Int(80),
                Data::Int(195),
            ])
        );
        assert_eq!(50000, (195 << 8) + 80);
    }

    #[test]
    fn list() {
        let bytes = render((
            1234u32, // magic
            3u32,    // length - 1
            (
                777u64, 888u64, 999u64, 444u64, // data to be parsed
                555u64, 666u64, // just some extra data
            ),
        ));

        enum Key {
            LENGTH,
        }

        let parser = {
            use prelude::*;
            all_of((
                le_magic_u32(1234),
                // after magic, the header specifies length of
                // upcoming array - 1
                // Store the computed length to 'Key::LENGTH'
                U32.add(1).store(Key::LENGTH as i64),
                // finally, specify an array of u64,
                // whose length is determined by the Key::LENGTH
                // value stored above
                array_of(LE_U64, getvar(Key::LENGTH as i64)),
            ))
        };

        let data = parser.parse(&bytes).unwrap();

        assert_eq!(
            data,
            Data::fseq(vec![
                Data::Int(1234), // magic,
                Data::Int(4),    // length
                // the actual array
                Data::fseq(vec![
                    Data::Int(777),
                    Data::Int(888),
                    Data::Int(999),
                    Data::Int(444),
                ]),
            ])
        );

        // try again, but with lenght a little longer
        let bytes = render((
            1234u32, // magic
            4u32,    // length - 1
            (
                777u64, 888u64, 999u64, 444u64, // data to be parsed
                555u64, 666u64, // just some extra data
            ),
        ));

        let data = parser.parse(&bytes).unwrap();

        assert_eq!(
            data,
            Data::fseq(vec![
                Data::Int(1234), // magic,
                Data::Int(5),    // length
                // the actual array
                Data::fseq(vec![
                    Data::Int(777),
                    Data::Int(888),
                    Data::Int(999),
                    Data::Int(444),
                    Data::Int(555),
                ]),
            ])
        );
    }
}
