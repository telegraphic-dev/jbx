---
title: jbx trust command
description: Pin, list, remove, or clear trusted hashes for remote scripts.
---

# `trust`

Pin, list, remove, or clear trusted hashes for remote scripts.

## Common use

```bash
jbx trust add https://example.com/Hello.java
jbx trust list
jbx trust remove https://example.com/Hello.java
```

## Agent notes

Use before running remote scripts in repeatable environments.

## JSON and schema

No JSON mode yet; keep remote trust changes explicit and human-reviewable.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-trust
```
