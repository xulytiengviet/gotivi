# GoTiVi 0.2.0 · Gõ tiếng Việt bằng Rust

**Rust linguistic engine + deterministic composer + transactional text actions + thin platform adapters.**

GoTiVi là bộ lõi gõ tiếng Việt thử nghiệm **local-first**, không lưu nội dung gõ. Các module độc lập xử lý Telex, Simple Telex, VNI và VIQR; WebAssembly và Python đều sử dụng **cùng thư viện Rust**, không viết lại thuật toán.

## Trạng thái

- ✅ Rust linguistic engine (`src/linguistic.rs`): dấu thanh, dấu mũ, móc, `ươ`, Telex escape `mixx → mix`, tùy chọn dấu cũ/mới, macro lúc chốt từ.
- ✅ Composer có giới hạn `MAX_RAW=24`, xử lý từng phím, sửa bằng Backspace, đổi chế độ, xuất `Action`.
- ✅ Transaction `revision/ACK` + hợp đồng host phải kiểm tra văn bản trước khi thay thế.
- ✅ C ABI, CLI, WebAssembly playground theo từng phím, FastAPI tham chiếu.
- 🧪 Golden test & integration test trong `tests/`; GitHub Actions xác nhận từng commit.
- ⏳ Windows TSF/hook, macOS IMK, Linux IBus/Fcitx5, Android/iOS, UEFI vẫn là **roadmap**, chưa có bản cài.

**Các ví dụ**

| Mode | Raw | Unicode |
| --- | --- | --- |
| Telex | `Tieengs Vieejt` | `Tiếng Việt` |
| Telex | `thuowng` | `thương` |
| Telex | `mixx` | `mix` |
| Telex | `ko vn` | `không Việt Nam` |
| VNI | `tie6ng1` | `tiếng` |
| VIQR | `tie^'ng` | `tiếng` |

## Build CLI + Rust core

```sh
git clone https://github.com/xulytiengviet/gotivi
cd gotivi
cargo test
cargo build --release
cargo run -- --demo
cargo run -- --mode vni --text "tie6ng1"
```

## WASM playground

```sh
cargo install wasm-pack --locked
wasm-pack build --target web --out-dir docs/pkg --features wasm
python3 -m http.server 8080 -d docs
```

Open http://localhost:8080. After Pages workflow completes and Pages source is set to **GitHub Actions**, the deployment target is https://xulytiengviet.github.io/gotivi/.

The playground has a plain ASCII-to-Unicode batch converter and an actual per-key transaction simulator with ACK trace, mode switching, undo-by-raw-Backspace, copy and reset. It does **not** intercept system keyboard events.

## Python/FastAPI reference adapter

```sh
cargo build --release
python -m pip install -r python/requirements.txt
python -m uvicorn python.app:app --reload
```

```http
POST /api/type
Content-Type: application/json

{"text":"Tieengs Vieejt","mode":"telex"}
```

`python/app.py` calls the Rust shared library through C ABI; it is NOT an independent linguistic oracle and is not production-hardened.

## Contract / documentation

- [Four-layer architecture](ARCHITECTURE.md)
- [C ABI header](include/gotivi.h)
- [Tests](tests)
- [Known limitations](LIMITATIONS.md)
- [MIT license](LICENSE)

The original user-provided ViFastKey Windows ZIPs are not redistributed. GoTiVi is an independent source implementation and not an official ViFastKey release.

Maintainer: GoTiVi contributors · 2026 · MIT.
