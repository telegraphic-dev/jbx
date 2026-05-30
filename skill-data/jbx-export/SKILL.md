---
name: jbx-export
description: Export local, portable, or native runnable artifacts.
---

# jbx-export

Export local, portable, or native runnable artifacts.

## Use when

- A task needs the `export` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx export local Hello.java -o app.jar
jbx export portable hello -o app.jar
```

## Agent workflow

Use after build/test evidence when an artifact is needed.

## JSON / structured output

No JSON mode yet; verify artifacts directly.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
