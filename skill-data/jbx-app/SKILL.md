---
name: jbx-app
description: Install, list, or uninstall Java scripts as PATH commands.
---

# jbx-app

Install, list, or uninstall Java scripts as PATH commands.

## Use when

- A task needs the `app` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx app install Hello.java
jbx app list
jbx app uninstall hello
```

## Agent workflow

Agents should prefer direct `jbx run` unless installation is the task.

## JSON / structured output

No JSON mode yet.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
