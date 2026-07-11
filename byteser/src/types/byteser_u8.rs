use crate::byteser::ByteSerializable;

impl ByteSerializable for u8 {
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        out.push(*self);
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        if input.is_empty() {
            return Err("Unexpected end of input".to_string());
        }
        let value = input[0];
        *input = &input[1..];
        Ok(value)
    }
}
