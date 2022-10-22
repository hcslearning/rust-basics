# Rust Learning

## Compile

```
SOURCE_CODE_FILE=./main.rs
rustc $SOURCE_CODE_FILE
```

## Cargo 

Cargo is a Build System and Package Manager for Rust.

### Create Project
```
# crea un proyecto
PROJECT_NAME=mi_proyecto
cargo new $PROJECT_NAME
```

### Build Project
```
cargo build
```

### Build and Run Project
```
cargo run 
```

## Docker

```
RUST_IMAGE=rust:1.64.0-slim-bullseye
WORKDIR=/home/zero/Documents/Aprendiendo/Rust/rust-basics
podman rm rust-vm
podman run -it --name rust-vm -v $WORKDIR $RUST_IMAGE /bin/bash
```