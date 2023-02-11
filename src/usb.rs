use std::time::Duration;

use anyhow::{Context, Result};
use rusb::{Device, DeviceHandle, GlobalContext};

// Identifiant de la carte de INF1900
const VENDOR_ID: u16 = 0x16c0;
const PRODUCT_ID: u16 = 0x05dc;

const USB_TYPE_VENDOR: u8 = 0x02 << 5;
const REQUEST_READ: u8 = USB_TYPE_VENDOR | (1 << 7);
const REQUEST_WRITE: u8 = USB_TYPE_VENDOR | (0 << 7);

const USBASP_FUNC_SETSERIOS: u8 = 11;
const USBASP_FUNC_READSER: u8 = 12;
const USBASP_FUNC_WRITESER: u8 = 13;

const USBASP_MODE_PARITYN: u16 = 1;

const USBASP_MODE_SETBAUD2400: u16 = 0x13;
const BAUDS_RATE: u16 = USBASP_MODE_SETBAUD2400;
const PACKET_BITS: u16 = 8;

fn is_device_corresponding(device: Device<GlobalContext>) -> Option<Device<GlobalContext>> {
    let device_descriptor = device.device_descriptor().ok()?;
    (device_descriptor.vendor_id() == VENDOR_ID && device_descriptor.product_id() == PRODUCT_ID)
        .then_some(device)
}

pub fn find_device() -> Option<Device<GlobalContext>> {
    rusb::devices()
        .ok()?
        .iter()
        .find_map(is_device_corresponding)
}

pub trait SerialUsb {
    fn init_serial_usb(&self) -> Result<()>;
    fn read_serial_usb(&self, buffer: &mut [u8; 8]) -> Result<()>;
    fn write_serial_usb(&self, buffer: &[u8]) -> Result<()>;
}

impl SerialUsb for DeviceHandle<GlobalContext> {
    fn init_serial_usb(&self) -> Result<()> {
        let mut buffer = [0; 4];
        let cmd = [
            BAUDS_RATE as u8,
            PACKET_BITS as u8,
            USBASP_MODE_PARITYN as u8,
            0,
        ];
        // Error with negative integer are handled by rusb
        let nb_bytes: usize = self.read_control(
            REQUEST_READ,
            USBASP_FUNC_SETSERIOS,
            (PACKET_BITS << 8) | BAUDS_RATE,
            USBASP_MODE_PARITYN,
            &mut buffer,
            Duration::from_secs(5),
        )?;
        (cmd == buffer && nb_bytes == 4)
            .then_some(())
            .context("Failed to set serial parameters")
    }

    fn read_serial_usb(&self, buffer: &mut [u8; 8]) -> Result<()> {
        self.read_control(
            REQUEST_READ,
            USBASP_FUNC_READSER,
            0,
            0,
            buffer,
            Duration::from_secs(5),
        )?;

        Ok(())
    }

    fn write_serial_usb(&self, buffer: &[u8]) -> Result<()> {
        self.write_control(
            REQUEST_WRITE,
            USBASP_FUNC_WRITESER,
            0,
            0,
            buffer,
            Duration::from_secs(5),
        )?;

        Ok(())
    }
}
