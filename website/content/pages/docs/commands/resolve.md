---
title: jbx resolve command
description: Resolve Maven coordinates to dependency coordinates without running code.
---

# `resolve`

Resolve Maven coordinates to dependency coordinates without running code.

## Common use

```bash
jbx resolve info.picocli:picocli:4.7.7
jbx resolve --classpath info.picocli:picocli:4.7.7
```

## Agent notes

Use when you need dependency graph/classpath facts but not execution.

## JSON and schema

No JSON mode yet; output is line-oriented coordinates or classpath. Prefer `jbx docs --json` for published sidecar metadata.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-resolve
```
