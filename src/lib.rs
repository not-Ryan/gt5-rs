mod biometrics_template;
mod commands;
mod data_packet;
mod errors;
mod gt5;
mod send_command;

pub async fn autodiscover() -> Result<GT5> {
    let device_info = nusb::list_devices()
        .context("Should list devices")?
        .find(|d| d.vendor_id() == 0x2009 && d.product_id() == 0x7638)
        .context("No device found")?;

    let device = device_info.open()?;

    let iface = match device.claim_interface(0x0) {
        Ok(iface) => iface,
        Err(err) => {
            println!("Could not claim device: {err}");
            println!("Retrying once...");

            device.reset()?;
            device.detach_kernel_driver(0x0)?;

            device
                .detach_and_claim_interface(0x0)
                .context("Claim device retry")?
        }
    };

    GT5::init(iface).await
}

use anyhow::{Context, Result};

pub use biometrics_template::BiometricsTemplate;
pub use errors::Gt5Response;
pub use gt5::GT5;
