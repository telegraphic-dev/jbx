---
name: jbx-jdk
description: List, install, and locate JDKs used by jbx.
---

# `jdk`

List, install, and locate JDKs used by jbx.

## When to use it

- Verify which JDK will run Java 25 compact scripts.
- Install a missing Temurin JDK for a CI runner.
- Debug why JAVA_HOME differs from the JDK selected by jbx.

## Common workflows

```bash
jbx jdk list
jbx jdk home 25
jbx jdk install 25
```

## Real-life examples

### Repository maintenance

Use `jdk` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Run the command in the narrowest scope that answers the task.
2. Prefer JSON/structured output when this command exposes it.
3. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

JDK installation downloads external binaries and changes local state. Prefer `doctor --json` and `jdk list` first; install only when needed and requested.

## JSON and schema

No `--json` mode yet. Use `jbx doctor --json` for structured environment checks until JDK subcommands grow dedicated JSON.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx jdk`

```text
Manage installed JDKs

Usage: jbx jdk <COMMAND>

Commands:
  list     List discovered and installed JDKs
  install  Install a JDK from Adoptium (Eclipse Temurin)
  home     Show JDK home directory for a given version
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `jbx jdk list`

```text
List discovered and installed JDKs

Usage: jbx jdk list

Options:
  -h, --help  Print help
```

### `jbx jdk install`

```text
Install a JDK from Adoptium (Eclipse Temurin)

Usage: jbx jdk install <VERSION>

Arguments:
  <VERSION>  JDK version to install (e.g. 21, 25, 25+)

Options:
  -h, --help  Print help
```

### `jbx jdk home`

```text
Show JDK home directory for a given version

Usage: jbx jdk home [VERSION]

Arguments:
  [VERSION]  JDK version (defaults to 25) [default: 25]

Options:
  -h, --help  Print help
```
