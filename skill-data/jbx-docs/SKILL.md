---
name: jbx-docs
description: Display documentation from Java sources or Maven artifacts.
---

# `docs`

Generate Markdown or JSON documentation from local Java sources, directories, docs sidecars, or Maven artifacts.

## When to use it

- Inspect an unfamiliar dependency before writing integration code.
- Publish or consume sidecar docs that agents can read without decompiling jars.
- Generate local API notes for a small script or library as part of CI.

## Common workflows

```bash
jbx docs src/main/java
jbx docs com.fasterxml.jackson.core:jackson-databind:2.17.2 --json
jbx docs docs/my-library-jbx-docs.json
```

## Real-life examples

### Repository maintenance

Use `docs` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Run the command in the narrowest scope that answers the task.
2. Prefer JSON/structured output when this command exposes it.
3. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

Use docs before guessing APIs. Prefer JSON when extracting types/methods programmatically; use Markdown for human handoff. If a Maven artifact has a sidecar, trust the sidecar version that matches the artifact coordinate.

## JSON and schema

`--json` follows the published docs sidecar schema in `/docs/jbx-docs-schema/` and is summarized at `/docs/schemas/#docs-json`.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx docs`

```text
Print agent-friendly documentation for source, directories, or Maven artifacts

Usage: jbx docs [OPTIONS] <TARGET>

Arguments:
  <TARGET>  Maven GAV, Java source file, docs sidecar, or directory to document

Options:
      --json                   Print JSON instead of Markdown
      --repo <REPOS>           Additional repository for remote Maven docs sidecars (id=url format or bare URL)
      --type <TYPES>           Limit structured output to matching type names. Repeatable; accepts simple or fully-qualified names
      --cache-dir <CACHE_DIR>  Override remote docs cache directory
  -h, --help                   Print help
```
