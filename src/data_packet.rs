use super::{commands::Commands, send_command::ToPayload};

pub struct DataPacket(Vec<u8>);

impl DataPacket {
    const CODE1: u8 = 0x5a;
    const CODE2: u8 = 0xa5;

    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl ToPayload for DataPacket {
    fn to_payload(&self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![];
        payload.push(Self::CODE1);
        payload.push(Self::CODE2);
        payload.extend_from_slice(&Commands::DEVICE_ID);
        payload.extend_from_slice(&self.0);

        let crc = Commands::calculate_crc(&payload);
        payload.extend_from_slice(&crc);

        payload
    }
}
