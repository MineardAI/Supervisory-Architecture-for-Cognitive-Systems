# Contributing to SACS

Contributions should preserve the distinction between the normative constitutional Contracts 000–012 and this subordinate implementation realization.

## Scope

Acceptable contributions include documentation corrections, bounded implementation maintenance, tests, evidence improvements, and public-surface hygiene. Semantic or architectural changes require an issue describing the affected contract, authority boundary, artifact identity, handoff, failure behavior, and compatibility impact before a pull request.

Do not expand authority, add successor contracts, introduce provider or runtime integrations, claim external verification, or convert implementation choices into constitutional requirements.

## Validation

Run from the repository root:

```text
cargo fmt --manifest-path implementation/Cargo.toml --all -- --check
cargo test --manifest-path implementation/Cargo.toml --workspace
cargo clippy --manifest-path implementation/Cargo.toml --workspace --all-targets -- -D warnings
cargo metadata --manifest-path implementation/Cargo.toml --no-deps
```

Implementation behavior changes must update the applicable source matrix, realization profile, conformance vectors, evidence index, and implementation baseline records. Constitutional changes require separate constitutional change control and must not be smuggled through an implementation pull request.

## Pull requests

Explain the scope, affected boundaries, tests, exclusions, and any manifest or public-surface consequences. Keep pull requests focused and do not include internal prompts, archives, status logs, credentials, or machine-specific paths.
