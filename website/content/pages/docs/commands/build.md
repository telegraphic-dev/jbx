---
title: jbx build command
description: Compile a script into the jbx cache without running it.
---

# `build`

Compile a script into the jbx cache without running it.

## Common use

```bash
jbx build Hello.java
jbx build --java 25 --deps info.picocli:picocli:4.7.7 Hello.java
```

## Agent notes

Use before execution when an agent needs to separate compilation failures from runtime behavior.

## JSON and schema

No JSON mode yet; compilation errors are javac diagnostics. Use `jbx check --json` for structured diagnostics.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-build
```
