# GoTiVi architecture (0.2.0)

## Four boundaries

1. **Rust linguistic engine** (`src/linguistic.rs`): stateless `compose(raw, config)`, `finish(raw, config)`, Telex/Simple Telex/VNI/VIQR, macro expansion at commit, Unicode NFC, traditional/modern tone placement. `analyse_syllable` is a **heuristic shape**, not a complete Vietnamese dictionary.
2. **Deterministic composer** (`src/composer.rs`): bounded current raw keys (`MAX_RAW=24`), render from scratch, backspace from raw, atomic boundary commit, revisioned actions and ACK.
3. **Transactional action**: `Action {kind,revision,expected,delete_utf16,insert,pass_key}`. Host must compare preceding text, apply edit atomically and acknowledge `ack(revision,true)`; on failure `ack(revision,false)` enters desynchronized state until `reset()`. While awaiting ACK, further keys return `Busy`; an adapter must queue or retry them, never silently drop them.
4. **Thin adapter** (`src/adapter.rs`): `TextHost` and a reference `MemoryHost`, plus profile policies. Actual OS IME implementations remain future work.

## Action contract

- `Pass`: OS should handle key when `pass_key=true`. A `Pass` with `pass_key=false` means internal state changed without a visible edit.
- `Replace`: replace exactly `expected` preceding host text with `insert`; `delete_utf16` must match prior text's length in UTF-16 code units.
- `Preedit`: adapter-specific composing text; `delete_utf16=0`, expected composing string still provided for consistency.
- `Commit`: one atomic replacement including word finalization and delimiter; macro expansion occurs here.
- `Busy`: ACK pending or rejected, never apply as a normal keystroke.

A single `Action` may contain a replacement involving multiple Unicode characters. The renderer never performs direct deletion itself.

## C ABI

`include/gotivi.h` provides opaque session pointers, `gotivi_feed`, `gotivi_ack`, `gotivi_switch_mode`, `gotivi_reset`. UTF-8 text pointers are borrowed until the next FFI call for that session. Sessions are not thread-safe. Calls must originate on the owning adapter's thread, and all foreign pointers are trusted; do not use untrusted native pointers.

## Browser

`WasmComposer` exposes `feed/ack/switch_mode/reset`; the docs demonstrate event-by-event host integration with an ACK trace. The browser demo is NOT an operating-system input method. Text is processed locally and not sent to the API.

## Application compatibility

Terminal: preedit when supported; browsers: checked replacement and ACK; Firefox: avoid editing committed text until verified; games and password fields: pass-through/disabled. `Profile` is descriptive policy, not OS process detection.

## Security

No raw key logging or network I/O in the Rust core. The Python API is an optional service, and any deployment requires HTTPS, request limits, origin policy and user-controlled data retention. No system-wide keyboard hook is shipped.
