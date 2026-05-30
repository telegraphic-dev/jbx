---
title: jbx alias command
description: Add, remove, and list aliases from nearby `jbang-catalog.json` files.
---

# `alias`

Add, remove, and list aliases from nearby `jbang-catalog.json` files.

## Common use

```bash
jbx alias list --json
jbx alias add Hello.java --name hello
jbx alias remove hello
```

## Agent notes

Use to inspect or maintain project-local JBang catalog aliases.

## JSON and schema

`alias list --json` returns resolved alias entries.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-alias
```
