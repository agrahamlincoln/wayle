# CLAUDE.md — Wayle (agrahamlincoln fork)

## Project Overview

Wayle is a compositor-agnostic Wayland desktop shell (statusbar + notification daemon) written in Rust with Relm4/GTK4. This is a personal fork of [Jas-SinghFSU/wayle](https://github.com/Jas-SinghFSU/wayle).

## Repository Layout

- `wayle/` — CLI binary (`wayle` command)
- `crates/wayle-shell/` — GUI panel binary (`wayle-shell`)
- `crates/wayle-config/` — TOML config schemas (defines all configurable options)
- `crates/wayle-styling/` — SCSS theme system (compiled into binary at build time)
- `crates/wayle-widgets/` — Reusable GTK4/Relm4 UI components (bar buttons, containers)
- `crates/wayle-hyprland/` — Hyprland IPC integration
- `crates/wayle-notification/` — freedesktop.org notification daemon
- `crates/wayle-icons/` — SVG icon CDN fetching and GTK registration
- `packaging/` — Arch Linux PKGBUILD (fork-only, not upstream)

## Build

```bash
cargo build --release
# Binaries: target/release/wayle, target/release/wayle-shell
```

Requires: GTK4 dev headers, gtk4-layer-shell, libpulse, pipewire, fftw, cmake, clang, pkg-config.

**PKGBUILD note:** makepkg's `LDFLAGS`/`CFLAGS` break `aws-lc-sys` compilation. The PKGBUILD unsets these in `build()`.

## Git Workflow

### Remotes
- `origin` — `agrahamlincoln/wayle` (this fork)
- `upstream` — `Jas-SinghFSU/wayle` (mainline)

### Branches
- `master` — tracks upstream master, kept in sync
- `agrahamlincoln` — fork-only branch with CLAUDE.md, packaging/, and any fork-specific changes. This is what we build packages from.
- Feature branches (`feat/...`) — for changes intended to be submitted upstream as PRs

### Contributing upstream
1. Branch from `master`: `git checkout master && git checkout -b feat/my-change`
2. Make changes, commit, push: `git push origin feat/my-change`
3. Open PR against upstream: `gh pr create --repo Jas-SinghFSU/wayle`
4. After merging upstream, sync: `git fetch upstream && git checkout master && git merge upstream/master`

### Syncing upstream into fork
```bash
git fetch upstream
git checkout master && git merge upstream/master && git push origin master
git checkout agrahamlincoln && git merge master
```

## Architecture Notes

### Adding/modifying a bar module
1. Config schema: `crates/wayle-config/src/schemas/modules/<module>/mod.rs`
2. Shell implementation: `crates/wayle-shell/src/shell/bar/modules/<module>/`
3. Styling: `crates/wayle-styling/scss/modules/`
4. The `BarButton` widget handles all button rendering — colors come from `BarButtonColors` struct
5. `auto_icon_color` is a hardcoded fallback when `icon-color` config is `Auto` — set per-module in the module's `mod.rs`

### Config key naming
TOML section names must match the Rust struct field names exactly (e.g., `[modules.notification]` not `[modules.notifications]`). Mismatched names are silently ignored.

### Theming
- SCSS is compiled at build time via `grass` crate — changes require rebuild
- Runtime theming uses CSS custom properties set from the palette config
- `--fg-muted` and `--fg-subtle` are used extensively in dropdowns/notifications — ensure palette values have sufficient contrast against `--bg-elevated`

## Packaging

```bash
# Build package
cd ~/projects/wayle && tatara build

# Publish to tatara repo
tatara publish packaging/wayle-agrahamlincoln-*.pkg.tar.zst --server grahamcube

# Install on any machine
pacman -S wayle-agrahamlincoln
```

Package name is `wayle-agrahamlincoln` to avoid conflict with AUR `wayle-git`.
