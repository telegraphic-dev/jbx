---
name: jbx-docs
description: Generate Markdown or JSON documentation from local sources or Maven artifacts.
---

# jbx-docs

Generate Markdown or JSON documentation from local sources or Maven artifacts.

## Use when

- A task needs the `docs` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx docs src/main/java
jbx docs group:artifact:version --json
```

## Agent workflow

Use before editing unfamiliar APIs.

## JSON / structured output

`--json` follows the docs sidecar schema.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
