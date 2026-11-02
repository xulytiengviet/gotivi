# Known limitations of GoTiVi 0.1.0

This is a source prototype, not a system IME. The Rust composer is intentionally small and does not yet cover full Vietnamese phonotactic validation, valid-word precedence (e.g. mix → mĩ), robust Telex escape, old/new tone placement, completed-word retroactive edits, all VIQR punctuation, canonical NFD input, application-specific ACK handling, password-field safeguards, automatic capitalization or configurable macros. The current Python sample is only a demonstration and should not be deployed publicly without hardening. There is no Windows keyboard hook, macOS IMK plugin, IBus/Fcitx5 adapter, Android/iOS keyboard or UEFI executable in this repository.

The user-supplied ViFastKey executables and dynamic library are reference artifacts only and were not copied, reverse-engineered, or packaged into GoTiVi. Numeric counts (13,337 core / 3,000 scenarios) are targets, not verified results.

The CI workflow must succeed and Pages must be configured for GitHub Actions before claiming a live WASM playground. Rust and wasm-pack were not available in the initial authoring environment.
