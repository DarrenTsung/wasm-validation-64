# wasm-validation-64
Test-case repository for wasm-bindgen issue (Validation error: Trying to access parent frame stack values.)

Rustc Version (using `rustup override set nightly` on directory):
```
~/Rust/wasm-validation-64 master
❯ rustc --version
rustc 1.27.0-nightly (f0fdaba04 2018-05-15)
```

Wasm-bindgen version:
```
~/Rust/wasm-validation-64 master*
❯ wasm-bindgen --version
wasm-bindgen 0.2.9
```

Run failing case with:
`./run-bindgen.sh`

My output from `run-bindgen.sh`:
```
~/Rust/wasm-validation-64 master*
❯ ./run-bindgen.sh
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
error: failed to create wasmi module
	caused by: Validation: Function #165 validation error: Trying to access parent frame stack values.
```
