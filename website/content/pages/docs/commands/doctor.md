---
title: jbx doctor command
description: Check JDK selection, Maven Central, cache writability, formatter fallback, remote trust, dependencies, update drift, and optional publish/native tools.
---

# `doctor`

Check JDK selection, Maven Central, cache writability, formatter fallback, remote trust, dependencies, update drift, and optional publish/native tools.

## Common use

```bash
jbx doctor --json
jbx doctor Hello.java --json
jbx doctor https://example.com/Hello.java --cache-dir .jbx-cache --repo snapshots=https://repo.example.test/maven --publish --native --json
```

## Agent notes

Run before guessing about local Java/tooling failures.

## JSON and schema

`--json` returns checks with names, status (`ok`, `warn`, `fail`, `skipped`), summaries, and details. Website schema: `/docs/schemas/#doctor-json`.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-doctor
```
