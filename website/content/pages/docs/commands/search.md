---

title: jbx search command
description: Search Maven Central artifacts
---

# `search`

Search Maven Central artifacts by text or coordinates, with filters.

## When to use it

- Find the current coordinate for a library before adding `//DEPS`.
- Resolve ambiguous artifact names when a README only gives a product name.
- Let an agent rank candidate dependencies without scraping Maven Central HTML.

## Common workflows

```bash
jbx search picocli --json
jbx search --group org.junit.platform --id junit-platform-console-standalone --json
jbx search --group com.fasterxml.jackson.core --id jackson-databind --version 2.17.2 --json
```

## Real-life examples

### Repository maintenance

Use `search` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Discover guidance with `jbx skill get jbx-search`.
2. Run the command in the narrowest scope that answers the task.
3. Prefer JSON/structured output when this command exposes it.
4. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

Prefer exact group/artifact filters once a candidate is known. Do not auto-upgrade production dependencies solely because search shows a newer version.

## JSON and schema

`--json` returns query metadata, `numFound`, and artifact records. Website schema: `/docs/schemas/#search-json`.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx search`

```text
Search Maven Central for artifacts

Usage: jbx search [OPTIONS] [QUERY]...

Arguments:
  [QUERY]...  Search text, Solr query, or Maven coordinate (group:artifact[:version])

Options:
      --group <GROUP>      Solr groupId filter (maps to g:<group>)
      --id <ID>            Solr artifactId filter (maps to a:<id>)
      --version <VERSION>  Solr version filter (maps to v:<version> and searches the gav core)
  -n, --limit <LIMIT>      Maximum number of results to return [default: 20]
      --json               Return structured JSON for agent/tool consumption
  -h, --help               Print help
```

## Skill

Agents can fetch the matching versioned skill with:

```bash
jbx skill get jbx-search
```
