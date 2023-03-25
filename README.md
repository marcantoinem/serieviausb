# serieViaUSB en Rust + lecture SVG
Ce programme est une réimplémentation du programme serieViaUsb du cours [INF1900](https://cours.polymtl.ca/inf1900/).

## Compatibilité avec serieViaUsb
Ce programme n'est pas conçu pour reproduire fidèlement les options de serieViaUsb, mais il est similaire dans ce qu'il offre en terme de fonctionnalités.

## Fonctionnalités
- Multiplateforme Mac et Linux
- Gère les interruptions par ctrl-c
- Inclus une option pour écrire/lire du svg
- Peut faire un rendu d'un aperçu du svg dans le terminal
- Contient de la couleur
- Peut enregistrer le svg dans un fichier
- Peut convertir le svg en png et l'enregistrer dans un fichier

## Comment compiler
1. Installer Rust (si pas installé) avec `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` voir [Install Rust](https://www.rust-lang.org/tools/install)

2. Cloner ce repo et aller à la racine du repo. `git clone https://github.com/marcantoinem/serieviausb && cd serieviausb`

3. Compiler en mode release avec la commande `cargo build --release`
> Plusieurs flags d'optimisations assez lourd pour le processeur sont activés, ce qui rend la compilation plus lente. (environ 1 minute)
Si le temps de compilation est trop long, il est possible de désactiver la lto en commentant la ligne `# lto = "fat"` dans `Cargo.toml`

4. Copier l'exécutable situé à `target/release/serieviausb` dans un emplacement dans la [PATH](https://en.wikipedia.org/wiki/PATH_(variable)) pour avoir la commande disponible partout.

## Screenshots
![Menu d'aide](screenshots/help.png)
![Envoi de fichier](screenshots/envoi.png)
![Exemple de SVG](screenshots/receptionsvg.png)