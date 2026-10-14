# Changelog
All notable changes to this project will be documented in this file.

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

