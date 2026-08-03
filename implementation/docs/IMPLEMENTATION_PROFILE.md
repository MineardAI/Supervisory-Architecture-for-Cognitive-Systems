# SACS-IMP-000 Implementation Profile

| Field | Value |
|---|---|
| Implementation identity | `sacs-imp-000-foundation` |
| Implementation version | `0.1.0` |
| Baseline | SACS Constitutional Baseline `1.0.0` |
| Supported contracts | `000-012` declarations only; no contract policy implemented |
| Registry version | `1.0.0` |
| Numeric profile | `exact` declaration; no supervisory numeric algorithm |
| Technology | Rust 2021, standard library plus SHA-256 crate |
| Release profile | unpublished, workspace-local, `publish = false` |
| Exclusions | Contract 003+ policy; production estimator mathematics, stateful estimator transitions, covariance computation, freshness/quality/confidence policy selection, replay-source execution, OSV publication, and external transport; governance, objectives, profiles, controller, translation, execution, modes, history, verification, APIs, networking, UI, deployment |

The profile is a declaration and validation surface. SACS-IMP-001 adds Observation, SACS-IMP-002 adds bounded State Estimation, SACS-IMP-003 adds bounded OSV publication, SACS-IMP-004 adds bounded Governance, SACS-IMP-005 adds bounded Supervisory Objectives, SACS-IMP-006 adds bounded Operational Profiles and Reference States, SACS-IMP-007 adds bounded Controller realization, SACS-IMP-008 adds semantic-preserving Inference Control translation, SACS-IMP-009 adds bounded Execution Interface disposition commitment, SACS-IMP-010 adds bounded Supervisory Continuity mode commitment, SACS-IMP-011 adds bounded Realization Certification and Supervisory Provenance, and SACS-IMP-012 adds bounded External Verification Interface exchange handling while retaining external-verifier sovereignty and successor-contract exclusions. It does not grant authority to any later contract and does not silently downgrade unsupported versions.
