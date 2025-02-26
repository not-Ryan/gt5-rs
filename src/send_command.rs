use anyhow::{Context, Ok, Result};
use nusb::{Interface, transfer::RequestBuffer};
use rand::random;
use scsi::{
    BufferPushable,
    scsi::commands::{CommandBlockWrapper, Direction},
};

use crate::commands::Commands;
use crate::errors::Gt5Response;

pub async fn send_data(interface: &Interface, data: impl ToPayload) -> Result<Gt5Response> {
    let payload = data.to_payload();

    interface
        .bulk_out(0x02, get_wrapper(Direction::OUT, payload.len()).to_vec())
        .await
        .into_result()
        .context("send receive wrapper")?;

    interface
        .bulk_out(0x02, payload)
        .await
        .into_result()
        .context("send command response")?;

    interface
        .bulk_in(0x81, RequestBuffer::new(13))
        .await
        .into_result()
        .context("consume command wrapper response")?;

    {
        interface
            .bulk_out(0x02, get_wrapper(Direction::IN, 12))
            .await
            .into_result()
            .context("send receive wrapper")?;

        // The first BULK_IN response is our data.
        let resp = interface
            .bulk_in(0x81, RequestBuffer::new(512))
            .await
            .into_result()
            .context("consume receive response")?;

        // The second has the `TAG` and if the first response was actually valid...
        interface
            .bulk_in(0x81, RequestBuffer::new(512))
            .await
            .into_result()
            .context("consume receive wrapper response")?;

        Ok(Gt5Response::from(resp))
    }
}

pub async fn receive_data(interface: &Interface, len: usize) -> Result<Vec<u8>> {
    interface
        .bulk_out(0x02, get_wrapper(Direction::IN, len))
        .await
        .into_result()
        .context("send receive wrapper")?;

    // The first BULK_IN response is our data.
    let resp = interface
        .bulk_in(0x81, RequestBuffer::new(len + 128))
        .await
        .into_result()
        .context("consume receive response")?;

    // The second has the `TAG` and if the first response was actually valid...
    interface
        .bulk_in(0x81, RequestBuffer::new(32))
        .await
        .into_result()
        .context("consume receive wrapper response")?;

    {
        let ours = Commands::calculate_crc(&resp[..4 + len]);

        let mut theirs = [0u8; 2];
        theirs.copy_from_slice(&resp[4 + len..]);

        anyhow::ensure!(
            ours == theirs,
            "CRC failed (ours != theirs) {ours:?} != {theirs:?}"
        );
    }

    let payload = &resp[4..4 + len];
    Ok(payload.to_vec())
}

fn get_wrapper(dir: Direction, buff_length: usize) -> Vec<u8> {
    let mut buff = [0x0; 31];
    let mut wrapper = CommandBlockWrapper::new(buff_length as u32, dir, 0, 10);

    wrapper.tag = random();
    let isi = wrapper
        .push_to_buffer(&mut buff)
        .expect("Could not push command wrapper to buffer");

    buff[isi + 0] = 0xef;
    buff[isi + 1] = match dir {
        Direction::IN => 0xff,
        Direction::OUT => 0xfe,
        Direction::NONE => panic!("None direction used..."),
    };

    buff.to_vec()
}

pub trait ToPayload {
    fn to_payload(&self) -> Vec<u8>;
}
