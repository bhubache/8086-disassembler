use crate::immediate::Immediate;

pub trait ByteReader {
    type Error;

    fn read_byte(&mut self) -> Result<u8, Self::Error>;

    fn read_word(&mut self) -> Result<u16, Self::Error> {
        let fst = self.read_byte()?;
        let snd = self.read_byte()?;

        Ok(u16::from_le_bytes([fst, snd]))
    }
}

pub trait ParseImmediate: Immediate + Sized {
    fn parse_immediate<R: ByteReader>(byte_reader: &mut R) -> Result<Self, R::Error>;
}
