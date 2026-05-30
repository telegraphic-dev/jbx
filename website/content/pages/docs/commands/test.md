---

title: jbx test command
description: Run JUnit tests
---

# `test`

Run JUnit tests with the standalone console launcher; optionally collect JaCoCo coverage.

## When to use it

- Run a small Java kata or library test suite without creating a full build file.
- Give an agent failing test names and stack traces in a parseable shape.
- Collect coverage during a refactor to prove the edited code path is exercised.

## Common workflows

```bash
jbx test src/test/java --json
jbx test --json tests/CalculatorTest.java -- --select-method CalculatorTest#adds
jbx test --coverage --json
```

## Real-life examples

### Repository maintenance

Use `test` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Discover guidance with `jbx skill get jbx-test`.
2. Run the command in the narrowest scope that answers the task.
3. Prefer JSON/structured output when this command exposes it.
4. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

Start with focused tests when repairing a failure, then broaden to the directory or suite. Preserve non-zero exits for failed tests; do not hide failures behind “JSON parsed successfully”.

## JSON and schema

`--json` reports status, selected tests, failures, console XML paths, and optional coverage paths/counters. Website schema: `/docs/schemas/#test-json`.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx test`

```text
Run JUnit tests with the standalone console launcher

Usage: jbx test [OPTIONS] [SCRIPT] [ARGS]...

Arguments:
  [SCRIPT]
          Java test source file or directory. Defaults to the current directory

          [default: .]

  [ARGS]...
          Extra arguments passed to the JUnit ConsoleLauncher after defaults

Options:
      --json
          Print converted JUnit XML report as JSON

      --xml
          Print the generated JUnit XML report

      --coverage
          Collect JaCoCo coverage data in target/jacoco.exec

      --jacoco-version <JACOCO_VERSION>
          JaCoCo agent version to use when --coverage is enabled.

          Defaults to the built-in version (0.8.13).

      --junit-version <JUNIT_VERSION>
          JUnit Platform Console Standalone version to use.

          Defaults to the cached latest Maven Central release, refreshed periodically.

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
          Additional java runtime option for the JUnit launcher JVM

      --java <JAVA_VERSION>
          Override //JAVA requested version

      --javaagent <JAVA_AGENTS>
          Additional java agent, same shape as //JAVAAGENT

      --cache-dir <CACHE_DIR>
          Override cache directory

      --trust
          Trust this remote script content hash before testing

  -h, --help
          Print help (see a summary with '-h')
```

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-test
```
