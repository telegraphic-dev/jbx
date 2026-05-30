---
name: jbx-run
description: Compile and run one Java source file, including Java 25 compact scripts, with JBang-style directives and CLI overrides.
---

# jbx-run

Compile and run one Java source file, including Java 25 compact scripts, with JBang-style directives and CLI overrides.

## Use when

- A task needs the `run` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx run Hello.java world
jbx Hello.java world
```

## Agent workflow

Use for normal script execution.

## JSON / structured output

No JSON mode because the payload is program output.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
