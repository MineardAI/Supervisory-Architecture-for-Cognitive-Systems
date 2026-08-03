# SACS Implementation Baseline Manifest v1.0.0

Version: 1.0.0  
Status: ADOPTED  
Baseline: SACS Implementation Baseline v1.0.0  
Constitutional dependency: SACS Constitutional Baseline v1.0.0  
Manifest role: deterministic inventory of the subordinate accepted implementation realization  
Workspace commit identity: unavailable; the SACS directory is not a Git repository.  

This manifest is subordinate to and separate from `SACS_BASELINE_MANIFEST_v1.0.0.md`. It excludes constitutional source files, archives, target/build outputs, editor files, caches, unrelated Project Suite material, and its own self-hash. Paths are relative to the SACS root and sorted deterministically. Hashes are SHA-256.

| Path | Classification | Implementation Contract | Status | SHA-256 |
|---|---|---|---|---|
| `implementation/.gitignore` | implementation | IMP-000..IMP-012 | ADOPTED | 306fd52e74fca6746e12acc750f232de15e71da917756a1270d74c91f5eb7368 |
| `implementation/Cargo.lock` | implementation | IMP-000..IMP-012 | ADOPTED | 748a4b0e457acd42593758acd887ad88788e32c22b37641ba1144eaabfc607ef |
| `implementation/Cargo.toml` | implementation | IMP-000..IMP-012 | ADOPTED | 785bf18aec255f3e363eb14b02ab5f93f3e2609df0ba3417d42dcdab6dba0d1b |
| `implementation/crates/canonical-artifacts/Cargo.toml` | implementation | IMP-000 | CONDITIONALLY COMPLETE | 60b499c4b5a52e1bbd296afacddea3309a6270e2a1f6a65498236b24920ef72c |
| `implementation/crates/canonical-artifacts/src/lib.rs` | implementation | IMP-000 | CONDITIONALLY COMPLETE | 883a40462f4e80654f601e82a299a71a0581d8afcf27a3c82f380a647d3bc0c5 |
| `implementation/crates/canonical-hashing/Cargo.toml` | implementation | IMP-000 | CONDITIONALLY COMPLETE | 0f66c2b3040bb58253ae0c29f20e9b9f77e853a1870c55943929c87a080ee531 |
| `implementation/crates/canonical-hashing/src/lib.rs` | implementation | IMP-000 | CONDITIONALLY COMPLETE | dbce04743b26ab81138547a59d28d7a87cc763114f7a466e459ac0aff909ff4c |
| `implementation/crates/canonical-identity/Cargo.toml` | implementation | IMP-000 | CONDITIONALLY COMPLETE | 6632b3262b82a22cfa340a42855e0d2e81ff281a0a6cd129bdace69c79342e5e |
| `implementation/crates/canonical-identity/src/lib.rs` | implementation | IMP-000 | CONDITIONALLY COMPLETE | f69b8828e64ed33b589e8c75c1c05ca526ae5de35bec78a8fe44f068d66a455e |
| `implementation/crates/canonical-serialization/Cargo.toml` | implementation | IMP-000 | CONDITIONALLY COMPLETE | c65bb967560d7f33727da6dc5aa6a8170d54f6de1510cb94fea4e7349cc6024f |
| `implementation/crates/canonical-serialization/src/lib.rs` | implementation | IMP-000 | CONDITIONALLY COMPLETE | 9c81b431ee882103c057f33d5939a5223d54fa331fb30993463811d6ba55bc64 |
| `implementation/crates/canonical-versioning/Cargo.toml` | implementation | IMP-000 | CONDITIONALLY COMPLETE | a5176c6087e50e0a514e443385b19bfc92b78c17e32d2faa98aa3c104aecad82 |
| `implementation/crates/canonical-versioning/src/lib.rs` | implementation | IMP-000 | CONDITIONALLY COMPLETE | cd0df0a982150cca26ae63fce1c718a3a9c1ff1fb88546fd5a1152cb166ff7a0 |
| `implementation/crates/commitment-engine/Cargo.toml` | implementation | IMP-000 | CONDITIONALLY COMPLETE | 674895792f9278c0a9496b2b0fa4ba69ba85ef8bf5ed3976b73734c2fdc2ed97 |
| `implementation/crates/commitment-engine/src/lib.rs` | implementation | IMP-000 | CONDITIONALLY COMPLETE | 28187e4e8269b5657db2fa45795aed8085bcbc4d8d336d874a9e05e7dfaf9584 |
| `implementation/crates/constitutional-conformance/Cargo.toml` | implementation | IMP-000 | CONDITIONALLY COMPLETE | 6a06dfb5bb61fe5fcf986b4cec9546dae2da1e97eb09dbcb8e4dcc97668d2741 |
| `implementation/crates/constitutional-conformance/src/lib.rs` | implementation | IMP-000 | CONDITIONALLY COMPLETE | ff7b39f6f0f5fbbed130e329b6e9090555788e1e98ffa3a8636f155560d088dd |
| `implementation/crates/constitutional-history/Cargo.toml` | implementation | IMP-011 | CONDITIONALLY COMPLETE | 47597a4cd9f262e39c0191d144039652b5b96dfdd6328e4e3093fef1a9539fd1 |
| `implementation/crates/constitutional-history/src/lib.rs` | implementation | IMP-011 | CONDITIONALLY COMPLETE | fe73e187e90e84def33d78b9439a317c25b0fe76666858f7a42658b15358c79c |
| `implementation/crates/controller/Cargo.toml` | implementation | IMP-007 | CONDITIONALLY COMPLETE | 5369e956495b7ee35782a6ac6aa63b7607706663672e5d79867fd7137958eb3c |
| `implementation/crates/controller/src/lib.rs` | implementation | IMP-007 | CONDITIONALLY COMPLETE | 2f6e89a4d92c1f1f44d9772bc03eb8393e0c615f7fed9a537b8922d18230e4d9 |
| `implementation/crates/deterministic-ordering/Cargo.toml` | implementation | IMP-000 | CONDITIONALLY COMPLETE | 7593d774433971def1f9eb32734e8f73a1231383c8db1030af51a94d7128483f |
| `implementation/crates/deterministic-ordering/src/lib.rs` | implementation | IMP-000 | CONDITIONALLY COMPLETE | 60b5615d97d60756f42339daada91538ac580ec208f7d365f03012f37e2fc4d6 |
| `implementation/crates/execution-interface/Cargo.toml` | implementation | IMP-009 | CONDITIONALLY COMPLETE | 56fae7d8df5748a7f2bc8689e2b4846720888dead5c32c0e92a6458919e6b70f |
| `implementation/crates/execution-interface/src/lib.rs` | implementation | IMP-009 | CONDITIONALLY COMPLETE | 490f1dd6c665cefc400615aa434b7a9eab61573c52d538e4e4b3cd417069e7bd |
| `implementation/crates/external-verification-interface/Cargo.toml` | implementation | IMP-012 | CONDITIONALLY COMPLETE | 1cbab9fe0c4131b7369155d7a771612e645e0ccfaa70a154609003186f7f817a |
| `implementation/crates/external-verification-interface/src/lib.rs` | implementation | IMP-012 | CONDITIONALLY COMPLETE | 466c7b9a0b15f08fb0f94a7e35b88f596aaca3745adc90e704772264fe05b0e7 |
| `implementation/crates/failure-model/Cargo.toml` | implementation | IMP-000 | CONDITIONALLY COMPLETE | 8555dd57f4284bf476e7cd30de53be9f2dd1b7053329b5f6fac755a115c74023 |
| `implementation/crates/failure-model/src/lib.rs` | implementation | IMP-000 | CONDITIONALLY COMPLETE | 862b9e106c63733c52fca1bbcb89de0f34db12cbea332d8dc844ed24e1d13722 |
| `implementation/crates/governance/Cargo.toml` | implementation | IMP-004 | CONDITIONALLY COMPLETE | a4a38047bc5cb74a86eed3651211644da3512aee5dcc62dd6b5a85a7d6665601 |
| `implementation/crates/governance/src/lib.rs` | implementation | IMP-004 | CONDITIONALLY COMPLETE | fbe37dfe39450191bee68acdfaa1b2f3f0524eaad21e84a8196a3ad66e5bb188 |
| `implementation/crates/inference-control/Cargo.toml` | implementation | IMP-008 | CONDITIONALLY COMPLETE | c135dbd272634545a9443f2a92d4e62d5baedf870027ff73926ff678acde0b1f |
| `implementation/crates/inference-control/src/lib.rs` | implementation | IMP-008 | CONDITIONALLY COMPLETE | d6f7c222b4dc05236716f61d5c8d8ce8db94b5512568c9254cb93951c7075594 |
| `implementation/crates/lineage-core/Cargo.toml` | implementation | IMP-000 | CONDITIONALLY COMPLETE | 772998a5f65ab8e968b863e29c19ab5127cc2c32badba3d35b06ee664a1a71e8 |
| `implementation/crates/lineage-core/src/lib.rs` | implementation | IMP-000 | CONDITIONALLY COMPLETE | f2d3c5e1f7ec51f3fa962218503e6da0174a64051327b497ad89bd6270ef2c5d |
| `implementation/crates/observation/Cargo.toml` | implementation | IMP-001 | CONDITIONALLY COMPLETE | 08e1be10a8da3a27383ae559af0565aaf9a256e6547a27f6be2ec463fe52cb7a |
| `implementation/crates/observation/src/lib.rs` | implementation | IMP-001 | CONDITIONALLY COMPLETE | 33dc15da7e41b95319af0d820ec28eca441b6c402c7f496a27a83de0a3900757 |
| `implementation/crates/operational-profiles/Cargo.toml` | implementation | IMP-006 | CONDITIONALLY COMPLETE | 29b8545c542f60804af1b2eefb15851f31c2e5f7367d0b2512b8dc40a18526bd |
| `implementation/crates/operational-profiles/src/lib.rs` | implementation | IMP-006 | CONDITIONALLY COMPLETE | 824a1fb24ce5df489841438ca9ec56458eb56dfe7c5ca34a438c005bbb874ca7 |
| `implementation/crates/operational-state-vector/Cargo.toml` | implementation | IMP-003 | CONDITIONALLY COMPLETE | 93707e6193f04db56dd7ebf865e287413668d5c37c45294b96832b83bba34c9c |
| `implementation/crates/operational-state-vector/src/lib.rs` | implementation | IMP-003 | CONDITIONALLY COMPLETE | 7528dff6737c5ba745d8cff629cd08e845d0bcd9b04a700c9594cbbbc2f08c17 |
| `implementation/crates/state-estimation/Cargo.toml` | implementation | IMP-002 | CONDITIONALLY COMPLETE | 161f77d698a4742b4cf728a3a6a68f3b2e4c723c21af55c18997090ba940c185 |
| `implementation/crates/state-estimation/src/lib.rs` | implementation | IMP-002 | CONDITIONALLY COMPLETE | 163971e43148be5296bee9b29bbc77dda8522a6fc9fe855f433587f67e835c75 |
| `implementation/crates/supervisory-continuity/Cargo.toml` | implementation | IMP-010 | CONDITIONALLY COMPLETE | 00384b01b74284876da8d0176e138669fafb81b27eebeb95b4deb52b62efd698 |
| `implementation/crates/supervisory-continuity/src/lib.rs` | implementation | IMP-010 | CONDITIONALLY COMPLETE | 3665bc1eebb97a496f9b0fe40c784b3262b06eaf9b592afa58da97b018669102 |
| `implementation/crates/supervisory-objectives/Cargo.toml` | implementation | IMP-005 | CONDITIONALLY COMPLETE | 6d694e531aab33a96109938e427a729311279e55678e2b7149ed4ca2c9569a35 |
| `implementation/crates/supervisory-objectives/src/lib.rs` | implementation | IMP-005 | CONDITIONALLY COMPLETE | 9fe883a536d65a6d00b296809251d619f1d0313e4966c241bbf27aaffc02a812 |
| `implementation/docs/CHANGE_IMPACT_LOG.md` | implementation | IMP-000..IMP-012 | ADOPTED | f2ffdfabd09ff37461e12e94792391025603abac4021f43923db9cb8b571c240 |
| `implementation/docs/IMPLEMENTATION_BASELINE.md` | implementation | IMP-000..IMP-012 | ADOPTED | b90e10544d1a2a8fd44b7363799349c8de71a2f0ec3bfcf086bf4382f680b90d |
| `implementation/docs/IMPLEMENTATION_PROFILE.md` | implementation | IMP-000..IMP-012 | ADOPTED | 570f05f668c9b2d8f2022f3b26bdd0decefb3648d58ddb709e35166ff7ce0d77 |
| `implementation/docs/REQUIREMENT_IMPLEMENTATION_MATRIX.md` | implementation | IMP-000..IMP-012 | ADOPTED | 234f35466ceb5b21d864e63ae64291b43397b1a96a33bbbd0af3662d645c5d8f |
| `implementation/docs/SACS_IMP_001_OBSERVATION_REALIZATION_PROFILE.md` | implementation | IMP-001 | CONDITIONALLY COMPLETE | fc202faf190c850f5438e29763bb375a478898786d4012a8feffe2ef500fa869 |
| `implementation/docs/SACS_IMP_001_SOURCE_TO_IMPLEMENTATION_MATRIX.md` | implementation | IMP-001 | CONDITIONALLY COMPLETE | b197b22fb14f5efa35498d0eb478391a823d9967cc9ff690f4daaa7501d1c252 |
| `implementation/docs/SACS_IMP_002_NON_NORMATIVE_ESTIMATOR_DECISIONS.md` | implementation | IMP-002 | CONDITIONALLY COMPLETE | 00f885f56265dec0dab0b44c4d578a4c8511e3aed0ef48e54bd3d6ca66e218a7 |
| `implementation/docs/SACS_IMP_002_SOURCE_TO_IMPLEMENTATION_MATRIX.md` | implementation | IMP-002 | CONDITIONALLY COMPLETE | 8fbcfad70b00c67d4a67c07e3010526aa90b77559717bd436f23f306fb545959 |
| `implementation/docs/SACS_IMP_002_STATE_ESTIMATION_REALIZATION_PROFILE.md` | implementation | IMP-002 | CONDITIONALLY COMPLETE | c6afd5fc8a9e252be339e5e143b4c9796fba2d7da0d97ee655d1208cdb5a05e9 |
| `implementation/docs/SACS_IMP_003_NON_NORMATIVE_OSV_DECISIONS.md` | implementation | IMP-003 | CONDITIONALLY COMPLETE | ea6ff8bd2d392f3f0b5d70e26b7be500ef220ed8e38bbb0603004133c7f3d15b |
| `implementation/docs/SACS_IMP_003_OSV_REALIZATION_PROFILE.md` | implementation | IMP-003 | CONDITIONALLY COMPLETE | a9fc5cb5e934a87f966540865566283e0a3d342f49f014dcc036a92ed94ca3e4 |
| `implementation/docs/SACS_IMP_003_SOURCE_TO_IMPLEMENTATION_MATRIX.md` | implementation | IMP-003 | CONDITIONALLY COMPLETE | 940a387ca8316ff159a3f63cedcfbe48cef25c3b17a3dcf38eb45be1af7759cc |
| `implementation/docs/SACS_IMP_004_GOVERNANCE_REALIZATION_PROFILE.md` | implementation | IMP-004 | CONDITIONALLY COMPLETE | 87bed624eea3015b01d790bb9962fcdab07896391a27f3cbb92514c08e305564 |
| `implementation/docs/SACS_IMP_004_NON_NORMATIVE_GOVERNANCE_DECISIONS.md` | implementation | IMP-004 | CONDITIONALLY COMPLETE | 2dd919806c53ce093642600f5796cfb74880638e89cdafc5c01610f0543c4203 |
| `implementation/docs/SACS_IMP_004_SOURCE_TO_IMPLEMENTATION_MATRIX.md` | implementation | IMP-004 | CONDITIONALLY COMPLETE | 088236ecb49bd068a32d555cebcc1c6542164ad38faeff0e016a1e044275b9ea |
| `implementation/docs/SACS_IMP_005_NON_NORMATIVE_OBJECTIVE_DECISIONS.md` | implementation | IMP-005 | CONDITIONALLY COMPLETE | f7fca7eeb23a81d791c856c47edc1eb3b799c50d56094b10cd18f69a3c613133 |
| `implementation/docs/SACS_IMP_005_SOURCE_TO_IMPLEMENTATION_MATRIX.md` | implementation | IMP-005 | CONDITIONALLY COMPLETE | 0c5b1cea91492964ecae70e4ce46e18fb92dc62bf8746102257dcd1676e9dd0b |
| `implementation/docs/SACS_IMP_005_SUPERVISORY_OBJECTIVES_REALIZATION_PROFILE.md` | implementation | IMP-005 | CONDITIONALLY COMPLETE | ae08646efe53eda16cb9f52f95182d2d819e60f47ea5ca56f6e325c31c2db815 |
| `implementation/docs/SACS_IMP_006_NON_NORMATIVE_PROFILE_DECISIONS.md` | implementation | IMP-006 | CONDITIONALLY COMPLETE | dbf8f7622ab36f462cc9b139df917cbb17ad4357bdc25cee9ad2badf033a523b |
| `implementation/docs/SACS_IMP_006_OPERATIONAL_PROFILES_REALIZATION_PROFILE.md` | implementation | IMP-006 | CONDITIONALLY COMPLETE | 2ebe069a847548e4a2864bfb4276c66e150f48702372e9682ca3c991560bb274 |
| `implementation/docs/SACS_IMP_006_SOURCE_TO_IMPLEMENTATION_MATRIX.md` | implementation | IMP-006 | CONDITIONALLY COMPLETE | 5d9d797c4fcb563f88cceb78d781d930b15c8fd84bfbf55ab9e39e468824213f |
| `implementation/docs/SACS_IMP_007_CONTROLLER_REALIZATION_PROFILE.md` | implementation | IMP-007 | CONDITIONALLY COMPLETE | 2364fcb21f07b3254c8e029a532c71011e416bda8a8a586e34bc7f8f7befa805 |
| `implementation/docs/SACS_IMP_007_NON_NORMATIVE_CONTROLLER_DECISIONS.md` | implementation | IMP-007 | CONDITIONALLY COMPLETE | 8e126bbb1d2da6013d791ef071c2277cd011c2081dd38295878bf0c476731608 |
| `implementation/docs/SACS_IMP_007_SOURCE_TO_IMPLEMENTATION_MATRIX.md` | implementation | IMP-007 | CONDITIONALLY COMPLETE | 0a671d91bd39dfb83e648ab96b238c7df269f86b1a7438a8531698d39eda043a |
| `implementation/docs/SACS_IMP_008_INFERENCE_CONTROL_REALIZATION_PROFILE.md` | implementation | IMP-008 | CONDITIONALLY COMPLETE | a0db549586343db9407ef298e741c990cd53f1467af29bf38741b86208a33383 |
| `implementation/docs/SACS_IMP_008_NON_NORMATIVE_TRANSLATION_DECISIONS.md` | implementation | IMP-008 | CONDITIONALLY COMPLETE | 5859dd6695c5c669f06847be952a905a122370aad46f30555da1f3033f2c1f21 |
| `implementation/docs/SACS_IMP_008_SOURCE_TO_IMPLEMENTATION_MATRIX.md` | implementation | IMP-008 | CONDITIONALLY COMPLETE | 4b6c9963b72ba98180e9d85386ecbe048a8fa480e5dea5db8c2a3ef36efdec87 |
| `implementation/docs/SACS_IMP_009_EXECUTION_INTERFACE_REALIZATION_PROFILE.md` | implementation | IMP-009 | CONDITIONALLY COMPLETE | 5077d7407080379c3f46762226286f7f64304b750525e852af3f0f60ec37f9bb |
| `implementation/docs/SACS_IMP_009_NON_NORMATIVE_EXECUTION_DECISIONS.md` | implementation | IMP-009 | CONDITIONALLY COMPLETE | 2ca3e0052cc166a4706bc2ba7eeb7511afad3f657aa76600d85bba50fc33f06d |
| `implementation/docs/SACS_IMP_009_SOURCE_TO_IMPLEMENTATION_MATRIX.md` | implementation | IMP-009 | CONDITIONALLY COMPLETE | 4bd46827fb7b7d03aba52e8e8e513114e3e0f0f9bdd5c2a8bfd1555e811cf4fe |
| `implementation/docs/SACS_IMP_010_NON_NORMATIVE_CONTINUITY_DECISIONS.md` | implementation | IMP-010 | CONDITIONALLY COMPLETE | b5702f321fb27a5cd68ce91b2ca995d35cb6ede7d15bf4a35debd03ccd6fa59c |
| `implementation/docs/SACS_IMP_010_SOURCE_TO_IMPLEMENTATION_MATRIX.md` | implementation | IMP-010 | CONDITIONALLY COMPLETE | 0062be8aec256d21e8c0562c33790f479d3e5926817d5d0bd2a44a7acf887cc1 |
| `implementation/docs/SACS_IMP_010_SUPERVISORY_CONTINUITY_REALIZATION_PROFILE.md` | implementation | IMP-010 | CONDITIONALLY COMPLETE | 387af5521aa3d9d01a9247c998a41219037e456f856cddea1e8d015ba6617dc3 |
| `implementation/docs/SACS_IMP_011_CONSTITUTIONAL_HISTORY_REALIZATION_PROFILE.md` | implementation | IMP-011 | CONDITIONALLY COMPLETE | 48b39c54a9925b7a149c1136f065a4cd4a33455feb5ba418acf17543beb02002 |
| `implementation/docs/SACS_IMP_011_NON_NORMATIVE_HISTORY_DECISIONS.md` | implementation | IMP-011 | CONDITIONALLY COMPLETE | 8d08ac72141d922dab0d2c461f9e37d3fcf34b1d0119bbd52926bb6f7641b7c2 |
| `implementation/docs/SACS_IMP_011_SOURCE_TO_IMPLEMENTATION_MATRIX.md` | implementation | IMP-011 | CONDITIONALLY COMPLETE | 4b2891a70ce6fda092cbc0f77c9fa11f76cbff9e801dae54079d42e647ec3806 |
| `implementation/docs/SACS_IMP_012_EXTERNAL_VERIFICATION_REALIZATION_PROFILE.md` | implementation | IMP-012 | CONDITIONALLY COMPLETE | 3065fb30454ce3ad2932e493e3681a14de08ccf60f10e81f75f58e78f3d0d923 |
| `implementation/docs/SACS_IMP_012_NON_NORMATIVE_VERIFICATION_DECISIONS.md` | implementation | IMP-012 | CONDITIONALLY COMPLETE | fca4bfebc5000a590ca1ccb666f98eaa7964754adffaa0cf15d41272d98c5ca8 |
| `implementation/docs/SACS_IMP_012_SOURCE_TO_IMPLEMENTATION_MATRIX.md` | implementation | IMP-012 | CONDITIONALLY COMPLETE | d40775d64507ecfdaece7e44e0af2c21020861c219da6716c411408c54c09932 |
| `implementation/docs/STATUS_CURRENT.md` | implementation | IMP-000..IMP-012 | ADOPTED | 56cfe1c13bef3be349c473ec2a6da0fdbb751d96dcff88b9ea234299debb561d |
| `implementation/docs/TYPE_IMPLEMENTATION_MATRIX.md` | implementation | IMP-000..IMP-012 | ADOPTED | 5ef45b73e4c4cb09ca9590e523e04675d06d3ee92e9da1a7207ac35fcbea28eb |
| `implementation/evidence/CONFORMANCE_VECTOR_INDEX.md` | implementation | IMP-000..IMP-012 | ADOPTED | e03ee059f02c2c92cc14816a0b82dc58c4c4be71e753b885c7837146c87f3580 |
| `implementation/evidence/EV-CONT-0001.md` | implementation | IMP-000..IMP-012 | ADOPTED | 8f4b24b788d796dc45c9122d64a6aafd1766ac1ebaf5225348e09ec8f09192e7 |
| `implementation/evidence/EV-CTRL-0001.md` | implementation | IMP-000..IMP-012 | ADOPTED | 3681c1821c5cc2094e72dada351a040a0fc1ad8a02ccdb9db8a57b58955131ad |
| `implementation/evidence/EV-EST-0001.md` | implementation | IMP-000..IMP-012 | ADOPTED | fcabec00dd9eea5094ab2bdc4e3717f11d6e0c5e4fdabb64e213257795d24fad |
| `implementation/evidence/EV-EXEC-0001.md` | implementation | IMP-000..IMP-012 | ADOPTED | 5f84ec55c09f0101a83aa840948c68afcd9745a9dcf4362d63b114bf632682a2 |
| `implementation/evidence/EV-GOV-0001.md` | implementation | IMP-000..IMP-012 | ADOPTED | 560c71513706cc6b14850646045932661dbfc816e047377671f4a2acc0761e6a |
| `implementation/evidence/EV-HIST-0001.md` | implementation | IMP-000..IMP-012 | ADOPTED | ec785260a8e6b68d9128b44f9bd85e424c6563113445f88141fe088b5db8f6b5 |
| `implementation/evidence/EVIDENCE_INDEX.md` | implementation | IMP-000..IMP-012 | ADOPTED | d23db830c257e6ba84751483ea15fa6f5de703351c59e61fecc7cf80159ea38d |
| `implementation/evidence/EV-OBJ-0001.md` | implementation | IMP-000..IMP-012 | ADOPTED | 07f6367d6b6f69b059eab2b12248e9e7be9c890fce58469e4d326c1cd84a8c5f |
| `implementation/evidence/EV-OBS-0001.md` | implementation | IMP-000..IMP-012 | ADOPTED | 270a6b747ab6542bb25a5dcf7a61bc758e98988b6b4e2b7fdf1e4fa8ca9402e7 |
| `implementation/evidence/EV-OSV-0001.md` | implementation | IMP-000..IMP-012 | ADOPTED | 4874c2fd5496e2e51b3f93525260ffab33b423bc5c63128f4383440c2925b523 |
| `implementation/evidence/EV-PROFILE-0001.md` | implementation | IMP-000..IMP-012 | ADOPTED | 0c740f9352e4edb367b26a12333eadb9d724c03bdd928a67510d2c4a3a588034 |
| `implementation/evidence/EV-TRANS-0001.md` | implementation | IMP-000..IMP-012 | ADOPTED | a1eb41d04b7272d9b89f103e4a05cd3716e50d0d3608f3da58635e50ba917df6 |
| `implementation/evidence/EV-VERX-0001.md` | implementation | IMP-000..IMP-012 | ADOPTED | 0537b6e9c6000a533cff904083abe670aa2570aa693070af7a0b548b064157bb |
| `implementation/README.md` | implementation | IMP-000..IMP-012 | ADOPTED | 721e71ad0ae8b585567a81fca202791f98e00f0e9ccd22f3a02ad95944e4b4b3 |
| `IMPLEMENTATION_MANIFEST_RECONCILIATION_REPORT.md` | implementation | IMP-000..IMP-012 | ADOPTED | dbfc388c4248a59a2819ff7c03212d9781475da9d33944b0eee053368a5b6ce5 |
| `SACS_IMPLEMENTATION_BASELINE_ADOPTION_REPORT.md` | implementation | IMP-000..IMP-012 | ADOPTED | d875a8fa7aaba47a06a2f6715d25162ab20ff2321fbdc7e4e8553e0ba6af545b |
| `SACS_IMPLEMENTATION_BASELINE_v1.0.0.md` | implementation | IMP-000..IMP-012 | ADOPTED | d3038a4610bba93da98dd871dd28226c91510f9ebe5c5508692310c6795e2a7d |

Entry count: 106

The implementation baseline remains subordinate to the constitutional baseline. A manifest match proves inventory integrity only; it does not confer constitutional authority, external verification, production certification, accreditation, or deployment approval.
