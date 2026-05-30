---
title: jbx run command
description: Compile and run one Java source file, including Java 25 compact scripts, with JBang-style directives and CLI overrides.
---

# `run`

Compile and run one Java source file, including Java 25 compact scripts, with JBang-style directives and CLI overrides.

## Common use

```bash
jbx run Hello.java world
jbx Hello.java world
jbx run --deps info.picocli:picocli:4.7.7 --repo central=https://repo.maven.apache.org/maven2 Hello.java -- --help
```

## Agent notes

Use for normal script execution. Prefer `jbx check --json` or `jbx build` before long autonomous loops when you need diagnostics without running application code.

## JSON and schema

No JSON mode because the payload is the program output. Use `jbx build`, `jbx check --json`, `jbx info ...`, or `jbx doctor --json` for structured preflight facts.

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-run
```
