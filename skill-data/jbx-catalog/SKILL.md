---
name: jbx-catalog
description: Add and list external catalogs in jbang-catalog.json.
---

# `catalog`

Add and list external catalogs in `jbang-catalog.json`.

## When to use it

- Share common script aliases across repositories.
- Inspect imported catalogs before resolving an alias.
- Add a team catalog during project setup.

## Common workflows

```bash
jbx catalog list --json
jbx catalog add team https://example.com/jbang-catalog.json
jbx catalog add local ./tools/jbang-catalog.json
```

## Real-life examples

### Repository maintenance

Use `catalog` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Run the command in the narrowest scope that answers the task.
2. Prefer JSON/structured output when this command exposes it.
3. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

Catalog changes affect command discovery. List first, avoid duplicate names, and prefer pinned/reviewed URLs over random raw links.

## JSON and schema

`jbx catalog list --json` returns catalog names, URLs, and local resolution details. Website schema: `/docs/schemas/#catalog-json`.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx catalog`

```text
Manage external catalogs from jbang-catalog.json

Usage: jbx catalog <COMMAND>

Commands:
  add   Add an external catalog reference
  list  List external catalog references
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `jbx catalog list`

```text
List external catalog references

Usage: jbx catalog list [OPTIONS]

Options:
      --json  Print JSON instead of tab-separated text
  -h, --help  Print help
```

### `jbx catalog add`

```text
Add an external catalog reference

Usage: jbx catalog add [OPTIONS] <NAME> <CATALOG_REF>

Arguments:
  <NAME>         Catalog name
  <CATALOG_REF>  Catalog path, URL, or directory

Options:
  -g, --global                     Use the global user catalog file (~/.jbang/jbang-catalog.json)
  -f, --file <FILE>                Path to the catalog file or directory to use
      --description <DESCRIPTION>  Description for the catalog
      --import                     Import aliases and templates from this catalog into local lookup
      --force                      Force overwrite of an existing catalog reference
  -h, --help                       Print help
```
