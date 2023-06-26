# Compile to wasm

## Add WASM support to your Rust installation
```sh
rustup target install wasm32-unknown-unknown
```

## Build for WebAssembly

### Install [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) CLI
```sh
cargo install wasm-bindgen-cli
```

### Run build script

```sh
./utils/wasm/build.sh
```