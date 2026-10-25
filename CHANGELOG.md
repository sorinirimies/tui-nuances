# Changelog
All notable changes to this project will be documented in this file.

## [0.1.9] - 2026-10-25

### Features
- Highlight role (file-match color) in per-theme UI roles

### Ci
- Bump clechasseur/rs-clippy-check from 4 to 5 (#4)
- Bump softprops/action-gh-release from 2 to 3 (#1)
- Bump actions/checkout from 4 to 7 (#2)

## [0.1.8] - 2026-10-25

### Documentation
- Add installs badge and crates.io install; chore: align justfile with tokenburn

### Features
- Add 31 themes ported from gitkraft (47 built-ins); accent-insensitive by_name
- Per-theme UI role colors (title/primary/border_focused/selection/muted/success) for explorer-compatible rendering

### Miscellaneous
- Bump version to 0.1.8

### Ci
- Automerge green PRs (workflow_run) + dependabot for actions
- Automerge major dependabot bumps too
- Nightly workflow-action bumps

## [0.1.7] - 2026-10-25

### Miscellaneous
- Push/pull current branch instead of hardcoded main
- Nightly library update (#1)
- Nightly library update (#1)
- Nightly library update (#1)
- Nightly library update (#1)
- Update Cargo.lock
- Bump version to 0.1.7

### Ci
- Nightly update pushes to main directly like tui-spinner (no manual-merge PR)

## [0.1.6] - 2026-10-14

### Documentation
- Regenerate changelog

### Miscellaneous
- Bump version to 0.1.6

### Ci
- Use prebuilt nushell; justfile: push/pull-all skip missing remotes

## [0.1.5] - 2026-10-14

### Miscellaneous
- Bump version to 0.1.5

## [0.1.4] - 2026-10-14

### Miscellaneous
- Bump version to 0.1.4

### Ci
- Install prebuilt Nushell binary (setup-nu needs Node 20)

## [0.1.3] - 2026-10-14

### Bug Fixes
- Install rustfmt and clippy for the nushell bump test

### Miscellaneous
- Remove setup_gitea script
- Bump version to 0.1.3

### Ci
- Install Nushell with setup-nu instead of compiling it

## [0.1.1] - 2026-10-14

### Miscellaneous
- Remove gitea migration script
- Remove gitea migration script
- Bump version to 0.1.1

## [0.1.0] - 2026-10-14

### Bug Fixes
- Expose CRATES_IO_TOKEN at job level so publish step runs

### Documentation
- Add CHANGELOG
- Re-record demos showing results only; reference tui-nuances version

### Features
- Add Theme, built-in themes, color math, styles, switcher, provider traits
- Add gallery, semantic and switcher examples with vhs tapes, fuzz targets and README
- Support ratatui 0.29..0.30 and MSRV 1.74

### Miscellaneous
- Add tooling, CI workflows, nushell scripts and docs

### Test
- Add integration, serde and property tests

