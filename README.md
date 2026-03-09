# Rove Community Plugins

Welcome to the community extension hub for Rove!

This repository acts as a staging ground and distribution catalog for third-party WebAssembly agent tools. Rove allows you to extend the capabilities of your autonomous local AI agents completely securely via WASM compilation.

## 🤝 Contribution Guidelines

1. **Safety First**: Your plugin must compile strictly to `wasm32-unknown-unknown`. It should only rely on host-mapped functions exposed via `rove-sdk` rather than native OS API calls.
2. **Deterministic Inputs**: The provided tool configuration string (used by LLMs) must properly document its exact JSON signature schemas.
3. **Stateless Operations**: Memory should not be assumed perfectly persistent across rapid invocation cycles outside of standard `Extism` persistent buffers.

## 🚀 Creating a Tool

To start developing a new tool:

1. Copy an existing template.
2. Ensure `rove-sdk` is heavily utilized to inject results via `host::log` and `host::read`.
3. Submit a Pull Request. Once approved, the CI handles building and attaching to the global Rove manifest!

## 📦 Ecosystem Context

| System             | Technology                                                                             | Description                                  | Link                        |
| ------------------ | -------------------------------------------------------------------------------------- | -------------------------------------------- | --------------------------- |
| **Engine Core**    | <img src="https://cdn.simpleicons.org/rust/white" width="18" align="center"/> Rust     | The host orchestrator consuming these tools. | [`/core/`](../core/)        |
| **Developer Docs** | <img src="https://cdn.simpleicons.org/markdown/white" width="18" align="center"/> Docs | Comprehensive extension documentation.       | [`/docs/dev/`](./docs/dev/) |
