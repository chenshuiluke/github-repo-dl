**You need to have a GitHub personal access token from here: https://github.com/settings/tokens**

# Compilation

1. Install cargo and rust.
2. `cd` into the directory where you downloaded `github-repo-dl`.
3. Run `cargo build --release`.
4. You will find the `github-repo-dl` binary in `target/release`.

# Usage

## Token Only

    github-repo-dl <token>

## Token and Download Directory

    github-repo-dl <token> <directory>

# Useful Commands

## Removing all files except source files

    shopt -s extglob ; sudo rm -r !(Cargo.lock|Cargo.toml|src|target|README.md)
