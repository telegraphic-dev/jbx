---
name: jbx-info
description: Print parsed directives and derived metadata from Java scripts.
---

# jbx-info

Print parsed directives and derived metadata from Java scripts.

## Use when

- A task needs the `info` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx info classpath Hello.java
jbx info deps Hello.java
jbx info tools Hello.java
```

## Agent workflow

Use for quick facts from one script.

## JSON / structured output

Some info subcommands are structured; others are one-value/line-oriented.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
