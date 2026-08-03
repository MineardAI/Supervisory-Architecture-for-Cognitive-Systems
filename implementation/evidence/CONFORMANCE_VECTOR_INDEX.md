# Conformance Vector Index

| Vector | Component | Input | Expected result |
|---|---|---|---|
| V-001 | canonical-versioning | malformed `1` | reject |
| V-002 | canonical-versioning | unequal version binding | reject as unsupported |
| V-003 | canonical-serialization | fields inserted `z`, then `a` | bytes equal to canonical `a`, then `z` |
| V-004 | canonical-serialization | encoded document | decode equals original |
| V-005 | canonical-hashing | bytes `SACS` | SHA-256 `07889eb79a3581eff7395cd0eab5d5814a5bc72a8c4b07eb0e3c7e5b70256ad9` |
| V-006 | canonical-identity | identical versions/binding/content | identical identifier and content hash |
| V-007 | deterministic-ordering | `z,a,m` | `a,m,z` |
| V-008 | commitment-engine | same artifact/content twice | second operation idempotent |
| V-009 | commitment-engine | same artifact/different content | conflict, no second event |
| V-010 | failure-model | same class/detail twice | identical immutable failure identity |
| O-001 | sacs-observation | registered source + valid record | admitted observation |
| O-002 | sacs-observation | unknown/unsupported source/type | typed rejection |
| O-003 | sacs-observation | receipt after cutoff | cutoff/late-data rejection or fault policy |
| O-004 | sacs-observation | open window after close | post-close rejection |
| O-005 | sacs-observation | records inserted in reverse order | canonical source-time/receipt/source/sequence/type/id order |
| O-006 | sacs-observation | identical duplicate | idempotent admission |
| O-007 | sacs-observation | same identity/different content or sequence | explicit conflict |
| O-008 | sacs-observation | unavailable + absent value | unavailable state preserved |
| O-009 | sacs-observation | committed batch retry | identical handoff identity and idempotent commitment |
| SE-001 | sacs-state-estimation | committed ObservationHandoff | accepted input binding |
| SE-002 | sacs-state-estimation | modified batch payload | invalid handoff failure |
| SE-003 | sacs-state-estimation | explicit estimator/profile/registry | deterministic binding |
| SE-004 | sacs-state-estimation | repeated exact numeric input | identical Estimated State identity/hash |
| SE-005 | sacs-state-estimation | missing required observation | explicit failure |
| SE-006 | sacs-state-estimation | stale input under reject profile | explicit stale failure |
| SE-007 | sacs-state-estimation | conflicting values | unresolved conflict failure |
| SE-008 | sacs-state-estimation | source confidence plus uncertainty | distinct preserved fields |
| SE-009 | sacs-state-estimation | unknown uncertainty/covariance | explicit Unknown/NOT_PRODUCED |
| SE-010 | sacs-state-estimation | Estimated State handoff | no OSV fields/publication |
| OVT-001 | sacs-operational-state-vector | valid EstimatedStateHandoff | admitted |
| OVT-002 | sacs-operational-state-vector | modified handoff hash | explicit rejection |
| OVT-003 | sacs-operational-state-vector | registered projection/target field | accepted or explicit failure |
| OVT-004 | sacs-operational-state-vector | missing required field/component | explicit rejection |
| OVT-005 | sacs-operational-state-vector | component validity and quality | preserved distinctly |
| OVT-006 | sacs-operational-state-vector | estimator uncertainty | field uncertainty preserved |
| OVT-007 | sacs-operational-state-vector | covariance posture | reference/policy preserved |
| OVT-008 | sacs-operational-state-vector | registry/schema mismatch | explicit rejection |
| OVT-009 | sacs-operational-state-vector | mixed cycle/cutoff | temporal atomicity rejection |
| OVT-010 | sacs-operational-state-vector | failed validation | no canonical partial OSV |
| OVT-011 | sacs-operational-state-vector | repeated valid commit | immutable/idempotent commitment |
| OVT-012 | sacs-operational-state-vector | prior OSV lineage | explicit sequence binding |
| OVT-013 | sacs-operational-state-vector | identical context | deterministic fields/hash |
| OVT-014 | sacs-operational-state-vector | downstream handoff | committed OSV only |
| OVT-015 | sacs-operational-state-vector | later cycle | prior artifact remains unchanged |
| OVT-016 | sacs-operational-state-vector | provenance binding | complete identity/provenance check |
| GT-001 | sacs-governance | registered external authority | admission succeeds |
| GT-002 | sacs-governance | source integrity mismatch | explicit failure |
| GT-003 | sacs-governance | adapter declaration | no source-authority invention |
| GT-004 | sacs-governance | permission absent | no ALLOW default |
| GT-005 | sacs-governance | prohibition with permission | prohibition preserved |
| GT-006 | sacs-governance | committed snapshot retry/mutation | immutable/idempotent commitment |
| GT-007 | sacs-governance | draft or Estimated State input | OSV-only rejection |
| GT-008 | sacs-governance | applicable scope/effective time | applicability accepted |
| GT-009 | sacs-governance | future/expired source | explicit inapplicability |
| GT-010 | sacs-governance | revocation | append-only evidence; later snapshot only |
| GT-011 | sacs-governance | conflicting rules | preserved conflict |
| GT-012 | sacs-governance | identical rule insertion orders | deterministic precedence |
| GT-013 | sacs-governance | missing governance | reduced-authority outcome |
| GT-014 | sacs-governance | schema/registry mismatch | explicit rejection |
| GT-015 | sacs-governance | objective-shaped candidate | no objective evaluation |
| GT-016 | sacs-governance | profile-shaped candidate | no profile selection |
| GT-017 | sacs-governance | controller action | no controller API |
| GT-018 | sacs-governance | translation | no translation API |
| GT-019 | sacs-governance | execution | no execution API |
| GT-020 | sacs-governance | provenance fields | complete source/OSV/commitment lineage |
| GT-021 | sacs-governance | identical explicit context | deterministic snapshot/hash |
| GT-022 | sacs-governance | downstream handoff | committed Governance Snapshot only |
| OJT-001 | sacs-supervisory-objectives | registered objective source | admission succeeds |
| OJT-002 | sacs-supervisory-objectives | raw/draft OSV input | OSV-only rejection |
| OJT-003 | sacs-supervisory-objectives | Governance Snapshot handoff | committed governance only |
| OJT-004 | sacs-supervisory-objectives | governance-prohibited objective | blocked/ineligible |
| OJT-005 | sacs-supervisory-objectives | governance-required objective | required status preserved |
| OJT-006 | sacs-supervisory-objectives | identical applicability context | deterministic result |
| OJT-007 | sacs-supervisory-objectives | deterministic satisfaction context | no implicit satisfaction |
| OJT-008 | sacs-supervisory-objectives | unresolved OSV field | unresolved, not satisfied |
| OJT-009 | sacs-supervisory-objectives | OSV uncertainty | preserved in evaluation profile |
| OJT-010 | sacs-supervisory-objectives | explicit priority | deterministic priority order |
| OJT-011 | sacs-supervisory-objectives | objective conflict | typed conflict |
| OJT-012 | sacs-supervisory-objectives | unresolved conflict | reduced-authority failure |
| OJT-013 | sacs-supervisory-objectives | suppression/withholding | non-active distinct status |
| OJT-014 | sacs-supervisory-objectives | supersession/persistence | prior set not mutated |
| OJT-015 | sacs-supervisory-objectives | candidate/commit atomicity | no partial canonical set |
| OJT-016 | sacs-supervisory-objectives | committed set retry | immutable/idempotent |
| OJT-017 | sacs-supervisory-objectives | cross-cycle OSV/governance | binding rejection |
| OJT-018 | sacs-supervisory-objectives | profile-shaped operation | no profile selection |
| OJT-019 | sacs-supervisory-objectives | control-shaped operation | no control API |
| OJT-020 | sacs-supervisory-objectives | provenance/registry binding | complete lineage |
| OJT-021 | sacs-supervisory-objectives | registry ordering/version | deterministic explicit binding |
| OJT-022 | sacs-supervisory-objectives | deterministic replay | identical canonical output/failure |
| OPR-001 | sacs-operational-profiles | valid OSV/Governance/Objective handoffs | accepted |
| OPR-002 | sacs-operational-profiles | upstream cycle mismatch | explicit rejection |
| OPR-003 | sacs-operational-profiles | profile definition publication | immutable definition |
| OPR-004 | sacs-operational-profiles | registry snapshot | deterministic immutable universe |
| OPR-005 | sacs-operational-profiles | candidate discovery | scope-only ordered set |
| OPR-006 | sacs-operational-profiles | eligible candidate | ELIGIBLE |
| OPR-007 | sacs-operational-profiles | ineligible candidate | explicit outcome/reasons |
| OPR-008 | sacs-operational-profiles | eligible incompatible candidate | distinct compatibility failure |
| OPR-009 | sacs-operational-profiles | no eligible candidate | explicit failure |
| OPR-010 | sacs-operational-profiles | no compatible candidate | explicit failure |
| OPR-011 | sacs-operational-profiles | deterministic single winner | selected candidate |
| OPR-012 | sacs-operational-profiles | explicit tie-break | stable winner |
| OPR-013 | sacs-operational-profiles | unresolved selection | no-winner failure |
| OPR-014 | sacs-operational-profiles | constructed profile | noncanonical deterministic artifact |
| OPR-015 | sacs-operational-profiles | committed profile | identity/hash preserved |
| OPR-016 | sacs-operational-profiles | constructed reference state | builder-bound deterministic artifact |
| OPR-017 | sacs-operational-profiles | committed reference state | identity/hash preserved |
| OPR-018 | sacs-operational-profiles | profile retry | idempotent commitment |
| OPR-019 | sacs-operational-profiles | profile conflict | explicit conflict |
| OPR-020 | sacs-operational-profiles | reference retry | idempotent commitment |
| OPR-021 | sacs-operational-profiles | reference conflict | explicit conflict |
| OPR-022 | sacs-operational-profiles | controller handoff | committed profile/reference only |
| OPR-023 | sacs-operational-profiles | configuration artifact | no control error/command |
| OPR-024 | sacs-operational-profiles | reference artifact | not control error |

| CTRL-001 | sacs-controller | valid committed upstream admission | accepted |
| CTRL-002 | sacs-controller | cycle mismatch | explicit failure |
| CTRL-003 | sacs-controller | definition/configuration binding | exact-bound |
| CTRL-004 | sacs-controller | configuration closure | no substitute |
| CTRL-005 | sacs-controller | explicit stateless input | accepted |
| CTRL-006 | sacs-controller | missing state partition | explicit failure |
| CTRL-007 | sacs-controller | deterministic Control Error | stable identity/hash |
| CTRL-008 | sacs-controller | metric/representation mismatch | rejected |
| CTRL-009 | sacs-controller | deterministic constraint binding | stable authority set |
| CTRL-010 | sacs-controller | Action Space expansion | rejected |
| CTRL-011 | sacs-controller | validated composition | stable identity/hash |
| CTRL-012 | sacs-controller | topology incompatibility | explicit failure |
| CTRL-013 | sacs-controller | reference-controller evaluation | deterministic |
| CTRL-014 | sacs-controller | Proposed Control Command | explicit identity/hash |
| CTRL-015 | sacs-controller | bounded command validation | accepted |
| CTRL-016 | sacs-controller | out-of-bounds command | rejected |
| CTRL-017 | sacs-controller | state-transition validation | separate result |
| CTRL-018 | sacs-controller | invalid retained state | explicit failure |
| CTRL-019 | sacs-controller | Authority Certificate | bounded evidence |
| CTRL-020 | sacs-controller | constraint certification failure | rejected |
| CTRL-021 | sacs-controller | atomic commitment bundle | one bundle |
| CTRL-022 | sacs-controller | incomplete bundle | rejected |
| CTRL-023 | sacs-controller | Committed Control Command | canonical |
| CTRL-024 | sacs-controller | identical retry | idempotent |
| CTRL-025 | sacs-controller | conflicting retry | explicit conflict |
| CTRL-026 | sacs-controller | exact-one terminal outcome | bounded store |
| CTRL-027 | sacs-controller | replay equivalence | deterministic |
| CTRL-028 | sacs-controller | Contract 008 handoff | command-only |
| CTRL-029 | sacs-controller | translation boundary | absent |
| CTRL-030 | sacs-controller | execution boundary | absent |

| TRANS-001 | sacs-inference-control | valid Contract 007 handoff | accepted |
| TRANS-002 | sacs-inference-control | Proposed Control Command rejection | rejected |
| TRANS-003 | sacs-inference-control | configuration closure | exact-bound |
| TRANS-004 | sacs-inference-control | missing configuration | explicit failure |
| TRANS-005 | sacs-inference-control | capability compatibility | accepted |
| TRANS-006 | sacs-inference-control | capability versus availability | distinct |
| TRANS-007 | sacs-inference-control | semantic unsupported | explicit failure |
| TRANS-008 | sacs-inference-control | Validated Translation Binding | deterministic |
| TRANS-009 | sacs-inference-control | requirement preservation | stable |
| TRANS-010 | sacs-inference-control | deterministic normalization | stable |
| TRANS-011 | sacs-inference-control | unauthorized semantic change | rejected |
| TRANS-012 | sacs-inference-control | exact numeric conversion | deterministic |
| TRANS-013 | sacs-inference-control | insufficient precision | explicit failure |
| TRANS-014 | sacs-inference-control | deterministic ordering | stable |
| TRANS-015 | sacs-inference-control | unrepresentable ordering | explicit failure |
| TRANS-016 | sacs-inference-control | Proposed Inference Command Set | noncanonical identity |
| TRANS-017 | sacs-inference-control | Translation Validation Report | separate success |
| TRANS-018 | sacs-inference-control | mandatory restriction loss | rejected |
| TRANS-019 | sacs-inference-control | Committed Inference Command Set | distinct canonical set |
| TRANS-020 | sacs-inference-control | Constructed Translation Transaction | deterministic identity |
| TRANS-021 | sacs-inference-control | Translation Commitment Record | canonical switch |
| TRANS-022 | sacs-inference-control | Committed Translation Transaction | canonical identity |
| TRANS-023 | sacs-inference-control | identical retry | idempotent |
| TRANS-024 | sacs-inference-control | conflicting retry | explicit conflict |
| TRANS-025 | sacs-inference-control | Execution Interface Handoff Envelope | preserved |
| TRANS-026 | sacs-inference-control | exact-one terminal outcome | bounded store |
| TRANS-027 | sacs-inference-control | deterministic replay | equivalent |
| TRANS-028 | sacs-inference-control | command set distinct from transaction | type separation |
| TRANS-029 | sacs-inference-control | commitment is not execution | no runtime call |
| TRANS-030 | sacs-inference-control | capability is not permission | authority preserved |

| EXEC-001 | sacs-execution-interface | valid committed transaction/envelope admission | accepted |
| EXEC-002 | sacs-execution-interface | invalid transaction lifecycle | explicit failure |
| EXEC-003 | sacs-execution-interface | transaction/envelope mismatch | rejected |
| EXEC-004 | sacs-execution-interface | deterministic eligibility | stable |
| EXEC-005 | sacs-execution-interface | reduced-authority disposition | preserved |
| EXEC-006 | sacs-execution-interface | plan identity | deterministic |
| EXEC-007 | sacs-execution-interface | adapter request binding | exact |
| EXEC-008 | sacs-execution-interface | adapter result binding | correlated |
| EXEC-009 | sacs-execution-interface | execution outcome identity | deterministic |
| EXEC-010 | sacs-execution-interface | no-outcome disposition | explicit |
| EXEC-011 | sacs-execution-interface | committed disposition identity | canonical |
| EXEC-012 | sacs-execution-interface | idempotent/conflicting commitment | bounded store |
| EXEC-013 | sacs-execution-interface | replay without adapter invocation | equivalent |

| CONT-001 | sacs-supervisory-continuity | valid continuity admission | accepted |
| CONT-002 | sacs-supervisory-continuity | invalid/uncommitted input | rejected |
| CONT-003 | sacs-supervisory-continuity | exact execution-disposition binding | preserved |
| CONT-004 | sacs-supervisory-continuity | execution-disposition mismatch | rejected |
| CONT-005 | sacs-supervisory-continuity | remain-in-mode result | deterministic |
| CONT-006 | sacs-supervisory-continuity | one eligible transition | selected |
| CONT-007 | sacs-supervisory-continuity | precedence resolution | explicit |
| CONT-008 | sacs-supervisory-continuity | unresolved precedence | failure |
| CONT-009 | sacs-supervisory-continuity | guard success | evidence-bearing |
| CONT-010 | sacs-supervisory-continuity | guard failure/conflict | explicit |
| CONT-011 | sacs-supervisory-continuity | reset not applicable | explicit |
| CONT-012 | sacs-supervisory-continuity | valid reset | immutable record |
| CONT-013 | sacs-supervisory-continuity | invalid reset | failure |
| CONT-014 | sacs-supervisory-continuity | hysteresis permits | accepted |
| CONT-015 | sacs-supervisory-continuity | hysteresis blocks | failure |
| CONT-016 | sacs-supervisory-continuity | hysteresis boundary | deterministic |
| CONT-017 | sacs-supervisory-continuity | dwell satisfied | accepted |
| CONT-018 | sacs-supervisory-continuity | dwell violation | failure |
| CONT-019 | sacs-supervisory-continuity | dwell boundary | deterministic |
| CONT-020 | sacs-supervisory-continuity | explicit time/cutoff | preserved |
| CONT-021 | sacs-supervisory-continuity | repeated evaluation | equivalent |
| CONT-022 | sacs-supervisory-continuity | mode identity stability | deterministic |
| CONT-023 | sacs-supervisory-continuity | continuity-record identity | deterministic |
| CONT-024 | sacs-supervisory-continuity | repeated commitment | idempotent |
| CONT-025 | sacs-supervisory-continuity | conflicting commitment | rejected |
| CONT-026 | sacs-supervisory-continuity | success/failure exclusivity | bounded |
| CONT-027 | sacs-supervisory-continuity | mode/record distinction | type-separated |
| CONT-028 | sacs-supervisory-continuity | prior artifact immutability | preserved |
| CONT-029 | sacs-supervisory-continuity | replay without recommitment | equivalent |
| CONT-030 | sacs-supervisory-continuity | replay non-equivalence | explicit |
| CONT-031 | sacs-supervisory-continuity | Contract 011 handoff | exact identities |
| CONT-032 | sacs-supervisory-continuity | downstream cannot reopen adjudication | boundary preserved |
| CONT-033 | sacs-supervisory-continuity | archive exclusion | no import |
| CONT-034 | sacs-supervisory-continuity | baseline/manifest | unchanged |

| HIST-001 | sacs-constitutional-history | exact committed disposition/continuity admission | accepted |
| HIST-002 | sacs-constitutional-history | uncommitted execution artifact | rejected |
| HIST-003 | sacs-constitutional-history | uncommitted continuity artifact | rejected |
| HIST-004 | sacs-constitutional-history | disposition/continuity mismatch | rejected |
| HIST-005 | sacs-constitutional-history | cycle mismatch | rejected |
| HIST-006 | sacs-constitutional-history | correlation mismatch | rejected |
| HIST-007 | sacs-constitutional-history | schema/version mismatch | rejected |
| HIST-008 | sacs-constitutional-history | missing evidence | rejected |
| HIST-009 | sacs-constitutional-history | substituted evidence | rejected |
| HIST-010 | sacs-constitutional-history | constructed realization identity | deterministic |
| HIST-011 | sacs-constitutional-history | repeated Stage One construction | equivalent |
| HIST-012 | sacs-constitutional-history | validation distinct from commitment | type-separated |
| HIST-013 | sacs-constitutional-history | realization commitment | atomic |
| HIST-014 | sacs-constitutional-history | repeated realization commitment | idempotent |
| HIST-015 | sacs-constitutional-history | conflicting realization identity | rejected |
| HIST-016 | sacs-constitutional-history | success/failure exclusivity | bounded |
| HIST-017 | sacs-constitutional-history | committed realization immutability | preserved |
| HIST-018 | sacs-constitutional-history | no Contract 009 reopening | boundary preserved |
| HIST-019 | sacs-constitutional-history | no Contract 010 reopening | boundary preserved |
| HIST-020 | sacs-constitutional-history | committed realization Stage Two admission | accepted |
| HIST-021 | sacs-constitutional-history | constructed realization Stage Two input | rejected |
| HIST-022 | sacs-constitutional-history | commitment event substitute | rejected |
| HIST-023 | sacs-constitutional-history | Stage One failure as success handoff | rejected |
| HIST-024 | sacs-constitutional-history | raw upstream substitute | rejected |
| HIST-025 | sacs-constitutional-history | required artifact resolution | complete |
| HIST-026 | sacs-constitutional-history | missing artifact | explicit failure |
| HIST-027 | sacs-constitutional-history | uncommitted membership | rejected |
| HIST-028 | sacs-constitutional-history | conflicting artifact | rejected |
| HIST-029 | sacs-constitutional-history | artifact hash mismatch | rejected |
| HIST-030 | sacs-constitutional-history | artifact version mismatch | rejected |
| HIST-031 | sacs-constitutional-history | artifact cycle mismatch | rejected |
| HIST-032 | sacs-constitutional-history | archive contamination | excluded |
| HIST-033 | sacs-constitutional-history | exact relationship resolution | deterministic |
| HIST-034 | sacs-constitutional-history | missing relationship | explicit failure |
| HIST-035 | sacs-constitutional-history | contradictory relationship | rejected |
| HIST-036 | sacs-constitutional-history | reversed relationship | rejected |
| HIST-037 | sacs-constitutional-history | authority-expanding relationship | rejected |
| HIST-038 | sacs-constitutional-history | lineage identity | deterministic |
| HIST-039 | sacs-constitutional-history | lineage bytes/hash | stable |
| HIST-040 | sacs-constitutional-history | missing membership | incomplete |
| HIST-041 | sacs-constitutional-history | missing relationship | incomplete |
| HIST-042 | sacs-constitutional-history | complete lineage | complete |
| HIST-043 | sacs-constitutional-history | identity inconsistency | integrity failure |
| HIST-044 | sacs-constitutional-history | commitment inconsistency | integrity failure |
| HIST-045 | sacs-constitutional-history | temporal inconsistency | integrity failure |
| HIST-046 | sacs-constitutional-history | authority-path inconsistency | integrity failure |
| HIST-047 | sacs-constitutional-history | integrity repair | prohibited |
| HIST-048 | sacs-constitutional-history | incomplete lineage cycle construction | rejected |
| HIST-049 | sacs-constitutional-history | invalid integrity cycle commitment | rejected |
| HIST-050 | sacs-constitutional-history | cycle record identity | deterministic |
| HIST-051 | sacs-constitutional-history | constructed cycle non-authority | preserved |
| HIST-052 | sacs-constitutional-history | cycle commitment | atomic |
| HIST-053 | sacs-constitutional-history | commitment record distinction | preserved |
| HIST-054 | sacs-constitutional-history | repeated cycle commitment | idempotent |
| HIST-055 | sacs-constitutional-history | conflicting cycle identity | rejected |
| HIST-056 | sacs-constitutional-history | cycle success/failure exclusivity | bounded |
| HIST-057 | sacs-constitutional-history | exact-one terminal outcome | enforced |
| HIST-058 | sacs-constitutional-history | prior cycle immutability | preserved |
| HIST-059 | sacs-constitutional-history | partial cycle canonicalization | rejected |
| HIST-060 | sacs-constitutional-history | no execution re-adjudication | boundary preserved |
| HIST-061 | sacs-constitutional-history | no continuity re-adjudication | boundary preserved |
| HIST-062 | sacs-constitutional-history | no realization recertification | boundary preserved |
| HIST-063 | sacs-constitutional-history | replay success equivalence | equivalent |
| HIST-064 | sacs-constitutional-history | replay failure support | explicit |
| HIST-065 | sacs-constitutional-history | replay mismatch | evidence only |
| HIST-066 | sacs-constitutional-history | replay recommitment | prohibited |
| HIST-067 | sacs-constitutional-history | replay execution | prohibited |
| HIST-068 | sacs-constitutional-history | replay authority recreation | prohibited |
| HIST-069 | sacs-constitutional-history | audit determinism | stable |
| HIST-070 | sacs-constitutional-history | audit history mutation | prohibited |
| HIST-071 | sacs-constitutional-history | audit governance enactment | prohibited |
| HIST-072 | sacs-constitutional-history | Contract 012 handoff | committed cycle only |
| HIST-073 | sacs-constitutional-history | constructed cycle downstream use | rejected |
| HIST-074 | sacs-constitutional-history | commitment record substitution | rejected |
| HIST-075 | sacs-constitutional-history | provenance failure as success | rejected |
| HIST-076 | sacs-constitutional-history | external verification authority | absent |
| HIST-077 | sacs-constitutional-history | baseline/manifest | unchanged |
| HIST-078 | sacs-constitutional-history | active source/archive boundary | preserved |

| VERX-001 | sacs-external-verification-interface | valid committed-cycle admission | accepted |
| VERX-002 | sacs-external-verification-interface | constructed cycle rejection | rejected |
| VERX-003 | sacs-external-verification-interface | provenance failure rejection | rejected |
| VERX-004 | sacs-external-verification-interface | stable exchange identity | deterministic |
| VERX-005 | sacs-external-verification-interface | exchange identity conflict | rejected |
| VERX-006 | sacs-external-verification-interface | deterministic claim ordering | stable |
| VERX-007 | sacs-external-verification-interface | duplicate claim | rejected |
| VERX-008 | sacs-external-verification-interface | unsupported claim | explicit failure |
| VERX-009 | sacs-external-verification-interface | out-of-scope claim | rejected |
| VERX-010 | sacs-external-verification-interface | applicability result | deterministic |
| VERX-011 | sacs-external-verification-interface | profile mismatch | rejected |
| VERX-012 | sacs-external-verification-interface | requirement mismatch | rejected |
| VERX-013 | sacs-external-verification-interface | method mismatch | rejected |
| VERX-014 | sacs-external-verification-interface | verifier mismatch | rejected |
| VERX-015 | sacs-external-verification-interface | version mismatch | rejected |
| VERX-016 | sacs-external-verification-interface | unresolved applicability | explicit |
| VERX-017 | sacs-external-verification-interface | deterministic evidence resolution | stable |
| VERX-018 | sacs-external-verification-interface | evidence subject correlation | exact |
| VERX-019 | sacs-external-verification-interface | missing evidence | failure |
| VERX-020 | sacs-external-verification-interface | substituted evidence | rejected |
| VERX-021 | sacs-external-verification-interface | invalid evidence | failure |
| VERX-022 | sacs-external-verification-interface | evidence package identity | deterministic |
| VERX-023 | sacs-external-verification-interface | evidence package validation | separate |
| VERX-024 | sacs-external-verification-interface | no package/no authority | preserved |
| VERX-025 | sacs-external-verification-interface | constructed submission | deterministic |
| VERX-026 | sacs-external-verification-interface | submission commitment | atomic |
| VERX-027 | sacs-external-verification-interface | repeated submission commitment | idempotent |
| VERX-028 | sacs-external-verification-interface | conflicting submission identity | rejected |
| VERX-029 | sacs-external-verification-interface | custody before submission | rejected |
| VERX-030 | sacs-external-verification-interface | result before custody | rejected |
| VERX-031 | sacs-external-verification-interface | custody ownership separation | preserved |
| VERX-032 | sacs-external-verification-interface | SACS-to-verifier custody | explicit |
| VERX-033 | sacs-external-verification-interface | verifier-to-SACS custody | distinct |
| VERX-034 | sacs-external-verification-interface | custody correlation failure | rejected |
| VERX-035 | sacs-external-verification-interface | custody ordering | deterministic |
| VERX-036 | sacs-external-verification-interface | custody no mutation | preserved |
| VERX-037 | sacs-external-verification-interface | custody no authority transfer | preserved |
| VERX-038 | sacs-external-verification-interface | exact external payload | preserved |
| VERX-039 | sacs-external-verification-interface | exact external judgment | preserved |
| VERX-040 | sacs-external-verification-interface | issuer/exchange correlation | exact |
| VERX-041 | sacs-external-verification-interface | wrong exchange result | rejected |
| VERX-042 | sacs-external-verification-interface | altered payload/hash | rejected |
| VERX-043 | sacs-external-verification-interface | unfavorable judgment | valid result |
| VERX-044 | sacs-external-verification-interface | qualified result | preserved |
| VERX-045 | sacs-external-verification-interface | no invented external result | prohibited |
| VERX-046 | sacs-external-verification-interface | result admission | explicit |
| VERX-047 | sacs-external-verification-interface | result integrity | validated |
| VERX-048 | sacs-external-verification-interface | constructed result noncanonical | preserved |
| VERX-049 | sacs-external-verification-interface | result certification | faithful preservation only |
| VERX-050 | sacs-external-verification-interface | committed result identity | deterministic |
| VERX-051 | sacs-external-verification-interface | repeated result commitment | idempotent |
| VERX-052 | sacs-external-verification-interface | conflicting result commitment | rejected |
| VERX-053 | sacs-external-verification-interface | external/constructed/committed distinction | type-separated |
| VERX-054 | sacs-external-verification-interface | successful exchange construction | accepted |
| VERX-055 | sacs-external-verification-interface | explicit exchange failure | explicit |
| VERX-056 | sacs-external-verification-interface | exact-one terminal invariant | enforced |
| VERX-057 | sacs-external-verification-interface | record/failure coexistence | rejected |
| VERX-058 | sacs-external-verification-interface | unfavorable result exchange success | permitted |
| VERX-059 | sacs-external-verification-interface | missing result exchange failure | explicit |
| VERX-060 | sacs-external-verification-interface | terminal retry | idempotent |
| VERX-061 | sacs-external-verification-interface | terminal identity conflict | rejected |
| VERX-062 | sacs-external-verification-interface | replay committed material | bounded |
| VERX-063 | sacs-external-verification-interface | replay no adapter | preserved |
| VERX-064 | sacs-external-verification-interface | replay no new external result | prohibited |
| VERX-065 | sacs-external-verification-interface | replay no undisclosed reasoning | preserved |
| VERX-066 | sacs-external-verification-interface | replay equivalence | accepted |
| VERX-067 | sacs-external-verification-interface | replay non-equivalence | evidence only |
| VERX-068 | sacs-external-verification-interface | audit immutable/non-constitutive | preserved |
| VERX-069 | sacs-external-verification-interface | no networking/provider/shell | absent |
| VERX-070 | sacs-external-verification-interface | no successor authority | absent |

These are implementation vectors, not constitutional claims or runtime contract fixtures.
