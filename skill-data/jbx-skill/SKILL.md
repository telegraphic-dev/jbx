---
name: jbx-skill
description: List and print version-matched bundled agent skills.
---

# jbx-skill

List and print version-matched bundled agent skills.

## Use when

- A task needs the `skill` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx skill list
jbx skill list --json
jbx skill get jbx-check
```

## Agent workflow

Use this before automating jbx.

## JSON / structured output

`skill list --json` returns bundled skill names and descriptions.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
