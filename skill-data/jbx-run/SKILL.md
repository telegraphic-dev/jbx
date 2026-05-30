---
name: jbx-run
description: Run Java source or Maven artifact
---

# `run`

Compile and run one Java source file, including Java 25 compact scripts, with JBang-style directives and CLI overrides.

## When to use it

- Run a one-file maintenance script from a repository without creating a Maven or Gradle project.
- Launch a Java 25 compact script that carries `//DEPS`, `//JAVA`, `//SOURCES`, and runtime options in the file.
- Smoke-test an executable example after `jbx check --json` has confirmed the source compiles.

## Common workflows

```bash
jbx run scripts/Report.java --month 2026-05
jbx scripts/Report.java --month 2026-05
jbx run --deps info.picocli:picocli:4.7.7 tools/Cli.java --help
```

## Passing arguments

`run` options go before the script path. After the script path, arguments belong to the Java program, including Picocli-style options such as `--help`, `--input`, or `--verbose`.

Use an explicit `--` only when the Java program needs to receive a literal double-dash argument.

## Real-life examples

### Repository maintenance

Use `run` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Run the command in the narrowest scope that answers the task.
2. Prefer JSON/structured output when this command exposes it.
3. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

Treat `run` as the boundary where arbitrary user code executes. For autonomous loops, first inspect with `info`, compile with `build`, or validate with `check --json`; only run after the command and arguments are understood.

## JSON and schema

No `--json` mode: stdout/stderr belong to the program being run. Use `jbx check --json`, `jbx build`, `jbx info ...`, or `jbx doctor --json` for machine-readable preflight facts before execution.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx run`

```text
Compile and run a Java source file

Usage: jbx run [OPTIONS] <SCRIPT> [ARGS]...

Arguments:
  <SCRIPT>   Java source file
  [ARGS]...  Arguments passed to the script

Options:
      --deps <DEPS>                       Additional dependency coordinates, same shape as //DEPS
      --repo <REPOS>                      Additional repository, same shape as //REPOS
      --source <SOURCES>                  Additional source file, same shape as //SOURCES
      --files <FILES>                     Additional file/resource, same shape as //FILES
      --class-path <CLASSPATH>            Additional classpath entries
      --javac-option <JAVAC_OPTIONS>      Additional javac option
      --runtime-option <RUNTIME_OPTIONS>  Additional java runtime option
      --java <JAVA_VERSION>               Override //JAVA requested version
      --javaagent <JAVA_AGENTS>           Additional java agent, same shape as //JAVAAGENT
      --main <MAIN_CLASS>                 Override //MAIN / inferred class name
      --cache-dir <CACHE_DIR>             Override cache directory
      --trust                             Trust this remote script content hash before running
  -h, --help                              Print help
```
