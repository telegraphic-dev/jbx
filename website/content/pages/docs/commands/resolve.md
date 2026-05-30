---

title: jbx resolve command
description: Resolve Maven coordinates
---

# `resolve`

Resolve Maven coordinates to dependency coordinates without running code.

## When to use it

- Inspect the dependency graph before committing a new `//DEPS`.
- Check whether exclusions or runtime scopes change the graph.
- Debug version mediation without executing user code.

## Common workflows

```bash
jbx resolve com.fasterxml.jackson.core:jackson-databind:2.17.2
jbx resolve --repo snapshots=https://repo.example.com/snapshots com.acme:tool:1.0.0-SNAPSHOT
jbx resolve --classpath com.acme:app:1.0.0
```

## Real-life examples

### Repository maintenance

Use `resolve` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Discover guidance with `jbx skill get jbx-resolve`.
2. Run the command in the narrowest scope that answers the task.
3. Prefer JSON/structured output when this command exposes it.
4. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

Use `resolve` for metadata questions. It should not be treated as proof that jars are already present locally; use `fetch` for that.

## JSON and schema

No `--json` mode yet. Output is dependency coordinates or classpath-style text depending on flags. Use `fetch` when artifacts must be downloaded.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx resolve`

```text
Resolve Maven dependencies without running

Usage: jbx resolve [OPTIONS] <COORDINATES>...

Arguments:
  <COORDINATES>...  Maven coordinates to resolve (groupId:artifactId:version)

Options:
      --repo <REPOS>           Additional repository (id=url format or bare URL)
      --cache-dir <CACHE_DIR>  Override cache directory
  -c, --classpath              Print classpath (JAR paths) instead of coordinates
  -h, --help                   Print help
```

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-resolve
```
