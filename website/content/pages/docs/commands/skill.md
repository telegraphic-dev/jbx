---
title: jbx skill command
description: List and print version-matched bundled agent skills.
---

# `skill`

List and print version-matched bundled agent skills.

## Common use

```bash
jbx skill list
jbx skill list --json
jbx skill get jbx-check
```

## Agent notes

Use this before automating jbx. Each command has a dedicated skill named `jbx-<command>`.

## JSON and schema

`skill list --json` returns bundled skill names and descriptions.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-skill
```
