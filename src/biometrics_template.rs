use std::fmt::Display;

#[derive(Debug)]
pub struct BiometricsTemplate(pub [u8; 498]);

impl From<Vec<u8>> for BiometricsTemplate {
    fn from(value: Vec<u8>) -> Self {
        let mut data = [0u8; 498];
        data.copy_from_slice(&value[0..498]);

        Self(data)
    }
}

impl Display for BiometricsTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.0 {
            f.write_fmt(format_args!("{:02X?} ", byte))?;
        }

        Ok(())
    }
}
