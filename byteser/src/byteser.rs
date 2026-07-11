pub trait ByteSerializable: Sized {
    fn byte_serialize(&self, out: &mut Vec<u8>);
    fn byte_deserialize(input: &mut &[u8]) -> Result<Self, String>;
}