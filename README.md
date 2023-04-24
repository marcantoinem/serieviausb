# serieViaUSB en Rust
Ce programme est une réimplémentation du programme serieViaUsb du cours [INF1900](https://cours.polymtl.ca/inf1900/). La partie SVG est maintenant dans une autre branche (`projet-2023-svg`), puisqu'elle n'est pas nécessaire la majorité du temps.

## Compatibilité avec serieViaUsb
Ce programme n'est pas conçu pour reproduire fidèlement les options de serieViaUsb, mais il est similaire dans ce qu'il offre en terme de fonctionnalités.

## Fonctionnalités
- Multiplateforme Mac et Linux
- Gère les interruptions par ctrl-c
- Imprime les messages d'erreurs avec de la couleur

## Comment installer
1. Installer Rust (si pas installé) avec `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` voir [Install Rust](https://www.rust-lang.org/tools/install)

2. Fermer et rouvrir le terminal pour relancer le shell.

3. Installer serieviausb avec `cargo install --git https://github.com/marcantoinem/serieviausb`

## Captures d'écran
![Menu d'aide](screenshots/help.png)
![Envoi de fichier](screenshots/envoi.png)