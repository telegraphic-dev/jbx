---
name: jbx-graph
description: Dump JavaParser native AST JSON or import it back to Java source.
---

# jbx-graph

Dump JavaParser native AST JSON or import it back to Java source.

## Use when

- A task needs the `graph` command or adjacent workflow.
- An agent needs version-matched jbx guidance instead of guessing CLI flags.

## Commands

```sh
jbx graph dump Hello.java > ast.json
jbx graph import ast.json -o Hello.java
```

## Agent workflow

Use for AST-level agent workflows.

## JSON / structured output

`graph dump` is JSON by design using JavaParser native serialization.

## Verification

After using this command, verify the artifact it claims to produce: exit code for checks, generated files for mutating commands, JSON parseability for `--json` modes, or rendered/docs output for documentation changes.
