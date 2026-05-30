---
title: jbx export command
description: Export local, portable, or native runnable artifacts.
---

# `export`

Export local, portable, or native runnable artifacts.

## Common use

```bash
jbx export local Hello.java -o app.jar
jbx export portable hello -o app.jar
jbx export native Hello.java -o hello
```

## Agent notes

Use after build/test evidence when an artifact is needed.

## JSON and schema

No JSON mode yet; verify artifacts directly after export.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-export
```
