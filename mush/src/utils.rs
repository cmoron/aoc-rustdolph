use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

/// Crée un fichier avec le contenu spécifié si celui-ci n'existe pas déjà.
///
/// Si le fichier existe déjà, affiche un avertissement et ne fait rien.
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
/// - L'écriture dans le fichier échoue
pub fn create_file(path: &Path, content: &str) -> Result<()> {
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
