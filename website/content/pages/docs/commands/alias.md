---

title: jbx alias command
description: Add, remove, and list aliases from nearby jbang-catalog.json files.
---

# `alias`

Add, remove, and list aliases from nearby `jbang-catalog.json` files.

## When to use it

- Expose a repository script as a stable team command.
- Inspect catalog aliases before deciding what `jbx <alias>` means.
- Clean up renamed scripts while keeping the catalog understandable.

## Common workflows

```bash
jbx alias list --json
jbx alias add hello Hello.java --description "Run the hello script"
jbx alias remove hello
```

## Real-life examples

### Repository maintenance

Use `alias` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Discover guidance with `jbx skill get jbx-alias`.
2. Run the command in the narrowest scope that answers the task.
3. Prefer JSON/structured output when this command exposes it.
4. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

Read `alias list --json` before changing a catalog. Preserve human descriptions because they become discovery text for future agents.

## JSON and schema

`jbx alias list --json` returns aliases and target metadata from the discovered catalog. Website schema: `/docs/schemas/#alias-json`.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx alias`

```text
Manage aliases from jbang-catalog.json

Usage: jbx alias <COMMAND>

Commands:
  add     Add alias for a script reference
  remove  Remove an existing alias
  list    List aliases from the nearest jbang-catalog.json
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `jbx alias list`

```text
List aliases from the nearest jbang-catalog.json

Usage: jbx alias list [OPTIONS]

Options:
      --json  Print JSON instead of tab-separated text
  -h, --help  Print help
```

### `jbx alias add`

```text
Add alias for a script reference

Usage: jbx alias add [OPTIONS] <SCRIPT> [ARGS]...

Arguments:
  <SCRIPT>   Script path, URL, or alias reference
  [ARGS]...  Arguments stored in the alias and prepended at run time

Options:
  -g, --global
          Use the global user catalog file (~/.jbang/jbang-catalog.json)
  -f, --file <FILE>
          Path to the catalog file or directory to use
      --name <NAME>
          Alias name (defaults to the script filename stem)
      --description <DESCRIPTION>
          Description for the alias
      --force
          Force overwrite of an existing alias
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
          Requested Java version
      --javaagent <JAVA_AGENTS>
          Additional java agent, same shape as //JAVAAGENT
      --main <MAIN_CLASS>
          Main class for the alias
      --docs <DOCS>
          Documentation reference for the alias
  -h, --help
          Print help
```

### `jbx alias remove`

```text
Remove an existing alias

Usage: jbx alias remove [OPTIONS] <NAME>

Arguments:
  <NAME>  Alias name to remove

Options:
  -g, --global       Use the global user catalog file (~/.jbang/jbang-catalog.json)
  -f, --file <FILE>  Path to the catalog file or directory to use
  -h, --help         Print help
```

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-alias
```
