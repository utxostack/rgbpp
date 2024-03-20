use super::ckb_gen_types::prelude::*;
use super::rgbpp::*;
use molecule::bytes::Bytes;

impl Pack<Uint16> for u16 {
    fn pack(&self) -> Uint16 {
        Uint16::new_unchecked(Bytes::from(self.to_le_bytes().to_vec()))
    }
}

impl<'r> Unpack<u16> for Uint16Reader<'r> {
    fn unpack(&self) -> u16 {
        let mut b = [0u8; 2];
        b.copy_from_slice(self.as_slice());
        u16::from_le_bytes(b)
    }
}

impl Unpack<u16> for Uint16 {
    fn unpack(&self) -> u16 {
        self.as_reader().unpack()
    }
}
