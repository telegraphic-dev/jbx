---
name: jbx-graph
description: Dump JavaParser native AST JSON or import it back to Java source.
---

# `graph`

Dump JavaParser native AST JSON or import it back to Java source.

## When to use it

- Let an agent perform structural analysis without regexing Java.
- Round-trip a generated AST back to source after a controlled transformation.
- Debug parser behavior around compact Java scripts.

## Common workflows

```bash
jbx graph dump Hello.java > Hello.ast.json
jbx graph import Hello.ast.json --output Hello.java
jbx graph dump src/main/java/com/acme/App.java | jq .
```

## Real-life examples

### Repository maintenance

Use `graph` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Run the command in the narrowest scope that answers the task.
2. Prefer JSON/structured output when this command exposes it.
3. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

Prefer AST operations for structural changes, but verify with `jbx check --json` after import. Treat generated source as code changes requiring review.

## JSON and schema

`dump` emits JavaParser native AST JSON. `import` consumes that JSON and writes Java source. This is an AST interchange format, not a stable semantic schema for public APIs.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx graph`

```text
Convert Java source to/from JavaParser's native JSON serialization

Usage: jbx graph <COMMAND>

Commands:
  dump    Convert a Java source file to JavaParser's native JSON serialization
  import  Convert JavaParser's native JSON serialization back to Java source
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `jbx graph dump`

```text
Convert a Java source file to JavaParser's native JSON serialization

Usage: jbx graph dump [OPTIONS] <SCRIPT>

Arguments:
  <SCRIPT>  Java source file

Options:
      --cache-dir <CACHE_DIR>  Override cache directory
  -h, --help                   Print help
```

### `jbx graph import`

```text
Convert JavaParser's native JSON serialization back to Java source

Usage: jbx graph import [OPTIONS] <JSON>

Arguments:
  <JSON>  JavaParser JSON file

Options:
  -o, --output <OUTPUT>        Write Java source to this file instead of stdout
      --cache-dir <CACHE_DIR>  Override cache directory
  -h, --help                   Print help
```
