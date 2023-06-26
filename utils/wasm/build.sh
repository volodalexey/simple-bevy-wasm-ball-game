#!/bin/sh

set -ex

cargo build --release --target wasm32-unknown-unknown

rm -rf dist
mkdir dist
cp -r assets dist/assets
wasm-bindgen --out-dir dist/ --target web target/wasm32-unknown-unknown/release/follow-bevy-ball-game.wasm
cp target/wasm32-unknown-unknown/release/follow-bevy-ball-game.wasm dist/
cp utils/wasm/index.html dist/
cp utils/wasm/styles.css dist/
cp utils/wasm/app.js dist/
ls -lh dist
