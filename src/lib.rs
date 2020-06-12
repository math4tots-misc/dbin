mod context;
mod data;
mod expr;
mod key;
mod parse;
mod render;
mod spec;

pub use context::Context;
pub use data::Data;
pub use expr::Expr;
pub use key::Key;
pub use render::render;
pub use render::Render;
pub use render::Renderable;
pub use spec::Spec;

pub enum Endian {
    Little,
    Big,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let bytes = render((
            1234u64,
            50000u16,
        ));
        let spec = Spec::st(vec![
            Spec::le_magic_u64(1234),
            Spec::le_uint(1),
            Spec::le_uint(1),
        ]);

        let data = spec.parse(&bytes).unwrap();

        assert_eq!(data, Data::Struct(vec![
            Data::Int(1234),
            Data::Int(80),
            Data::Int(195),
        ]));
        assert_eq!(50000, (195 << 8) + 80);
    }

    #[test]
    fn alternatives() {
        let bytes = render((
            1234u64,
            50000u16,
        ));
        let spec = Spec::en(vec![
            ("big", Spec::st(vec![
                Spec::be_magic_u64(1234),
                Spec::be_uint(1),
                Spec::be_uint(1),
            ])),
            ("little", Spec::st(vec![
                Spec::le_magic_u64(1234),
                Spec::le_uint(1),
                Spec::le_uint(1),
            ])),
        ]);

        let data = spec.parse(&bytes).unwrap();

        assert_eq!(data, Data::Enum(
            "little".into(),
            Data::Struct(vec![
                Data::Int(1234),
                Data::Int(80),
                Data::Int(195),
            ]).into(),
        ));
        assert_eq!(50000, (195 << 8) + 80);
    }
}
