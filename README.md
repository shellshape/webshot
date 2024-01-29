# Rust CLI Template

This is a simple Rust CLI template which I am using for my projects like those in my [shellshape](https://github.com/shellshape) collection.

This template is based on [Clap](https://crates.io/crates/clap) in combination with [Figment](https://crates.io/crates/figment) and [dirs](https://crates.io/crates/dirs) for parsing configuration files. Also, [anyhow](https://crates.io/crates/anyhow) is used for basic error handling.

Feel free to use this template as basis for your own projects!

## Usage

Simply clone this repository and run the `init.sh` script with the name of your project. This will set the specified name in the `Cargo.toml`, will delete the `LICENSE` and overwrite the `README.md`. Also, it will remove the `.git` folder and re-initialize a new repository (only if you cloned this repository; not if you used the "use template" feature from GitHub).