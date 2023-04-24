use clap::{ArgGroup, Parser, ValueEnum};
use derive_more::Display;

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
    long_about = "Programme permettant de recevoir et d'envoyer des octets de facon sérielle mais indirectement via le cable USB pour échange avec la carte microcontrôleur du cours inf1900. Ce programme est fortement inspiré de serieViaUSB écrit par Matthew Khouzam, Jérome Collin, Michaël Ferris et Mathieu Marengère-Gosselin."
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
