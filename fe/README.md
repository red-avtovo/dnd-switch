# Prepare

    cargo install wasm-pack
# Build

    wasm-pack build --no-typescript --target web --out-name wasm --out-dir ./static
    wasm-pack build --release --no-typescript --target web --out-name wasm --out-dir ./static


# Automatic recompilation during dev
    
    cargo install cargo-watch
    cargo-watch -s "wasm-pack build --no-typescript --target web --out-name wasm --out-dir ./static"