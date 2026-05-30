---
title: jbx — Java toolbox for scripts, tools, and automation
description: One practical CLI for Java scripts, Maven tools, tests, formatting, publishing, dependency lookup, documentation, diagnostics, and JDK handling.
---

# Java tools in one small CLI

`jbx` runs Java scripts, launches Maven tools, checks code, runs tests, formats sources, inspects dependencies, publishes artifacts, and documents APIs from one command.

```bash
jbx Hello.java world
jbx check src --json
jbx docs com.fasterxml.jackson.core:jackson-databind --json
jbx doctor Hello.java --json
```

## What it gives you

- **One entry point.** Run scripts, launch Maven tools, inspect dependencies, test, format, export, publish, and diagnose without switching toolchains.
- **Structured output.** Commands that produce facts expose JSON modes, and the matching schemas live with the docs.
- **Bundled command guidance.** `jbx skill list` and `jbx skill get <name>` ship detailed command docs with the binary, so automation can use the same documentation offline.
- **Real examples.** Every command page explains arguments, common workflows, JSON output, and concrete command lines.

## Start here

```bash
curl -fsSL https://jbx.telegraphic.dev/install.sh | bash
export PATH="$HOME/.jbx/bin:$PATH"
jbx skill list
```

Then use the [command reference](/docs/commands/) for the task at hand.
