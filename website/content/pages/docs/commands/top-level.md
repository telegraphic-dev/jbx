---
title: jbx jbx command
description: Start with `jbx skill list` / `jbx skill get`; use direct script or Maven coordinate execution only for the common run path.
---

# `jbx`

Start with `jbx skill list` / `jbx skill get`; use direct script or Maven coordinate execution only for the common run path.

## Common use

```bash
jbx skill list
jbx skill get
jbx Hello.java world
jbx dev.telegraphic:hello-tool:1.0.0 -- --help
```

## Agent notes

The top level is intentionally thin: discover skills, read version-matched guidance, then use the dedicated command surface.

## JSON and schema

No top-level JSON mode. Use the dedicated subcommand JSON modes such as `jbx doctor --json`, `jbx check --json`, `jbx docs --json`, `jbx search --json`, `jbx rewrite ... --json`, and `jbx test --json`.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx
```
