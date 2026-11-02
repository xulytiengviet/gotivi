# GoTiVi · Gõ tiếng Việt, mượt như ý

A small, original, local-first Vietnamese input-method **Rust core** with a WebAssembly playground, CLI, C ABI and optional Python/FastAPI bridge.

> **Status: experimental 0.1.0.** Source prototype, not a production system-wide IME. Windows keyboard hook, macOS IMK, Linux IBus/Fcitx5, mobile, UEFI, extensive language corpora and 13,337+ conformance cases are **roadmap**, not shipped capabilities.

## Highlights

- Telex, Simple Telex, VNI and VIQR; switch modes instantly.
- Rebuild each current word from its bounded raw keystrokes (`MAX_RAW = 24`).
- Tone order-independent within a word, UTF-8/NFC output and UTF-16 deletion lengths for adapters.
- Telex `dd`, `aa`, `ee`, `oo`, `aw`, `ow`, `uw`, quick `w`, `[` and `]`.
- English escape such as `mixx → mix`, macrolike `ko → không`, `vn → Việt Nam`.
- A single action per key: pass, replace, preedit or commit.
- No keystroke telemetry, no tracking, no network in the Rust core.

## Try it

When the GitHub Pages workflow succeeds and Pages is set to **GitHub Actions**, open
**https://xulytiengviet.github.io/gotivi/**.

## Build

```sh
cargo test
cargo run -- --demo
cargo run -- --text "Tieengs Vieejt"
cargo run -- --mode vni --text "Tie6ng1 Vie6t5"
cargo build --release
# WebAssembly
cargo install wasm-pack
wasm-pack build --target web --out-dir docs/pkg --features wasm
python3 -m http.server 8080 -d docs
```

Python bridge (requires the Rust dynamic library in `target/release`):

```sh
cargo build --release
pip install -r python/requirements.txt
uvicorn python.app:app --reload
# POST /api/type {"text":"Tieengs Vieejt","mode":"telex"}
```

## Architecture

```text
raw keys → Composer (max 24) → NFC Unicode → Action
                  ↓                   ↓
          mode / tone / macros    Rust CLI / WASM / C ABI
                                             ↓
                                       Python FastAPI
```

See [architecture](ARCHITECTURE.md), [web demo](docs/index.html), [C ABI](include/gotivi.h) and [known limitations](LIMITATIONS.md).

The two user-supplied ViFastKey Windows ZIPs were reviewed for public interfaces and usage concepts. No binary was redistributed and the GoTiVi implementation is written independently. ViFastKey product names and brand are not part of GoTiVi.

MIT © 2026 GoTiVi contributors.
