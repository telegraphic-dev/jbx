---
name: jbx-jdk
description: List, install, and locate JDKs used by jbx.
---

# jbx-jdk

List, install, and locate JDKs used by jbx.

## Use when

- A task needs the `jdk` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx jdk list
jbx jdk home 25
jbx jdk install 25
```

## Agent workflow

Use when `doctor` reports missing JDKs.

## JSON / structured output

Use `jbx doctor --json` for structured JDK diagnostics.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
