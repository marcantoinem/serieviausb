use std::fmt::Display;

use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum DisplayingMode {
    Hexadecimal,
    Decimal,
    Binary,
    Ascii,
}

impl Display for DisplayingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayingMode::Hexadecimal => write!(f, "hexadecimal"),
            DisplayingMode::Decimal => write!(f, "decimal"),
            DisplayingMode::Binary => write!(f, "binary"),
            DisplayingMode::Ascii => write!(f, "ascii"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    author = "Marc-Antoine Manningham, Charles Khouzry",
    version = "0.1",
    about = "Permet de lire et d'écrire sur le robot de INF1900 par USB en série.",
    long_about = "Programme permettant de recevoir et d'envoyer des octets de facon sérielle mais indirectement via le cable USB pour échange avec la carte microcontrôleur du cours inf1900. Ce programme est fortement inspiré de serieViaUSB écrit par Matthew Khouzam, Jérome Collin, Michaël Ferris et Mathieu Marengère-Gosselin.",
    infer_subcommands = true
)]
pub enum Args {
    /// Pour réception des données en provenance de la carte
    #[command()]
    Lecture {
        /// Afficher les octets envoyés ou reçus dans une représentation spécifique.
        #[arg(short, long, default_value_t = DisplayingMode::Ascii)]
        affichage: DisplayingMode,
        /// Effectue un retour à la ligne à chaque n caractère.
        #[arg(short, long)]
        retour: Option<u32>,
    },
    /// Pour envoyer des données vers la la carte. Cette option demande l'utilisation de l'option -f
    Ecriture { fichier: Option<String> },
}
