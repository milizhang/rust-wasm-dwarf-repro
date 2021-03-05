# Repro for Chromium: 1182658

This repo contains a minimal repro for [Chromium: 1182658](https://bugs.chromium.org/p/chromium/issues/detail?id=1182658).

On a system with WASM32 Rust compiler installed, use build.sh to build. Please serve with a HTTP server that handles MIME for WASM correctly (as application/wasm). Test with [docs/index.htm](docs/index.htm).