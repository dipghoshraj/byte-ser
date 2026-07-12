# byte-ser

`byte-ser` is a lightweight Rust crate for serializing and deserializing Rust values to and from raw bytes without using `serde`.

The goal is minimal overhead for simple types and structs when you want to avoid the binary size impact of a full serialization framework. If you do not have a strict binary-size constraint, using `serde` is usually the better choice.

## Features

- Trait-based serialization with `ByteSerializable`
- Derive support via `byteser_derive::ByteSerializable`
- Built-in support for:
  - primitive integers: `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `usize`
  - `bool`
  - `String`
  - `Vec<T>`
  - `Option<T>`
  - `HashMap<K, V>`
- Struct derive support for named-field structs

## Usage

Add the workspace crates as dependencies (or publish the crate to crates.io and use the published names):

```toml
[dependencies]
byteser = { path = "./byteser" }
byteser_derive = { path = "./byteser_derive" }
```

In your Rust code:

```rust
use byteser::ByteSerializable;
use byteser_derive::ByteSerializable;

#[derive(ByteSerializable, Debug, PartialEq)]
struct TestStruct {
    a: u8,
    b: u32,
    c: String,
}

let value = TestStruct {
    a: 42,
    b: 0x12345678,
    c: "hello".to_string(),
};

let mut bytes = Vec::new();
value.byte_serialize(&mut bytes);

let mut slice: &[u8] = &bytes;
let decoded = TestStruct::byte_deserialize(&mut slice).expect("deserialize failed");
assert_eq!(decoded, value);
```

## Notes

- This crate is intended for simple, binary-size-sensitive use cases.
- The serialization format is not self-describing, so structure changes must be managed carefully.
- If you need rich formats, backward/forward compatibility, or broader ecosystem support, prefer `serde`.

## Testing

Run the test suite with:

```bash
cargo test
```
