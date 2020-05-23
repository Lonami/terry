pub trait Serializable {
    fn write_bytes(&self, out: &mut Vec<u8>);
}