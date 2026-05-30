---
name: jbx-doctor
description: Diagnose JDKs, Maven, caches, trust, dependencies, and optional native/publish tools.
---

# jbx-doctor

Diagnose JDKs, Maven, caches, trust, dependencies, and optional native/publish tools.

## Use when

- A task needs the `doctor` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx doctor --json
jbx doctor Hello.java --json
```

## Agent workflow

Run before guessing about local Java/tooling failures.

## JSON / structured output

`--json` returns checks with stable status values.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
