# Wayle Enhancement Ideas

Collected from the initial Wayle adoption session. These are changes we'd like to make to the Wayle source — some are upstream PR candidates, others are fork-only.

## Module Enhancements

### CPU/RAM dynamic color thresholds
The built-in CPU and RAM modules have no threshold/color-change behavior. They should change icon and label color based on usage level, following the attention hierarchy principle (normal is invisible, color means attention needed):
- Below 70%: default foreground color (monochrome, blends in)
- 70-90%: amber (`#d97706`) — warning
- Above 90%: red (`#f43f5e`) — critical

This is how Waybar's `states` system worked and it was effective. The config could look like:
```toml
[modules.cpu]
warning-threshold = 70
critical-threshold = 90
warning-color = "yellow"
critical-color = "red"
```

### CPU/RAM sparkline widget
A built-in sparkline/mini-chart widget that shows rolling history using Unicode block characters (▁▂▃▄▅▆▇█) or a proper Cairo-drawn chart. We prototyped this with a custom module + shell script, but a native implementation would be smoother, more performant, and could support hover-to-expand.

### CPU format — remove zero-padding
`{{ percent }}` is hardcoded to `{:02.0}` (zero-padded to 2 digits) in `crates/wayle-shell/src/shell/bar/modules/cpu/helpers.rs`. This should be configurable or default to no padding. Zero-padding prevents jitter but looks unnatural for single-digit values.

### Media module auto-hide
The built-in media module has no way to hide when nothing is playing. We worked around this with a custom module using `playerctl --follow` + `hide-if-empty`, but native auto-hide would be better. A `hide-when-stopped` config option would solve this.

## Theming / Styling

### Notification popup contrast
The notification popup body text uses `--fg-muted` on `--bg-elevated`. With dark themes where these values are close, contrast can be unreadable. Either:
- Use `--fg-default` for body text instead of `--fg-muted`
- Add a minimum contrast ratio check
- Make notification text colors configurable via TOML

### Per-module icon color override for `basic` variant
When `button-variant = "basic"` and `icon-color` is set to a non-Auto value, the config is respected. But the `auto_icon_color` fallback is hardcoded per-module (e.g., Green for notifications). If a user sets `icon-color = "fg-default"` but the value resolves through the Auto path for any reason, they get the hardcoded color. Consider making `auto_icon_color` configurable or defaulting to `FgDefault` for the `basic` variant.

## Config / DX

### Silent config key mismatches
TOML section names that don't match Rust struct field names are silently ignored. For example, `[modules.notifications]` (plural) is silently dropped — only `[modules.notification]` (singular) works. A config validation warning on startup would save debugging time.

### Display-adaptive layout
No built-in way to switch layouts based on screen effective width (resolution/scale). Waybar had this via a launcher script that selected between `config.jsonc` and `config-compact.jsonc`. Wayle's per-monitor layouts with `extends` get partway there, but automatic compact/full switching based on pixel width thresholds would be valuable.

## Upstream PR Candidates
- Config key mismatch warnings
- Media auto-hide option
- CPU/RAM threshold colors
- Notification text contrast fix

## Fork-Only
- CLAUDE.md
- packaging/PKGBUILD
- Sparkline widget (if too opinionated for upstream)
