---
title: jbx app command
description: Install, list, or uninstall Java scripts as PATH commands.
---

# `app`

Install, list, or uninstall Java scripts as PATH commands.

## Common use

```bash
jbx app install Hello.java
jbx app list
jbx app uninstall hello
```

## Agent notes

Use for durable local commands. Agents should prefer direct `jbx run` unless installation is the task.

## JSON and schema

No JSON mode yet.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-app
```
