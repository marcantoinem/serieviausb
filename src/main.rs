use anyhow::{Context, Result};
use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use usb::SerialUsb;

mod args;
mod modes;
mod usb;

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
    let args = args::Args::parse();
    let sigint_requested = initialize_sigint_handler();
    let device = usb::find_device().context("La carte mère est introuvable.")?;
    let mut handle = device.open()?;
    handle.init_serial_usb()?;
    if args.svg && args.lecture {
        modes::read_svg(&mut handle, &sigint_requested, args.fichier)?;
    } else if args.svg && args.ecriture {
        let fichier = args.fichier.context("Fichier non fourni")?;
        modes::write_svg(fichier, &handle, &sigint_requested)?;
    } else if args.lecture {
        modes::read(&mut handle, &sigint_requested, args.affichage, args.retour)?;
    } else if args.ecriture {
        let fichier = args.fichier.context("Fichier non fourni")?;
        modes::write(fichier, &handle, &sigint_requested)?;
    }
    Ok(())
}

fn main() {
    if let Err(error) = serie_via_usb() {
        color_print::cprintln!("<red>Erreur: {}</>", error);
    }
}
