//! # Mush - Advent of Code Rustdolph CLI
//!
//! Outil de scaffolding et d'exécution pour les défis Advent of Code.
//! Automatise la création de la structure de projet, le téléchargement des inputs
//! et l'exécution des solutions.

mod commands;
mod fetch;
mod results;
mod utils;

use anyhow::Result;
use chrono::Datelike;
use clap::{Parser, Subcommand};

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

    /// Lance l'exécution de tous les jours d'une année et affiche un bilan
    RunAll {
        /// L'année (ex: 2024). Optionnel, par défaut l'année en cours.
        #[arg(short, long)]
        year: Option<u16>,

        /// Lance en mode release (optimisé)
        #[arg(short, long, default_value_t = false)]
        release: bool,

        /// Affiche uniquement le bilan final
        #[arg(short, long, default_value_t = false)]
        summary_only: bool,
    },
}

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            println!("🎄 Initialisation du Workspace aoc-rustdolph...");
            commands::initialize_workspace()?;
        }
        Commands::Scaffold { day, year } => {
            let current_year = chrono::Utc::now().year() as u16;
            let year = year.unwrap_or(current_year);

            println!("🎄 Préparation du jour {} de l'année {}...", day, year);
            commands::create_scaffold(*day, year)?;
        }
        Commands::Run { day, year, release } => {
            let current_year = chrono::Utc::now().year() as u16;
            let year = year.unwrap_or(current_year);

            let package_name = format!("day{:02}-{}", day, year);
            println!(
                "🚀 Lancement du jour {} de l'année {} (package: {})...",
                day, year, package_name
            );

            let mut command = std::process::Command::new("cargo");
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
        Commands::RunAll {
            year,
            release,
            summary_only,
        } => {
            let current_year = chrono::Utc::now().year() as u16;
            let year = year.unwrap_or(current_year);

            let mode = if *release { " (mode release)" } else { "" };
            println!("🎄 Lancement de tous les jours de {}{}...", year, mode);
            commands::run_all(year, *release, *summary_only)?;
        }
    }

    Ok(())
}

use anyhow::Context;

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
        F: FnOnce(&TempDir) + std::panic::UnwindSafe,
    {
        // S'assurer qu'on est dans un répertoire valide avant de commencer
        // En cas d'échec, utiliser /tmp comme fallback
        let original_dir = env::current_dir().unwrap_or_else(|_| {
            let fallback = std::path::PathBuf::from("/tmp");
            let _ = env::set_current_dir(&fallback);
            fallback
        });

        let temp_dir = setup_temp_dir();

        env::set_current_dir(temp_dir.path())
            .expect("Impossible de changer de répertoire vers le temp_dir");

        let result = std::panic::catch_unwind(|| test(&temp_dir));

        // Toujours essayer de revenir au répertoire d'origine
        // Ignorer les erreurs si le répertoire n'existe plus
        let _ = env::set_current_dir(&original_dir);

        if let Err(e) = result {
            std::panic::resume_unwind(e);
        }
    }

    #[test]
    fn test_create_file_success() {
        with_temp_dir(|temp_dir| {
            let file_path = temp_dir.path().join("test_file.txt");
            let content = "Hello, world!";

            let result = utils::create_file(&file_path, content);
            assert!(result.is_ok());
            assert!(file_path.exists());

            let written_content =
                fs::read_to_string(&file_path).expect("Impossible de lire le fichier");
            assert_eq!(written_content, content);
        });
    }

    #[test]
    fn test_create_file_already_exists() {
        with_temp_dir(|temp_dir| {
            let file_path = temp_dir.path().join("existing_file.txt");
            let original_content = "Original content";
            let new_content = "New content";

            fs::write(&file_path, original_content).expect("Impossible d'écrire le fichier");

            let result = utils::create_file(&file_path, new_content);
            assert!(result.is_ok());

            let content = fs::read_to_string(&file_path).expect("Impossible de lire le fichier");
            assert_eq!(content, original_content);
        });
    }

    #[test]
    fn test_create_file_creates_parent_dirs_not_required() {
        with_temp_dir(|temp_dir| {
            let file_path = temp_dir.path().join("nonexistent_dir").join("test.txt");

            let result = utils::create_file(&file_path, "content");
            assert!(result.is_err());
        });
    }

    #[test]
    #[serial]
    fn test_initialize_workspace() {
        with_temp_dir(|temp_dir| {
            let result = commands::initialize_workspace();
            assert!(result.is_ok());

            assert!(temp_dir.path().join("Cargo.toml").exists());
            assert!(temp_dir.path().join(".gitignore").exists());
            assert!(temp_dir.path().join(".env").exists());

            let cargo_content = fs::read_to_string(temp_dir.path().join("Cargo.toml"))
                .expect("Impossible de lire Cargo.toml");
            assert!(cargo_content.contains("solutions/*/*"));
        });
    }

    #[test]
    #[serial]
    fn test_create_scaffold_structure() {
        with_temp_dir(|_temp_dir| {
            env::set_var("AOC_SESSION", "test_session_cookie");

            let day = 1;
            let year = 2024;

            let _ = commands::create_scaffold(day, year);

            let day_path = std::path::PathBuf::from("solutions/2024/day01");
            assert!(day_path.exists());
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
        with_temp_dir(|_temp_dir| {
            env::set_var("AOC_SESSION", "test_session_cookie");

            let day = 25;
            let year = 2023;

            let _ = commands::create_scaffold(day, year);

            // Vérifier que le jour est bien formaté avec deux chiffres
            let day_path = std::path::PathBuf::from("solutions/2023/day25");
            assert!(day_path.exists());

            // Vérifier le nom du package dans Cargo.toml
            let cargo_content = fs::read_to_string(day_path.join("Cargo.toml"))
                .expect("Impossible de lire Cargo.toml");
            assert!(cargo_content.contains("name = \"day25-2023\""));

            env::remove_var("AOC_SESSION");
        });
    }

    #[test]
    #[serial]
    fn test_scaffold_does_not_overwrite_existing_files() {
        with_temp_dir(|_temp_dir| {
            env::set_var("AOC_SESSION", "test_session_cookie");

            let day = 1;
            let year = 2024;

            // Premier scaffold
            commands::create_scaffold(day, year).expect("Premier scaffold échoué");

            let day_path = std::path::PathBuf::from("solutions/2024/day01");
            let main_path = day_path.join("src/main.rs");

            // Modifier le fichier main.rs
            let custom_content = "// Modified content";
            fs::write(&main_path, custom_content).expect("Impossible d'écrire dans main.rs");

            // Re-scaffolder
            commands::create_scaffold(day, year).expect("Second scaffold échoué");

            // Vérifier que le fichier n'a pas été écrasé
            let content = fs::read_to_string(&main_path).expect("Impossible de lire main.rs");
            assert_eq!(content, custom_content);

            env::remove_var("AOC_SESSION");
        });
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

        let result = fetch::fetch_input_with_base_url(1, 2024, &server.url());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test input data\n");
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
            .create();

        let result = fetch::fetch_input_with_base_url(1, 2024, &server.url());

        assert!(result.is_err());
        mock.assert();

        env::remove_var("AOC_SESSION");
    }

    #[test]
    #[serial]
    fn test_fetch_input_missing_session() {
        env::remove_var("AOC_SESSION");

        let result = fetch::fetch_input(1, 2024);
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn test_fetch_input_preserves_content() {
        use mockito::Server;

        env::set_var("AOC_SESSION", "test_cookie");

        let mut server = Server::new();
        let mock = server
            .mock("GET", "/2024/day/1/input")
            .with_status(200)
            .with_body("Input with trailing whitespace   \n\n\n")
            .create();

        let result = fetch::fetch_input_with_base_url(1, 2024, &server.url());

        assert!(result.is_ok());
        // Vérifier que le contenu est préservé tel quel
        assert_eq!(result.unwrap(), "Input with trailing whitespace   \n\n\n");
        mock.assert();

        env::remove_var("AOC_SESSION");
    }
}
