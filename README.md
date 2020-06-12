# dbin: Declarative binary parser

Like regular expression, but for binary data

A lightweight utility for parsing and rendering binary data

It's meant to be a bit quick and dirty and convenient.
Even if it might not exhaustively cover all cases, it should
be able to cover a lot of them with minimal effort. You know,
like regular expressions.

## Example

```rust
extern crate::dbin;
use dbin::Spec;
use dbin::Data;

fn main() {
    // The dbin::render, dbin::Render, dbin::Renderable
    // functions and enums make it easy to quickly create
    // Vec<u8> from some data (from tuples, vectors,
    // various integral types, and combinations of them
    // with varying endianness).
    // If endianness is not specified, usually
    // little endian is assumed.
    let bytes = dbin::render((
        1234u64,
        50000u16,
    ));

    // For parsing, you first create a 'dbin::Spec' to
    // create a description of what you want to parse
    // together with 'dbin::Expr', you can dynamically
    // specify the length of array types based on
    // input seen earlier in the data.
    // NOTE: 'en' is short for 'enum' and 'st' is short
    // for 'struct'.
    // 'en' creates a spec that chooses from a list of
    // alternatives, and
    // 'st' creates a spec that requires a heterogeneous
    // list of specs to be parsed in sequence.
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

    // Just pass a &[u8] to the parse method on a Spec
    // to try and parse it.
    // The result will be a dbin::Data instance
    let data = spec.parse(&bytes).unwrap();

    assert_eq!(data, Data::Enum(
        "little".into(),
        Data::Struct(vec![
            Data::Int(1234),
            Data::Int(80),
            Data::Int(195),
        ]).into(),
    ));
}
```
