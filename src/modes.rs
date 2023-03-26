use crate::{
    args::{bits_from_buffer, DisplayingMode},
    usb::SerialUsb,
};
use anyhow::{Context, Result};
use color_print::cprintln;
use indicatif::{ProgressIterator, ProgressStyle};
use resvg::{
    tiny_skia::{Pixmap, Transform},
    usvg::{FitTo, Options, Tree},
    usvg_text_layout::{fontdb, TreeTextToPath},
};
use rusb::{DeviceHandle, GlobalContext};
use std::{
    fs::File,
    io::{Cursor, Read, Write},
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};
use viuer::Config;

type Handle = DeviceHandle<GlobalContext>;

pub fn write(file: String, handle: &Handle, sigint: &AtomicBool) -> Result<()> {
    let mut f = std::fs::File::open(file)?;
    let mut file_buffer = vec![];
    f.read_to_end(&mut file_buffer)?;
    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.green/red} {pos:>7}/{len:7} {msg}",
    )?;
    for buffer in file_buffer.chunks(7).progress_with_style(style) {
        if sigint.load(Ordering::Relaxed) {
            cprintln!("\n<red>Écriture interrompu par l'utilisateur</>");
            return Ok(());
        }
        handle.write_serial_usb(buffer)?;
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    cprintln!("<green>{} bits ont été écris.</>", file_buffer.len());
    Ok(())
}

pub fn read(
    handle: &mut Handle,
    sigint: &AtomicBool,
    mode: DisplayingMode,
    saut: Option<u32>,
) -> Result<()> {
    let mut pos = 0;
    while !sigint.load(Ordering::Relaxed) {
        let mut buffer = [0xff; 8];
        handle.read_serial_usb(&mut buffer)?;
        mode.print(&buffer, saut, &mut pos);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    cprintln!("<red>Arrêt de lecture</>");
    Ok(())
}

pub fn read_svg(handle: &mut Handle, sigint: &AtomicBool, sortie: Option<String>) -> Result<()> {
    let mut buffer = [0x00; 8];
    let mut data: Vec<u8> = vec![];
    let time = Instant::now();
    cprintln!(
        "<yellow>{:.2}s </>\t<green>En attente de l'octet de commencement (0x02)</>",
        time.elapsed().as_secs_f64()
    );
    loop {
        handle.read_serial_usb(&mut buffer)?;
        if let Some(position) = bits_from_buffer(&buffer).iter().position(|x| x == &2) {
            if buffer[0] > position as u8 {
                data.extend_from_slice(&bits_from_buffer(&buffer)[position + 1..]);
            }
            break;
        }
        if sigint.load(Ordering::Relaxed) {
            cprintln!("<red>Arrêt de lecture</red>");
            return Ok(());
        }
    }

    cprintln!(
        "<yellow>{:.2}s </>\t<green>Début de l'image reçue, en attente de l'octet de fin (0x03)</>",
        time.elapsed().as_secs_f64()
    );

    let mut checksum = vec![];
    loop {
        handle.read_serial_usb(&mut buffer)?;
        if let Some(position) = bits_from_buffer(&buffer).iter().position(|x| x == &3) {
            data.extend_from_slice(&bits_from_buffer(&buffer)[..position]);
            if buffer[0] > position as u8 {
                checksum.extend_from_slice(&bits_from_buffer(&buffer)[(position + 1)..]);
            }
            break;
        }
        data.extend_from_slice(bits_from_buffer(&buffer));
        if sigint.load(Ordering::Relaxed) {
            cprintln!("<red>Arrêt de lecture</>");
            return Ok(());
        }
    }

    cprintln!(
        "<yellow>{:.2}s </><green>\tFin de l'image reçue, en attente de l'octet de fin de checksum (0x04)</>",
        time.elapsed().as_secs_f64()
    );

    loop {
        handle.read_serial_usb(&mut buffer)?;
        if let Some(position) = bits_from_buffer(&buffer).iter().position(|x| x == &4) {
            if buffer[0] > position as u8 {
                checksum.extend_from_slice(&bits_from_buffer(&buffer)[(position + 1)..]);
            }
            break;
        }
        checksum.extend_from_slice(bits_from_buffer(&buffer));
        if sigint.load(Ordering::Relaxed) {
            cprintln!("<red>Arrêt de lecture</>");
            return Ok(());
        }
    }

    let computed_checksum = crc32fast::hash(&data);
    let checksum: String = checksum.iter().rev().map(|x| format!("{:X}", x)).collect();
    cprintln!(
        "<yellow>{:.2}s </>\t<green>Fin du checksum: {} {:X}</>",
        time.elapsed().as_secs_f64(),
        checksum,
        computed_checksum
    );

    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();
    let mut tree = Tree::from_data(&data, &Options::default())?;
    tree.convert_text(&fontdb);

    let size = (tree.size.width() as u32, tree.size.height() as u32);
    let mut pixmap = Pixmap::new(size.0, size.1).context("Erreur de la transformation du svg.")?;

    resvg::render(
        &tree,
        FitTo::Size(size.0, size.1),
        Transform::default(),
        pixmap.as_mut(),
    );

    let png = pixmap.encode_png()?;
    let img = image::io::Reader::new(Cursor::new(&png))
        .with_guessed_format()?
        .decode()?;
    let term_size = terminal_size::terminal_size()
        .context("Impossibilité d'obtenir les dimensions du terminal.")?;
    let conf = Config {
        absolute_offset: false,
        width: Some(term_size.0 .0 as u32),
        height: None,
        ..Default::default()
    };

    if let Some(sortie) = sortie {
        let mut file = File::create(&sortie)?;
        if &sortie[sortie.len() - 3..] == "png" {
            file.write_all(&png)?;
        } else {
            file.write_all(&data)?;
        }
        cprintln!(
            "<yellow>{:.2}s </>\t<green>Fichier écrit à {}</>",
            time.elapsed().as_secs_f64(),
            sortie
        );
    }

    viuer::print(&img, &conf).context("L'impression de l'image dans la terminal a échouée")?;

    Ok(())
}

pub fn write_svg(file: String, handle: &Handle, sigint: &AtomicBool) -> Result<()> {
    let mut f = std::fs::File::open(file)?;
    let mut file_buffer = vec![];
    f.read_to_end(&mut file_buffer)?;
    let checksum = crc32fast::hash(&file_buffer);
    file_buffer.push(0x03);
    file_buffer.extend_from_slice(&checksum.to_le_bytes());
    file_buffer.push(0x04);
    let mut first_byte = ((file_buffer.len() + 3) >> 8) as u8;
    let mut second_byte = (file_buffer.len() + 3) as u8;
    if first_byte == 2 {
        first_byte += 1;
        file_buffer.extend([0; 0x0100]);
    }
    if second_byte == 2 {
        second_byte += 1;
        file_buffer.push(0);
    }
    file_buffer.insert(0, first_byte);
    file_buffer.insert(1, second_byte);
    file_buffer.insert(2, 0x02);
    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.green/red} {pos:>7}/{len:7} {msg}",
    )?;
    for buffer in file_buffer.chunks(7).progress_with_style(style) {
        if sigint.load(Ordering::Relaxed) {
            cprintln!("<red>Écriture interrompu par l'utilisateur</>");
            return Ok(());
        }
        handle.write_serial_usb(buffer)?;
        std::thread::sleep(Duration::from_millis(50));
    }
    cprintln!(
        "<green>{} octets écrits avec la checksum {:X}</>",
        first_byte | second_byte,
        checksum
    );
    Ok(())
}
