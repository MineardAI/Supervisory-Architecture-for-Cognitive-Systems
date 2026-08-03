# SACS-IMP-000 Constitutional Conformance Foundation

This workspace is the first realization layer for SACS v1.0.0. It provides deterministic, authority-neutral mechanisms only. It does not implement Contract 001 or any later contract policy, runtime semantics, provider, API, persistence deployment, history, or verification logic.

Run from this directory:

```text
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

The implementation profile and traceability records in `docs/` are part of the conformance evidence. `evidence/` contains vector and gate indexes; it is not constitutional history.
