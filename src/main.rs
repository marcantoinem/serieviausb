use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use rusb::{Device, DeviceDescriptor, DeviceHandle, GlobalContext};

// Identifiant de la carte de INF1900
const VENDOR_ID: u16 = 0x16c0;
const PRODUCT_ID: u16 = 0x05dc;

const USB_TYPE_VENDOR: u8 = 0x02 << 5;
const USB_RECIP_DEVICE: u8 = 0;
const REQUEST_TYPE: u8 = USB_TYPE_VENDOR | USB_RECIP_DEVICE | (1 << 7);

const USBASP_FUNC_SETSERIOS: u8 = 11;
const USBASP_FUNC_READSER: u8 = 12;
const USBASP_FUNC_WRITESER: u8 = 13;

const USBASP_MODE_PARITYN: u16 = 1;

const BAUDS_RATE: u16 = 2400;
const PACKET_BITS: u16 = 8;
const STOP_BITS: u16 = 1;

#[derive(Parser, Debug)]
#[command(
    author = "Marc-Antoine Manningham, Charles Khouzry",
    version = "0.1",
    about = "Permet de lire et d'écrire sur le robot de INF1900 par USB en série.",
    long_about = "Programme permettant de recevoir et d'envoyer des octets de facon sérielle mais indirectement via le cable USB pour échange avec la carte microcontroleur du cours inf1900. Ce programme est fortement inspiré de serieViaUSB écrit par Matthew Khouzam, Jerome Collin, Michaël Ferris et Mathieu Marengère-Gosselin."
)]
struct Args {
    /// Pour envoyer des données vers la la carte. Cette option demande l'utilisation de l'option -f
    #[arg(short, long)]
    ecriture: bool,
    /// Pour réception des données en provenance de la carte
    #[arg(short, long)]
    lecture: bool,
    /// Terminer le programme directement apres le transfert de n octets.  Sans cette option, lit ou ecrit indefiniment.
    #[arg(short, long)]
    nb_bytes: Option<u32>,
    /// Prendre les donnees a envoyer vers la carte dans le fichier specifie (implique l'option -e) ou ecrire les donnees dans le fichier lorsqu'elles proviennent de la carte (implique l'option -l). stdout est utilise avec l'option -e si l'option -f n'est pas utilisee (cas par defaut).
    #[arg(short, long)]
    fichier: Option<String>,
    /// Afficher les octets envoyés ou reçus dans une représentation hexadécimale.
    #[arg(long)]
    hexadecimal: bool,
    /// Afficher les octets envoyés ou reçus dans une représentation décimale.
    #[arg(short, long)]
    decimal: bool,
    /// Afficher les octets envoyés ou reçus dans une représentation binaire.
    #[arg(short, long)]
    binaire: bool,
    /// Effectue un retour à la ligne à chaque n caractère.
    #[arg(short, long)]
    saut: Option<u32>,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
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
    let mut buf = [0; 4];
    let cmd = [BAUDS_RATE as u8, PACKET_BITS as u8, 0, 0];
    // Error with negative integer are handled by rusb
    let nb_bytes: usize = handle.read_control(
        REQUEST_TYPE,
        USBASP_FUNC_SETSERIOS,
        (PACKET_BITS << 8) | BAUDS_RATE,
        USBASP_MODE_PARITYN,
        &mut buf,
        Duration::from_secs(5),
    )?;

    (cmd != buf && nb_bytes == 4)
        .then_some(())
        .context("Failed to set serial parameters")?;
    Ok(())
}

fn read_serial_usb(handle: &DeviceHandle<GlobalContext>, buffer: &mut [u8; 8]) -> Result<()> {
    handle.read_control(
        REQUEST_TYPE,
        USBASP_FUNC_READSER,
        0,
        0,
        buffer,
        Duration::from_secs(5),
    )?;
    Ok(())
}

fn write_serial_usb(handle: &DeviceHandle<GlobalContext>, buffer: &mut [u8; 8]) {}

fn bits_from_buffer(bytes: &[u8; 8]) -> &[u8] {
    let buffer_size = bytes[0] as usize;
    &bytes[1..(buffer_size + 1)]
}

fn main() -> Result<()> {
    let _args = Args::parse();
    let (device_descriptor, device) = find_device().context("Device not found")?;
    let handle = device.open()?;
    init_serial_usb(&handle)?;
    loop {
        let mut buffer = [0xFF; 8];
        // Read mode.
        read_serial_usb(&handle, &mut buffer)?;
        println!("{:?}", bits_from_buffer(&buffer));
    }
}

// Example of write
fn _main() -> Result<()> {
    let (device_descriptor, device) = find_device().context("Device not found")?;
    let handle = device.open()?;
    init_serial_usb(&handle)?;
    let mut buffer = [0; 8];
    loop {
        let mut buffer = [0xFF; 8];
        // Write mode.
    }
}
