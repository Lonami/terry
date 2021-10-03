use std::fmt;

pub(crate) struct HexString<'a>(pub &'a [u8]);

impl<'a> fmt::Display for HexString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
