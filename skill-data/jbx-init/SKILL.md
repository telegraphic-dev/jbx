---
name: jbx-init
description: Create Java 25+ scripts from built-in or imported templates.
---

# jbx-init

Create Java 25+ scripts from built-in or imported templates.

## Use when

- A task needs the `init` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx init Hello.java
jbx init Tool.java --template cli
```

## Agent workflow

Inspect templates first with `jbx template list --json`.

## JSON / structured output

No JSON mode; the command writes files.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
