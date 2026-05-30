---
title: jbx docs command
description: Generate Markdown or JSON documentation from local Java sources, directories, docs sidecars, or Maven artifacts.
---

# `docs`

Generate Markdown or JSON documentation from local Java sources, directories, docs sidecars, or Maven artifacts.

## Common use

```bash
jbx docs src/main/java
jbx docs com.fasterxml.jackson.core:jackson-databind --json
jbx docs dev.telegraphic:hello-tool:1.0.0 --type Hello --json
```

## Agent notes

Use before editing unfamiliar APIs. Prefer Markdown for context, JSON for targeted lookup.

## JSON and schema

`--json` follows the docs sidecar schema in `/docs/jbx-docs-schema/` and summarized at `/docs/schemas/#docs-json`.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-docs
```
