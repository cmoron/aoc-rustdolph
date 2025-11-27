use clap::{Parser, Subcommand};
use std::fs;
use std::process::Command as ShellCommand;
use std::io::Write;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use chrono::Datelike;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scaffold {
        /// Le jour du challenge (1-25)
        #[arg(short, long)]
        day: u8,

        /// L'année (ex: 2024). Optionnel, par défaut l'année en cours.
        #[arg(short, long)]
        year: Option<u16>,
    },

    Run {
        /// Le jour du challenge (1-25)
        #[arg(short, long)]
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
        Commands::Scaffold { day, year } => {
            let current_year = chrono::Utc::now().year() as u16;
            let year = year.unwrap_or(current_year);

            println!("🎄 Préparation du jour {} de l'année {}...", day, year);
            let _ = create_scaffold(*day, year);
        }
        Commands::Run { day, year, release } => {
            let current_year = chrono::Utc::now().year() as u16;
            let year = year.unwrap_or(current_year);

            let package_name = format!("day{:02}-{}", day, year);
            println!("🚀 Lancement du jour {} de l'année {} (package: {})...", day, year, package_name);

            let mut command = ShellCommand::new("cargo");
            command.arg("run").arg("-p").arg(&package_name);
            if *release {
                command.arg("--release");
            }

            let status = command
                .status()
                .with_context(|| "Échec de l'exécution de la commande cargo run")?;

            if !status.success() {
                return Err(anyhow::anyhow!("La commande cargo run a échoué avec le statut {}", status));
            }
        }
    }

    Ok(())
}

fn create_scaffold(day: u8, year: u16) -> Result<()> {

    // 1. Définir les chemins
    // Le format {:02} permet d'avoir "day01" au lieu de "day1"
    let package_name = format!("day{:02}-{}", day, year);
    let day_str = format!("day{:02}", day);
    let base_path = PathBuf::from("solutions").join(year.to_string()).join(&day_str);
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
"#, package_name);

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

fn part1(input: &str) -> String {
    "todo!".to_string()
}

fn part2(input: &str) -> String {
    "todo!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let example_input = include_str!("../example.txt");
        assert_eq!(part1(example_input), "todo!");
    }
}
"#;

    create_file(&src_path.join("main.rs"), main_rs_content)?;

    // 5. Récupérer et écrire l'input dans input.txt
    let input_path = base_path.join("input.txt");

    if !input_path.exists() || fs::read_to_string(&input_path)?.is_empty() {
        println!("🌐 Récupération de l'input pour le jour {} de l'année {}...", day, year);
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
        println!("⚠️  Le fichier {:?} contient déjà des données, il ne sera pas écrasé.", input_path);
    }

    // 6. On créé example.txt vide s'il n'existe pas déjà
    let example_path = base_path.join("example.txt");
    if !example_path.exists() {
        create_file(&example_path, "")?;
    }

    println!("✅ Scaffold pour le jour {} de l'année {} créé avec succès!", day, year);
    Ok(())
}

fn create_file(path: &Path, content: &str) -> Result<()> {

    if !path.exists() {
        let mut file = fs::File::create(path)
            .with_context(|| format!("Impossible de créer le fichier {:?}", path))?;
        file.write_all(content.as_bytes())
            .with_context(|| format!("Impossible d'écrire dans le fichier {:?}", path))?;

    } else {
        println!("⚠️  Le fichier {:?} existe déjà, il ne sera pas écrasé.", path);
    }

    Ok(())
}

fn fetch_input(day: u8, year: u16) -> Result<String> {
    let session = std::env::var("AOC_SESSION")
        .context("La variable d'environnement AOC_SESSION n'est pas définie dans .env")?;

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()
        .with_context(|| format!("Erreur lors de la requête vers {}", url))?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Erreur lors de la récupération de l'input: statut {}", response.status()));
    }

    let text = response
        .text()
        .with_context(|| "Erreur lors de la lecture de la réponse")?;

    Ok(text.trim_end().to_string())
}
