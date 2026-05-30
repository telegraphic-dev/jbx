---
name: jbx-build
description: Compile a script into the jbx cache without running it.
---

# jbx-build

Compile a script into the jbx cache without running it.

## Use when

- A task needs the `build` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx build Hello.java
```

## Agent workflow

Use before execution when an agent needs to separate compilation from runtime.

## JSON / structured output

No JSON mode yet; use `jbx check --json` for structured diagnostics.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
