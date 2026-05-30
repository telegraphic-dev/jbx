---
title: jbx template command
description: List built-in and imported templates for `jbx init`.
---

# `template`

List built-in and imported templates for `jbx init`.

## Common use

```bash
jbx template list --json
jbx init Hello.java --template cli
```

## Agent notes

Use before creating scripts so agents pick an existing template instead of inventing one.

## JSON and schema

`template list --json` returns template metadata.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-template
```
