//! # Mush - Advent of Code Rustdolph CLI
//!
//! Outil de scaffolding et d'exécution pour les défis Advent of Code.
//! Automatise la création de la structure de projet, le téléchargement des inputs
//! et l'exécution des solutions.

use anyhow::{Context, Result};
use chrono::Datelike;
use clap::{Parser, Subcommand};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command as ShellCommand;

/// Point d'entrée de la CLI Mush
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Commandes disponibles dans la CLI
#[derive(Subcommand)]
enum Commands {
    /// Initialise le workspace avec les fichiers de configuration nécessaires
    Init,

    /// Génère la structure de projet pour un jour spécifique et télécharge l'input
    Scaffold {
        /// Le jour du challenge (1-25)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
        day: u8,

        /// L'année (ex: 2024). Optionnel, par défaut l'année en cours.
        #[arg(short, long)]
        year: Option<u16>,
    },

    /// Lance l'exécution d'une solution pour un jour donné
    Run {
        /// Le jour du challenge (1-25)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
        day: u8,

        /// L'année (ex: 2024). Optionnel, par défaut l'année en cours.
        #[arg(short, long)]
        year: Option<u16>,

        /// Lance en mode release (optimisé)
        #[arg(short, long, default_value_t = false)]
        release: bool,
    },
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            println!("🎄 Initialisation du Workspace aoc-rustdolph...");
            initialize_workspace()?;
        }
        Commands::Scaffold { day, year } => {
            let current_year = chrono::Utc::now().year() as u16;
            let year = year.unwrap_or(current_year);

            println!("🎄 Préparation du jour {} de l'année {}...", day, year);
            create_scaffold(*day, year)?;
        }
        Commands::Run { day, year, release } => {
            let current_year = chrono::Utc::now().year() as u16;
            let year = year.unwrap_or(current_year);

            let package_name = format!("day{:02}-{}", day, year);
            println!(
                "🚀 Lancement du jour {} de l'année {} (package: {})...",
                day, year, package_name
            );

            let mut command = ShellCommand::new("cargo");
            command.arg("run").arg("-p").arg(&package_name);
            if *release {
                command.arg("--release");
            }

            let status = command
                .status()
                .with_context(|| "Échec de l'exécution de la commande cargo run")?;

            if !status.success() {
                println!("❌ Le scaffold pour le jour {} de l'année {} n'a pas été trouvé ou une erreur est survenue lors de l'exécution.", day, year);
                return Err(anyhow::anyhow!(
                    "La commande cargo run a échoué avec le statut {}",
                    status
                ));
            }
        }
    }

    Ok(())
}

/// Initialise le workspace Advent of Code avec les fichiers de configuration nécessaires.
///
/// Cette fonction crée :
/// - `Cargo.toml` : définition du workspace avec le pattern `solutions/*/*`
/// - `.gitignore` : fichiers à ignorer dans git
/// - `.env` : template pour le cookie de session AOC
///
/// # Errors
///
/// Retourne une erreur si l'écriture des fichiers échoue.
fn initialize_workspace() -> Result<()> {
    // 1. Créer le fichier Cargo.toml à la racine
    let cargo_toml_content = r#"[workspace]
members = [
    "solutions/*/*"
]
resolver = "2"
"#;
    create_file(&PathBuf::from("Cargo.toml"), cargo_toml_content)?;

    // 2. Créer le fichier .gitignore à la racine
    let gitignore_content = r#"/target
**/target
.env
.DS_Store
**/*.rs.bk
**/input.txt
"#;
    create_file(&PathBuf::from(".gitignore"), gitignore_content)?;

    // 3. Créer le fichier .env à la racine
    let env_content = r#"AOC_SESSION=your_session_cookie_here
"#;
    create_file(&PathBuf::from(".env"), env_content)?;

    println!("✅ Workspace initialisé !");
    println!("👉 N'oublie pas de mettre ton token dans le fichier .env");

    Ok(())
}

/// Crée la structure complète d'un jour de challenge Advent of Code.
///
/// Cette fonction génère :
/// - L'arborescence de répertoires : `solutions/{year}/day{XX}/src/`
/// - Le fichier `Cargo.toml` avec les dépendances nécessaires
/// - Un template `main.rs` avec les fonctions part1/part2 et benchmarking
/// - Le fichier `input.txt` téléchargé automatiquement depuis adventofcode.com
/// - Un fichier `example.txt` vide pour les tests
///
/// # Arguments
///
/// * `day` - Le jour du challenge (1-25)
/// * `year` - L'année du challenge
///
/// # Errors
///
/// Retourne une erreur si :
/// - La création des répertoires échoue
/// - L'écriture des fichiers échoue
/// - Le téléchargement de l'input échoue (mais continue avec un fichier vide)
fn create_scaffold(day: u8, year: u16) -> Result<()> {
    // 1. Définir les chemins
    // Le format {:02} permet d'avoir "day01" au lieu de "day1"
    let package_name = format!("day{:02}-{}", day, year);
    let day_str = format!("day{:02}", day);
    let base_path = PathBuf::from("solutions")
        .join(year.to_string())
        .join(&day_str);
    let src_path = base_path.join("src");

    // 2. Créer les répertoires nécessaires
    fs::create_dir_all(&src_path)
        .with_context(|| format!("Impossible de créer le répertoire {:?}", src_path))?;

    // 3. Créer le Cargo.toml du jour
    // On nomme le package day01 pour pouvoir faire "cargo run -p day01" plus tard
    let cargo_toml_content = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
itertools = "0.10.5"
regex = "1.10.3"
"#,
        package_name
    );

    create_file(&base_path.join("Cargo.toml"), &cargo_toml_content)?;

    // 4. Créer le template Rust (main.rs)
    // On prépare la structure pour le benchmak
    let main_rs_content = r#"fn main() {
    let input = include_str!("../input.txt");

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let example_input = include_str!("../example.txt");
        assert_eq!(part1(example_input), 0);
    }
}
"#;

    create_file(&src_path.join("main.rs"), main_rs_content)?;

    // 5. Récupérer et écrire l'input dans input.txt
    let input_path = base_path.join("input.txt");

    if !input_path.exists() || fs::read_to_string(&input_path)?.is_empty() {
        println!(
            "🌐 Récupération de l'input pour le jour {} de l'année {}...",
            day, year
        );
        match fetch_input(day, year) {
            Ok(input_data) => {
                create_file(&input_path, &input_data)?;
                println!("✅ Input récupéré et écrit dans {:?}", input_path);
            }
            Err(e) => {
                println!("❌ Échec de la récupération de l'input: {}", e);
                println!("⚠️  Le fichier {:?} reste vide. Vous pouvez remplir manuellement l'input plus tard.", input_path);
                create_file(&input_path, "")?;
            }
        }
    } else {
        println!(
            "⚠️  Le fichier {:?} contient déjà des données, il ne sera pas écrasé.",
            input_path
        );
    }

    // 6. On créé example.txt vide s'il n'existe pas déjà
    let example_path = base_path.join("example.txt");
    if !example_path.exists() {
        create_file(&example_path, "")?;
    }

    println!(
        "✅ Scaffold pour le jour {} de l'année {} créé avec succès!",
        day, year
    );
    Ok(())
}

/// Crée un fichier avec le contenu spécifié s'il n'existe pas déjà.
///
/// Cette fonction ne fait rien si le fichier existe déjà, évitant ainsi
/// d'écraser accidentellement des modifications de l'utilisateur.
///
/// # Arguments
///
/// * `path` - Le chemin du fichier à créer
/// * `content` - Le contenu à écrire dans le fichier
///
/// # Errors
///
/// Retourne une erreur si :
/// - La création du fichier échoue
/// - L'écriture du contenu échoue
fn create_file(path: &Path, content: &str) -> Result<()> {
    if !path.exists() {
        let mut file = fs::File::create(path)
            .with_context(|| format!("Impossible de créer le fichier {:?}", path))?;
        file.write_all(content.as_bytes())
            .with_context(|| format!("Impossible d'écrire dans le fichier {:?}", path))?;
    } else {
        println!(
            "⚠️  Le fichier {:?} existe déjà, il ne sera pas écrasé.",
            path
        );
    }

    Ok(())
}

/// Télécharge l'input d'un challenge depuis le site adventofcode.com.
///
/// Utilise le cookie de session stocké dans la variable d'environnement
/// `AOC_SESSION` pour s'authentifier auprès de l'API Advent of Code.
///
/// # Arguments
///
/// * `day` - Le jour du challenge (1-25)
/// * `year` - L'année du challenge
///
/// # Errors
///
/// Retourne une erreur si :
/// - La variable d'environnement `AOC_SESSION` n'est pas définie
/// - La requête HTTP échoue
/// - Le serveur retourne une erreur (status non-200)
/// - La lecture de la réponse échoue
///
/// # Notes
///
/// Inclut un User-Agent conformément aux recommandations de l'API AOC.
fn fetch_input(day: u8, year: u16) -> Result<String> {
    fetch_input_with_base_url(day, year, "https://adventofcode.com")
}

/// Version interne de fetch_input permettant de spécifier l'URL de base (pour les tests).
fn fetch_input_with_base_url(day: u8, year: u16, base_url: &str) -> Result<String> {
    let session = std::env::var("AOC_SESSION")
        .context("La variable d'environnement AOC_SESSION n'est pas définie dans .env")?;

    let url = format!("{}/{}/day/{}/input", base_url, year, day);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .header(
            "User-Agent",
            "github.com/cmoron/aoc-rustdolph by cyril.moron@gmail.com",
        )
        .send()
        .with_context(|| format!("Erreur lors de la requête vers {}", url))?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Erreur lors de la récupération de l'input: statut {}",
            response.status()
        ));
    }

    let text = response
        .text()
        .with_context(|| "Erreur lors de la lecture de la réponse")?;

    Ok(text.trim_end().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::fs;
    use tempfile::TempDir;

    /// Helper pour créer un répertoire temporaire de test
    fn setup_temp_dir() -> TempDir {
        TempDir::new().expect("Impossible de créer un répertoire temporaire")
    }

    /// Helper pour se déplacer dans un répertoire temporaire
    fn with_temp_dir<F>(test: F)
    where
        F: FnOnce(&TempDir),
    {
        let temp_dir = setup_temp_dir();
        let original_dir =
            env::current_dir().expect("Impossible de récupérer le répertoire actuel");
        env::set_current_dir(&temp_dir)
            .expect("Impossible de se déplacer dans le répertoire temporaire");

        test(&temp_dir);

        env::set_current_dir(original_dir).expect("Impossible de revenir au répertoire original");
    }

    #[test]
    fn test_create_file_success() {
        let temp_dir = setup_temp_dir();
        let file_path = temp_dir.path().join("test.txt");
        let content = "Hello, AOC!";

        let result = create_file(&file_path, content);

        assert!(result.is_ok());
        assert!(file_path.exists());
        let read_content = fs::read_to_string(&file_path).expect("Impossible de lire le fichier");
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_create_file_already_exists() {
        let temp_dir = setup_temp_dir();
        let file_path = temp_dir.path().join("existing.txt");

        // Créer le fichier une première fois
        fs::write(&file_path, "original content").expect("Impossible de créer le fichier");

        // Essayer de le créer à nouveau avec un contenu différent
        let result = create_file(&file_path, "new content");

        assert!(result.is_ok());
        // Le fichier ne doit pas avoir été écrasé
        let read_content = fs::read_to_string(&file_path).expect("Impossible de lire le fichier");
        assert_eq!(read_content, "original content");
    }

    #[test]
    #[serial]
    fn test_initialize_workspace() {
        with_temp_dir(|_temp_dir| {
            let result = initialize_workspace();

            assert!(result.is_ok());

            // Vérifier que les fichiers ont été créés
            assert!(PathBuf::from("Cargo.toml").exists());
            assert!(PathBuf::from(".gitignore").exists());
            assert!(PathBuf::from(".env").exists());

            // Vérifier le contenu du Cargo.toml
            let cargo_content =
                fs::read_to_string("Cargo.toml").expect("Impossible de lire Cargo.toml");
            assert!(cargo_content.contains("[workspace]"));
            assert!(cargo_content.contains("solutions/*/*"));

            // Vérifier le contenu du .gitignore
            let gitignore_content =
                fs::read_to_string(".gitignore").expect("Impossible de lire .gitignore");
            assert!(gitignore_content.contains("/target"));
            assert!(gitignore_content.contains(".env"));

            // Vérifier le contenu du .env
            let env_content = fs::read_to_string(".env").expect("Impossible de lire .env");
            assert!(env_content.contains("AOC_SESSION"));
        });
    }

    #[test]
    #[serial]
    fn test_create_scaffold_structure() {
        with_temp_dir(|temp_dir| {
            // Mock de la variable d'environnement pour éviter l'erreur de fetch
            env::set_var("AOC_SESSION", "test_session_cookie");

            let day = 1;
            let year = 2024;

            // Note: create_scaffold essaiera de fetch l'input, ce qui échouera,
            // mais il créera quand même la structure
            let _ = create_scaffold(day, year);

            // Vérifier la structure créée
            let day_path = temp_dir.path().join("solutions/2024/day01");
            assert!(day_path.exists());
            assert!(day_path.join("src").exists());
            assert!(day_path.join("Cargo.toml").exists());
            assert!(day_path.join("src/main.rs").exists());
            assert!(day_path.join("input.txt").exists());
            assert!(day_path.join("example.txt").exists());

            // Vérifier le contenu du Cargo.toml
            let cargo_content = fs::read_to_string(day_path.join("Cargo.toml"))
                .expect("Impossible de lire Cargo.toml");
            assert!(cargo_content.contains("name = \"day01-2024\""));
            assert!(cargo_content.contains("itertools"));
            assert!(cargo_content.contains("regex"));

            // Vérifier le contenu du main.rs
            let main_content = fs::read_to_string(day_path.join("src/main.rs"))
                .expect("Impossible de lire main.rs");
            assert!(main_content.contains("fn part1"));
            assert!(main_content.contains("fn part2"));
            assert!(main_content.contains("#[cfg(test)]"));

            env::remove_var("AOC_SESSION");
        });
    }

    #[test]
    #[serial]
    fn test_create_scaffold_with_double_digit_day() {
        with_temp_dir(|temp_dir| {
            env::set_var("AOC_SESSION", "test_session_cookie");

            let day = 25;
            let year = 2023;

            let _ = create_scaffold(day, year);

            // Vérifier que le jour est bien formaté avec deux chiffres
            let day_path = temp_dir.path().join("solutions/2023/day25");
            assert!(day_path.exists());

            let cargo_content = fs::read_to_string(day_path.join("Cargo.toml"))
                .expect("Impossible de lire Cargo.toml");
            assert!(cargo_content.contains("name = \"day25-2023\""));

            env::remove_var("AOC_SESSION");
        });
    }

    #[test]
    #[serial]
    fn test_fetch_input_missing_session() {
        // S'assurer que AOC_SESSION n'est pas définie
        env::remove_var("AOC_SESSION");

        let result = fetch_input(1, 2024);

        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("AOC_SESSION"));
    }

    #[test]
    #[serial]
    fn test_fetch_input_with_mock_server() {
        use mockito::Server;

        env::set_var("AOC_SESSION", "test_cookie");

        let mut server = Server::new();
        let mock = server
            .mock("GET", "/2024/day/1/input")
            .match_header("cookie", "session=test_cookie")
            .match_header(
                "user-agent",
                "github.com/cmoron/aoc-rustdolph by cyril.moron@gmail.com",
            )
            .with_status(200)
            .with_body("Test input data\n")
            .create();

        let result = fetch_input_with_base_url(1, 2024, &server.url());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test input data");
        mock.assert();

        env::remove_var("AOC_SESSION");
    }

    #[test]
    #[serial]
    fn test_fetch_input_http_error() {
        use mockito::Server;

        env::set_var("AOC_SESSION", "test_cookie");

        let mut server = Server::new();
        let mock = server
            .mock("GET", "/2024/day/1/input")
            .with_status(404)
            .with_body("Not Found")
            .create();

        let result = fetch_input_with_base_url(1, 2024, &server.url());

        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("404"));
        mock.assert();

        env::remove_var("AOC_SESSION");
    }

    #[test]
    #[serial]
    fn test_fetch_input_trims_whitespace() {
        use mockito::Server;

        env::set_var("AOC_SESSION", "test_cookie");

        let mut server = Server::new();
        let mock = server
            .mock("GET", "/2024/day/1/input")
            .with_status(200)
            .with_body("Input with trailing whitespace   \n\n\n")
            .create();

        let result = fetch_input_with_base_url(1, 2024, &server.url());

        assert!(result.is_ok());
        // Vérifier que les espaces de fin sont supprimés
        assert_eq!(result.unwrap(), "Input with trailing whitespace");
        mock.assert();

        env::remove_var("AOC_SESSION");
    }

    #[test]
    fn test_create_file_creates_parent_dirs_not_required() {
        // Test que create_file ne crée PAS les répertoires parents
        let temp_dir = setup_temp_dir();
        let nested_path = temp_dir.path().join("non/existent/path/file.txt");

        let result = create_file(&nested_path, "content");

        // Devrait échouer car les répertoires parents n'existent pas
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn test_scaffold_does_not_overwrite_existing_files() {
        with_temp_dir(|temp_dir| {
            env::set_var("AOC_SESSION", "test_session");

            let day = 5;
            let year = 2024;

            // Créer une première fois
            let _ = create_scaffold(day, year);

            // Modifier le main.rs
            let main_path = temp_dir.path().join("solutions/2024/day05/src/main.rs");
            fs::write(&main_path, "// Modified content").expect("Impossible de modifier main.rs");

            // Créer à nouveau
            let _ = create_scaffold(day, year);

            // Vérifier que le fichier n'a pas été écrasé
            let content = fs::read_to_string(&main_path).expect("Impossible de lire main.rs");
            assert_eq!(content, "// Modified content");

            env::remove_var("AOC_SESSION");
        });
    }
}
