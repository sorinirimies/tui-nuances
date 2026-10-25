# tui-nuances — task runner
# Install just:      cargo install just
# Install vhs:       brew install vhs  OR  go install github.com/charmbracelet/vhs@latest
# Install git-cliff: cargo install git-cliff
# Usage: just <task>
# ── Default ───────────────────────────────────────────────────────────────────

default:
    @just --list

# ── Tool checks ───────────────────────────────────────────────────────────────

_check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { \
        echo "❌ git-cliff not found. Install with: cargo install git-cliff"; exit 1; \
    }

# Check nu (nushell) is available
_check-nu:
    @command -v nu >/dev/null 2>&1 || { \
        echo "❌ nu (nushell) not found. Install: https://www.nushell.sh"; exit 1; \
    }

_check-vhs:
    @command -v vhs >/dev/null 2>&1 || { \
        echo "❌ vhs not found."; \
        echo "   macOS:      brew install vhs"; \
        echo "   Any:        go install github.com/charmbracelet/vhs@latest"; \
        exit 1; \
    }

# Install all recommended development tools
install-tools:
    @echo "Installing development tools…"
    @command -v git-cliff >/dev/null 2>&1 || cargo install git-cliff --locked
    @command -v nu >/dev/null 2>&1 || cargo install nu --locked
    @echo "✅ All tools installed!"

# ── Build ─────────────────────────────────────────────────────────────────────

# Build the crate (dev)
build:
    cargo build

# Build the release version
build-release:
    cargo build --release

# ── Run ───────────────────────────────────────────────────────────────────────

# Run the gallery example
example-gallery:
    cargo run --example gallery

# Run the semantic example
example-semantic:
    cargo run --example semantic

# Run the switcher example
example-switcher:
    cargo run --example switcher

# Alias: default run launches the gallery
run: example-gallery

# ── Test ──────────────────────────────────────────────────────────────────────

# Run the full test suite
test:
    cargo test --locked --all-features --all-targets

# Run Nu script tests
test-nu: _check-nu
    nu scripts/tests/run_all.nu

# Run both Rust and Nu tests
test-all-nu: test test-nu
    @echo "✅ All Rust and Nu tests passed!"

# Line coverage (needs cargo-llvm-cov); writes target/llvm-cov/html
coverage:
    cargo llvm-cov --all-features --html
    @echo "→ target/llvm-cov/html/index.html"

# ── Code quality ──────────────────────────────────────────────────────────────

# Check without building
check:
    cargo check --all-features

# Format all code
fmt:
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Run clippy
clippy:
    cargo clippy --all-targets --all-features -- -D warnings -A deprecated

# Run all quality checks (format, clippy, test, nu, audit) — must pass before a release.

# Auto-formats first, then verifies no changes remain (catches unstaged format diffs).
check-all: fmt clippy test test-nu audit
    @echo "🔍 Verifying formatting is clean…"
    cargo fmt --all -- --check
    @echo "✅ All checks passed!"

# Full pre-release quality gate — everything in check-all plus fuzz smoke and a release build.
check-release: check-all fuzz-smoke build-release
    @echo "✅ Release quality gate passed (fmt + clippy + test + nu + audit + fuzz + release build)!"

# ── Security & fuzzing ────────────────────────────────────────────────────────

# Security: dependency vulnerability scan (needs `cargo install cargo-audit`)
audit:
    cargo audit

# Security: licenses, advisories, bans, sources (needs `cargo install cargo-deny`)
deny:
    cargo deny check

# Build all fuzz targets (needs nightly + cargo-fuzz)
fuzz-build:
    cargo +nightly fuzz build

# Fuzz a target (theme_json, from_base, color_math, name_lookup): just fuzz theme_json
fuzz target="theme_json" secs="60":
    cargo +nightly fuzz run {{ target }} -- -max_total_time={{ secs }}

# Short fuzz run of every target (needs nightly + cargo-fuzz)
fuzz-smoke secs="10":
    #!/usr/bin/env bash
    set -euo pipefail
    for t in $(cargo +nightly fuzz list); do
        cargo +nightly fuzz run "$t" -- -max_total_time={{ secs }}
    done

# ── VHS Demo GIFs ─────────────────────────────────────────────────────────────

VHS := "examples/vhs"

# Generate every demo GIF (all tapes in examples/vhs)
vhs-all: _check-vhs _check-nu
    nu scripts/generate_all_tapes.nu

# Render a single tape by name (e.g. just vhs-tape gallery)
vhs-tape name: _check-vhs
    #!/usr/bin/env sh
    if [ -f "{{ VHS }}/{{ name }}.tape" ]; then
        vhs "{{ VHS }}/{{ name }}.tape" && echo "✅ Done."
    else
        echo "❌ Tape not found: {{ name }}.tape"
        just vhs-list
        exit 1
    fi

# List all VHS tapes
vhs-list:
    @ls {{ VHS }}/*.tape | sed 's|.*/||; s|\.tape||' | sed 's/^/  /'

# ── Documentation ─────────────────────────────────────────────────────────────

# Generate and open the docs
doc:
    cargo doc --no-deps --all-features --open

# ── Changelog ─────────────────────────────────────────────────────────────────

# Regenerate the full CHANGELOG.md from all tags
changelog: _check-git-cliff
    @echo "Generating full changelog…"
    git-cliff --output CHANGELOG.md
    @echo "✅ CHANGELOG.md updated."

# Prepend only unreleased commits to CHANGELOG.md
changelog-unreleased: _check-git-cliff
    git-cliff --unreleased --prepend CHANGELOG.md
    @echo "✅ Unreleased changes prepended."

# Preview changelog for the next release without writing the file
changelog-preview: _check-git-cliff
    @git-cliff --unreleased

# Show the latest tagged release entry (no file write)
changelog-latest: _check-git-cliff
    @git-cliff --latest

# ── Version bump ─────────────────────────────────────────────────────────────

# Bump the version, regenerate CHANGELOG.md, commit and tag — only after the full release gate passes.
bump version: check-release _check-git-cliff _check-nu
    nu scripts/bump_version.nu --yes {{ version }}

# ── Publish (crates.io) ───────────────────────────────────────────────────────

# Run the full pre-publish readiness check (fmt, clippy, tests, docs, dry-run)
check-publish: _check-nu
    nu scripts/check_publish.nu

# Dry-run publish
publish-dry: check-all
    cargo publish --dry-run

# Publish to crates.io
publish: check-all
    @echo "📦 Publishing tui-nuances…"
    cargo publish
    @echo "✅ tui-nuances published to crates.io!"

# Show what would be released without making any changes
release-preview: _check-git-cliff
    @echo "Current version: $(just version)"
    @echo ""
    @echo "Unreleased commits:"
    @git-cliff --unreleased

# ── Housekeeping ──────────────────────────────────────────────────────────────

# Remove build artifacts
clean:
    cargo clean

# Update all dependencies (Cargo.lock only)
update:
    cargo update

# Update dependencies, run the full quality gate, then commit and push if all green.
update-deps:
    @echo "⬆️  Updating dependencies…"
    cargo update
    @echo "🔍 Running quality gate…"
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings -A deprecated
    cargo test --locked --all-features --all-targets
    @echo "✅ All checks passed — committing dependency updates…"
    git add Cargo.lock
    git diff --cached --quiet || git commit -m "chore: update dependencies"
    git push origin "$(git branch --show-current)"
    @echo "✅ Dependency updates pushed to GitHub."

# Upgrade dependency pins (dry run), sync the lock file, run the quality gate
upgrade-deps: _check-nu
    nu scripts/upgrade_deps.nu --dry-run

# Show outdated dependencies (requires cargo-outdated)
outdated:
    cargo outdated

# Show the current version
version: _check-nu
    @nu scripts/version.nu

# Show project info
info:
    @echo "Project: tui-nuances"
    @echo "Version: $(just version)"
    @echo "License: MIT"

# Show all configured remotes
remotes:
    @git remote -v

# ── Git remotes & pushing ────────────────────────────────────────────────────
# tui-nuances lives on GitHub (origin) and the Gitea Starscream instance. Every
# *-all recipe loops over REMOTES and keeps going when one fails.

REMOTES := "origin gitea-starscream"

# Push the current branch to GitHub (origin)
push:
    git push origin "$(git branch --show-current)"

# Push the current branch to Gitea Starscream
push-gitea-starscream:
    git push gitea-starscream "$(git branch --show-current)"

# Push the current branch to every remote (continues on failure)
push-all:
    #!/usr/bin/env sh
    failed=""; skipped=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured here; add with: git remote add $r <url>)"; skipped="$skipped $r"; continue; }
        git push "$r" "$(git branch --show-current)" || failed="$failed $r"
    done
    if [ -n "$skipped" ]; then echo "ℹ️  Skipped (not configured):$skipped"; fi
    if [ -n "$failed" ]; then echo "⚠️  Failed to push to:$failed"; exit 1; fi
    echo "✅ Pushed to: {{ REMOTES }}"

# Force-push the current branch to every remote (leases are NOT used — be sure!)
push-all-force:
    #!/usr/bin/env sh
    failed=""; skipped=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured here; add with: git remote add $r <url>)"; skipped="$skipped $r"; continue; }
        git push --force "$r" "$(git branch --show-current)" || failed="$failed $r"
    done
    if [ -n "$skipped" ]; then echo "ℹ️  Skipped (not configured):$skipped"; fi
    if [ -n "$failed" ]; then echo "⚠️  Failed to force-push to:$failed"; exit 1; fi
    echo "✅ Force-pushed to: {{ REMOTES }}"

# Pull the current branch from GitHub (origin)
pull:
    git pull origin "$(git branch --show-current)"

# Pull the current branch from Gitea Starscream
pull-gitea-starscream:
    git pull gitea-starscream "$(git branch --show-current)"

# Pull the current branch from every remote (continues on failure)
pull-all:
    #!/usr/bin/env sh
    failed=""; skipped=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured here; add with: git remote add $r <url>)"; skipped="$skipped $r"; continue; }
        git pull "$r" "$(git branch --show-current)" || failed="$failed $r"
    done
    if [ -n "$skipped" ]; then echo "ℹ️  Skipped (not configured):$skipped"; fi
    if [ -n "$failed" ]; then echo "⚠️  Failed to pull from:$failed"; exit 1; fi
    echo "✅ Pulled from: {{ REMOTES }}"

# Push all tags to GitHub
push-tags:
    git push origin --tags

# Push all tags to every remote (continues on failure)
push-tags-all:
    #!/usr/bin/env sh
    failed=""; skipped=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured here; add with: git remote add $r <url>)"; skipped="$skipped $r"; continue; }
        git push "$r" --tags || failed="$failed $r"
    done
    if [ -n "$skipped" ]; then echo "ℹ️  Skipped (not configured):$skipped"; fi
    if [ -n "$failed" ]; then echo "⚠️  Failed to push tags to:$failed"; exit 1; fi
    echo "✅ Tags pushed to: {{ REMOTES }}"

# Force-sync Gitea Starscream with GitHub (main + tags)
sync-gitea-starscream:
    git push gitea-starscream "$(git branch --show-current)" --force
    git push gitea-starscream --tags --force
    @echo "✅ Gitea Starscream force-synced."

# Add the origin and Gitea Starscream remotes if missing
setup-remotes:
    #!/usr/bin/env sh
    git remote get-url origin >/dev/null 2>&1 || git remote add origin git@github.com:sorinirimies/tui-nuances.git
    git remote get-url gitea-starscream >/dev/null 2>&1 || git remote add gitea-starscream gitea@192.168.1.44:sorin/tui-nuances.git
    git remote -v

# ── Release workflows ─────────────────────────────────────────────────────────
# Pushing a vX.Y.Z tag starts the Release workflow on whichever host receives it
# (GitHub: .github/workflows/release.yml, Gitea: .gitea/workflows/release.yml).

# Bump, commit, tag, then push to GitHub only — triggers the GitHub Release workflow.
release version: (bump version)
    git push --follow-tags origin "$(git branch --show-current)"
    @echo "✅ Release v{{ version }} pushed to GitHub."

# Bump, commit, tag, then push to every remote — triggers the Release workflows.
release-all version: (bump version)
    #!/usr/bin/env sh
    failed=""; skipped=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured here; add with: git remote add $r <url>)"; skipped="$skipped $r"; continue; }
        git push --follow-tags "$r" "$(git branch --show-current)" || failed="$failed $r"
    done
    if [ -n "$skipped" ]; then echo "ℹ️  Skipped (not configured):$skipped"; fi
    if [ -n "$failed" ]; then echo "⚠️  Release v{{ version }} failed to push to:$failed"; exit 1; fi
    echo "✅ Release v{{ version }} pushed to: {{ REMOTES }}"

# Bump, commit, tag, then push to Gitea Starscream only.
release-gitea-starscream version: (bump version)
    git push --follow-tags gitea-starscream "$(git branch --show-current)"
    @echo "✅ Release v{{ version }} pushed to Gitea Starscream."

# Push the latest commit and all tags to GitHub only (no bump, no gate beyond check-all).
push-release: check-all
    git push --follow-tags origin "$(git branch --show-current)"
    @echo "✅ Latest commit + tags pushed to GitHub."

# Push the latest commit and all tags to every remote (no bump, continues on failure).
push-release-all: check-all
    #!/usr/bin/env sh
    failed=""; skipped=""
    for r in {{ REMOTES }}; do
        git remote get-url "$r" >/dev/null 2>&1 || { echo "↷ skipping $r (not configured here; add with: git remote add $r <url>)"; skipped="$skipped $r"; continue; }
        git push --follow-tags "$r" "$(git branch --show-current)" || failed="$failed $r"
    done
    if [ -n "$skipped" ]; then echo "ℹ️  Skipped (not configured):$skipped"; fi
    if [ -n "$failed" ]; then echo "⚠️  Failed to push to:$failed"; exit 1; fi
    echo "✅ Latest commit + tags pushed to: {{ REMOTES }}"

# Manually re-trigger the GitHub Release workflow for an existing tag via the gh CLI.
release-retrigger version:
    @command -v gh >/dev/null 2>&1 || { \
        echo "❌ GitHub CLI (gh) not found. Install from https://cli.github.com"; exit 1; \
    }
    @echo "Manually dispatching Release workflow for tag v{{ version }}…"
    gh workflow run release.yml --field tag=v{{ version }}
    @echo "✅ Dispatched — check progress at: https://github.com/$(gh repo view --json nameWithOwner -q .nameWithOwner)/actions"

# Run the nightly dependency update workflow on GitHub on demand.
deps-update-now:
    gh workflow run deps-update.yml
