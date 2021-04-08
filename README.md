# DnD Switch

Instead of making a toaster out of Unifi USG by enabling QoS it is possible to lower the traffic for lower priority clients

## Prepare

    cargo install wasm-pack

## Build

    wasm-pack build --target web --out-name wasm --out-dir ./static

## Automatic recompilation during dev
    
    cargo install cargo-watch
    cargo-watch -s "wasm-pack build --target web --out-name wasm --out-dir ./static"