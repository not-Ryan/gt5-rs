#[derive(Debug)]
#[allow(dead_code)]
pub enum Gt5Response {
    /// Request was acknowledged
    Ack(u32),

    /// Obsolete, capture timeout
    NackTimeout,

    /// Obsolete, Invalid serial baud rate
    NackInvalidBaudrate,
    /// The specified ID is not between 0~999
    NackInvalidPos,

    /// The specified ID is not used
    NackIsNotUsed,
    /// The specified ID is already used
    NackIsAlreadyUsed,
    /// Communication Error
    NackCommErr,

    /// 1:1 Verification Failure
    NackVerifyFailed,

    /// 1:N Identification Failure
    NackIdentifyFailed,

    /// The database is full
    NackDbIsFull,

    /// The database is empty
    NackDbIsEmpty,

    /// Obsolete, Invalid order of the enrollment
    /// (The order was not as:
    /// EnrollStart - Enroll1 →
    /// Enroll2 -> Enroll3)
    NackTurnErr,

    /// Too bad fingerprint
    NackBadFinger,

    /// Enrollment Failure
    NackEnrollFailed,

    /// The specified command is not supported
    NackIsNotSupported,

    /// Device Error, especially if Crypto-Chip is trouble
    NackDevErr,

    /// Obsolete, The capturing is canceled
    NackCaptureCanceled,

    /// Invalid parameter
    NackInvalidParam,

    /// Finger is not pressed
    NackFingerIsNotPressed,

    /// If you got this, sorry...
    Other(u32),
}

impl Gt5Response {
    pub fn into_result(self) -> anyhow::Result<u32> {
        match self {
            Gt5Response::Ack(resp) => Ok(resp),
            nack => anyhow::bail!("Got NACK: {nack:?}"),
        }
    }

    pub fn ok(self) -> anyhow::Result<()> {
        match self {
            Gt5Response::Ack(_resp) => Ok(()),
            nack => anyhow::bail!("Got NACK: {nack:?}"),
        }
    }
}

impl From<Vec<u8>> for Gt5Response {
    fn from(resp_raw: Vec<u8>) -> Self {
        let mut resp_params = [0u8; 4];
        resp_params.copy_from_slice(&resp_raw[4..8]);
        let resp = u32::from_le_bytes(resp_params);

        if resp_raw[8] == 0x30 {
            return Gt5Response::Ack(resp);
        }

        match resp {
            0x1001 => Self::NackTimeout,
            0x1002 => Self::NackInvalidBaudrate,
            0x1003 => Self::NackInvalidPos,
            0x1004 => Self::NackIsNotUsed,
            0x1005 => Self::NackIsAlreadyUsed,
            0x1006 => Self::NackCommErr,
            0x1007 => Self::NackVerifyFailed,
            0x1008 => Self::NackIdentifyFailed,
            0x1009 => Self::NackDbIsFull,
            0x100A => Self::NackDbIsEmpty,
            0x100B => Self::NackTurnErr,
            0x100C => Self::NackBadFinger,
            0x100D => Self::NackEnrollFailed,
            0x100E => Self::NackIsNotSupported,
            0x100F => Self::NackDevErr,
            0x1010 => Self::NackCaptureCanceled,
            0x1011 => Self::NackInvalidParam,
            0x1012 => Self::NackFingerIsNotPressed,
            code => Self::Other(code),
        }
    }
}
