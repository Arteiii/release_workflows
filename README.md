# Release Workflows

## Features

- Initialize and manage Git repositories using `git2`
- Expose Git operations as HTTP endpoints with POEM.

## Project Structure

```
root/
│
├── src/
│ ├── main.rs
│ │
│ ├── git/
│ │ ├── mod.rs
│ │ └── ...
│ │
│ ├── api/
│ │ ├── mod.rs
│ │ └── ...
│ │
│ └── util/
│   ├── mod.rs
│   └── ...
│
├── .env
├── Cargo.toml
├── LICENSE
└── README.md
```

[//]: # ()

[//]: # (## Getting Started)

[//]: # ()

[//]: # (1. Clone the repository:)

[//]: # ()

[//]: # (     ```bash)

[//]: # (     git clone https://github.com/Arteiii/release_workflows.git)

[//]: # (     cd my_git_origin)

[//]: # (     ```)

[//]: # ()

[//]: # (2. Build the project:)

[//]: # ()

[//]: # (    ```bash)

[//]: # (    cargo build)

[//]: # (    ```)

[//]: # ()

[//]: # (3. Run the application:)

[//]: # ()

[//]: # (    ```bash)

[//]: # (    cargo run)

[//]: # (    ```)

[//]: # ()

[//]: # (Access the API at http://localhost:3000.)

[//]: # ()

[//]: # (## Usage)

[//]: # ()

[//]: # (### Create a new Git repository:)

[//]: # ()

[//]: # (```bash)

[//]: # (curl -X POST http://localhost:3000/create_repo/my_repository)

[//]: # (```)

## Contributing

If you'd like to contribute to this project, please follow these guidelines:

1. Fork the repository.
2. Create a new branch: git checkout -b feature/my-feature.
3. Make your changes and commit them: git commit -m 'Add new feature'.
4. Push to the branch: git push origin feature/my-feature.
5. Submit a pull request.

## [LICENSE](LICENSE)

This project is licensed under the [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.html) License.

