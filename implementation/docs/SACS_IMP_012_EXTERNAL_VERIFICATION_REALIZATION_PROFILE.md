# SACS-IMP-012 External Verification Profile and Evidence Interface

Status: CONDITIONALLY COMPLETE. `sacs-external-verification-interface` consumes only the Contract 011 `Contract012Handoff` containing a committed supervisory cycle record. It constructs claims, binds a profile/requirements/method, evaluates applicability, resolves and validates evidence, commits a submission, records custody, admits an externally issued result, certifies only faithful SACS preservation, commits the result, and commits one terminal exchange record.

External judgment remains external. Payload, judgment, limitations, issuer correlation, and hashes are preserved exactly. Unfavorable or inconclusive judgments remain valid results and do not become exchange failures. Replay and audit are non-constitutive and do not contact a verifier or mutate history. The adapter trait is transport-neutral and fixture-only; no production network/provider integration exists.
