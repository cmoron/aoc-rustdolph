use anyhow::{Context, Result};

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
pub fn fetch_input(day: u8, year: u16) -> Result<String> {
    fetch_input_with_base_url(day, year, "https://adventofcode.com")
}

/// Version interne de fetch_input permettant de spécifier l'URL de base (pour les tests).
pub fn fetch_input_with_base_url(day: u8, year: u16, base_url: &str) -> Result<String> {
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

    Ok(text)
}
