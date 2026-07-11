pub trait ByteSerializable: Sized {
    fn byteSerialize(&self, out: &mut Vec<u8>);
    fn byteDeserialize(input: &mut &[u8]) -> Result<Self, String>;
}