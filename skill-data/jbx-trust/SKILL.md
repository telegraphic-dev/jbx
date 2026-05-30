---
name: jbx-trust
description: Pin, list, remove, or clear trusted hashes for remote scripts.
---

# jbx-trust

Pin, list, remove, or clear trusted hashes for remote scripts.

## Use when

- A task needs the `trust` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx trust add https://example.com/Hello.java
jbx trust list
```

## Agent workflow

Use before running remote scripts repeatably.

## JSON / structured output

No JSON mode yet.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
