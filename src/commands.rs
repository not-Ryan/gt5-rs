use super::send_command::ToPayload;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Commands {
    /// Initialization
    Open,
    /// Termination
    Close,
    /// Check if the connected USB device is valid
    UsbInternalCheck,
    /// Change UART baud rate
    ChangeBaudrate,
    /// Control CMOS LED
    CmosLed(bool),
    /// Get enrolled fingerprint count
    GetEnrolCount,
    /// Check whether the specified ID is already enrolled
    CheckEnrolled(u32),
    /// Start an enrollment
    EnrollStart(i32),
    /// Make 1 template for an enrollment
    Enroll1,
    /// Make 2 template for an enrollment
    Enroll2,
    /// Make 3=template for an enrollment, merge three templates into one template, save merged template to the database
    Enroll3,
    /// Check if a finger is placed on the sensor
    IsPressFinger,
    /// Delete the fingerprint with the specified ID
    DeleteID(u32),
    /// Delete all fingerprints from the database
    DeleteAll,
    /// 1:1 Verification of the capture fingerprint image with the specified ID
    Verify,
    /// 1:N Identification of the capture fingerprint image with the database
    Identify,
    /// 1:1 Verification of a fingerprint template with the specified ID
    VerifyTemplate,
    /// 1:N Identification of a fingerprint template with the database
    IdentifyTemplate,
    /// Capture a fingerprint image (256x256) from the sensor
    ///
    /// The fingerprint algorithm uses 450dpi 256x256 image for its input.
    /// This command captures raw image from the sensor and converts it to 256x256 image for the fingerprint algorithm. If the finger is not pressed, this command returns with non-acknowledge.
    ///
    /// Note: Please use
    /// - `true` / best image for enrollment to get best enrollment data.
    /// - `false` / not best image for identification (verification) to get fast user sensibility.
    ///
    CaptureFinger(bool),
    /// Make template for transmission
    MakeTemplate,
    /// Download the captured fingerprint image (256x256)
    GetImage,
    /// Capture & Download raw fingerprint image (320×240)
    GetRawImage,
    /// Download the template of the specified ID
    GetTemplate(u32),
    /// Upload the template of the specified ID
    SetTemplate(u32),
    /// Start database download, obsolete
    GetDatabaseStart,
    /// End database download, obsolete
    GetDatabaseEnd,
    /// Set Security Level
    SetSecurityLevel,
    /// Get Security Level
    GetSecurityLevel,
    /// Identify the captured fingerprint image with the specified template
    IdentifyTemplate2,
    /// Enter Standby Mode (Low power mode)
    EnterStandbyMode,
    /// Acknowledge
    Ack,
    /// Non-acknowledge
    Nack,
}

impl Commands {
    const CODE1: u8 = 0x55;
    const CODE2: u8 = 0xaa;
    pub(super) const DEVICE_ID: [u8; 2] = 0x01u16.to_le_bytes();

    pub(super) fn calculate_crc(payload: &[u8]) -> [u8; 2] {
        let mut crc = 0u16;
        for byte in payload.iter() {
            crc = crc.wrapping_add(*byte as u16);
        }

        crc.to_le_bytes()
    }

    fn params_to_le_bytes(self) -> [u8; 4] {
        match self {
            // bool
            Self::CmosLed(val) | Commands::CaptureFinger(val) => [val.into(), 0, 0, 0],

            // u32
            Self::DeleteID(uid)
            | Self::CheckEnrolled(uid)
            | Self::GetTemplate(uid)
            | Self::SetTemplate(uid) => uid.to_le_bytes(),

            // i32
            Self::EnrollStart(uid) => uid.to_le_bytes(),

            _no_params => [0u8; 4],
        }
    }

    fn code_to_le_bytes(self) -> [u8; 2] {
        let nr: u16 = match self {
            Self::Open => 0x01,
            Self::Close => 0x02,
            Self::UsbInternalCheck => 0x03,
            Self::ChangeBaudrate => 0x04,
            Self::CmosLed(..) => 0x12,
            Self::GetEnrolCount => 0x20,
            Self::CheckEnrolled(..) => 0x21,
            Self::EnrollStart(..) => 0x22,
            Self::Enroll1 => 0x23,
            Self::Enroll2 => 0x24,
            Self::Enroll3 => 0x25,
            Self::IsPressFinger => 0x26,
            Self::DeleteID(..) => 0x40,
            Self::DeleteAll => 0x41,
            Self::Verify => 0x50,
            Self::Identify => 0x51,
            Self::VerifyTemplate => 0x52,
            Self::IdentifyTemplate => 0x53,
            Self::CaptureFinger(..) => 0x60,
            Self::MakeTemplate => 0x61,
            Self::GetImage => 0x62,
            Self::GetRawImage => 0x63,
            Self::GetTemplate(..) => 0x70,
            Self::SetTemplate(..) => 0x71,
            Self::GetDatabaseStart => 0x72,
            Self::GetDatabaseEnd => 0x73,
            Self::SetSecurityLevel => 0xF0,
            Self::GetSecurityLevel => 0xF1,
            Self::IdentifyTemplate2 => 0xF4,
            Self::EnterStandbyMode => 0xF9,
            Self::Ack => 0x30,
            Self::Nack => 0x31,
        };

        nr.to_le_bytes()
    }
}

impl ToPayload for Commands {
    fn to_payload(&self) -> Vec<u8> {
        let cmd_code = self.code_to_le_bytes();
        let cmd_params = self.params_to_le_bytes();

        let mut payload: [u8; 12] = [
            Self::CODE1,
            Self::CODE2,
            Self::DEVICE_ID[0],
            Self::DEVICE_ID[1],
            cmd_params[0],
            cmd_params[1],
            cmd_params[2],
            cmd_params[3],
            cmd_code[0],
            cmd_code[1],
            0,
            0,
        ];

        let crc = Self::calculate_crc(&payload[..10]);
        payload[10..].copy_from_slice(&crc);

        payload.to_vec()
    }
}
