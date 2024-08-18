#!/bin/bash
# This script builds, optimizes and packages the game for wasm release

set -ex

EXAMPLE="indie-games-website-simulator"

RUSTFLAGS="-Zlocation-detail=none --cfg=web_sys_unstable_apis" cargo build --profile=wasm-release --no-default-features --target=wasm32-unknown-unknown -Z build-std-features=panic_immediate_abort -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size"
# RUSTFLAGS="--cfg=web_sys_unstable_apis" cargo build --no-default-features --target=wasm32-unknown-unknown --release

time wasm-bindgen --out-name "$EXAMPLE" --no-typescript --out-dir web-src/wasm --target web "target/wasm32-unknown-unknown/wasm-release/$EXAMPLE.wasm"
# time wasm-bindgen --out-name "$EXAMPLE" --no-typescript --out-dir web-src/wasm --target web "target/wasm32-unknown-unknown/release/$EXAMPLE.wasm"

# time wasm-opt -all -Oz -ol 10 -s 10 -o "web-src/wasm/${EXAMPLE}_bg.wasm" "web-src/wasm/${EXAMPLE}_bg.wasm" 

time gzip -9 -c "web-src/wasm/${EXAMPLE}_bg.wasm"  > "web-src/wasm/${EXAMPLE}_bg.wasm.gz"

time brotli -9 -c "web-src/wasm/${EXAMPLE}_bg.wasm" > "web-src/wasm/${EXAMPLE}_bg.wasm.br"

time cp -r assets web-src/

time zip -r release.zip web-src

echo "ALL DONE"
