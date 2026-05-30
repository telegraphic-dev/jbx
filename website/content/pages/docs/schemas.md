---
title: jbx JSON schemas
description: Stable JSON output shapes for agent-facing jbx commands.
---

# JSON schemas

Commands that advertise `--json` keep their output predictable for agents. Use the schema notes below as the public contract and prefer these modes over scraping terminal text.

## check JSON

`jbx check --json` returns a command status plus diagnostics. Diagnostics include source path, line, column, severity, tool, code when available, and message. Empty diagnostics with success means the source passed the configured javac/Error Prone gate.

## test JSON

`jbx test --json` returns test execution status, counts, failures, and output paths. With `--coverage`, it includes a `coverage` object containing JaCoCo exec/html/xml paths and aggregate counters.

## docs JSON

`jbx docs --json` follows the published sidecar schema: package/module metadata, documented types, methods/fields/constructors, signatures, descriptions, examples, and source provenance. Full schema: [`jbx-docs-schema.md`](/docs/jbx-docs-schema/).

## doctor JSON

`jbx doctor --json` returns a list of checks. Each check has a stable `name`, `status` (`ok`, `warn`, `fail`, or `skipped`), `summary`, and optional details. Agents should fail closed on `fail`, surface `warn`, and ignore `skipped` unless the skipped capability was explicitly requested.

## rewrite JSON

`jbx rewrite patch --json` and `jbx rewrite apply --json` report requested recipes/modules, scanned sources, patch/report paths, whether changes were produced/applied, and process status. `jbx rewrite modules --json` returns module search hits; `jbx rewrite recipes --json` returns recipe descriptors and option metadata.

## search JSON

`jbx search --json` returns the normalized query, Maven Central `numFound`, and artifact records with group, artifact, latest/version, packaging, timestamp, and version-count metadata when available.

## cache/catalog/alias/template JSON

`jbx cache list --json`, `jbx catalog list --json`, `jbx alias list --json`, and `jbx template list --json` return arrays of discovered entries with the same field names used internally by the command output. Treat missing optional fields as absent, not empty strings.

## skill list JSON

`jbx skill list --json` returns:

```json
{
  "skills": [
    { "name": "jbx-check", "description": "Check Java source with structured diagnostics." }
  ]
}
```
