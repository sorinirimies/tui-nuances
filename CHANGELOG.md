# Changelog

All notable changes to this project are documented here. It is generated from
[Conventional Commits](https://www.conventionalcommits.org) by
[git-cliff](https://git-cliff.org) — do not edit it by hand.

## [0.1.11](https://github.com/sorinirimies/tui-nuances/compare/v0.1.10...v0.1.11) — 2026-11-10

### ✨ Features

- Reorder cycle with light pairs up front, add light + synthwave themes ([`380a7ac`](https://github.com/sorinirimies/tui-nuances/commit/380a7acdf48d422ccf3766a6dc58d8a91ee5e5ac))

### ♻️ Refactor

- Remove dead_code allows the build doesn't need; gate on unused dependencies ([`34b74c6`](https://github.com/sorinirimies/tui-nuances/commit/34b74c646f62c11adbfaa8d2b0867f0cad00b8c3))

### 📚 Documentation

- Drop the per-repo agent rules skill; the shared skills cover it ([`cc2d721`](https://github.com/sorinirimies/tui-nuances/commit/cc2d721d959142669c0f23646387d227e02053ea))
- This repo's agent rules as a project skill in .agents/skills ([`b71843f`](https://github.com/sorinirimies/tui-nuances/commit/b71843f0c03ebb99a244eb5def3310269c6c743f))
- **readme:** Add the Audit and Fuzz badges ([`c8db148`](https://github.com/sorinirimies/tui-nuances/commit/c8db1481153e7639f199817f3c9bdaefdd5381aa))
- Move agent rules from .zed/rules.md to AGENTS.md ([`17b3d27`](https://github.com/sorinirimies/tui-nuances/commit/17b3d276b1df5d4dd3de5ec1b74a008295cbba11))

### 📦 Dependencies

- Nightly cargo update 2026-11-10 ([`3ab878f`](https://github.com/sorinirimies/tui-nuances/commit/3ab878fd60a49dd6fa3874926ddef5dd660b4a2e))
- Nightly cargo update 2026-11-09 ([`e146cb4`](https://github.com/sorinirimies/tui-nuances/commit/e146cb464d9c08af550a63957e9ed6fe7d158dd7))
- Nightly cargo update 2026-11-07 ([`612dd68`](https://github.com/sorinirimies/tui-nuances/commit/612dd685514107220be9f1390ef4b4bc4c6200a5))
- Nightly cargo update 2026-11-06 ([`d1d5932`](https://github.com/sorinirimies/tui-nuances/commit/d1d59326f6bc8d10652fdf0162d81946890115e7))
- Update Cargo.lock ([`cb90f1d`](https://github.com/sorinirimies/tui-nuances/commit/cb90f1dbbf6eb5b6134f44568b836928a6afc851))
- Update Cargo.lock ([`2615f5d`](https://github.com/sorinirimies/tui-nuances/commit/2615f5da040a93b1dc1159b1e0553fb97f9006bf))
- Update Cargo.lock ([`8697d0a`](https://github.com/sorinirimies/tui-nuances/commit/8697d0a5ffc58f40174d74cd9dd7b24b1795d400))
- Update Cargo.lock ([`e06fe6c`](https://github.com/sorinirimies/tui-nuances/commit/e06fe6cbedb8fc580989dedb268c45c41229db7c))
- Update Cargo.lock ([`052914f`](https://github.com/sorinirimies/tui-nuances/commit/052914f627ca7d4ba85ef6da9b28eba7c1e6638e))

### 📦 Build

- Align tooling with the library template ([`3bd3146`](https://github.com/sorinirimies/tui-nuances/commit/3bd314659e264b57717247603c3b3755a0382497))

### 🔄 CI and release

- **gitea:** Check the MSRV on Gitea too, with the same command as GitHub ([`f621998`](https://github.com/sorinirimies/tui-nuances/commit/f621998c3dfdd924bb400915b7e82b8c374185a9))
- Download archives to a file so curl retries a dropped transfer, instead of piping into tar ([`52ab647`](https://github.com/sorinirimies/tui-nuances/commit/52ab64719b8fa59ee82b6ec85dbc34051aa64d5b))
- Bump softprops/action-gh-release from 2 to 3 (#5) ([`ffd9355`](https://github.com/sorinirimies/tui-nuances/commit/ffd93558bec11a32516068c66107f4dbeeeafac6))
- The Dependabot sweep asks for a rebase on conflicts and keeps going after a failed merge ([`a57d4fc`](https://github.com/sorinirimies/tui-nuances/commit/a57d4fcf2d20cde694d71182882bef451c801ef6))
- Merge green Dependabot PRs, majors included, with a nightly sweep; nightly commits skip ignored lockfiles ([`ccd9b8d`](https://github.com/sorinirimies/tui-nuances/commit/ccd9b8d5d35f692d21e3d6fd330aa05f16357915))
- Scan the git history for secrets with gitleaks ([`e1983f7`](https://github.com/sorinirimies/tui-nuances/commit/e1983f7d5d4689db945acea0e223e86cc3b72fc9))
- Bump dtolnay/rust-toolchain from 1.88.0 to 1.120.0 (#3) ([`de13d62`](https://github.com/sorinirimies/tui-nuances/commit/de13d62c5997f1a19401fba688948c0f819d1932))

### 🔧 Chores

- Remove AGENTS.md ([`476fe38`](https://github.com/sorinirimies/tui-nuances/commit/476fe3830ce4b3642fb2786a64370dda850464d9))

## [0.1.10](https://github.com/sorinirimies/tui-nuances/compare/v0.1.9...v0.1.10) — 2026-10-26

### ✨ Features

- Add 43 themes ported from nuance (90 built-ins) ([`ff7a64a`](https://github.com/sorinirimies/tui-nuances/commit/ff7a64a39432a11f047444e264634e87c7da8c2a))

## [0.1.9](https://github.com/sorinirimies/tui-nuances/compare/v0.1.8...v0.1.9) — 2026-10-25

### ✨ Features

- Highlight role (file-match color) in per-theme UI roles ([`4f96931`](https://github.com/sorinirimies/tui-nuances/commit/4f96931e2d18a5d9b2d92c8e808f7f8cf4aba979))

### 🔄 CI and release

- Bump actions/checkout from 4 to 7 (#2) ([`6d3c4f6`](https://github.com/sorinirimies/tui-nuances/commit/6d3c4f644f9158ded4a5a224c8567ee3fa97e78d))
- Bump softprops/action-gh-release from 2 to 3 (#1) ([`a4a3007`](https://github.com/sorinirimies/tui-nuances/commit/a4a3007b574ffb3eddf2e549ade4022422a29c7b))
- Bump clechasseur/rs-clippy-check from 4 to 5 (#4) ([`2138260`](https://github.com/sorinirimies/tui-nuances/commit/21382602d2bdd07009964fdb05143ca90ba42642))

## [0.1.8](https://github.com/sorinirimies/tui-nuances/compare/v0.1.7...v0.1.8) — 2026-10-25

### ✨ Features

- Per-theme UI role colors (title/primary/border_focused/selection/muted/success) for explorer-compatible rendering ([`5ac079c`](https://github.com/sorinirimies/tui-nuances/commit/5ac079cbc1f26c1be09ea46bb52a3b71fe773631))
- Add 31 themes ported from gitkraft (47 built-ins); accent-insensitive by_name ([`cb7ff8e`](https://github.com/sorinirimies/tui-nuances/commit/cb7ff8e9c1a42100c9e4b918f57130e21dd9d3f6))

### 📚 Documentation

- Add installs badge and crates.io install; chore: align justfile with tokenburn ([`5a17382`](https://github.com/sorinirimies/tui-nuances/commit/5a17382a38abc82fb0dfc5f196f0539ae0772c4d))

### 🔄 CI and release

- **gitea:** Nightly workflow-action bumps ([`2382b5b`](https://github.com/sorinirimies/tui-nuances/commit/2382b5b878bff956747497509a64d08603d93634))
- Automerge major dependabot bumps too ([`8db081f`](https://github.com/sorinirimies/tui-nuances/commit/8db081f65ee5baa83fe266102250a86bfc37bbd7))
- Automerge green PRs (workflow_run) + dependabot for actions ([`73b7b00`](https://github.com/sorinirimies/tui-nuances/commit/73b7b005a8bbb0a5ed829f59cfca730a108d1d1a))

## [0.1.7](https://github.com/sorinirimies/tui-nuances/compare/v0.1.6...v0.1.7) — 2026-10-25

### 📦 Dependencies

- Update Cargo.lock ([`69812b0`](https://github.com/sorinirimies/tui-nuances/commit/69812b0ab5f970ac4ae1817c885cb305028dfcc9))
- Nightly library update (#1) ([`9acb771`](https://github.com/sorinirimies/tui-nuances/commit/9acb7719a1c7236c02173f5bd9c813f35474cc6a))
- Nightly library update (#1) ([`3d24738`](https://github.com/sorinirimies/tui-nuances/commit/3d24738976718a280173e65f095f7a3bc0a5d619))
- Nightly library update (#1) ([`8f2afc7`](https://github.com/sorinirimies/tui-nuances/commit/8f2afc75e1dfa323023c56978c01ba68a1d715e5))
- Nightly library update (#1) ([`77917d1`](https://github.com/sorinirimies/tui-nuances/commit/77917d14c29ba565a5c20b930fa504cbd9135c96))

### 🔄 CI and release

- Nightly update pushes to main directly like tui-spinner (no manual-merge PR) ([`968f3a3`](https://github.com/sorinirimies/tui-nuances/commit/968f3a300c6d1f8f8f3df086fa5954a6280134d2))

### 🔧 Chores

- **justfile:** Push/pull current branch instead of hardcoded main ([`599ffe4`](https://github.com/sorinirimies/tui-nuances/commit/599ffe4f480eecc42e045e2745fc7f59bae910b5))

## [0.1.6](https://github.com/sorinirimies/tui-nuances/compare/v0.1.5...v0.1.6) — 2026-10-14

### 📚 Documentation

- Regenerate changelog ([`1414b7c`](https://github.com/sorinirimies/tui-nuances/commit/1414b7c92207258366993fe67702a99cc62df353))

### 🔄 CI and release

- Use prebuilt nushell; justfile: push/pull-all skip missing remotes ([`5af97fd`](https://github.com/sorinirimies/tui-nuances/commit/5af97fddaa40e49929c803c0035bdc71f9c70feb))

## [0.1.4](https://github.com/sorinirimies/tui-nuances/compare/v0.1.3...v0.1.4) — 2026-10-14

### 🔄 CI and release

- Install prebuilt Nushell binary (setup-nu needs Node 20) ([`c02511a`](https://github.com/sorinirimies/tui-nuances/commit/c02511a37aadfd15615607214a6d54c76cbe2127))

## [0.1.3](https://github.com/sorinirimies/tui-nuances/compare/v0.1.1...v0.1.3) — 2026-10-14

### 🐛 Bug fixes

- **ci:** Install rustfmt and clippy for the nushell bump test ([`f4fb811`](https://github.com/sorinirimies/tui-nuances/commit/f4fb8116d3ff7846b1c388c7c84ee53feb05368d))

### 🔄 CI and release

- Install Nushell with setup-nu instead of compiling it ([`b33e3d3`](https://github.com/sorinirimies/tui-nuances/commit/b33e3d3ac73c166271ab5719297df77eb701ef89))

### 🔧 Chores

- Remove setup_gitea script ([`2b62428`](https://github.com/sorinirimies/tui-nuances/commit/2b624281a096ccc5e3e80b7a5ff221d1e17d136f))

## [0.1.1](https://github.com/sorinirimies/tui-nuances/compare/v0.1.0...v0.1.1) — 2026-10-14

### 🔧 Chores

- Remove gitea migration script ([`404cfdf`](https://github.com/sorinirimies/tui-nuances/commit/404cfdf324fc0dc9dd7a522d692c79d1c607fb17))
- Remove gitea migration script ([`a30e170`](https://github.com/sorinirimies/tui-nuances/commit/a30e170e9bac62e245494e93b1358ba00587510b))

## [0.1.0](https://github.com/sorinirimies/tui-nuances/releases/tag/v0.1.0) — 2026-10-14

### ✨ Features

- Support ratatui 0.29..0.30 and MSRV 1.74 ([`3a9526f`](https://github.com/sorinirimies/tui-nuances/commit/3a9526f1469125952c63b538ba6f6d944ffce4fc))
- Add gallery, semantic and switcher examples with vhs tapes, fuzz targets and README ([`8bc7087`](https://github.com/sorinirimies/tui-nuances/commit/8bc70873d7db6f315561bed26102d6e914c2fc27))
- Add Theme, built-in themes, color math, styles, switcher, provider traits ([`fa37440`](https://github.com/sorinirimies/tui-nuances/commit/fa37440bee74bac7b77a79f53a2b6cf3c54ef80c))

### 🐛 Bug fixes

- **ci:** Expose CRATES_IO_TOKEN at job level so publish step runs ([`244789f`](https://github.com/sorinirimies/tui-nuances/commit/244789f8cf16bafd3bd661221ab8a50af5d8465d))

### 📚 Documentation

- Re-record demos showing results only; reference tui-nuances version ([`8845b71`](https://github.com/sorinirimies/tui-nuances/commit/8845b71f06d32aa1b7de43fc155c1bf854f9dfef))
- Add CHANGELOG ([`1222f39`](https://github.com/sorinirimies/tui-nuances/commit/1222f395209395ddf511d74e42a21f5a5f427e26))

### 🧪 Tests

- Add integration, serde and property tests ([`7f67f57`](https://github.com/sorinirimies/tui-nuances/commit/7f67f5751b603a556dec776301faa2e6455f5877))

### 🔧 Chores

- Add tooling, CI workflows, nushell scripts and docs ([`96beb18`](https://github.com/sorinirimies/tui-nuances/commit/96beb1860a67cee415d1185dfb5d3db93aa33b6b))


