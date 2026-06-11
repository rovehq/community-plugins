# Test Native Driver

This is a generated native Plugin extension scaffold for Rove.

## Files

- `Cargo.toml` - Rust crate configured for a native `cdylib`
- `manifest.json` - extension manifest with placeholder signature
- `plugin-package.json` - install metadata with placeholder hash/signature
- `runtime.json` - tool catalog consumed by Rove
- `src/lib.rs` - native tool entry point exporting `create_tool`
- `tests/integration.rs` - local unit test for the scaffold logic

## Authoring loop

1. Place this package under `core/tools/<name>` or adjust the `sdk` path in `Cargo.toml`
2. `cargo test`
3. `cargo build --release`
4. `rove native test test-native-driver --input "hello"`

## Packaging and registry flow

1. `rove native pack test-native-driver`
2. `rove native publish test-native-driver --registry-dir ./registry`
3. `rove native install test-native-driver --registry ./registry --version 0.1.0`

## Before install or publish

1. Build the native artifact with `cargo build --release`
2. Replace the placeholder permissions in `manifest.json`
3. Compute the SHA256 of the built artifact and place it in `plugin-package.json`
4. Sign the built artifact and place the signature in `plugin-package.json`
5. Sign `manifest.json` and replace `signature`
6. Install with `rove native install test-native-driver`
