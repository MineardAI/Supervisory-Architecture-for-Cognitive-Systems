# SACS-IMP-007 Source-to-Implementation Matrix

| Requirement | Source | Implementation | Evidence | Status |
|---|---|---|---|---|
| committed upstream admission and cycle coherence | Contract 007 Controller, §§1, 7 | `UpstreamContext::validate` | tests and EV-CTRL-0001 | normative |
| explicit controller definitions/configuration | Section 014, §§14-18 | `ControllerDefinition`, `CommittedControllerConfiguration` | definition/configuration API | normative |
| explicit or stateless controller state | Section 04, §§4.4-4.10 | `ControllerStateInput`, state types | state validation test | normative |
| deterministic committed error | Section 05 Error Metrics, §§5.1-5.18 | `control_error` | deterministic construction | normative/reference-derived |
| constraint and action-space binding | Section 017, §§17.4-17.13; Section 06, §§6.5-6.12 | `bind_constraints` | bounded-command test | normative |
| validated composition and evaluation | Section 019, §§19.4-19.14 | `bind_composition`, `evaluate` | reference evaluation API | normative/reference-derived |
| separate validation and certification | Contract 007 Controller, §§20-21 | `validate_command`, `validate_state`, `certify` | typed API | normative |
| atomic canonical commitment | Section 022, §§22.4-22.14 | `prepare`, `commit`, `ControllerStore` | EV-CTRL-0001 | normative |
| Contract 008 boundary | Section 022, §§22.11-22.14 | `CommittedControlCommandHandoff` | handoff API | normative |
| production control laws and translation | Contract 007 exclusions; Section 022 | not implemented | decisions/evidence | deferred |
