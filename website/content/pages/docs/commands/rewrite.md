---
title: jbx rewrite command
description: Preview or apply OpenRewrite recipes with jbx-managed dependencies and JDKs; discover modules and recipes.
---

# `rewrite`

Preview or apply OpenRewrite recipes with jbx-managed dependencies and JDKs; discover modules and recipes.

## Common use

```bash
jbx rewrite patch --recipe auto-format --source src/main/java --json
jbx rewrite apply --recipe cleanup --source src/main/java --json
jbx rewrite modules --search yaml --json
jbx rewrite recipes yaml --detail --json
```

## Agent notes

Always prefer `patch` before `apply` unless mutation is explicitly intended.

## JSON and schema

JSON modes exist for patch/apply/modules/recipes. Schemas are summarized at `/docs/schemas/#rewrite-json`.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-rewrite
```
