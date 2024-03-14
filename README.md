# Read Me

Tested on WSL2 Ubuntu 22.04.3 LTS.

`pkg-config` and `GCC` might be needed to build on Linux.

Rust is required for building the project. You can find the installer [here](https://www.rust-lang.org/tools/install).

If you dont want to build it yourself, you can download the compiled executable from the GitHub release page.

- **NOTE:** To generate static documentation, run the following command:
   ```bash
   cargo run --release -- --static-docs
   ```

## Features

- Initialize and manage Git repositories using `git2`.
- Expose Git operations as HTTP endpoints with Poem.

## Compiling the Project on Debian and Ubuntu 22.04.3 LTS

To compile the project on Debian and Ubuntu 22.04.3 LTS, follow these steps:

Update the package manager:

   ```bash
   sudo apt update
   ```

Install `pkg-config`, `GCC`,  `make` and `libssl-dev`:

```bash
sudo apt install pkg-config gcc make libssl-dev
```

Install Rust by following the instructions [here](https://rustup.rs/).

Clone the repository:

```bash
git clone https://github.com/your_username/release_workflows.git
```

Navigate to the project directory:

```bash
cd release_workflows
```

Build the project:

```bash
cargo build --release
```

If the build is successful, you will find the compiled executable in the target/release directory

## Contributing

If youd like to contribute to this project, please follow these guidelines:

- Fork the repository.
- Create a new branch: `git checkout -b feature/my-feature`.
- Make your changes and commit them: `git commit -m "Add new feature"`.
- Push to the branch: `git push origin feature/my-feature`.
- Submit a pull request.

## LICENSE

This project is licensed under the AGPL-3.0 [License](LICENSE).
