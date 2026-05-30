---
name: jbx-rewrite
description: Preview/apply OpenRewrite recipes and discover modules or recipes.
---

# jbx-rewrite

Preview/apply OpenRewrite recipes and discover modules or recipes.

## Use when

- A task needs the `rewrite` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx rewrite patch --recipe auto-format --source src --json
jbx rewrite modules --json
```

## Agent workflow

Prefer `patch` before `apply`.

## JSON / structured output

JSON modes exist for patch/apply/modules/recipes.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
