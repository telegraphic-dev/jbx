---
name: jbx-check
description: Diagnose the Java source file for errors
---

# `check`

Run javac `-Xlint:all` and Error Prone by default, optionally as structured diagnostics.

## When to use it

- Gate an agent edit before running tests.
- Collect exact diagnostics for a PR review comment or automated repair loop.
- Check a Java 25 compact script where a plain build tool would not understand the wrapper semantics.

## Common workflows

```bash
jbx check src --json
jbx check src/main/java --json
jbx check Hello.java --no-error-prone --json
jbx check src test --warnings-as-errors --json
```

## Real-life examples

### Repository maintenance

Use `check` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Run the command in the narrowest scope that answers the task.
2. Prefer JSON/structured output when this command exposes it.
3. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

Use this as the first quality gate after edits. JSON diagnostics are the contract; make fixes from structured file/line/column fields, then rerun until the status is clean or only accepted warnings remain.

## JSON and schema

`--json` returns command status and diagnostics with file, line, column, severity, tool, code/message, and suggested next action. Website schema: `/docs/schemas/#check-json`.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx check`

```text
Check Java source files with javac diagnostics and Error Prone by default

Usage: jbx check [OPTIONS] [PATHS]...

Arguments:
  [PATHS]...  Java source files or directories. Defaults to the current directory [default: .]

Options:
      --json
          Emit structured diagnostics JSON
      --no-error-prone
          Disable Error Prone checks and run only javac/-Xlint diagnostics
      --error-prone-version <ERROR_PRONE_VERSION>
          Error Prone version to use when Error Prone is enabled [default: 2.39.0]
      --warnings-as-errors
          Treat javac and Error Prone warnings as errors
      --deps <DEPS>
          Additional dependency coordinates, same shape as //DEPS
      --repo <REPOS>
          Additional repository, same shape as //REPOS
      --class-path <CLASSPATH>
          Additional classpath entries
      --javac-option <JAVAC_OPTIONS>
          Additional javac option
      --java <JAVA_VERSION>
          Override requested Java version
      --cache-dir <CACHE_DIR>
          Override cache directory
  -h, --help
          Print help
```

> For exact behavior, prefer the skill bundled with the `jbx` binary on the machine running the task.
