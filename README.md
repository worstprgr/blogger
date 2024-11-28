# Blogger
Builds a static blog using \*.md files

## Usage
`blogger -e <Entry Title>` -> Creates a new \*.md file for today, and opens neovim (currently only on Windows)  
`blogger -u` -> Builds the blog

## Build
With make:

`make` -> Builds for the current OS  
`make linux-static` -> Builds for x86_64 linux (statically linked)

`cargo build --release`  

Static Linux binary (needs "rustup target add x86_64-unknown-linux-musl")     
`cargo build --release --target x86_64-unknown-linux-musl`

## Dependencies
Check `cargo.toml` or `cargo tree`  
