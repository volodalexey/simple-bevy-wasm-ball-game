#!/bin/sh

set -ex
# initial wasm build
cargo build --release --target wasm32-unknown-unknown
# prepare js module that loads wasm
mkdir -p target/wasm32-unknown-unknown/bindgen
wasm-bindgen --out-dir target/wasm32-unknown-unknown/bindgen/ --target web target/wasm32-unknown-unknown/release/simple-bevy-wasm-ball-game.wasm

mkdir -p dist
# remove all files in dist folder except .git folder
find dist -mindepth 1 -not \( -path '*/.git' \) -not \( -path '*/.git/*' \) -exec rm -rf {} +
cp -r assets dist/assets
cp target/wasm32-unknown-unknown/bindgen/simple-bevy-wasm-ball-game_bg.wasm dist/
cp target/wasm32-unknown-unknown/bindgen/simple-bevy-wasm-ball-game.js dist/
cp utils/wasm/index.html dist/
cp utils/wasm/styles.css dist/
cp utils/wasm/*.js dist/
ls -la dist
