---
name: jbx-fmt
description: Format Java files with Palantir Java Format.
---

# jbx-fmt

Format Java files with Palantir Java Format.

## Use when

- A task needs the `fmt` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx fmt src
jbx fmt Hello.java
```

## Agent workflow

Use after source edits.

## JSON / structured output

No JSON mode; the command mutates files.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
