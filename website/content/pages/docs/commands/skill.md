---

title: jbx skill command
description: List and print version-matched bundled agent skills.
---

# `skill`

List and print version-matched bundled agent skills.

## When to use it

- Bootstrap an agent with command-specific guidance before touching a Java repo.
- Discover whether the installed jbx release knows a command or workflow.
- Keep offline automation aligned with the exact binary version, not a stale website page.

## Common workflows

```bash
jbx skill list
jbx skill list --json
jbx skill get jbx-check
jbx skill get jbx
```

## Real-life examples

### Repository maintenance

Use `skill` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Discover guidance with `jbx skill get jbx-skill`.
2. Run the command in the narrowest scope that answers the task.
3. Prefer JSON/structured output when this command exposes it.
4. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

This is the first command an agent should run. Fetch the specific command skill, follow it, then use the command page only for broader human context.

## JSON and schema

`jbx skill list --json` returns installed skill names and descriptions. `skill get` returns Markdown skill content.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx skill`

```text
Print version-matched agent skills bundled with this jbx release

Usage: jbx skill <COMMAND>

Commands:
  list  List version-matched skills bundled with this jbx binary
  get   Print a bundled skill. Defaults to the main jbx skill
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `jbx skill list`

```text
List version-matched skills bundled with this jbx binary

Usage: jbx skill list [OPTIONS]

Options:
      --json  Emit structured JSON for agents
  -h, --help  Print help
```

### `jbx skill get`

```text
Print a bundled skill. Defaults to the main jbx skill

Usage: jbx skill get [NAME]

Arguments:
  [NAME]  Skill name to print. Defaults to jbx

Options:
  -h, --help  Print help
```

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-skill
```
