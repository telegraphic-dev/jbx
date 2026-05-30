---
title: jbx publish command
description: Build Maven Central-ready bundles, local served repositories, or Portal uploads from `jbx.json` and sources.
---

# `publish`

Build Maven Central-ready bundles, local served repositories, or Portal uploads from `jbx.json` and sources.

## Common use

```bash
jbx publish --file jbx.json --dry-run --gpg-key you@example.com
jbx publish --file jbx.json --serve 0
CENTRAL_TOKEN_USERNAME=... CENTRAL_TOKEN_PASSWORD=... jbx publish --file jbx.json --publish --gpg-key you@example.com
```

## Agent notes

Always dry-run and inspect staged POM/bundle before real publishing. Never put credentials in files.

## JSON and schema

No JSON mode yet; publishing has external side effects and should be verified by artifact inspection.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-publish
```
