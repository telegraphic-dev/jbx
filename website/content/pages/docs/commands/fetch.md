---
title: jbx fetch command
description: Download artifacts and print classpath or dependency coordinates.
---

# `fetch`

Download artifacts and print classpath or dependency coordinates.

## Common use

```bash
jbx fetch info.picocli:picocli:4.7.7
jbx fetch --deps-only info.picocli:picocli:4.7.7
```

## Agent notes

Use when an agent needs materialized jars/classpaths.

## JSON and schema

No JSON mode yet; output is line-oriented for shell composition.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-fetch
```
