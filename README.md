# dbin: Declarative binary parser

Like regular expressions, but for binary data

A lightweight utility for parsing and rendering binary data

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

    // For parsing, you first create a 'dbin::Pattern' to
    // create a description of what you want to parse
    // together with 'dbin::Expr', you can dynamically
    // specify the length of array types based on
    // input seen earlier in the data.
    let parser = {
        use dbin::prelude::*;
        any_of((
            all_of((
                all_of(()).mapval("big-endian"),
                be_magic_u64(1234),
                BE_U16,
            )),
            all_of((
                all_of(()).mapval("little-endian"),
                le_magic_u64(1234),
                U8,
                U8,
            )),
        ))
    };

    // Just pass a &[u8] to the parse method on a Pattern
    // to try and parse it.
    // The resulting value will be a dbin::Data instance
    let data = parser.parse(&bytes).unwrap();

    assert_eq!(data, Data::fseq(vec![
        "little-endian".into(),
        Data::Int(1234),
        Data::Int(80),
        Data::Int(195),
    ]));
    assert_eq!(50000, (195 << 8) + 80);

    // Test arrays whose lengths are determined by parsed data:
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
}
```

## Release notes

### 0.1.4

Simplified the parsing usage
