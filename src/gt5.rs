use anyhow::{ensure, Ok, Result};
use nusb::Interface;
use std::time::Duration;

use crate::biometrics_template::BiometricsTemplate;
use crate::commands::Commands;
use crate::data_packet::DataPacket;
use crate::send_command::{receive_data, send_data};

pub struct GT5 {
    iface: Interface,
}

impl GT5 {
    pub async fn init(iface: Interface) -> Result<Self> {
        send_data(&iface, Commands::Open).await?.ok()?;

        Ok(Self { iface })
    }

    pub async fn set_led(&self, to: bool) -> Result<()> {
        send_data(&self.iface, Commands::CmosLed(to)).await?.ok()
    }

    pub async fn is_pressed(&self) -> Result<bool> {
        let resp = send_data(&self.iface, Commands::IsPressFinger)
            .await?
            .into_result()?;

        Ok(resp == 0)
    }

    pub async fn delete_all(&self) -> Result<()> {
        send_data(&self.iface, Commands::DeleteAll).await?.ok()
    }

    pub async fn delete_one(&self, user_id: u32) -> Result<()> {
        ensure!(user_id < 2999, "User id must not be above 2999");

        send_data(&self.iface, Commands::DeleteID(user_id))
            .await?
            .ok()
    }

    pub async fn get_enroll_count(&self) -> Result<u32> {
        send_data(&self.iface, Commands::GetEnrolCount)
            .await?
            .into_result()
    }

    // TODO: Check if this works
    // Since NACK means its not enrolled. This will probably fail.
    pub async fn check_enrolled(&self, user_id: u32) -> Result<bool> {
        let resp = send_data(&self.iface, Commands::CheckEnrolled(user_id))
            .await?
            .into_result()?;
        Ok(resp == 0)
    }

    pub async fn enroll_start(&self, user_id: u32) -> Result<()> {
        ensure!(user_id < 2999, "User id must not be above 2999");
        send_data(&self.iface, Commands::EnrollStart(user_id as i32))
            .await?
            .ok()
    }

    /// Will capture the finger.
    /// Keep in mind that the finger should be pressed for this to succeed
    pub async fn capture_finger(&self, best: bool) -> Result<()> {
        send_data(&self.iface, Commands::CaptureFinger(best))
            .await?
            .ok()
    }

    pub async fn enroll_start_no_save(&self) -> Result<()> {
        send_data(&self.iface, Commands::EnrollStart(-1))
            .await?
            .ok()
    }

    /// Will do an enroll step.
    /// Capture finger must have been done before this step.
    pub async fn enroll_x(&self, phase: u8) -> Result<()> {
        let cmd = match phase {
            1 => Commands::Enroll1,
            2 => Commands::Enroll2,
            3 => Commands::Enroll3,
            _ => anyhow::bail!("Phase must be 1, 2 or 3"),
        };

        send_data(&self.iface, cmd).await?.ok()
    }

    pub async fn wait_pressed(&self, to: bool) -> Result<()> {
        while self.is_pressed().await? != to {
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        Ok(())
    }

    pub async fn make_template(&self) -> Result<BiometricsTemplate> {
        send_data(&self.iface, Commands::MakeTemplate).await?.ok()?;

        let resp = receive_data(&self.iface, 498).await?;
        Ok(BiometricsTemplate::from(resp))
    }

    pub async fn get_template(&self, user_id: u32) -> Result<BiometricsTemplate> {
        ensure!(user_id < 2999, "User id must not be above 2999");

        send_data(&self.iface, Commands::GetTemplate(user_id))
            .await?
            .ok()?;

        let resp = receive_data(&self.iface, 498).await?;
        Ok(BiometricsTemplate::from(resp))
    }

    pub async fn set_template(&self, user_id: u32, template: BiometricsTemplate) -> Result<()> {
        ensure!(user_id < 2999, "User id must not be above 2999");

        send_data(&self.iface, Commands::SetTemplate(user_id))
            .await?
            .ok()?;

        let packet = DataPacket::new(template.0.to_vec());
        send_data(&self.iface, packet).await?.ok()
    }

    pub async fn identify_user(&self) -> Result<u32> {
        send_data(&self.iface, Commands::Identify)
            .await?
            .into_result()
    }
}
