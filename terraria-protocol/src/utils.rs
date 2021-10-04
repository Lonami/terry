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

#[derive(PartialEq, Eq, Default, Clone)]
pub struct HexVec(pub Vec<u8>);

impl fmt::Display for HexVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        HexString(&self.0).fmt(f)
    }
}

impl fmt::Debug for HexVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&HexString(&self.0), f)
    }
}
