use crate::byteser::ByteSerializable;

impl ByteSerializable for u8 {
    fn byteSerialize(&self, out: &mut Vec<u8>) {
        out.push(*self);
    }

    fn byteDeserialize(input: &mut &[u8]) -> Result<Self, String> {
        if input.is_empty() {
            return Err("Unexpected end of input".to_string());
        }
        let value = input[0];
        *input = &input[1..];
        Ok(value)
    }
}
