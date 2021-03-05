#!/bin/sh
cargo build --target=wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/RustWasmTest.wasm docs/