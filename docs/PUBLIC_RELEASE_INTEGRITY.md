# Public Release Integrity

This repository is an intentionally curated public-release surface derived from the private SACS workspace.

## Scope

The public tree includes the Contracts 000–012 source corpus, subordinate Source Pack guidance, adopted baseline documents, the bounded Rust implementation workspace, public implementation matrices and conformance indexes, license and contributor documentation, and release hygiene controls.

It excludes Project Suite material, status logs, archives, Codex and implementation prompts, private review records, machine-specific paths, temporary files, build output, caches, credentials, and other internal working material.

## Source mapping

- Private SACS contract files map to `contracts/` without changing their normative content.
- Private `Source Pack/` maps to `source-pack/` and remains non-authoritative.
- Private `implementation/` maps to `implementation/`; public code and selected documentation preserve the bounded workspace behavior.
- Adopted baseline documents retain their original workspace-context identity. The public tree is not claimed to be path-identical to either private manifest.

## Verification

Run from the repository root:

```text
cargo fmt --manifest-path implementation/Cargo.toml --all -- --check
cargo test --manifest-path implementation/Cargo.toml --workspace
cargo clippy --manifest-path implementation/Cargo.toml --workspace --all-targets -- -D warnings
cargo metadata --manifest-path implementation/Cargo.toml --no-deps
```

The constitutional and implementation manifests authenticate their adopted source workspaces. Public-release integrity is verified separately by the public-surface review, exclusion scans, Git diff, and CI workflow. Neither public packaging nor the public-release manifest replaces an adopted baseline.

## Authority statement

No public-release copy changes constitutional authority, contract ownership, canonical artifact identity, terminal boundaries, or external-verification ownership. The public repository does not claim external verification, production certification, accreditation, or deployment approval.
