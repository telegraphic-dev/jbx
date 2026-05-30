---
name: jbx-resolve
description: Resolve Maven coordinates to dependency coordinates or classpaths.
---

# jbx-resolve

Resolve Maven coordinates to dependency coordinates or classpaths.

## Use when

- A task needs the `resolve` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx resolve info.picocli:picocli:4.7.7
jbx resolve --classpath info.picocli:picocli:4.7.7
```

## Agent workflow

Use for dependency graph/classpath facts without execution.

## JSON / structured output

No JSON mode yet; output is line-oriented.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
