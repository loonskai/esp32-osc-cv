use esp_idf_hal::spi::{SpiDeviceDriver, SpiDriver};

pub struct Mcp4822<'d> {
    spi: SpiDeviceDriver<'d, SpiDriver<'d>>,
}

impl<'d> Mcp4822<'d> {
    pub fn new(spi: SpiDeviceDriver<'d, SpiDriver<'d>>) -> Self {
        Self { spi }
    }

    pub fn write(&mut self, channel: u8, value: u16) {
        // 16=bit command:
        // bit 15: channel (0=A, 1=B)
        // bit 14: ignored
        // bit 13: gain (0=2x, 1=1x)
        // bit 12: shutdown (1=active)
        // bits 11-0: 12-bit value
        let cmd: u16 = ((channel as u16) << 15)
            | (0 << 13) // 2x gain -> max 4.096V
            | (1 << 12) // active
            | (value & 0x0FFF);
        
        let bytes = cmd.to_be_bytes();
        self.spi.write(&bytes).unwrap();
    }

    // convert 0.0-1.0 float to 12-bit value and write
    pub fn write_float(&mut self, channel: u8, value: f32) {
        let clamped = value.clamp(0.0, 1.0);
        let raw = (clamped * 4095.0) as u16;
        self.write(channel, raw);
    }
}
