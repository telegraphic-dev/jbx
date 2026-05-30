---
title: jbx init command
description: Create Java 25+ scripts from built-in or imported templates.
---

# `init`

Create Java 25+ scripts from built-in or imported templates.

## Common use

```bash
jbx init Hello.java
jbx init Tool.java --template cli --deps info.picocli:picocli:4.7.7
jbx init Hello.java --force
```

## Agent notes

Use to bootstrap scripts; inspect templates first with `jbx template list --json`.

## JSON and schema

No JSON mode; the command writes files.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-init
```
