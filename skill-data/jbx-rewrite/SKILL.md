---
name: jbx-rewrite
description: Discover OpenRewrite modules and recipes and preview or apply them
---

# `rewrite`

Preview or apply OpenRewrite recipes with jbx-managed dependencies and JDKs; discover modules and recipes.

## When to use it

- Preview a modernization recipe and review the patch before touching files.
- Discover which recipe module contains a migration an agent wants to run.
- Apply a mechanical cleanup after tests already cover the behavior.

## Common workflows

```bash
jbx rewrite modules --search spring --json
jbx rewrite recipes org.openrewrite.recipe:rewrite-testing-frameworks:3.8.0 --detail --json
jbx rewrite patch --recipe org.openrewrite.java.format.AutoFormat --source src/main/java --json
jbx rewrite apply --recipe org.openrewrite.java.format.AutoFormat --source src/main/java --json
```

## Real-life examples

### Repository maintenance

Use `rewrite` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Run the command in the narrowest scope that answers the task.
2. Prefer JSON/structured output when this command exposes it.
3. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

Default to `patch`, not `apply`. Treat `apply` as a mutating operation that needs an explicit task. After applying, run `jbx check --json` and relevant tests.

## JSON and schema

JSON modes exist for `patch`, `apply`, `modules`, and `recipes`. Schemas are summarized at `/docs/schemas/#rewrite-json`.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx rewrite`

```text
Run OpenRewrite recipes against Java source trees

Usage: jbx rewrite <COMMAND>

Commands:
  apply    Apply OpenRewrite recipes and modify sources
  patch    Preview OpenRewrite recipes and write rewrite/rewrite.patch without modifying sources
  modules  Search Maven Central for OpenRewrite modules
  recipes  List or search recipes available from an OpenRewrite module
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `jbx rewrite patch`

```text
Preview OpenRewrite recipes and write rewrite/rewrite.patch without modifying sources

Usage: jbx rewrite patch [OPTIONS]

Options:
      --recipe <RECIPES>
          OpenRewrite recipe to run (short alias or fully-qualified recipe name). Repeatable and comma-splittable
      --module <MODULES>
          OpenRewrite module to add (short name for org.openrewrite:rewrite-*, or full GAV). Repeatable and comma-splittable
      --source <SOURCES>
          Java source file or directory. Repeatable; defaults to the current directory
      --option <OPTIONS>
          Recipe option as key=value. For multiple recipes, use RecipeName.key=value
      --report <REPORT>
          Report directory for rewrite.patch [default: rewrite]
      --json
          Print JSON summary after the human summary
      --fail-on-changes
          Exit with code 2 when recipes would make changes
      --no-fail-on-invalid-recipes
          Continue when OpenRewrite reports invalid active recipes
      --cache-dir <CACHE_DIR>
          Override dependency/helper cache directory
      --repo <REPOS>
          Additional repository for recipe modules
      --rewrite-version <REWRITE_VERSION>
          OpenRewrite version for built-in modules [default: 8.56.1]
  -h, --help
          Print help
```

### `jbx rewrite apply`

```text
Apply OpenRewrite recipes and modify sources

Usage: jbx rewrite apply [OPTIONS]

Options:
      --recipe <RECIPES>
          OpenRewrite recipe to run (short alias or fully-qualified recipe name). Repeatable and comma-splittable
      --module <MODULES>
          OpenRewrite module to add (short name for org.openrewrite:rewrite-*, or full GAV). Repeatable and comma-splittable
      --source <SOURCES>
          Java source file or directory. Repeatable; defaults to the current directory
      --option <OPTIONS>
          Recipe option as key=value. For multiple recipes, use RecipeName.key=value
      --report <REPORT>
          Report directory for rewrite.patch [default: rewrite]
      --json
          Print JSON summary after the human summary
      --fail-on-changes
          Exit with code 2 when recipes would make changes
      --no-fail-on-invalid-recipes
          Continue when OpenRewrite reports invalid active recipes
      --cache-dir <CACHE_DIR>
          Override dependency/helper cache directory
      --repo <REPOS>
          Additional repository for recipe modules
      --rewrite-version <REWRITE_VERSION>
          OpenRewrite version for built-in modules [default: 8.56.1]
  -h, --help
          Print help
```

### `jbx rewrite modules`

```text
Search Maven Central for OpenRewrite modules

Usage: jbx rewrite modules [OPTIONS]

Options:
      --search <SEARCH>
          Filter Maven Central modules by recipe/module name
      --group <GROUPS>
          Maven groupId to search. Defaults to org.openrewrite.recipe and org.openrewrite
      --limit <LIMIT>
          Maximum number of modules to print
      --json
          Print machine-readable JSON
      --rewrite-version <REWRITE_VERSION>
          OpenRewrite version used when expanding short module names
  -h, --help
          Print help
```

### `jbx rewrite recipes`

```text
List or search recipes available from an OpenRewrite module

Usage: jbx rewrite recipes [OPTIONS] <MODULE>

Arguments:
  <MODULE>  OpenRewrite module to inspect (short name or full GAV)

Options:
      --search <SEARCH>
          Filter recipes by short name, fully-qualified name, display name, or description
      --limit <LIMIT>
          Maximum number of recipes to print
      --detail
          Include recipe descriptions and options
      --json
          Print machine-readable JSON
      --cache-dir <CACHE_DIR>
          Override dependency/helper cache directory
      --repo <REPOS>
          Additional repository for recipe modules
      --rewrite-version <REWRITE_VERSION>
          OpenRewrite version for built-in modules [default: 8.56.1]
  -h, --help
          Print help
```
