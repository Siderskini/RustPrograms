use anyhow::{anyhow, Context, Result};
use hidapi::{HidApi, HidDevice};

use crate::rgb::{HsvColor, RgbEffect};

// Magi96 keyboard USB identifiers
const VENDOR_ID: u16 = 0x320F;
const PRODUCT_ID: u16 = 0x5088;

// VIA/QMK protocol constants
const VIA_COMMAND_RGBLIGHT_SETVALUE: u8 = 0x07;
const RGB_MATRIX_CHANNEL: u8 = 0x03;

// RGB matrix control IDs (from config JSON)
const RGB_MATRIX_BRIGHTNESS: u8 = 0x01;
const RGB_MATRIX_EFFECT: u8 = 0x02;
const RGB_MATRIX_EFFECT_SPEED: u8 = 0x03;
const RGB_MATRIX_COLOR: u8 = 0x04;

// HID report size for VIA protocol
const REPORT_SIZE: usize = 32;

pub struct Magi96Driver {
    device: HidDevice,
}

impl Magi96Driver {
    /// Open a connection to the Magi96 keyboard
    pub fn open() -> Result<Self> {
        let api = HidApi::new().context("Failed to initialize HID API")?;
        
        // Find the VIA/QMK Raw HID interface (usage page 0xFF60)
        let device_info = api.device_list()
            .find(|d| {
                d.vendor_id() == VENDOR_ID && 
                d.product_id() == PRODUCT_ID && 
                d.usage_page() == 0xFF60
            })
            .ok_or_else(|| anyhow!(
                "Failed to find VIA interface for Magi96 keyboard. Is it connected in 2.4GHz or wired mode?"
            ))?;
        
        let device = device_info.open_device(&api)
            .context("Failed to open VIA interface")?;

        Ok(Self { device })
    }

    /// Send a VIA command to the keyboard
    fn send_via_command(&self, channel: u8, id: u8, data: &[u8]) -> Result<()> {
        // Standard VIA format with 0x07
        let mut buf1 = [0u8; REPORT_SIZE];
        buf1[0] = 0x00;
        buf1[1] = VIA_COMMAND_RGBLIGHT_SETVALUE;
        buf1[2] = channel;
        buf1[3] = id;
        let data_len = data.len().min(REPORT_SIZE - 4);
        buf1[4..4 + data_len].copy_from_slice(&data[..data_len]);
        
        let bytes_written = self.device.write(&buf1)
            .context("Failed to write HID report")?;
        if bytes_written != buf1.len() {
            return Err(anyhow!(
                "Incomplete HID write: expected {} bytes, wrote {}",
                buf1.len(),
                bytes_written
            ));
        }

        Ok(())
    }

    /// Set RGB backlight brightness
    /// 
    /// # Arguments
    /// * `brightness` - Brightness level (0-9)
    pub fn set_brightness(&self, brightness: u8) -> Result<()> {
        if brightness > 9 {
            return Err(anyhow!("Brightness must be between 0 and 9"));
        }

        self.send_via_command(RGB_MATRIX_CHANNEL, RGB_MATRIX_BRIGHTNESS, &[brightness])
            .context("Failed to set brightness")?;

        Ok(())
    }

    /// Set RGB effect
    /// 
    /// # Arguments
    /// * `effect` - The RGB effect to apply
    pub fn set_effect(&self, effect: RgbEffect) -> Result<()> {
        let effect_value = effect as u8;
        self.send_via_command(RGB_MATRIX_CHANNEL, RGB_MATRIX_EFFECT, &[effect_value])
            .context("Failed to set effect")?;

        Ok(())
    }

    /// Set RGB effect speed
    /// 
    /// # Arguments
    /// * `speed` - Effect speed (0-4)
    pub fn set_effect_speed(&self, speed: u8) -> Result<()> {
        if speed > 4 {
            return Err(anyhow!("Speed must be between 0 and 4"));
        }

        self.send_via_command(RGB_MATRIX_CHANNEL, RGB_MATRIX_EFFECT_SPEED, &[speed])
            .context("Failed to set effect speed")?;

        Ok(())
    }

    /// Set RGB color (HSV)
    /// 
    /// # Arguments
    /// * `color` - HSV color to set
    pub fn set_color(&self, color: HsvColor) -> Result<()> {
        let data = [color.hue, color.saturation, color.value];
        self.send_via_command(RGB_MATRIX_CHANNEL, RGB_MATRIX_COLOR, &data)
            .context("Failed to set color")?;

        Ok(())
    }

    /// Get device information
    pub fn get_device_info(&self) -> Result<String> {
        let info = self.device.get_device_info().context("Failed to get device info")?;
        
        Ok(format!(
            "Manufacturer: {}\nProduct: {}\nSerial: {}",
            info.manufacturer_string().unwrap_or("Unknown"),
            info.product_string().unwrap_or("Unknown"),
            info.serial_number().unwrap_or("Unknown")
        ))
    }
}
