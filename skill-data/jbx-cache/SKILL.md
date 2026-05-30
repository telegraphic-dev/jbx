---
name: jbx-cache
description: Inspect or clear compiled-script cache paths and entries.
---

# jbx-cache

Inspect or clear compiled-script cache paths and entries.

## Use when

- A task needs the `cache` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx cache path
jbx cache list --json
jbx cache clear
```

## Agent workflow

Use `path` for reproducible agent cache setup.

## JSON / structured output

`cache list --json` returns cache entries.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
