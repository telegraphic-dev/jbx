---
title: jbx — agent-friendly Java entry point
description: Single agent-friendly entry point to the Java ecosystem.
---

# Single agent-friendly entry point to the Java ecosystem

`jbx` gives developers and coding agents one practical CLI for Java scripts, Maven tools, tests, formatting, publishing, dependency lookup, documentation, diagnostics, and JDK handling.

```bash
jbx Hello.java world
jbx dev.telegraphic:hello-tool:1.0.0 -- --help
jbx check src --json
jbx docs com.fasterxml.jackson.core:jackson-databind --json
jbx doctor Hello.java --json
```

## What it gives you

- **One entry point.** Run scripts, launch Maven tools, inspect dependencies, test, format, export, publish, and diagnose without switching toolchains.
- **Agent-ready output.** Commands that produce facts expose JSON modes, and the matching schemas live with the docs.
- **Version-matched skills.** `jbx skill list` and `jbx skill get <name>` ship command guidance with the binary, so agents do not rely on stale web snippets.
- **Human-readable docs.** Every command has a short page with examples, agent notes, and structured-output expectations.

## Start here

```bash
curl -fsSL https://jbx.telegraphic.dev/install.sh | bash
export PATH="$HOME/.jbx/bin:$PATH"
jbx skill list
```

Then use the [command reference](/docs/commands/) for the task at hand.
