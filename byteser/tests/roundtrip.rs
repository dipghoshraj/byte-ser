use byteser::ByteSerializable;
use byteser_derive::ByteSerializable;
use std::collections::HashMap;

#[derive(ByteSerializable, Debug, PartialEq)]
struct TestStruct {
    a: u8,
    b: u32,
    c: String,
}

#[derive(ByteSerializable, Debug, PartialEq)]
struct AllSupportedTypes {
    a: u16,
    b: u64,
    c: i8,
    d: i16,
    e: i32,
    f: i64,
    g: usize,
    h: bool,
    i: Vec<u32>,
    j: Option<String>,
    k: HashMap<String, u32>,
}

#[test]
fn roundtrip_works_for_supported_types() {
    let original = TestStruct {
        a: 42,
        b: 0x12345678,
        c: "hello".to_string(),
    };

    let mut bytes = Vec::new();
    original.byte_serialize(&mut bytes);

    let mut slice: &[u8] = &bytes;
    let decoded = TestStruct::byte_deserialize(&mut slice).expect("deserialize failed");

    assert_eq!(decoded, original);
    assert!(slice.is_empty(), "input should be fully consumed");
}

#[test]
fn roundtrip_works_for_all_new_types() {
    let mut map = HashMap::new();
    map.insert("one".to_string(), 1u32);
    map.insert("two".to_string(), 2u32);

    let original = AllSupportedTypes {
        a: 0x1234,
        b: 0x1234567890abcdef,
        c: -42,
        d: -1234,
        e: -123456,
        f: -1234567890123456,
        g: 12345678,
        h: true,
        i: vec![1, 2, 3],
        j: Some("hello".to_string()),
        k: map,
    };

    let mut bytes = Vec::new();
    original.byte_serialize(&mut bytes);

    let mut slice: &[u8] = &bytes;
    let decoded = AllSupportedTypes::byte_deserialize(&mut slice).expect("deserialize failed");

    assert_eq!(decoded, original);
    assert!(slice.is_empty(), "input should be fully consumed");
}
