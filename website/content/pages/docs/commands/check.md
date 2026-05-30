---
title: jbx check command
description: Run javac `-Xlint:all` and Error Prone by default, optionally as structured diagnostics.
---

# `check`

Run javac `-Xlint:all` and Error Prone by default, optionally as structured diagnostics.

## Common use

```bash
jbx check src --json
jbx check Hello.java --no-error-prone --json
jbx check src --warnings-as-errors
```

## Agent notes

Use this as the first agent-safe Java quality gate.

## JSON and schema

`--json` returns source diagnostics with file, line, column, severity, message, tool, and command status fields. Website schema: `/docs/schemas/#check-json`.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-check
```
