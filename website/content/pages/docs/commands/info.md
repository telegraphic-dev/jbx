---
title: jbx info command
description: Print parsed directives and derived metadata from Java scripts.
---

# `info`

Print parsed directives and derived metadata from Java scripts.

## Common use

```bash
jbx info classpath Hello.java
jbx info deps Hello.java
jbx info tools Hello.java
jbx info main Hello.java
```

## Agent notes

Use for quick facts from one script without building a whole docs corpus.

## JSON and schema

Some info subcommands are already structured (`tools`); others are intentionally one-value/line-oriented. Prefer `jbx docs --json` for full structured API documentation.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-info
```
