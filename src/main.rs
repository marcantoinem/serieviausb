#![feature(iter_array_chunks)]
use anyhow::{Context, Result};
use args::{Args, DisplayingMode, SerialMode};
use clap::Parser;
use indicatif::{ProgressIterator, ProgressStyle};
use rusb::{DeviceHandle, GlobalContext};
use std::sync::atomic::AtomicBool;
use std::{
    fs::File,
    io::{BufReader, Read},
    sync::{atomic::Ordering, Arc},
    time::Duration,
};
use usb::{find_device, SerialUsb};

mod args;
mod usb;

fn write_mode(
    file: String,
    handle: &DeviceHandle<GlobalContext>,
    sigint_requested: &AtomicBool,
) -> Result<()> {
    let mut f = BufReader::new(File::open(file)?);
    let mut file_buffer = vec![];
    f.read_to_end(&mut file_buffer)?;
    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.green/red} {pos:>7}/{len:7} {msg}",
    )?;
    for buffer in file_buffer.chunks(8).progress_with_style(style) {
        if sigint_requested.load(Ordering::Relaxed) {
            println!("Écriture interrompu par l'utilisateur");
            return Ok(());
        }
        handle.write_serial_usb(buffer)?;
        std::thread::sleep(Duration::from_millis(5));
    }
    Ok(())
}

fn read_mode(
    handle: &mut DeviceHandle<GlobalContext>,
    sigint_requested: &AtomicBool,
    mode: DisplayingMode,
) -> Result<()> {
    while !sigint_requested.load(Ordering::Relaxed) {
        let mut buffer = [0xFF; 8];
        // Read mode.
        handle.read_serial_usb(&mut buffer)?;
        mode.print(&buffer);
        std::thread::sleep(Duration::from_millis(20));
    }
    println!("\n-Arrêt de lecture-");
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let sigint_requested = Arc::new(AtomicBool::new(false));
    let sigint_handler = Arc::clone(&sigint_requested);
    ctrlc::set_handler(move || {
        sigint_handler.store(true, Ordering::Relaxed);
    })
    .expect("L'initialisation de Ctrl+C a échoué.");
    let device = find_device().context("Device not found")?;
    let mut handle = device.open()?;
    handle.init_serial_usb()?;
    match args.mode {
        SerialMode::Ecriture { fichier } => write_mode(fichier, &handle, &sigint_requested)?,
        SerialMode::Lecture => read_mode(&mut handle, &sigint_requested, args.affichage)?,
    }
    Ok(())
}
