---
title: jbx test command
description: Run JUnit tests with the standalone console launcher; optionally collect JaCoCo coverage.
---

# `test`

Run JUnit tests with the standalone console launcher; optionally collect JaCoCo coverage.

## Common use

```bash
jbx test
jbx test src/test/java --json
jbx test --coverage --json
jbx test --coverage --jacoco-version 0.8.13
```

## Agent notes

Use after `jbx check` when source contains tests or when coverage evidence matters.

## JSON and schema

`--json` reports status, executed tests, failures, console paths, and when enabled a `coverage` object with JaCoCo exec/html/xml paths and counters. Website schema: `/docs/schemas/#test-json`.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-test
```
