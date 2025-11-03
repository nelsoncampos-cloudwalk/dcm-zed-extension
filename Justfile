#!/usr/bin/env just --justfile

# Default recipe to display help
default:
    @just --list

compile:
    #!/usr/bin/env bash
    rustup target add wasm32-wasip2
    cargo build --target wasm32-wasip2 --release    
    cp target/wasm32-wasip2/release/dcm.wasm extension.wasm