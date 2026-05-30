---
name: jbx-check
description: Check Java source with structured diagnostics.
---

# jbx-check

Check Java source with structured diagnostics.

## Use when

- A task needs the `check` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx check src --json
```

## Agent workflow

Use this as the first agent-safe Java quality gate.

## JSON / structured output

`--json` returns source diagnostics. Website schema: `/docs/schemas/#check-json`.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
