---
name: jbx-catalog
description: Add and list external catalogs in `jbang-catalog.json`.
---

# jbx-catalog

Add and list external catalogs in `jbang-catalog.json`.

## Use when

- A task needs the `catalog` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx catalog list --json
jbx catalog add tools https://example.com/jbang-catalog.json --import
```

## Agent workflow

Use to discover imported script/template catalogs.

## JSON / structured output

`catalog list --json` returns catalog references.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
