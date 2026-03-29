# Contributing to Upstream Wayle

Guidelines for contributing PRs to [Jas-SinghFSU/wayle](https://github.com/Jas-SinghFSU/wayle).
Check upstream for a CONTRIBUTING.md or PR template before submitting — these notes supplement any formal guidelines.

## PR Title

Conventional commit format. Scope is optional for subsystem disambiguation.

```
feat(shell): add reboot button to dashboard dropdown
fix: also load icons from filepaths for the systray
docs: fix build instructions
```

## PR Body

Include:

1. **Motivation** — why this change exists (1-2 sentences)
2. **What it does** — brief description
3. **Config examples** — TOML code blocks when adding config options
4. **Screenshots** — required for visual changes (use `<img>` tags with width/height)
5. **Issue linking** — `Closes #N` or `Related to #N` when applicable

Scale detail with complexity. Small fixes get 1-2 sentences; features get a paragraph plus config examples and screenshots.

## Review Expectations

- Expect at least one revision round on feature PRs
- Config key naming is scrutinized — match existing conventions
- Architectural consistency matters — follow existing codebase patterns
- Keep PRs tightly scoped

## Workflow

Per CLAUDE.md:

```bash
git checkout master && git checkout -b feat/my-change
# make changes, commit
git push origin feat/my-change
gh pr create --repo Jas-SinghFSU/wayle
```
