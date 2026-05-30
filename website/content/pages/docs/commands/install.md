---
title: jbx install command
description: Install the current project into a Maven repository layout, usually `~/.m2/repository`.
---

# `install`

Install the current project into a Maven repository layout, usually `~/.m2/repository`.

## Common use

```bash
jbx install --file jbx.json
jbx install --file jbx.json --destination /tmp/repo
```

## Agent notes

Use for local Maven repository smoke tests.

## JSON and schema

No JSON mode yet; verify generated repository files directly.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-install
```
