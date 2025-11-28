# Guide de contribution

Merci de votre intérêt pour contribuer à AOC Rustdolph ! 🎄

## Comment contribuer

### Signaler un bug

Si vous trouvez un bug, ouvrez une [issue](https://github.com/cmoron/aoc-rustdolph/issues) avec :
- Une description claire du problème
- Les étapes pour reproduire le bug
- Le comportement attendu vs. le comportement observé
- Votre environnement (OS, version de Rust)

### Proposer une fonctionnalité

Pour proposer une nouvelle fonctionnalité :
1. Vérifiez qu'elle n'existe pas déjà dans les issues
2. Ouvrez une issue décrivant votre idée
3. Expliquez pourquoi cette fonctionnalité serait utile

### Soumettre une Pull Request

1. **Fork** le projet
2. **Créez une branche** pour votre fonctionnalité :
   ```bash
   git checkout -b feature/ma-super-fonctionnalite
   ```
3. **Commitez vos changements** avec des messages clairs :
   ```bash
   git commit -m "feat: ajoute la possibilité de ..."
   ```
4. **Testez** que tout fonctionne :
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt -- --check
   ```
5. **Poussez** vers votre fork :
   ```bash
   git push origin feature/ma-super-fonctionnalite
   ```
6. **Ouvrez une Pull Request** sur le repo principal

## Standards de code

### Style

- Utilisez `cargo fmt` pour formater le code
- Passez `cargo clippy` sans warnings
- Ajoutez des doc comments (`///`) pour les fonctions publiques
- Écrivez des messages de commit clairs (idéalement en suivant [Conventional Commits](https://www.conventionalcommits.org/))

### Tests

- Ajoutez des tests pour les nouvelles fonctionnalités
- Assurez-vous que tous les tests passent avec `cargo test`

### Documentation

- Documentez les nouvelles fonctions avec des doc comments
- Mettez à jour le README.md si nécessaire
- Ajoutez des exemples d'utilisation

## Code de conduite

- Soyez respectueux et constructif
- Acceptez les critiques de manière constructive
- Concentrez-vous sur ce qui est le mieux pour la communauté

## Questions ?

N'hésitez pas à ouvrir une issue pour toute question !

Merci de contribuer ! 🦀✨
