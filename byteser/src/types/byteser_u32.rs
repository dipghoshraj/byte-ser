use crate::byteser::ByteSerializable;

impl ByteSerializable for u32 {
    fn byteSerialize(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }

    fn byteDeserialize(input: &mut &[u8]) -> Result<Self, String> {
        if input.len() < 4 {
            return Err("Unexpected end of input".to_string());
        }
        let value = u32::from_le_bytes(input[0..4].try_into().unwrap());
        *input = &input[4..];
        Ok(value)
    }
}