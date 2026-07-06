# Getting Started

## Installing Rust

To install Rust, you can use the official installer called `rustup`. This tool manages Rust versions and associated tools. You can install it by running the following command in your terminal:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, you can verify that Rust is installed correctly by checking the version:

```sh
rustc --version
```

### Warning for Windows Users

On Windows, the process is slightly different, due to the existence of the `.exe` installer. If you are using Windows, follow the following step:

1. Go to the [Rust installation page](https://www.rust-lang.org/tools/install) and install the `.exe` installer.

## The Anatomy of a Rust Program

1. The `main` function is the entry point that runs in every Rust program.
2. The `println!` is classified as `Rust Macro`, which are a way to write code that **generates code to extend Rust syntax**. The `!` indicates that it is a macro rather than a function.

## Compiling and Running Rust Programs

For compiling and running basic Rust programs, you can use the `rustc` command. For example, if you have a file named `main.rs`, you can compile it with:

```sh
rustc main.rs
```

This will produce an executable file named `main` (or `main.exe` on Windows). You can then run the executable:

```sh
./main
```


## Using Cargo

Cargo is Rust's build system and package manager. It simplifies the process of managing dependencies, compiling code, and running tests. To create a new Rust project using Cargo, you can use the following command:

```sh
cargo new project_name
```

The `cargo new` command creates a new directory with the specified project name and initializes it with a basic Rust project structure, including a `Cargo.toml` file for managing dependencies.


### Building and Running with Cargo

To build and run your Rust project using Cargo, navigate to your project directory and use the following commands:

```sh
cargo build
cargo run
```

This will compile your project and execute the resulting binary. Cargo also handles dependencies specified in the `Cargo.toml` file, making it easier to manage external libraries. It is also generated a `Cargo.lock` file, which ensures that the same versions of dependencies are used across different builds, providing consistency and reproducibility.

### Building for Release

When you are ready to build your project for release, you can use the following command:

```sh
cargo build --release
```

## Comparing `Rust` and `JavaScript`

| Command | Rust | JavaScript |
|---------|------|------------|
| Install | `rustup` | `npm` |
| Build | `cargo build` | `npm run build` |
| Run | `cargo run` | `npm start` |
| Bootstrap | `cargo new` | `npx create-react-app` (or similar) |