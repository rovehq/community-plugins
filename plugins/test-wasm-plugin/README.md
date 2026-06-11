# Test Wasm Plugin

This is a generated Plugin plugin scaffold for Rove.

## Files

- `Cargo.toml` - Rust crate configured for `wasm32-wasip1`
- `manifest.json` - plugin manifest with placeholder signature
- `plugin-package.json` - install metadata with placeholder hash/signature
- `runtime.json` - tool catalog consumed by Rove
- `src/lib.rs` - plugin entry point exporting `run`
- `tests/integration.rs` - local unit test for the scaffold logic

## Authoring loop

1. `rustup target add wasm32-wasip1`
2. `cargo test`
3. `cargo build --target wasm32-wasip1 --release`
4. `rove plugin test test-wasm-plugin --input "hello"`

## Packaging and registry flow

1. `rove plugin pack test-wasm-plugin`
2. `rove plugin publish test-wasm-plugin --registry-dir ./registry`
3. `rove plugin install test-wasm-plugin --registry ./registry --version 0.1.0`

## Before install or publish

1. Build the wasm artifact at `target/wasm32-wasip1/release/test_wasm_plugin.wasm`
2. Replace the placeholder permissions in `manifest.json`
3. Compute the SHA256 of the built artifact and place it in `plugin-package.json`
4. Sign the built artifact and place the signature in `plugin-package.json`
5. Sign `manifest.json` and replace `signature`
6. Install with `rove plugin install test-wasm-plugin`
