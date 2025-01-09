# IAM Project

Ce projet est une démonstration de l'intégration de Keycloak avec une application Rust pour gérer l'authentification et la déconnexion des utilisateurs.

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) et Cargo installés.
- Keycloak installé et configuré.
- Un navigateur web pour accéder aux pages d'authentification et de déconnexion.

## Configuration de Keycloak

1. **Créer un Realm** :
    - Connectez-vous à l'interface d'administration de Keycloak.
    - Créez un nouveau Realm appelé `myrealm`.

2. **Créer un Client** :
    - Dans le Realm `myrealm`, créez un nouveau client appelé `myclient`.
    - Configurez l'URL de redirection du client sur `http://localhost:8000/auth/callback`.

3. **Configurer les Identifiants du Client** :
    - Notez l'ID client (`client_id`) et le secret client (`client_secret`).

## Structure du Projet
```sh
iam_test/
├── Cargo.toml
├── src/
│ ├── main.rs
│ └── server.rs
└── response.html
```

## Installation et Lancement

1. Clonez le dépôt :

    ```sh
    git clone https://github.com/ines1102/IAM_Projet.git
    cd iam_test
    ```

2. Compilez et démarrez le serveur :

    ```sh
    cargo run --bin server
    ```

3. Ouvrez un nouveau terminal et générez l'URL d'authentification :

    ```sh
    cargo run --bin iam_test
    ```

4. Copiez l'URL générée et ouvrez-la dans votre navigateur pour vous authentifier via Keycloak.

5. Après l'authentification, vous serez redirigé vers la page de réponse `response.html`.

## Utilisation

### Authentification

Accédez à l'URL générée par la commande `cargo run --bin iam_test` pour vous authentifier via Keycloak.

### Déconnexion

Sur la page `response.html`, cliquez sur le lien "Logout" pour vous déconnecter. Cela invalidera votre session et vous redirigera vers la page de connexion de Keycloak.

## Fichiers Importants

- **Cargo.toml** : Fichier de configuration pour les dépendances Rust.
- **src/main.rs** : Génère l'URL d'authentification pour Keycloak.
- **src/server.rs** : Gère les redirections après l'authentification et la déconnexion.
- **response.html** : Page HTML affichée après l'authentification, avec un lien de déconnexion.

## Dépendances

Les dépendances principales utilisées dans ce projet sont :

- [openidconnect](https://crates.io/crates/openidconnect)
- [reqwest](https://crates.io/crates/reqwest)
- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)

## Support

Pour toute question ou problème, veuillez ouvrir une issue sur le dépôt ou contacter l'auteur.

---