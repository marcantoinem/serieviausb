#![feature(iter_array_chunks)]
use std::{
    fs::File,
    io::{BufReader, Read},
    sync::{atomic::Ordering, Arc},
    time::Duration,
};

use crate::usb::SerialUsb;
use anyhow::{Context, Result};
use args::{Args, DisplayingMode, SerialMode};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rusb::{DeviceHandle, GlobalContext};
use std::sync::atomic::AtomicBool;
use usb::{bits_from_buffer, find_device};

mod args;
mod usb;

fn write_mode(
    file: String,
    handle: &DeviceHandle<GlobalContext>,
    sigint_requested: &AtomicBool,
) -> Result<()> {
    let f = BufReader::new(File::open(file.clone())?);
    let len = f.bytes().count();
    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.green/red} {pos:>7}/{len:7} {msg}",
    )?;
    let progress_bar = ProgressBar::new(len as u64);
    progress_bar.set_style(style);
    let f = File::open(file.clone())?;
    for buffer in f.bytes().filter_map(|x| x.ok()).array_chunks::<8>() {
        if sigint_requested.load(Ordering::Relaxed) {
            println!("Écriture interrompu par l'utilisateur");
            break;
        }
        handle.write_serial_usb(&buffer)?;
        std::thread::sleep(Duration::from_millis(5));
        progress_bar.inc(8);
    }
    progress_bar.finish();
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
        mode.print(bits_from_buffer(&buffer));
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
        SerialMode::Ecriture { fichier } => write_mode(fichier, &mut handle, &sigint_requested)?,
        SerialMode::Lecture => read_mode(&mut handle, &sigint_requested, args.affichage)?,
    }
    Ok(())
}
