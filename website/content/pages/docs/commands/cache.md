---
title: jbx cache command
description: Inspect or clear compiled-script cache paths and entries.
---

# `cache`

Inspect or clear compiled-script cache paths and entries.

## Common use

```bash
jbx cache path
jbx cache list --json
jbx cache clear
```

## Agent notes

Use `path` for reproducible agent cache setup and `list --json` for inventory.

## JSON and schema

`cache list --json` returns cache entries; `path` is line-oriented for shell use.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-cache
```
