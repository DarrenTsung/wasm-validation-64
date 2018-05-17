#!/bin/bash

cargo build --target wasm32-unknown-unknown && wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_validation_64.wasm --out-dir .
