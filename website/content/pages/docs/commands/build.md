---

title: jbx build command
description: Compile Java source without running it
---

# `build`

Compile a script into the jbx cache without running it.

## When to use it

- Precompile a script during CI so the later run path starts from a warm cache.
- Validate generated Java without executing side effects.
- Check that dependency directives and CLI dependency overrides resolve together.

## Common workflows

```bash
jbx build scripts/Report.java
jbx build --deps org.slf4j:slf4j-api:2.0.17 tools/Probe.java
jbx build --java 25 Hello.java
```

## Real-life examples

### Repository maintenance

Use `build` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Discover guidance with `jbx skill get jbx-build`.
2. Run the command in the narrowest scope that answers the task.
3. Prefer JSON/structured output when this command exposes it.
4. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

Prefer `build` over `run` when the task is “can this script compile?” rather than “execute this script”. Parse the process exit code; do not scrape localized compiler prose when `check --json` would fit better.

## JSON and schema

No `--json` mode yet. Success/failure is the exit code and compiler diagnostics are printed for humans. Use `jbx check --json` when an agent needs structured diagnostics.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx build`

```text
Compile and store script in the cache without running it

Usage: jbx build [OPTIONS] <SCRIPT>

Arguments:
  <SCRIPT>  Java source file

Options:
      --deps <DEPS>
          Additional dependency coordinates, same shape as //DEPS
      --repo <REPOS>
          Additional repository, same shape as //REPOS
      --source <SOURCES>
          Additional source file, same shape as //SOURCES
      --files <FILES>
          Additional file/resource, same shape as //FILES
      --class-path <CLASSPATH>
          Additional classpath entries
      --javac-option <JAVAC_OPTIONS>
          Additional javac option
      --runtime-option <RUNTIME_OPTIONS>
          Additional java runtime option, same shape as //JAVA_OPTIONS
      --java <JAVA_VERSION>
          Override //JAVA requested version
      --javaagent <JAVA_AGENTS>
          Additional java agent, same shape as //JAVAAGENT
      --main <MAIN_CLASS>
          Override //MAIN / inferred class name
      --cache-dir <CACHE_DIR>
          Override cache directory
      --trust
          Trust this remote script content hash before building
  -h, --help
          Print help
```

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-build
```
