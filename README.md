# SACS — Supervisory Architecture for Cognitive Systems

SACS is a deterministic supervisory architecture for cognitive, generative, agentic, and adaptive systems.

## What SACS is

SACS is a constitutional supervisory control plane. It separates observation, state estimation, governance, supervisory objectives, bounded control, runtime translation, execution disposition, continuity, constitutional history, and external-verification interfaces into explicit authority boundaries.

Committed artifacts are the only authoritative downstream inputs. Authority may narrow downstream, but it may not broaden. Construction is not commitment. Validation is not certification. Certification is not execution. Execution is not history.

## What SACS is not

SACS supervises systems; it does not perform their primary work, host models, or act as a provider runtime. This repository does not include production runtime integration, provider integration, networking, shell execution, deployment topology, UI, or external-verifier implementation.

Internal certification is not external verification. SACS does not self-verify and does not issue external verification judgments. Contract 012 is the terminal contract in the v1.0.0 core; institutional learning, constitutional evolution, and successor contracts are outside that core.

## Constitutional flow

```text
Observation
  -> State Estimation
  -> Operational State Vector
  -> Governance
  -> Supervisory Objectives
  -> Operational Profiles and Reference States
  -> Controller
  -> Inference Control
  -> Execution Interface
  -> Supervisory Continuity
  -> Constitutional History
  -> External Verification Interface
```

## Repository structure

- `contracts/` — public copies of the Contracts 000–012 source corpus.
- `source-pack/` — subordinate constitutional-supporting guidance.
- `implementation/` — Rust workspace and bounded reference realization for SACS-IMP-000 through SACS-IMP-012.
- `docs/` — public release integrity documentation.
- Baseline and manifest files — adopted integrity records from the original SACS workspace; they are not replaced by this curated public tree.

## Implementation status

SACS Implementation Baseline v1.0.0 is adopted as a derivative realization of SACS Constitutional Baseline v1.0.0. The implementation contracts are conditionally complete within their bounded scope. Conditional completion does not claim production deployment readiness, accreditation, certification, or external verification.

## Verification

From the repository root:

```text
cargo fmt --manifest-path implementation/Cargo.toml --all -- --check
cargo test --manifest-path implementation/Cargo.toml --workspace
cargo clippy --manifest-path implementation/Cargo.toml --workspace --all-targets -- -D warnings
cargo metadata --manifest-path implementation/Cargo.toml --no-deps
```

The adopted local implementation verification observed 60 unit tests passed, zero failures, and zero doc-tests present. Run the commands locally for the checked-out public tree.

## Non-goals

SACS does not define domain-specific control laws, production policy catalogs, provider protocols, deployment operations, legal interpretation, institutional learning, or successor constitutional authority.

## Contributing and security

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution boundaries and [SECURITY.md](SECURITY.md) for private vulnerability reporting guidance.

## License

SACS is released under the Apache License 2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

## Project status

This branch is a private release-preparation surface. Public visibility, merge, tagging, GitHub Release publication, and deployment approval are not part of this preparation step.
