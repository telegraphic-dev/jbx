---
name: jbx-template
description: List built-in and imported templates for `jbx init`.
---

# jbx-template

List built-in and imported templates for `jbx init`.

## Use when

- A task needs the `template` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx template list --json
jbx init Hello.java --template cli
```

## Agent workflow

Use before creating scripts.

## JSON / structured output

`template list --json` returns template metadata.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
