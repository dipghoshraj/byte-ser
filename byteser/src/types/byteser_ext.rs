use crate::byteser::ByteSerializable;
use std::collections::HashMap;
use std::hash::Hash;

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

impl ByteSerializable for u16 {
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        if input.len() < 2 {
            return Err("Unexpected end of input".to_string());
        }
        let value = u16::from_le_bytes(input[0..2].try_into().unwrap());
        *input = &input[2..];
        Ok(value)
    }
}

impl ByteSerializable for u32 {
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        if input.len() < 4 {
            return Err("Unexpected end of input".to_string());
        }
        let value = u32::from_le_bytes(input[0..4].try_into().unwrap());
        *input = &input[4..];
        Ok(value)
    }
}

impl ByteSerializable for u64 {
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        if input.len() < 8 {
            return Err("Unexpected end of input".to_string());
        }
        let value = u64::from_le_bytes(input[0..8].try_into().unwrap());
        *input = &input[8..];
        Ok(value)
    }
}

impl ByteSerializable for i8 {
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        out.push(*self as u8);
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        if input.is_empty() {
            return Err("Unexpected end of input".to_string());
        }
        let value = input[0] as i8;
        *input = &input[1..];
        Ok(value)
    }
}

impl ByteSerializable for i16 {
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        if input.len() < 2 {
            return Err("Unexpected end of input".to_string());
        }
        let value = i16::from_le_bytes(input[0..2].try_into().unwrap());
        *input = &input[2..];
        Ok(value)
    }
}

impl ByteSerializable for i32 {
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        if input.len() < 4 {
            return Err("Unexpected end of input".to_string());
        }
        let value = i32::from_le_bytes(input[0..4].try_into().unwrap());
        *input = &input[4..];
        Ok(value)
    }
}

impl ByteSerializable for i64 {
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        if input.len() < 8 {
            return Err("Unexpected end of input".to_string());
        }
        let value = i64::from_le_bytes(input[0..8].try_into().unwrap());
        *input = &input[8..];
        Ok(value)
    }
}

impl ByteSerializable for usize {
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        let size = std::mem::size_of::<usize>();
        if input.len() < size {
            return Err("Unexpected end of input".to_string());
        }
        let value = usize::from_le_bytes(input[0..size].try_into().unwrap());
        *input = &input[size..];
        Ok(value)
    }
}

impl ByteSerializable for bool {
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        out.push(if *self { 1 } else { 0 });
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        if input.is_empty() {
            return Err("Unexpected end of input".to_string());
        }
        let byte = input[0];
        *input = &input[1..];
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err("Invalid boolean value".to_string()),
        }
    }
}

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

impl<T: ByteSerializable> ByteSerializable for Vec<T> {
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        let length = self.len() as u32;
        length.byte_serialize(out);
        for item in self.iter() {
            item.byte_serialize(out);
        }
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        let length = u32::byte_deserialize(input)?;
        let mut vec = Vec::with_capacity(length as usize);
        for _ in 0..length {
            vec.push(T::byte_deserialize(input)?);
        }
        Ok(vec)
    }
}

impl<T: ByteSerializable> ByteSerializable for Option<T> {
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        match self {
            Some(value) => {
                out.push(1);
                value.byte_serialize(out);
            }
            None => {
                out.push(0);
            }
        }
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        if input.is_empty() {
            return Err("Unexpected end of input".to_string());
        }
        let tag = input[0];
        *input = &input[1..];
        match tag {
            0 => Ok(None),
            1 => Ok(Some(T::byte_deserialize(input)?)),
            _ => Err("Invalid option tag".to_string()),
        }
    }
}

impl<K, V> ByteSerializable for HashMap<K, V>
where
    K: ByteSerializable + Eq + Hash,
    V: ByteSerializable,
{
    fn byte_serialize(&self, out: &mut Vec<u8>) {
        let length = self.len() as u32;
        length.byte_serialize(out);
        for (key, value) in self.iter() {
            key.byte_serialize(out);
            value.byte_serialize(out);
        }
    }

    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String> {
        let length = u32::byte_deserialize(input)?;
        let mut map = HashMap::with_capacity(length as usize);
        for _ in 0..length {
            let key = K::byte_deserialize(input)?;
            let value = V::byte_deserialize(input)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}
