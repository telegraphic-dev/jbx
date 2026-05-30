---
title: jbx jdk command
description: List, install, and locate JDKs used by jbx.
---

# `jdk`

List, install, and locate JDKs used by jbx.

## Common use

```bash
jbx jdk list
jbx jdk home 25
jbx jdk install 25
```

## Agent notes

Use when `doctor` reports missing or unsuitable JDKs.

## JSON and schema

No JSON mode yet; use `jbx doctor --json` for structured JDK diagnostics.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-jdk
```
