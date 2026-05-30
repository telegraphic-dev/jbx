---
name: jbx
description: Single agent-friendly entry point to the Java ecosystem.
---

# jbx

jbx is the single agent-friendly entry point to the Java ecosystem: Java script running, Maven tool execution, testing, formatting, publishing, cache management, documentation sidecars, diagnostics, and JDK handling behind one CLI.

Use this top-level skill only to orient yourself. For a concrete task, fetch the command-specific skill with `jbx skill get jbx-<command>`.

## Install

```sh
curl -fsSL https://jbx.telegraphic.dev/install.sh | bash
export PATH="$HOME/.jbx/bin:$PATH"
jbx --version
```

In a jbx repository checkout, prefer the local binary so behavior matches the code under edit:

```sh
cargo run --locked -- --version
cargo run --locked -- <command>
```

## Thin top-level entry point

Start agents with bundled skills, then use the dedicated command surface:

```sh
jbx skill list
jbx skill list --json
jbx skill get jbx-check
jbx skill get jbx-doctor
jbx skill get jbx-docs
```

The bare `jbx <script.java|GAV>` form is for the common run path:

```sh
jbx Hello.java world
jbx dev.telegraphic:hello-tool:1.0.0 -- --help
```

For automation, prefer explicit subcommands and JSON modes.

## Common workflows

```sh
jbx check [path...] --json
jbx test [script.java|directory] --json
jbx test --coverage --json
jbx doctor [script.java|url] --json
jbx docs <GAV|source|dir> --json
jbx search <query> --json
jbx rewrite patch --recipe <short|fqn> --source <path> --json
```

## Command skills

Every command has a matching bundled skill named `jbx-<command>`:

```sh
jbx skill get jbx-run
jbx skill get jbx-build
jbx skill get jbx-check
jbx skill get jbx-test
jbx skill get jbx-docs
jbx skill get jbx-doctor
jbx skill get jbx-rewrite
jbx skill get jbx-publish
```

## Agent workflow

1. Run `jbx skill list --json` to discover installed guidance.
2. Fetch the specific skill for the command you need.
3. Prefer JSON modes when they exist; parse the JSON instead of scraping human output.
4. Use `jbx doctor --json` before guessing about JDKs, caches, Maven reachability, remote trust, formatter fallback, dependency drift, publishing, or native-image setup.
5. Verify generated artifacts directly: files for mutating commands, schemas for JSON commands, and exit codes for gates.

## Repository development gate

When changing jbx itself, run:

```sh
source ~/.cargo/env 2>/dev/null || true
cargo fmt --check
cargo test --locked
cargo clippy --all-targets --all-features -- -D warnings
RUSTFLAGS="-D warnings" cargo test --locked
```

For docs-only or website-facing changes, run:

```sh
scripts/check-docs-website.sh
git diff --stat
git status --short
```

## Compatibility notes

- Preserve JBang-compatible command shape and directives unless a task explicitly asks for a difference.
- Preserve Java 25 compact/unnamed-class behavior unless a test proves otherwise.
- Prefer clear deterministic errors over silent partial compatibility.
- Keep agent-facing output parseable; use JSON modes for automation.
