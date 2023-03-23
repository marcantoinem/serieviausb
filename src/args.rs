use clap::{ArgGroup, Parser, ValueEnum};
use derive_more::Display;

use crate::usb::PACKET_SIZE;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Display)]
pub enum DisplayingMode {
    #[display(fmt = "hexadecimal")]
    Hexadecimal,
    #[display(fmt = "decimal")]
    Decimal,
    #[display(fmt = "binaire")]
    Binaire,
    #[display(fmt = "ascii")]
    Ascii,
}

#[derive(Parser, Debug)]
#[command(
    author = "Marc-Antoine Manningham, Charles Khouzry",
    version = "0.1",
    about = "Permet de lire et d'écrire sur le robot de INF1900 par USB en série.",
    long_about = "Programme permettant de recevoir et d'envoyer des octets de facon sérielle mais indirectement via le cable USB pour échange avec la carte microcontroleur du cours inf1900. Ce programme est fortement inspiré de serieViaUSB écrit par Matthew Khouzam, Jérome Collin, Michaël Ferris et Mathieu Marengère-Gosselin."
)]
#[clap(group(
            ArgGroup::new("Mode")
                .required(true)
                .args(&["lecture", "ecriture"])))
]
pub struct Args {
    /// Pour réception des données en provenance de la carte
    #[arg(short, long)]
    pub lecture: bool,

    /// Pour envoyer des données vers la la carte. Cette option demande l'utilisation de l'option -f
    #[arg(short, long)]
    pub ecriture: bool,

    /// Pour recevoir l'image svg du robot pour le projet final
    #[arg(short, long)]
    pub svg: bool,

    /// Afficher les octets envoyés ou reçus dans une représentation spécifique.
    #[arg(short, long, default_value_t = DisplayingMode::Ascii)]
    pub affichage: DisplayingMode,

    /// Pour enregistrer dans un fichier de sortie ce qui a été reçu en entrée.
    #[arg(short, long)]
    pub output: Option<String>,

    pub fichier: Option<String>,
    // /// Effectue un retour à la ligne à chaque n caractère.
    // #[arg(short, long)]
    // pub saut: Option<u32>,
}

pub fn bits_from_buffer(bytes: &[u8; PACKET_SIZE as usize]) -> &[u8] {
    let buffer_size = bytes[0] as usize;
    &bytes[1..(buffer_size + 1)]
}

impl DisplayingMode {
    pub fn print(&self, buffer: &[u8; PACKET_SIZE as usize]) {
        let bytes = bits_from_buffer(buffer);
        match self {
            DisplayingMode::Binaire => {
                bytes.iter().for_each(|byte| print!("{byte:b}"));
            }
            DisplayingMode::Decimal => {
                bytes.iter().for_each(|byte| print!("{byte}"));
            }
            DisplayingMode::Hexadecimal => {
                bytes.iter().for_each(|byte| print!("{byte:X}"));
            }
            DisplayingMode::Ascii => {
                bytes
                    .iter()
                    .filter(|x| {
                        x.is_ascii_alphanumeric()
                            | x.is_ascii_control()
                            | x.is_ascii_whitespace()
                            | x.is_ascii_graphic()
                    })
                    .for_each(|byte| print!("{}", *byte as char));
            }
        }
    }
}
