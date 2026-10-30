# githubActions

Rust search CLI with case-sensitive search by default. Set `IGNORE_CASE=1`
to match without case sensitivity.

Run the same formatting and test checks used by CI:

```sh
./scripts/check.sh
```

Activate the version-controlled pre-commit hook for this checkout:

```sh
./scripts/install-hooks.sh
```

The hook checks the working tree before each commit. GitHub Actions runs the
same checks on pushes and pull requests to `main`, plus a locked build.
Rust and rustfmt are required locally; CI installs the stable toolchain.

Tests cover search matches and edge cases, argument parsing, both CLI search
modes, missing arguments, missing files, and invalid UTF-8. CLI tests isolate
environment variables in child processes and clean up their temporary files.
