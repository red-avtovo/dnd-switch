# DnD Switch

Instead of making a toaster out of Unifi USG by enabling QoS it is possible to lower the traffic for lower priority clients

## Prepare

    cargo install wasm-pack

## Build

    wasm-pack build --target web --out-name wasm --out-dir ./static

## Automatic recompilation during dev
    
    cargo install cargo-watch
    cargo-watch -s "wasm-pack build --target web --out-name wasm --out-dir ./static"

## Debug ci

    docker run --rm -it -v $(pwd):/opt ubuntu:latest bash

## ArmV7 Building Note

- install compiler `gcc-arm-linux-gnueabihf`
- specify linker `arm-linux-gnueabihf-gcc` in `.cargo/config`
- add openssl vendored in Cargo.toml

## Run on Arm

    docker run -it --name=dnd \
        --rm -e UNIFI_URL=https://localhost:8443 \
        -e UNIFI_USER=admin 
        -e UNIFI_PWD=admin123 
        -e UNIFI_GROUP=De-prio 
        -p 8000:80 -p 8001:8080 \
        redavtovo/dnd-switch:arm_latest