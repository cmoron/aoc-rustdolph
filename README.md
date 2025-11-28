# 🦌 AOC Rustdolph

> **Framework Rust pour [Advent of Code](https://adventofcode.com)** - Automatisation du scaffolding, téléchargement des inputs et exécution des solutions.

[![Rust](https://img.shields.io/badge/rust-1.83%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-11%20passed-brightgreen.svg)](https://github.com/cmoron/aoc-rustdolph)

## 📋 Table des matières

- [Fonctionnalités](#-fonctionnalités)
- [Prérequis](#-prérequis)
- [Installation](#-installation)
- [Configuration](#-configuration)
- [Utilisation](#-utilisation)
  - [Initialiser le workspace](#1-initialiser-le-workspace-optionnel)
  - [Créer un scaffold](#2-créer-un-scaffold-pour-un-jour)
  - [Résoudre le challenge](#3-résoudre-le-challenge)
  - [Exécuter une solution](#4-exécuter-la-solution)
- [Structure du projet](#-structure-du-projet)
- [Raccourcis pratiques](#-raccourcis-pratiques)
- [Contribuer](#-contribuer)
- [Licence](#-licence)

## ✨ Fonctionnalités

- 🚀 **Scaffolding automatique** : génération de la structure de projet pour chaque jour
- 📥 **Téléchargement automatique** : récupère les inputs depuis adventofcode.com
- ⏱️ **Benchmarking intégré** : mesure automatique du temps d'exécution
- 🧪 **Tests prêts à l'emploi** : template de tests avec fichier `example.txt`
- 🔧 **CLI intuitive** : commandes simples via l'outil `mush`
- 📦 **Workspace Cargo** : organisation propre en monorepo

## 🛠️ Prérequis

- [Rust](https://www.rust-lang.org/tools/install) 1.83+ (avec Cargo)
- Un compte sur [Advent of Code](https://adventofcode.com)

## 📦 Installation

```bash
# Cloner le repository
git clone https://github.com/cmoron/aoc-rustdolph.git
cd aoc-rustdolph

# Vérifier que Rust est installé
cargo --version
```

## ⚙️ Configuration

### Récupérer votre cookie de session

1. Connectez-vous sur [adventofcode.com](https://adventofcode.com)
2. Ouvrez les outils de développement de votre navigateur (F12)
3. Allez dans l'onglet **Application** > **Cookies**
4. Copiez la valeur du cookie `session`

### Créer le fichier `.env`

Créez un fichier `.env` à la racine du projet :

```env
AOC_SESSION=votre_cookie_de_session_ici
```

> ⚠️ **Important** : Ne commitez jamais votre fichier `.env` (déjà dans `.gitignore`)

## 🎯 Utilisation

### 1. Initialiser le workspace (optionnel)

Si le projet n'est pas déjà initialisé :

```bash
cargo run -p mush -- init
```

Cela crée :
- `Cargo.toml` (configuration du workspace)
- `.gitignore` (fichiers à ignorer)
- `.env` (template pour le cookie de session)

### 2. Créer un scaffold pour un jour

```bash
# Pour le jour 1 de l'année en cours
cargo run -p mush -- scaffold -d 1

# Pour une année spécifique
cargo run -p mush -- scaffold -d 1 -y 2015

# Vous pouvez aussi spécifier n'importe quel jour (1-25)
cargo run -p mush -- scaffold -d 25 -y 2024
```

Cette commande génère :
```
solutions/2024/day01/
├── Cargo.toml
├── input.txt          # ✅ Téléchargé automatiquement
├── example.txt        # À remplir avec l'exemple du challenge
└── src/
    └── main.rs        # Template avec part1(), part2() et tests
```

### 3. Résoudre le challenge

Ouvrez `solutions/{année}/day{XX}/src/main.rs` et implémentez :

```rust
fn part1(input: &str) -> String {
    // Votre solution pour la partie 1
    "42".to_string()
}

fn part2(input: &str) -> String {
    // Votre solution pour la partie 2
    "1337".to_string()
}
```

### 4. Exécuter la solution

```bash
# Mode Debug (compilation rapide, exécution plus lente)
cargo run -p mush -- run -d 1

# Mode Release (compilation plus lente, exécution ultra-rapide)
cargo run -p mush -- run -d 1 -r
```

**Sortie typique :**
```
Part 1: 42
Time: 0.0023ms
Part 2: 1337
Time: 0.0156ms
```

### Tester vos solutions

```bash
# Depuis la racine
cargo test -p day01-2024

# Ou depuis le répertoire du jour
cd solutions/2024/day01
cargo test
```

### Lancer les tests du framework

Le projet inclut une suite complète de tests unitaires pour l'outil `mush` :

```bash
# Lancer tous les tests
cargo test -p mush

# Lancer les tests avec des détails
cargo test -p mush -- --nocapture

# Lancer un test spécifique
cargo test -p mush test_create_scaffold_structure
```

**Tests couverts :**
- ✅ Création de fichiers (`create_file`)
- ✅ Initialisation du workspace (`initialize_workspace`)
- ✅ Génération de scaffolds (`create_scaffold`)
- ✅ Téléchargement d'inputs HTTP avec mocking (`fetch_input`)
- ✅ Validation de non-écrasement des fichiers existants

> 📚 Pour plus de détails sur les tests, consultez le [Guide des Tests](docs/TESTING.md)

## 📁 Structure du projet

```
aoc-rustdolph/
├── Cargo.toml              # Configuration du workspace
├── .env                    # Cookie de session (à créer)
├── .gitignore
├── LICENSE
├── README.md
├── mush/                   # Outil CLI
│   ├── Cargo.toml
│   └── src/
│       └── main.rs         # Logique de scaffolding et d'exécution
└── solutions/              # Solutions par année
    ├── 2024/
    │   ├── day01/
    │   │   ├── Cargo.toml
    │   │   ├── input.txt
    │   │   ├── example.txt
    │   │   └── src/
    │   │       └── main.rs
    │   └── day02/
    │       └── ...
    └── 2023/
        └── ...
```

## 💡 Raccourcis pratiques

Vous pouvez créer des alias dans votre shell pour simplifier les commandes :

```bash
# Dans ~/.bashrc ou ~/.zshrc
alias aoc-new='cargo run -p mush -- scaffold -d'
alias aoc-run='cargo run -p mush -- run -d'
alias aoc-test='cargo test -p'
```

Utilisation :
```bash
aoc-new 5              # Scaffold du jour 5
aoc-run 5 -r           # Exécuter le jour 5 en mode release
aoc-test day05-2024    # Tester le jour 5
```

## 🤝 Contribuer

Les contributions sont les bienvenues ! N'hésitez pas à :
- Ouvrir une issue pour signaler un bug
- Proposer une pull request pour ajouter une fonctionnalité
- Améliorer la documentation

## 📄 Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

---

<div align="center">

**Bon code ! 🎄✨**

*Créé avec ❤️ pour faciliter les défis Advent of Code*

</div>
