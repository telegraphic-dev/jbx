---
name: jbx-publish
description: Publish Java projects to Maven repositories, including Maven Central.
---

# `publish`

Publish Java projects to Maven repositories, including Maven Central.

## When to use it

- Prepare a library or script artifact for Maven Central review.
- Create a local Maven repository for integration tests.
- Publish documentation sidecars next to Java artifacts so agents can inspect APIs.

## Common workflows

```bash
jbx publish --file jbx.json --dry-run
jbx publish --file jbx.json --serve 8080
jbx publish --file jbx.json --publish
```

## Real-life examples

### Repository maintenance

Use `publish` as part of a repeatable repository workflow rather than a one-off shell trick. Start from the smallest safe command, inspect its output, then widen the scope only after the result is clear.

### Agent loop

1. Run the command in the narrowest scope that answers the task.
2. Prefer JSON/structured output when this command exposes it.
3. Verify the claimed result with files, exit codes, or the next quality gate.

## Agent notes

Publishing can be external and irreversible. Use `--dry-run` first, inspect generated POMs/artifacts/signatures, and ask before real Portal upload unless explicitly requested.

## JSON and schema

No `--json` mode yet. Use dry-run output and generated bundle files as the verification contract.

## Verification checklist

- Confirm the command exit code matches the intended gate.
- For mutating commands, inspect `git diff` or the generated artifact path.
- For JSON modes, parse the output instead of scraping the human form.
- For dependency/JDK/network behavior, run `jbx doctor --json` when the environment is suspect.

## Arguments and flags

This section is copied from the CLI help for this release so the page explains the actual accepted arguments.

### `jbx publish`

```text
Prepare Maven Central publishing artifacts

Usage: jbx publish [OPTIONS] [SCRIPT]

Arguments:
  [SCRIPT]  Java source file to publish. Defaults to jbx.json main when --file is used

Options:
      --file <FILE>
          jbx descriptor file. Defaults to ./jbx.json when present
      --version <VERSION>
          Override version from jbx.json or //GAV
  -o, --output <OUTPUT>
          Output Maven Central bundle ZIP path
      --target-dir <TARGET_DIR>
          Working directory for staged publish artifacts
      --package <PACKAGE_NAME>
          Override package used when staging default-package sources
      --cache-dir <CACHE_DIR>
          Override cache directory
      --dry-run
          Prepare and verify artifacts without uploading
      --skip-signing
          Allow unsigned dry-run bundles for local inspection
      --gpg-key <GPG_KEY>
          GPG key ID/email to use for detached ASCII signatures
      --publish
          Upload to Maven Central and publish after validation
      --serve <SERVE>
          Serve a local Maven repository containing the artifact on the given port
      --publishing-type <PUBLISHING_TYPE>
          Maven Central Portal publishing type for the upload [default: automatic] [possible values: automatic, user-managed]
      --no-wait
          Do not poll Central after uploading the deployment bundle
      --max-wait-seconds <MAX_WAIT_SECONDS>
          Maximum seconds to wait for Maven Central publication before exiting [default: 600]
  -h, --help
          Print help
```
