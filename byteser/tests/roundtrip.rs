use byteser::ByteSerializable;
use byteser_derive::ByteSerializable;

#[derive(ByteSerializable, Debug, PartialEq)]
struct TestStruct {
    a: u8,
    b: u32,
    c: String,
}

#[test]
fn roundtrip_works_for_supported_types() {
    let original = TestStruct {
        a: 42,
        b: 0x12345678,
        c: "hello".to_string(),
    };

    let mut bytes = Vec::new();
    original.byteSerialize(&mut bytes);

    let mut slice: &[u8] = &bytes;
    let decoded = TestStruct::byteDeserialize(&mut slice).expect("deserialize failed");

    assert_eq!(decoded, original);
    assert!(slice.is_empty(), "input should be fully consumed");
}
