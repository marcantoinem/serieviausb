use anyhow::{Context, Result};
use clap::Parser;
use owo_colors::OwoColorize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use usb::SerialUsb;

use crate::args::Args;

pub mod args;
pub mod modes;
pub mod usb;

fn initialize_sigint_handler() -> Arc<AtomicBool> {
    let sigint_requested = Arc::new(AtomicBool::new(false));
    let sigint_handler = Arc::clone(&sigint_requested);
    ctrlc::set_handler(move || {
        sigint_handler.store(true, Ordering::Relaxed);
    })
    .expect("L'initialisation de Ctrl+C a échoué.");
    sigint_requested
}

fn serie_via_usb() -> Result<()> {
    let args = Args::parse();
    let sigint_requested = initialize_sigint_handler();
    let device = usb::find_device().context("La carte mère est introuvable.")?;
    let mut handle = device.open()?;
    handle.init_serial_usb()?;
    match args {
        Args::Lecture { affichage, retour } => {
            modes::read(&mut handle, &sigint_requested, affichage, retour)?;
        }
        Args::Ecriture { fichier } => {
            let fichier = fichier.context("Fichier non fourni")?;
            modes::write(fichier, &handle, &sigint_requested)?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = serie_via_usb() {
        println!("{}{}", "Erreur: ".red(), error.red());
    }
}
