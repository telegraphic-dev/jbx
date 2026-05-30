---
title: jbx search command
description: Search Maven Central artifacts by text or coordinates, with filters.
---

# `search`

Search Maven Central artifacts by text or coordinates, with filters.

## Common use

```bash
jbx search picocli --json
jbx search group:artifact --json
jbx search --group info.picocli --id picocli --limit 5 --json
```

## Agent notes

Use to find artifacts without scraping Maven Central HTML.

## JSON and schema

`--json` returns query metadata, `numFound`, and artifact records. Website schema: `/docs/schemas/#search-json`.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-search
```
