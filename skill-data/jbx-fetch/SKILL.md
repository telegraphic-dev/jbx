---
name: jbx-fetch
description: Download artifacts and print classpath or dependency coordinates.
---

# jbx-fetch

Download artifacts and print classpath or dependency coordinates.

## Use when

- A task needs the `fetch` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx fetch info.picocli:picocli:4.7.7
jbx fetch --deps-only info.picocli:picocli:4.7.7
```

## Agent workflow

Use when an agent needs materialized jars/classpaths.

## JSON / structured output

No JSON mode yet; output is line-oriented.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
