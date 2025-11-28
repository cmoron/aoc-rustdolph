# 🦌 AOC Rustdolph 🦌

Un template Rust pour résoudre les défis d'[Advent of Code](https://adventofcode.com).
Mush : utilitaire de scaffolding et d'exécution des solutions AOC.

## Installation

1. Cloner le repo.
2. Récupérer son cookie de session sur [adventofcode.com](https://adventofcode.com).
3. Créer un fichier `.env` à la racine :
```env
AOC_SESSION=votre_chaine_hexadecimale_ici
```

## Utilisation

### Préparer une journée (Scaffold)
Crée les dossiers, télécharge l'input et prépare le template.

```bash
# Pour le jour 1 de l'année en cours
cargo run -p mush -- scaffold -d 1

# Pour une année spécifique
cargo run -p mush -- scaffold -d 1 -y 2015
```

### Lancer une solution
```bash
# Mode Debug (rapide à compiler, lent à exécuter)
cargo run -p mush -- run -d 1

# Mode Release (lent à compiler, ultra rapide à exécuter)
cargo run -p mush -- run -d 1 -r
```
