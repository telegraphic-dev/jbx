---
title: jbx graph command
description: Dump JavaParser native AST JSON or import it back to Java source.
---

# `graph`

Dump JavaParser native AST JSON or import it back to Java source.

## Common use

```bash
jbx graph dump Hello.java > ast.json
jbx graph import ast.json -o Hello.java
```

## Agent notes

Use for AST-level agent workflows that need JavaParser-compatible JSON.

## JSON and schema

`graph dump` is JSON by design; there is no jbx-specific schema because it is JavaParser native serialization.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-graph
```
