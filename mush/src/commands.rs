use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command as ShellCommand;

use crate::fetch::fetch_input;
use crate::results::{parse_part, DayResult};
use crate::utils::create_file;

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
pub fn initialize_workspace() -> Result<()> {
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
pub fn create_scaffold(day: u8, year: u16) -> Result<()> {
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

/// Lance tous les jours d'une année et affiche un bilan global
pub fn run_all(year: u16, release: bool, summary_only: bool) -> Result<()> {
    let mut results = Vec::new();

    for day in 1..=25 {
        let package_name = format!("day{:02}-{}", day, year);
        let day_path = PathBuf::from("solutions")
            .join(year.to_string())
            .join(format!("day{:02}", day));

        // Vérifier si le jour existe
        if !day_path.exists() {
            continue;
        }

        // Exécuter le jour
        let mut command = ShellCommand::new("cargo");
        command
            .arg("run")
            .arg("-p")
            .arg(&package_name)
            .arg("--quiet");
        if release {
            command.arg("--release");
        }

        let output = command
            .output()
            .with_context(|| format!("Échec de l'exécution du jour {}", day))?;

        if !output.status.success() {
            if !summary_only {
                println!("\n❌ Day {:02}: Erreur d'exécution", day);
            }
            continue;
        }

        // Parser la sortie
        let stdout = String::from_utf8_lossy(&output.stdout);
        let (part1_result, part1_time) = parse_part(&stdout, "Part 1");
        let (part2_result, part2_time) = parse_part(&stdout, "Part 2");

        let day_result = DayResult {
            day,
            part1_result,
            part1_time,
            part2_result,
            part2_time,
        };

        // Afficher le résultat du jour si pas en mode summary_only
        if !summary_only {
            println!("\nDay {:02}:", day);
            if let Some(r) = &day_result.part1_result {
                print!("  Part 1: {}", r);
                if let Some(t) = day_result.part1_time {
                    print!(" ({:.4}ms)", t);
                }
                println!();
            }
            if let Some(r) = &day_result.part2_result {
                print!("  Part 2: {}", r);
                if let Some(t) = day_result.part2_time {
                    print!(" ({:.4}ms)", t);
                }
                println!();
            }
            println!("  Total: {:.4}ms", day_result.total_time());
        }

        results.push(day_result);
    }

    // Afficher le bilan global
    if results.is_empty() {
        println!("\n📊 Aucun jour trouvé pour l'année {}", year);
        return Ok(());
    }

    let total_time: f64 = results.iter().map(|r| r.total_time()).sum();
    let avg_time = total_time / results.len() as f64;
    let fastest = results
        .iter()
        .min_by(|a, b| a.total_time().partial_cmp(&b.total_time()).unwrap());
    let slowest = results
        .iter()
        .max_by(|a, b| a.total_time().partial_cmp(&b.total_time()).unwrap());

    let mode = if release { " (mode release)" } else { "" };
    println!("\n📊 Bilan global{}:", mode);
    println!("  Jours complétés: {}/25", results.len());
    println!("  Temps total: {:.4}ms", total_time);
    println!("  Temps moyen: {:.4}ms/jour", avg_time);
    if let Some(f) = fastest {
        println!(
            "  Jour le plus rapide: Day {:02} ({:.4}ms)",
            f.day,
            f.total_time()
        );
    }
    if let Some(s) = slowest {
        println!(
            "  Jour le plus lent: Day {:02} ({:.4}ms)",
            s.day,
            s.total_time()
        );
    }

    Ok(())
}
