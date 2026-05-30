---
title: jbx fmt command
description: Format Java files with Palantir Java Format, including Java 25 compact scripts.
---

# `fmt`

Format Java files with Palantir Java Format, including Java 25 compact scripts.

## Common use

```bash
jbx fmt src
jbx fmt Hello.java
```

## Agent notes

Use after source edits or before tests when style matters.

## JSON and schema

No JSON mode; the command mutates files and reports failures in normal process output.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-fmt
```
