# Astra UI: AI development context

This file is the detailed, repository-local context for coding assistants working on Astra UI. Read `README.md` for the user-facing introduction and `llms.txt` for the short project index.

## Project identity

- Project: Astra UI
- Cargo package: `iced-astraui`
- Rust crate: `astra_ui`
- Version: `0.0.1`
- Language and edition: Rust 2024
- Minimum Rust version: 1.85
- UI framework: iced 0.14
- License: MIT
- Repository: <https://github.com/AstraBrew-Labs/iced-astraui>
- Status: early development; API changes are possible during `0.x`

Astra UI is a native desktop component library and design system for iced. It is inspired by HeroUI v3 but is not a Web component port and has no official relationship with HeroUI. Components must follow iced's application-owned state and message-driven update model.

## Source map

- `src/lib.rs`: crate entry point; exposes `components`, `fonts`, `icons`, and `ui`; re-exports all component APIs at the crate root.
- `src/ui.rs`: compatibility facade that re-exports `crate::components::*`.
- `src/components/mod.rs`: includes the component implementation files into one shared module.
- `src/components/tokens.rs`: shared colors, dimensions, semantic variants, `MotionState`, and `app_theme()`.
- `src/components/*.rs`: component implementations and related style functions.
- `src/fonts.rs`: six embedded HarmonyOS Sans weights and `FONT_MAPPINGS`.
- `src/icons.rs`: converts Lucide glyphs to iced elements.
- `examples/showcase.rs`: executable Showcase setup, font registration, icon loading, and window configuration.
- `examples/showcase/app.rs`: Showcase state, update logic, component examples, tokens, patterns, and tests.
- `assets/fonts/`: embedded HarmonyOS Sans font files.
- `assets/icon/`: project and Showcase icons.

There is no production binary in `src/main.rs`. The repository's runnable application is the `showcase` example.

## Public import paths

These forms are supported:

```rust
use astra_ui::{Alert, AlertKind, Card};
use astra_ui::ui::{app_theme, button_style, ButtonVariant};
use astra_ui::prelude::*;
```

Prefer crate-root imports in new user documentation. Keep `astra_ui::ui::*` working because it is the compatibility facade.

## Component inventory

### Content and layout

- `Avatar`: label, image or custom fallback; circle, rounded and square shapes; semantic colors and three sizes.
- `badge`: dot, count or label attached to an element at any corner.
- `Card`: content with optional header and footer; transparent, default, secondary and tertiary variants.
- `chip`: compact semantic label.
- `Kbd`, `KbdKey`, `KbdPlatform`, `KbdVariant`, and `kbd`: compact keyboard shortcuts with macOS and Windows modifier labels, special/navigation keys, and default/light treatments.
- `Separator`: horizontal or vertical separator with default, secondary and tertiary color variants.
- `Typography`: heading, paragraph, code and copyable text behavior.
- `ScrollShadow`: scrollable content with configurable edge shadows and visibility callbacks.

### Input and action

- `button_style` and `button_style_animated`: styles for iced buttons using `ButtonVariant`.
- `checkbox_style` and `checkbox_style_animated`: styles for iced checkboxes.
- `Label` and `label`: semantic form labels with required, disabled and invalid states.
- `TextArea` and `text_area`: controlled multiline editors backed by `iced::widget::text_editor::Content`.
- `InputOtp` (`InputOTP` alias) and `input_otp`: controlled one-character OTP slots with primary/secondary variants and separators. `InputOtp::new` reports an `InputOtpChange` containing the latest value, edited slot, `Input`/`Backspace` action, and stable next-focus ID; applications can return `iced::widget::operation::focus(change.focus_id)` from `update`. The `input_otp` convenience helper keeps a value-only callback for compatibility.
- `radio`: Astra-styled radio control.
- `slider`: Astra-styled numeric slider.
- `switch`: binary switch.
- `toggle_button` and `toggle_button_group`: standalone or grouped selection controls.
- `tag_group`: removable or selectable grouped tags.
- `text_input_style`, `pick_list_style`, `pick_list_menu_style`: integration styles for iced primitives.

### Navigation and commands

- `Accordion` and `AccordionItem`: single or multiple expanded items; default or surface appearance.
- `disclosure`: one expandable section.
- `pagination` and pagination primitives: page navigation with previous, next and ellipsis items.
- `Tabs`, `TabItem` (`Tab` alias), and `tabs`: controlled tab list with primary/secondary variants, horizontal/vertical orientation, disabled items, separators, and selected panels. Use `.selected(...)` and `.on_selection_change(...)` from application state.
- `ListBox`, `ListBoxItem`, `ListBoxSection`, and `list_box`: controlled single/multiple/no-selection lists with descriptions, leading content, sections, disabled rows, danger variants, and indicators.
- `tab` and `tab_animated`: low-level styles for tab buttons kept for compatibility.
- `toolbar`: grouped command surface.

### Layout

- `Surface`, `SurfaceVariant`, and `surface`: semantic default, secondary, tertiary and transparent surfaces.

### Feedback and overlays

- `Alert`: persistent inline status with optional description, indicator, action and close message.
- `global_message`, `global_message_animated`, `global_message_animated_with_phase`, and `global_message_animated_with_placement`: transient app-level status messages with placement-aware progress-driven transitions shared with Toast.
- `ProgressBar` and `ProgressCircle`: determinate or indeterminate progress with sizes and semantic colors.
- `toast`, `toast_animated`, `toast_animated_with_placement`, and `toast_region`: transient notifications with configurable `ToastPlacement`; placement-aware transitions use vertical motion for centered top/bottom regions and horizontal motion for corner regions, retracing the entry path while closing.
- `dropdown` and `context_menu`: caller-controlled menus using `MenuItem`.
- `tooltip`: helper text with configurable placement.
- `AlertDialog`, `global_modal`, `global_modal_animated`, `global_modal_with_options`, and `global_modal_with_options_animated`: caller-controlled modal overlays with progress-driven slide and backdrop transitions.
- `Drawer`, `DrawerPlacement`, `DrawerBackdrop`, `DrawerOptions`, and `drawer`: caller-controlled edge-aligned overlays with top/bottom/left/right placement, opaque/blur/transparent backdrops, scrollable bodies, optional handles and footers, configurable backdrop dismissal, and `animation_progress` transitions. `Drawer::new` takes `title`, `body`, `on_close`, and `on_interact`; the latter consumes clicks inside the panel and on non-dismissable backdrops.
- `GlobalLayer` and `global_layer`: consistent overlay ordering.

## Design system rules

- Reuse tokens from `src/components/tokens.rs`; do not introduce isolated colors, radii or control heights when an existing semantic token fits.
- Preserve the restrained neutral canvas, white surfaces, blue accent, and semantic success/warning/danger colors.
- Keep component state in the application. Components emit `Message` values and must not hide business state in global storage.
- Cover applicable default, hover, pressed, focused, selected and disabled states.
- Keep components composable with standard `iced::widget` types and return `Element` where a builder type is unnecessary.
- Use Lucide icons through `lucide-icons`; do not hand-draw replacement icon geometry when a suitable glyph exists.
- Keep motion brief and optional. The application owns `MotionState` and subscriptions; style functions remain stateless.
- Public APIs and source documentation are English. User-facing Showcase text may demonstrate both English and Chinese rendering.
- Comments should explain non-obvious rendering, layout or state decisions rather than narrating straightforward code.

## Fonts and icons

Register embedded fonts before setting the Astra default font:

```rust
let mut application = iced::application(App::new, App::update, App::view);

for (_, bytes) in astra_ui::fonts::FONT_MAPPINGS {
    application = application.font(bytes);
}

application.default_font(astra_ui::fonts::REGULAR).run()
```

Available font constants are `THIN`, `LIGHT`, `REGULAR`, `MEDIUM`, `BOLD`, and `BLACK`.

For Lucide icons, add the `lucide-icons` dependency and register `lucide_icons::LUCIDE_FONT_BYTES` on the iced application builder. Use `astra_ui::icons::icon` when a glyph must be returned as an `Element`.

## State and overlay model

The caller owns values such as selected tab, expanded accordion items, switch state, open menu, active modal, toast queue, and expiration time. Component callbacks produce application messages. The Showcase is the canonical reference for update logic.

Overlays should be composed in a predictable order through `GlobalLayer`. Dismissal, focus behavior, and timeout behavior belong to application state unless a component API explicitly handles rendering-level interaction.

## Adding or changing a component

1. Inspect `tokens.rs` and similar existing components before designing an API.
2. Add the implementation under `src/components/` and include it from `src/components/mod.rs`.
3. Keep the public API accessible from both the crate root and `astra_ui::ui` through the existing re-exports.
4. Add focused unit tests for state helpers and edge cases.
5. Add an interactive example to the appropriate Showcase page.
6. Update `README.md`, this file, `llms.txt`, and `CHANGELOG.md` when public behavior changes.
7. Run formatting, checks, tests and documentation validation.

## Commands

Fast compile check:

```bash
cargo check --all-targets
```

Full local validation:

```bash
cargo fmt --check
cargo test --all-targets
cargo doc --no-deps
cargo package
```

Run the interactive Showcase:

```bash
cargo run --release --example showcase
```

Do not treat the Showcase launch as a replacement for compile checks or tests. Running it opens a native desktop window and therefore requires a graphical session.

## Known constraints

- macOS is the primary development and verification platform at present; avoid claiming complete cross-platform validation without testing it.
- The API is pre-1.0 and may change.
- Version 0.0.1 is not currently listed on crates.io; user installation examples should use the Git repository or a local path until it is published.
- The project currently uses one shared components module assembled with `include!`; preserve this structure unless a deliberate public API migration is requested.
- The deprecation warning for a user-level `~/.cargo/config` and future-incompatibility notices from transitive dependencies are environment/upstream issues, not warnings emitted by Astra UI source.

## Canonical references

- User guide: `README.md`
- Compact AI index: `llms.txt`
- Release history: `CHANGELOG.md`
- Package metadata and dependency versions: `Cargo.toml`
- Complete usage example: `examples/showcase.rs` and `examples/showcase/app.rs`
