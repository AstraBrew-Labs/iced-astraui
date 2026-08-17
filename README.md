# AstraBrew Launcher for macOS

A native macOS launcher shell built with Rust, [iced](https://iced.rs), and the
[Lucide](https://lucide.dev) icon set.

## Requirements

- macOS 13 or newer
- Rust 1.88 or newer

## Run locally

```sh
cargo run
```

## Quality checks

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

The current launcher includes local navigation, release-channel selection, and
a launch-state prototype. Process management and update services can be added
behind the existing `Launcher::update` message boundary.

The application embeds every HarmonyOS Sans face in `assets/fonts` and uses
the Regular face as its global default. UI hierarchy maps to the bundled Light,
Medium, Bold, and Black faces, while Lucide keeps its dedicated icon font.
