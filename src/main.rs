#![allow(unused)]
use std::{
    fs::File,
    io::BufReader,
    sync::{atomic::Ordering, Arc},
    time::Duration,
};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use derive_more::Display;
use indicatif::{ProgressIterator, ProgressStyle};
use rusb::{Device, DeviceDescriptor, DeviceHandle, GlobalContext};
use std::sync::atomic::AtomicBool;

// Identifiant de la carte de INF1900
const VENDOR_ID: u16 = 0x16c0;
const PRODUCT_ID: u16 = 0x05dc;

const USB_TYPE_VENDOR: u8 = 0x02 << 5;
const USB_RECIP_DEVICE: u8 = 0;
const REQUEST_READ: u8 = USB_TYPE_VENDOR | USB_RECIP_DEVICE | (1 << 7);
const REQUEST_WRITE: u8 = USB_TYPE_VENDOR | USB_RECIP_DEVICE | (0 << 7);

const USBASP_FUNC_SETSERIOS: u8 = 11;
const USBASP_FUNC_READSER: u8 = 12;
const USBASP_FUNC_WRITESER: u8 = 13;

const USBASP_MODE_PARITYN: u16 = 1;

const USBASP_MODE_SETBAUD2400: u16 = 0x13;
const BAUDS_RATE: u16 = USBASP_MODE_SETBAUD2400;
const PACKET_BITS: u16 = 8;
const STOP_BITS: u16 = 1;

#[derive(Debug, Subcommand)]
enum SerialMode {
    /// Pour envoyer des données vers la la carte. Cette option demande l'utilisation de l'option -f
    #[command(arg_required_else_help = true)]
    Ecriture {
        #[arg(short, long)]
        fichier: String,
    },
    /// Pour réception des données en provenance de la carte
    Lecture,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Display)]
enum DisplayingMode {
    #[display(fmt = "hexadecimal")]
    Hexadecimal,
    #[display(fmt = "decimal")]
    Decimal,
    #[display(fmt = "binaire")]
    Binaire,
}

#[derive(Parser, Debug)]
#[command(
    author = "Marc-Antoine Manningham, Charles Khouzry",
    version = "0.1",
    about = "Permet de lire et d'écrire sur le robot de INF1900 par USB en série.",
    long_about = "Programme permettant de recevoir et d'envoyer des octets de facon sérielle mais indirectement via le cable USB pour échange avec la carte microcontroleur du cours inf1900. Ce programme est fortement inspiré de serieViaUSB écrit par Matthew Khouzam, Jerome Collin, Michaël Ferris et Mathieu Marengère-Gosselin."
)]
struct Args {
    #[command(subcommand)]
    mode: SerialMode,
    /// Terminer le programme directement apres le transfert de n octets. Sans cette option, lit ou écrit indéfiniment.
    #[arg(short, long)]
    nb_bytes: Option<u32>,
    /// Afficher les octets envoyés ou reçus dans une représentation spécifique.
    #[arg(short, long, default_value_t = DisplayingMode::Hexadecimal)]
    affichage: DisplayingMode,
    /// Effectue un retour à la ligne à chaque n caractère.
    #[arg(short, long)]
    saut: Option<u32>,
}

fn get_device_descriptor(
    device: Device<GlobalContext>,
) -> Option<(DeviceDescriptor, Device<GlobalContext>)> {
    Some((device.device_descriptor().ok()?, device))
}

fn is_device_corresponding(device: &(DeviceDescriptor, Device<GlobalContext>)) -> bool {
    device.0.vendor_id() == VENDOR_ID && device.0.product_id() == PRODUCT_ID
}

fn find_device() -> Option<(DeviceDescriptor, Device<GlobalContext>)> {
    rusb::devices()
        .ok()?
        .iter()
        .filter_map(get_device_descriptor)
        .find(is_device_corresponding)
}

fn init_serial_usb(handle: &DeviceHandle<GlobalContext>) -> Result<()> {
    let mut buffer = [0; 4];
    let cmd = [
        BAUDS_RATE as u8,
        PACKET_BITS as u8,
        USBASP_MODE_PARITYN as u8,
        0,
    ];
    // Error with negative integer are handled by rusb
    let nb_bytes: usize = handle.read_control(
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

fn read_serial_usb(handle: &DeviceHandle<GlobalContext>, buffer: &mut [u8; 8]) -> Result<()> {
    handle.read_control(
        REQUEST_READ,
        USBASP_FUNC_READSER,
        0,
        0,
        buffer,
        Duration::from_secs(5),
    )?;

    Ok(())
}

fn write_serial_usb(handle: &DeviceHandle<GlobalContext>, buffer: &[u8; 8]) -> Result<()> {
    handle.write_control(
        REQUEST_WRITE,
        USBASP_FUNC_WRITESER,
        0,
        0,
        buffer,
        Duration::from_secs(5),
    )?;

    Ok(())
}

fn bits_from_buffer(bytes: &[u8; 8]) -> &[u8] {
    let buffer_size = bytes[0] as usize;
    &bytes[1..(buffer_size + 1)]
}

fn write_serial(
    file: String,
    handle: &DeviceHandle<GlobalContext>,
    sigint_requested: &AtomicBool,
) -> Result<()> {
    println!("Not implemented yet!");
    let f = File::open(file)?;
    let reader = BufReader::new(f);
    for i in reader.buffer().iter().progress() {}
    while !sigint_requested.load(Ordering::Relaxed) {}
    Ok(())
}

fn read_serial(
    handle: &mut DeviceHandle<GlobalContext>,
    sigint_requested: &AtomicBool,
) -> Result<()> {
    while !sigint_requested.load(Ordering::Relaxed) {
        let mut buffer = [0xFF; 8];
        // Read mode.
        read_serial_usb(handle, &mut buffer)?;
        for bits in bits_from_buffer(&buffer) {
            print!("{}", *bits as char);
        }
        std::thread::sleep(Duration::from_millis(1));
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
    // let style = ProgressStyle::with_template(
    //     "[{elapsed_precise}] {bar:40.green/red} {pos:>7}/{len:7} {msg}",
    // )?
    // .progress_chars("#-");
    // for i in (0..100).into_iter().progress_with_style(style) {
    //     std::thread::sleep(Duration::from_millis(50));
    // }
    let (device_descriptor, device) = find_device().context("Device not found")?;
    let mut handle = device.open()?;
    init_serial_usb(&handle)?;
    match args.mode {
        SerialMode::Ecriture { fichier } => write_serial(fichier, &mut handle, &sigint_requested)?,
        SerialMode::Lecture => read_serial(&mut handle, &sigint_requested)?,
    }
    Ok(())
}
