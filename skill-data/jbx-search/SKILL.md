---
name: jbx-search
description: Search Maven Central artifacts by text or coordinates.
---

# jbx-search

Search Maven Central artifacts by text or coordinates.

## Use when

- A task needs the `search` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx search picocli --json
```

## Agent workflow

Use to find artifacts without scraping HTML.

## JSON / structured output

`--json` returns query metadata and artifacts.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
