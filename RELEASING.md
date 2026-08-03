# Releasing SACS

SACS releases are prepared privately and reviewed before any visibility change, tag, or GitHub Release publication.

## Preparation gates

1. Review the public-surface matrix and risk register.
2. Verify the constitutional and implementation manifests in their original workspace contexts.
3. Run formatting, workspace tests, Clippy, and Cargo metadata checks.
4. Scan included files for machine-specific paths, secrets, internal prompts, archives, and excluded project-management material.
5. Review the changelog, license, NOTICE, security guidance, and public integrity note.
6. Obtain owner approval for unresolved review items.

## Versioning and immutability

The v1.0.0 release line corresponds to the adopted SACS Constitutional Baseline v1.0.0 and SACS Implementation Baseline v1.0.0. Tags and release assets must be created only after owner approval. Published release assets are immutable: corrections require a new version or an explicitly documented replacement process, never silent retroactive mutation.

Crates.io publication, deployment, provider integration, and external-verifier integration require separate authorization and are not implied by this repository release.
