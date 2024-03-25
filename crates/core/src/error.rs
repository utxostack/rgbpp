use ckb_std::error::SysError;

#[repr(i8)]
pub enum Error {
    // 0x01 ~ 0x0f: Errors from ckb-std
    IndexOutOfBound = 0x01,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    Unknown,

    // 0x10 ~ 0x11: Load config cell
    /// Can't find config
    ConfigNotFound = 0x10,
    /// Invalid config
    ConfigMalformed,

    // 0x20 ~ 0x22: Bitcoin Spv
    SpvClientNotFound = 0x20,
    SpvClientMalformed,
    SpvProofIncorrect,
    SpvProofMalformed,

    // 0x30 ~ 0x33: Data error
    BadBtcTx = 0x30,
    BadRGBPPLock,
    BadBTCTimeLock,
    BadBtcCommitment,
    BadRGBPPUnlock,
    BadBTCTimeUnlock,

    // 0x40 ~ 0x44: verification error
    OutputCellMismatch = 0x40,
    OutputCellWithUnknownLock,
    CommitmentMismatch,
    UnknownCommitmentVersion,
    UtxoSealMismatch,
}

impl From<SysError> for Error {
    fn from(value: SysError) -> Self {
        match value {
            SysError::IndexOutOfBound => Self::IndexOutOfBound,
            SysError::ItemMissing => Self::ItemMissing,
            SysError::LengthNotEnough(_) => Self::LengthNotEnough,
            SysError::Encoding => Self::Encoding,
            SysError::Unknown(_) => Self::Unknown,
        }
    }
}

#[macro_export]
macro_rules! ensure {
    ($cond: expr, $err: expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

#[macro_export]
macro_rules! ensure_eq {
    ($a: expr,$b: expr, $err: expr) => {
        if $a != $b {
            return Err($err);
        }
    };
}
