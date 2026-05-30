---
title: jbx catalog command
description: Add and list external catalogs in `jbang-catalog.json`.
---

# `catalog`

Add and list external catalogs in `jbang-catalog.json`.

## Common use

```bash
jbx catalog list --json
jbx catalog add tools https://example.com/jbang-catalog.json --import
```

## Agent notes

Use to discover imported script/template catalogs.

## JSON and schema

`catalog list --json` returns catalog references.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-catalog
```
