#!/bin/sh

cargo build --release --bins --target wasm32-unknown-emscripten
cp ./target/wasm32-unknown-emscripten/release/*.wasm ./compiled