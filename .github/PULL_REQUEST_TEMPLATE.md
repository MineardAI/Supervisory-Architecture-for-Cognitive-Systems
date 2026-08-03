## Summary

Describe the change and its intended scope.

## Authority and boundary review

- [ ] This change preserves constitutional authority and ownership boundaries.
- [ ] No successor contract or authority expansion is introduced.
- [ ] Public-surface exclusions and machine-specific paths were reviewed.

## Verification

- [ ] `cargo fmt --manifest-path implementation/Cargo.toml --all -- --check`
- [ ] `cargo test --manifest-path implementation/Cargo.toml --workspace`
- [ ] `cargo clippy --manifest-path implementation/Cargo.toml --workspace --all-targets -- -D warnings`
- [ ] `cargo metadata --manifest-path implementation/Cargo.toml --no-deps`
