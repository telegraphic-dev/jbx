---
name: jbx-trust
description: Pin, list, remove, or clear trusted hashes for remote scripts.
---

# `trust`

Pin, list, remove, or clear trusted hashes for remote scripts.

## When to use it

- Pin a reviewed remote script before automation runs it.
- Rotate trust after a remote script intentionally changes.
- Audit which URLs are allowed to run without prompting.

## Common workflows

```bash
jbx trust list
jbx trust add https://example.com/tool.java
jbx trust remove https://example.com/tool.java
```

## Real-life examples

### Repository maintenance

Use `trust` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Run the command in the narrowest scope that answers the task.
2. Prefer JSON/structured output when this command exposes it.
3. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

Remote trust changes are security-sensitive. Ask before adding/removing trust unless the user explicitly requested it, and always show the URL/hash being trusted.

## JSON and schema

No `--json` mode yet. Trust operations are small and human-auditable; use explicit subcommands and verify the listed hash after changes.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx trust`

```text
Manage trusted remote scripts

Usage: jbx trust <COMMAND>

Commands:
  add     Trust the current content hash of a remote script URL
  remove  Remove a trusted remote script URL
  list    List trusted remote script URLs and hashes
  clear   Clear all trusted remote script entries
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `jbx trust list`

```text
List trusted remote script URLs and hashes

Usage: jbx trust list [OPTIONS]

Options:
      --cache-dir <CACHE_DIR>  Override cache directory
  -h, --help                   Print help
```

### `jbx trust add`

```text
Trust the current content hash of a remote script URL

Usage: jbx trust add [OPTIONS] <URL>

Arguments:
  <URL>  Remote http(s) Java source URL

Options:
      --cache-dir <CACHE_DIR>  Override cache directory
  -h, --help                   Print help
```

### `jbx trust remove`

```text
Remove a trusted remote script URL

Usage: jbx trust remove [OPTIONS] <URL>

Arguments:
  <URL>  Remote http(s) Java source URL

Options:
      --cache-dir <CACHE_DIR>  Override cache directory
  -h, --help                   Print help
```

### `jbx trust clear`

```text
Clear all trusted remote script entries

Usage: jbx trust clear [OPTIONS]

Options:
      --cache-dir <CACHE_DIR>  Override cache directory
  -h, --help                   Print help
```

> For exact behavior, prefer the skill bundled with the `jbx` binary on the machine running the task.
