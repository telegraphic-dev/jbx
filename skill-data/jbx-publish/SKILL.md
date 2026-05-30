---
name: jbx-publish
description: Build Maven-ready bundles, local served repositories, or Portal uploads.
---

# jbx-publish

Build Maven-ready bundles, local served repositories, or Portal uploads.

## Use when

- A task needs the `publish` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx publish --file jbx.json --dry-run --gpg-key you@example.com
jbx publish --file jbx.json --serve 0
```

## Agent workflow

Always dry-run and inspect staged artifacts before real publishing.

## JSON / structured output

No JSON mode yet; verify artifacts directly.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
