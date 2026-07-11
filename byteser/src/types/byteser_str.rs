use crate::byteser::ByteSerializable;


impl ByteSerializable for String {
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        let bytes = self.as_bytes();
        let length = bytes.len() as u32;
        length.byte_serialize(out);
        out.extend_from_slice(bytes);
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        let length = u32::byte_deserialize(input)?;
        if input.len() < length as usize {
            return Err("Unexpected end of input".to_string());
        }
        let value = String::from_utf8(input[0..length as usize].to_vec())
            .map_err(|_| "Invalid UTF-8 string".to_string())?;
        *input = &input[length as usize..];
        Ok(value)
    }
}