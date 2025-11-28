# Guide des Tests

Ce document explique comment les tests sont organisés dans AOC Rustdolph et comment en ajouter de nouveaux.

## 📋 Table des matières

- [Architecture des tests](#architecture-des-tests)
- [Lancer les tests](#lancer-les-tests)
- [Tests existants](#tests-existants)
- [Ajouter de nouveaux tests](#ajouter-de-nouveaux-tests)
- [Dépendances de test](#dépendances-de-test)

## Architecture des tests

Les tests sont organisés dans un module `tests` à la fin de `mush/src/main.rs`. Ils utilisent plusieurs bibliothèques :

- **`tempfile`** : Crée des répertoires temporaires pour tester la création de fichiers
- **`mockito`** : Simule les requêtes HTTP pour tester `fetch_input`
- **`serial_test`** : Garantit que certains tests s'exécutent séquentiellement (nécessaire pour les tests qui modifient le répertoire courant)

## Lancer les tests

### Tous les tests

```bash
cargo test -p mush
```

### Tests en mode séquentiel (recommandé)

Certains tests modifient le répertoire courant et doivent s'exécuter séquentiellement :

```bash
cargo test -p mush -- --test-threads=1
```

### Un test spécifique

```bash
cargo test -p mush test_create_file_success
```

### Avec affichage des println!

```bash
cargo test -p mush -- --nocapture
```

### Vérifier la couverture

Pour voir quelles lignes sont couvertes, utilisez `cargo-tarpaulin` :

```bash
# Installation
cargo install cargo-tarpaulin

# Exécution
cargo tarpaulin -p mush --out Html
```

## Tests existants

### 1. Tests de `create_file()`

#### `test_create_file_success`
Vérifie que la création d'un fichier fonctionne correctement.

```rust
#[test]
fn test_create_file_success() { ... }
```

#### `test_create_file_already_exists`
Vérifie que `create_file` ne remplace pas un fichier existant.

```rust
#[test]
fn test_create_file_already_exists() { ... }
```

#### `test_create_file_creates_parent_dirs_not_required`
Vérifie que `create_file` échoue si les répertoires parents n'existent pas.

```rust
#[test]
fn test_create_file_creates_parent_dirs_not_required() { ... }
```

### 2. Tests de `initialize_workspace()`

#### `test_initialize_workspace`
Vérifie que l'initialisation crée tous les fichiers nécessaires avec le bon contenu.

```rust
#[test]
#[serial]
fn test_initialize_workspace() { ... }
```

**Note** : Utilise `#[serial]` car il modifie le répertoire courant.

### 3. Tests de `create_scaffold()`

#### `test_create_scaffold_structure`
Vérifie que `create_scaffold` crée la structure complète d'un jour.

```rust
#[test]
#[serial]
fn test_create_scaffold_structure() { ... }
```

#### `test_create_scaffold_with_double_digit_day`
Vérifie le formatage correct des jours à deux chiffres (ex: `day25`).

```rust
#[test]
#[serial]
fn test_create_scaffold_with_double_digit_day() { ... }
```

#### `test_scaffold_does_not_overwrite_existing_files`
Vérifie qu'un second appel à `create_scaffold` ne remplace pas les fichiers existants.

```rust
#[test]
#[serial]
fn test_scaffold_does_not_overwrite_existing_files() { ... }
```

### 4. Tests de `fetch_input()`

#### `test_fetch_input_missing_session`
Vérifie qu'une erreur est renvoyée si `AOC_SESSION` n'est pas défini.

```rust
#[test]
#[serial]
fn test_fetch_input_missing_session() { ... }
```

#### `test_fetch_input_with_mock_server`
Teste une requête HTTP réussie avec un serveur mock.

```rust
#[test]
#[serial]
fn test_fetch_input_with_mock_server() { ... }
```

#### `test_fetch_input_http_error`
Vérifie le comportement en cas d'erreur HTTP (404).

```rust
#[test]
#[serial]
fn test_fetch_input_http_error() { ... }
```

#### `test_fetch_input_trims_whitespace`
Vérifie que les espaces de fin sont correctement supprimés.

```rust
#[test]
#[serial]
fn test_fetch_input_trims_whitespace() { ... }
```

## Ajouter de nouveaux tests

### Template de base

```rust
#[test]
fn test_nom_du_test() {
    // Arrange : Préparer les données de test
    let input = "test data";

    // Act : Exécuter la fonction à tester
    let result = ma_fonction(input);

    // Assert : Vérifier le résultat
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "expected output");
}
```

### Tests avec répertoire temporaire

```rust
#[test]
fn test_avec_temp_dir() {
    let temp_dir = setup_temp_dir();
    let file_path = temp_dir.path().join("test.txt");

    // Votre test ici

    // Le répertoire temporaire est automatiquement supprimé
}
```

### Tests avec changement de répertoire

```rust
#[test]
#[serial]  // Important : utiliser #[serial]
fn test_avec_changement_dir() {
    with_temp_dir(|temp_dir| {
        // Le répertoire courant est maintenant temp_dir

        // Votre test ici

        // Le répertoire courant est automatiquement restauré
    });
}
```

### Tests avec mock HTTP

```rust
#[test]
#[serial]
fn test_http_mock() {
    use mockito::Server;

    env::set_var("AOC_SESSION", "test_cookie");

    let mut server = Server::new();
    let mock = server
        .mock("GET", "/2024/day/1/input")
        .with_status(200)
        .with_body("Test data")
        .create();

    let result = fetch_input_with_base_url(1, 2024, &server.url());

    assert!(result.is_ok());
    mock.assert();  // Vérifie que la requête a été faite

    env::remove_var("AOC_SESSION");
}
```

## Dépendances de test

Les dépendances de test sont déclarées dans `mush/Cargo.toml` :

```toml
[dev-dependencies]
tempfile = "3.8"      # Répertoires temporaires
mockito = "1.2"       # Mock de serveurs HTTP
serial_test = "3.0"   # Tests séquentiels
```

## Bonnes pratiques

1. **Isolation** : Chaque test doit être indépendant et ne pas dépendre de l'état d'autres tests
2. **Cleanup** : Utilisez `tempfile` pour les tests de fichiers (nettoyage automatique)
3. **Variables d'environnement** : Toujours nettoyer avec `env::remove_var()` après usage
4. **Nommage** : Utilisez des noms descriptifs : `test_<fonction>_<scenario>_<resultat_attendu>`
5. **Documentation** : Ajoutez un commentaire expliquant ce que teste chaque test

## Couverture actuelle

Au moment de la rédaction de ce document :

- **11 tests** au total
- **100% des fonctions principales** sont testées
- **Tous les cas d'erreur critiques** sont couverts

## Contribuer

Lors de l'ajout de nouvelles fonctionnalités :

1. Écrivez d'abord le test (TDD recommandé)
2. Assurez-vous que tous les tests passent
3. Vérifiez avec Clippy : `cargo clippy -p mush -- -D warnings`
4. Formatez le code : `cargo fmt`

Merci de contribuer ! 🦀✨
