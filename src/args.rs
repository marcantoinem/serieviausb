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
    long_about = "Programme permettant de recevoir et d'envoyer des octets de facon
     sérielle mais indirectement via le cable USB pour échange avec la carte microcontrôleur 
     du cours inf1900. Ce programme est fortement inspiré de serieViaUSB écrit par Matthew 
     Khouzam, Jérome Collin, Michaël Ferris et Mathieu Marengère-Gosselin."
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

    pub fichier: Option<String>,

    /// Effectue un retour à la ligne à chaque n caractère.
    #[arg(short, long)]
    pub retour: Option<u32>,
}

pub fn bits_from_buffer(bytes: &[u8; PACKET_SIZE as usize]) -> &[u8] {
    let buffer_size = bytes[0] as usize;
    &bytes[1..(buffer_size + 1)]
}

fn print_saut(pos: &mut u32, saut: Option<u32>) {
    *pos += 1;
    if let Some(saut) = saut {
        if saut == *pos {
            println!();
            *pos = 0;
        }
    }
}

impl DisplayingMode {
    pub fn print(&self, buffer: &[u8; PACKET_SIZE as usize], saut: Option<u32>, pos: &mut u32) {
        let bytes = bits_from_buffer(buffer);
        match self {
            DisplayingMode::Binaire => {
                bytes.iter().for_each(|byte| {
                    print!("{byte:b}");
                    print_saut(pos, saut);
                });
            }
            DisplayingMode::Decimal => {
                bytes.iter().for_each(|byte| {
                    print!("{byte}");
                    print_saut(pos, saut)
                });
            }
            DisplayingMode::Hexadecimal => {
                bytes.iter().for_each(|byte| {
                    print!("{byte:X}");
                    print_saut(pos, saut)
                });
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
                    .for_each(|byte| {
                        print!("{}", *byte as char);
                        print_saut(pos, saut)
                    });
            }
        }
    }
}
