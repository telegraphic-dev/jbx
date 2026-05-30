---
name: jbx-test
description: Run JUnit tests with optional JaCoCo coverage.
---

# jbx-test

Run JUnit tests with optional JaCoCo coverage.

## Use when

- A task needs the `test` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx test --json
jbx test --coverage --json
```

## Agent workflow

Use after `jbx check` when tests or coverage evidence matter.

## JSON / structured output

`--json` reports status, tests, failures, and optional coverage.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
