---
name: jbx-install
description: Install the current project into a Maven repository layout.
---

# jbx-install

Install the current project into a Maven repository layout.

## Use when

- A task needs the `install` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx install --file jbx.json
jbx install --file jbx.json --destination /tmp/repo
```

## Agent workflow

Use for local Maven repository smoke tests.

## JSON / structured output

No JSON mode yet.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
