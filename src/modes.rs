use crate::{args::DisplayingMode, usb::SerialUsb};
use anyhow::Result;
use color_print::cprintln;
use indicatif::{ProgressIterator, ProgressStyle};
use rusb::{DeviceHandle, GlobalContext};
use std::fs::File;
use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

const WRITE_DELAY: Duration = Duration::from_millis(80);
const READ_DELAY: Duration = Duration::from_micros(1);

pub fn write(
    file: String,
    handle: &DeviceHandle<GlobalContext>,
    sigint: &AtomicBool,
) -> Result<()> {
    let mut file = File::open(file)?;
    let mut file_buffer = vec![];
    file.read_to_end(&mut file_buffer)?;
    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.green/red} {pos:>7}/{len:7} {msg}",
    )?;
    for buffer in file_buffer.chunks(7).progress_with_style(style) {
        if sigint.load(Ordering::Relaxed) {
            cprintln!("\n<red>Écriture interrompu par l'utilisateur</>");
            return Ok(());
        }
        handle.write_serial_usb(buffer)?;
        thread::sleep(WRITE_DELAY);
    }
    cprintln!("<green>{} bits ont été écris.</>", file_buffer.len());
    Ok(())
}

pub fn read(
    handle: &mut DeviceHandle<GlobalContext>,
    sigint: &AtomicBool,
    mode: DisplayingMode,
    saut: Option<u32>,
) -> Result<()> {
    let mut pos = 0;
    while !sigint.load(Ordering::Relaxed) {
        let mut buffer = [0xff; 8];
        handle.read_serial_usb(&mut buffer)?;
        mode.print(&buffer, saut, &mut pos);
        thread::sleep(READ_DELAY);
    }
    cprintln!("<red>Arrêt de lecture</>");
    Ok(())
}
