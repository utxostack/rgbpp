// Generated by Molecule 0.7.5

use super::blockchain::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct RGBPPConfig(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for RGBPPConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for RGBPPConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for RGBPPConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(
            f,
            "{}: {}",
            "bitcoin_lc_type_hash",
            self.bitcoin_lc_type_hash()
        )?;
        write!(
            f,
            ", {}: {}",
            "bitcoin_time_lock_type_hash",
            self.bitcoin_time_lock_type_hash()
        )?;
        write!(f, " }}")
    }
}
impl ::core::default::Default for RGBPPConfig {
    fn default() -> Self {
        let v = molecule::bytes::Bytes::from_static(&Self::DEFAULT_VALUE);
        RGBPPConfig::new_unchecked(v)
    }
}
impl RGBPPConfig {
    const DEFAULT_VALUE: [u8; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    pub const TOTAL_SIZE: usize = 64;
    pub const FIELD_SIZES: [usize; 2] = [32, 32];
    pub const FIELD_COUNT: usize = 2;
    pub fn bitcoin_lc_type_hash(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(0..32))
    }
    pub fn bitcoin_time_lock_type_hash(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(32..64))
    }
    pub fn as_reader<'r>(&'r self) -> RGBPPConfigReader<'r> {
        RGBPPConfigReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for RGBPPConfig {
    type Builder = RGBPPConfigBuilder;
    const NAME: &'static str = "RGBPPConfig";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        RGBPPConfig(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RGBPPConfigReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RGBPPConfigReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .bitcoin_lc_type_hash(self.bitcoin_lc_type_hash())
            .bitcoin_time_lock_type_hash(self.bitcoin_time_lock_type_hash())
    }
}
#[derive(Clone, Copy)]
pub struct RGBPPConfigReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for RGBPPConfigReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for RGBPPConfigReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for RGBPPConfigReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(
            f,
            "{}: {}",
            "bitcoin_lc_type_hash",
            self.bitcoin_lc_type_hash()
        )?;
        write!(
            f,
            ", {}: {}",
            "bitcoin_time_lock_type_hash",
            self.bitcoin_time_lock_type_hash()
        )?;
        write!(f, " }}")
    }
}
impl<'r> RGBPPConfigReader<'r> {
    pub const TOTAL_SIZE: usize = 64;
    pub const FIELD_SIZES: [usize; 2] = [32, 32];
    pub const FIELD_COUNT: usize = 2;
    pub fn bitcoin_lc_type_hash(&self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(&self.as_slice()[0..32])
    }
    pub fn bitcoin_time_lock_type_hash(&self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(&self.as_slice()[32..64])
    }
}
impl<'r> molecule::prelude::Reader<'r> for RGBPPConfigReader<'r> {
    type Entity = RGBPPConfig;
    const NAME: &'static str = "RGBPPConfigReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        RGBPPConfigReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len != Self::TOTAL_SIZE {
            return ve!(Self, TotalSizeNotMatch, Self::TOTAL_SIZE, slice_len);
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct RGBPPConfigBuilder {
    pub(crate) bitcoin_lc_type_hash: Byte32,
    pub(crate) bitcoin_time_lock_type_hash: Byte32,
}
impl RGBPPConfigBuilder {
    pub const TOTAL_SIZE: usize = 64;
    pub const FIELD_SIZES: [usize; 2] = [32, 32];
    pub const FIELD_COUNT: usize = 2;
    pub fn bitcoin_lc_type_hash(mut self, v: Byte32) -> Self {
        self.bitcoin_lc_type_hash = v;
        self
    }
    pub fn bitcoin_time_lock_type_hash(mut self, v: Byte32) -> Self {
        self.bitcoin_time_lock_type_hash = v;
        self
    }
}
impl molecule::prelude::Builder for RGBPPConfigBuilder {
    type Entity = RGBPPConfig;
    const NAME: &'static str = "RGBPPConfigBuilder";
    fn expected_length(&self) -> usize {
        Self::TOTAL_SIZE
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(self.bitcoin_lc_type_hash.as_slice())?;
        writer.write_all(self.bitcoin_time_lock_type_hash.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        RGBPPConfig::new_unchecked(inner.into())
    }
}
#[derive(Clone)]
pub struct RGBPPLock(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for RGBPPLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for RGBPPLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for RGBPPLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "out_index", self.out_index())?;
        write!(
            f,
            ", {}: {}",
            "bitcoin_transaction_hash",
            self.bitcoin_transaction_hash()
        )?;
        write!(f, " }}")
    }
}
impl ::core::default::Default for RGBPPLock {
    fn default() -> Self {
        let v = molecule::bytes::Bytes::from_static(&Self::DEFAULT_VALUE);
        RGBPPLock::new_unchecked(v)
    }
}
impl RGBPPLock {
    const DEFAULT_VALUE: [u8; 36] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];
    pub const TOTAL_SIZE: usize = 36;
    pub const FIELD_SIZES: [usize; 2] = [4, 32];
    pub const FIELD_COUNT: usize = 2;
    pub fn out_index(&self) -> Uint32 {
        Uint32::new_unchecked(self.0.slice(0..4))
    }
    pub fn bitcoin_transaction_hash(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(4..36))
    }
    pub fn as_reader<'r>(&'r self) -> RGBPPLockReader<'r> {
        RGBPPLockReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for RGBPPLock {
    type Builder = RGBPPLockBuilder;
    const NAME: &'static str = "RGBPPLock";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        RGBPPLock(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RGBPPLockReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RGBPPLockReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .out_index(self.out_index())
            .bitcoin_transaction_hash(self.bitcoin_transaction_hash())
    }
}
#[derive(Clone, Copy)]
pub struct RGBPPLockReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for RGBPPLockReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for RGBPPLockReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for RGBPPLockReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "out_index", self.out_index())?;
        write!(
            f,
            ", {}: {}",
            "bitcoin_transaction_hash",
            self.bitcoin_transaction_hash()
        )?;
        write!(f, " }}")
    }
}
impl<'r> RGBPPLockReader<'r> {
    pub const TOTAL_SIZE: usize = 36;
    pub const FIELD_SIZES: [usize; 2] = [4, 32];
    pub const FIELD_COUNT: usize = 2;
    pub fn out_index(&self) -> Uint32Reader<'r> {
        Uint32Reader::new_unchecked(&self.as_slice()[0..4])
    }
    pub fn bitcoin_transaction_hash(&self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(&self.as_slice()[4..36])
    }
}
impl<'r> molecule::prelude::Reader<'r> for RGBPPLockReader<'r> {
    type Entity = RGBPPLock;
    const NAME: &'static str = "RGBPPLockReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        RGBPPLockReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len != Self::TOTAL_SIZE {
            return ve!(Self, TotalSizeNotMatch, Self::TOTAL_SIZE, slice_len);
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct RGBPPLockBuilder {
    pub(crate) out_index: Uint32,
    pub(crate) bitcoin_transaction_hash: Byte32,
}
impl RGBPPLockBuilder {
    pub const TOTAL_SIZE: usize = 36;
    pub const FIELD_SIZES: [usize; 2] = [4, 32];
    pub const FIELD_COUNT: usize = 2;
    pub fn out_index(mut self, v: Uint32) -> Self {
        self.out_index = v;
        self
    }
    pub fn bitcoin_transaction_hash(mut self, v: Byte32) -> Self {
        self.bitcoin_transaction_hash = v;
        self
    }
}
impl molecule::prelude::Builder for RGBPPLockBuilder {
    type Entity = RGBPPLock;
    const NAME: &'static str = "RGBPPLockBuilder";
    fn expected_length(&self) -> usize {
        Self::TOTAL_SIZE
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(self.out_index.as_slice())?;
        writer.write_all(self.bitcoin_transaction_hash.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        RGBPPLock::new_unchecked(inner.into())
    }
}
#[derive(Clone)]
pub struct RGBPPUnlock(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for RGBPPUnlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for RGBPPUnlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for RGBPPUnlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(
            f,
            "{}: {}",
            "bitcoin_transaction",
            self.bitcoin_transaction()
        )?;
        write!(
            f,
            ", {}: {}",
            "previous_ckb_transaction",
            self.previous_ckb_transaction()
        )?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for RGBPPUnlock {
    fn default() -> Self {
        let v = molecule::bytes::Bytes::from_static(&Self::DEFAULT_VALUE);
        RGBPPUnlock::new_unchecked(v)
    }
}
impl RGBPPUnlock {
    const DEFAULT_VALUE: [u8; 84] = [
        84, 0, 0, 0, 12, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 12, 0, 0, 0, 64, 0, 0, 0,
        52, 0, 0, 0, 28, 0, 0, 0, 32, 0, 0, 0, 36, 0, 0, 0, 40, 0, 0, 0, 44, 0, 0, 0, 48, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0,
    ];
    pub const FIELD_COUNT: usize = 2;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn bitcoin_transaction(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Bytes::new_unchecked(self.0.slice(start..end))
    }
    pub fn previous_ckb_transaction(&self) -> Transaction {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[12..]) as usize;
            Transaction::new_unchecked(self.0.slice(start..end))
        } else {
            Transaction::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> RGBPPUnlockReader<'r> {
        RGBPPUnlockReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for RGBPPUnlock {
    type Builder = RGBPPUnlockBuilder;
    const NAME: &'static str = "RGBPPUnlock";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        RGBPPUnlock(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RGBPPUnlockReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        RGBPPUnlockReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .bitcoin_transaction(self.bitcoin_transaction())
            .previous_ckb_transaction(self.previous_ckb_transaction())
    }
}
#[derive(Clone, Copy)]
pub struct RGBPPUnlockReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for RGBPPUnlockReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for RGBPPUnlockReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for RGBPPUnlockReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(
            f,
            "{}: {}",
            "bitcoin_transaction",
            self.bitcoin_transaction()
        )?;
        write!(
            f,
            ", {}: {}",
            "previous_ckb_transaction",
            self.previous_ckb_transaction()
        )?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> RGBPPUnlockReader<'r> {
    pub const FIELD_COUNT: usize = 2;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn bitcoin_transaction(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        BytesReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn previous_ckb_transaction(&self) -> TransactionReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[12..]) as usize;
            TransactionReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            TransactionReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for RGBPPUnlockReader<'r> {
    type Entity = RGBPPUnlock;
    const NAME: &'static str = "RGBPPUnlockReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        RGBPPUnlockReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % molecule::NUMBER_SIZE != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        if slice_len < offset_first {
            return ve!(Self, HeaderIsBroken, offset_first, slice_len);
        }
        let field_count = offset_first / molecule::NUMBER_SIZE - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..offset_first]
            .chunks_exact(molecule::NUMBER_SIZE)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        BytesReader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        TransactionReader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct RGBPPUnlockBuilder {
    pub(crate) bitcoin_transaction: Bytes,
    pub(crate) previous_ckb_transaction: Transaction,
}
impl RGBPPUnlockBuilder {
    pub const FIELD_COUNT: usize = 2;
    pub fn bitcoin_transaction(mut self, v: Bytes) -> Self {
        self.bitcoin_transaction = v;
        self
    }
    pub fn previous_ckb_transaction(mut self, v: Transaction) -> Self {
        self.previous_ckb_transaction = v;
        self
    }
}
impl molecule::prelude::Builder for RGBPPUnlockBuilder {
    type Entity = RGBPPUnlock;
    const NAME: &'static str = "RGBPPUnlockBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.bitcoin_transaction.as_slice().len()
            + self.previous_ckb_transaction.as_slice().len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.bitcoin_transaction.as_slice().len();
        offsets.push(total_size);
        total_size += self.previous_ckb_transaction.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.bitcoin_transaction.as_slice())?;
        writer.write_all(self.previous_ckb_transaction.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        RGBPPUnlock::new_unchecked(inner.into())
    }
}
