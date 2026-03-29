# CLAUDE.md — Wayle (agrahamlincoln fork)

## Project Overview

Wayle is a compositor-agnostic Wayland desktop shell (statusbar + notification daemon) written in Rust with Relm4/GTK4. This is a personal fork of [wayle-rs/wayle](https://github.com/wayle-rs/wayle).

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

Requires: GTK4 dev headers, gtk4-layer-shell, gtksourceview5, libpulse, pipewire, fftw, cmake, clang, pkg-config.

**PKGBUILD note:** makepkg's `LDFLAGS`/`CFLAGS` break `aws-lc-sys` compilation. The PKGBUILD unsets these in `build()`.

## Git Workflow

### Remotes
- `origin` — `agrahamlincoln/wayle` (this fork)
- `upstream` — `wayle-rs/wayle` (mainline)

### Branches
- `master` — tracks upstream master, kept in sync
- `agrahamlincoln` — fork-only branch with CLAUDE.md, packaging/, and any fork-specific changes. This is what we build packages from.
- Feature branches (`feat/...`) — for changes intended to be submitted upstream as PRs

### Contributing upstream
1. Branch from `master`: `git checkout master && git checkout -b feat/my-change`
2. Make changes, commit, push: `git push origin feat/my-change`
3. Open PR against upstream: `gh pr create --repo wayle-rs/wayle`
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

## Packaging & Deployment

Package name is `wayle-agrahamlincoln` to avoid conflict with AUR `wayle-git`.
Registered on the tatara build server (grahamcube) for remote builds.

### Release workflow
```bash
# 1. Commit and push changes to the agrahamlincoln branch
git push origin agrahamlincoln

# 2. Tag the release
git tag v0.X.Y && git push origin v0.X.Y

# 3. Build remotely on grahamcube (builds, packages, and publishes to tatara repo)
tatara build-remote wayle-agrahamlincoln v0.X.Y --server grahamcube

# 4. Install/update on any machine with tatara repo configured
sudo pacman -Syu wayle-agrahamlincoln
```

### Local build (for development iteration)
```bash
# Build a local package
tatara build

# Install the package (kill panel first, then install and restart)
pkill wayle-shell
sudo pacman -U packaging/wayle-agrahamlincoln-*.pkg.tar.zst
wayle panel start &
```

**Note:** Do not copy binaries directly (`sudo cp target/release/wayle-shell /usr/bin/`). The binary is locked while the process runs and it bypasses package management.

### Build server notes
- grahamcube must have build deps installed: `gtk4 gtk4-layer-shell libpulse pipewire fftw clang cmake cargo pkg-config`
- The PKGBUILD inits git submodules (cava) and unsets `LDFLAGS`/`CFLAGS` to fix `aws-lc-sys` linking
- Remote builds use the `agrahamlincoln` branch (set as default on the fork)
- Force-pushed tags require manual `git fetch --tags -f` on the build server if cached
