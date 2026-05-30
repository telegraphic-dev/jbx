---
name: jbx-alias
description: Add, remove, and list aliases from `jbang-catalog.json`.
---

# jbx-alias

Add, remove, and list aliases from `jbang-catalog.json`.

## Use when

- A task needs the `alias` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx alias list --json
jbx alias add Hello.java
jbx alias remove hello
```

## Agent workflow

Use to inspect or maintain JBang catalog aliases.

## JSON / structured output

`alias list --json` returns alias entries.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
