# Implementation progress

This is the single status ledger. Done requires original-gate evidence; fake or
analog fixtures cannot satisfy a real-storage gate. Every implementation slice
adds evidence here in its enclosing Jujutsu change.

## Tasks and gates

| ID          | Status      | Depends   | Requirements                                                | Gate                                                                | Next                                                        | Evidence          |
| ----------- | ----------- | --------- | ----------------------------------------------------------- | ------------------------------------------------------------------- | ----------------------------------------------------------- | ----------------- |
| Bootstrap   | Done        | -         | JST-GOV-001,JST-GOV-002,JST-GOV-003,JST-GOV-004,JST-GOV-005 | Spec, ledger, skeleton, gates and independent commit                | Maintain governance each slice                              | E-BOOT-001        |
| Bootstrap.1 | Done        | -         | JST-GOV-001,JST-GOV-004                                     | Reference checksum, requirements, progress checker negative tests   | Governance maintained each slice                            | E-BOOT-001        |
| M0          | Done        | Bootstrap | JST-COMPAT-001,JST-COMPAT-002                               | Compiled extension and exact RustFS contract qualified              | Maintain pinned compatibility probes                        | E-M0-001,E-M0-002 |
| M0.1        | Done        | Bootstrap | JST-COMPAT-001                                              | Pin references/licenses and complete trait/codec matrix             | Re-audit on upstream upgrades                               | E-M0-001          |
| M0.2        | Done        | M0.1      | JST-JJ-001,JST-JJ-002,JST-ID-002                            | Compile custom CLI/RPC context and query spikes; golden IDs         | Retain probes across upstream changes                       | E-M0-002          |
| M0.3        | Done        | M0.1      | JST-OBJ-002                                                 | Real image create/CAS/range/restart/fault qualification             | Requalify topology or image changes                         | E-M0-001          |
| M1          | Done        | M0        | JST-ID-001,JST-ID-003,JST-OBJ-001,JST-OBJ-003               | Complete lossless objects and streamed storage                      | Maintain codec and storage conformance                      | E-M1-001,E-M1-002 |
| M1.1        | Done        | M0        | JST-ID-003,JST-ID-004,JST-ID-005                            | All native variants and golden identities                           | Retain V1 fixtures across codec changes                     | E-M1-001          |
| M1.2        | Done        | M0        | JST-OBJ-001,JST-OBJ-003,JST-OBJ-004                         | Fault fake and real streamed immutable object transport             | Retain real RustFS and fault-store conformance              | E-M1-002          |
| M2          | In progress | M1        | JST-PUB-001,JST-PUB-006                                     | Publication fault matrix and cold replay pass                       | Implement manifest publication and recovery                 | E-M2-001          |
| M2.1        | Partial     | M1        | JST-PUB-001,JST-PUB-002                                     | Complete inventory and unique candidate CAS                         | Persist inventories and unique candidates then CAS manifest | E-M2-001          |
| M2.2        | Planned     | M2.1      | JST-PUB-003,JST-PUB-004,JST-PUB-005                         | Retry receipts and unknown outcome recovery                         | Inject late/unknown races                                   | -                 |
| M2.3        | Planned     | M2.2      | JST-PUB-006                                                 | Checkpoint equivalence and cache-free restart                       | Verify retained history                                     | -                 |
| M3          | Planned     | M2        | JST-JJ-001,JST-JJ-002,JST-CLI-001,JST-SEC-001               | Native CLI/server vertical slice with authentication                | Integrate native stores                                     | -                 |
| M3.1        | Planned     | M2        | JST-SEC-001,JST-SEC-002,JST-SEC-003,JST-SEC-004             | Revocable tokens and transport authorization                        | Implement auth boundaries                                   | -                 |
| M3.2        | Planned     | M3.1      | JST-JJ-001,JST-JJ-004,JST-JJ-005                            | Create edit describe log restart op-log evolog; multi-workspace     | Implement factories and RPC                                 | -                 |
| M3.3        | Planned     | M3.2      | JST-CLI-001,JST-CLI-002,JST-CLI-003,JST-CLI-004             | Custom command presentation and capability parity                   | Exercise CLI family matrix                                  | -                 |
| M4          | Planned     | M3        | JST-API-001,JST-API-005                                     | Structured reads and basic served UI match native semantics         | Implement read surface                                      | -                 |
| M4.1        | Planned     | M3        | JST-API-001,JST-API-002,JST-JJ-003                          | Pinned three-graph reads with zero GET publications                 | Implement bounded graph sessions                            | -                 |
| M4.2        | Planned     | M4.1      | JST-API-003,JST-API-004,JST-API-005                         | SSE, safe UI, generated contracts and cold interaction tests        | Build Svelte UI                                             | -                 |
| M5          | Planned     | M4        | JST-MUT-001,JST-MUT-002,JST-MUT-005                         | Authenticated typed commands and shared RPC policy                  | Implement commands                                          | -                 |
| M5.1        | Planned     | M4        | JST-MUT-001,JST-MUT-002,JST-MUT-005                         | Basic changes/refs/reconcile; preview receipt and semantic policy   | Implement command transaction facade                        | -                 |
| M5.2        | Planned     | M5.1      | JST-MUT-003                                                 | Full abandon descendant bookmark workspace/evolution parity         | Implement abandon                                           | -                 |
| M5.3        | Planned     | M5.1      | JST-MUT-004                                                 | Atomic stale-safe resolution with native conflict variants          | Implement resolve                                           | -                 |
| M6          | Planned     | M5        | JST-ADAPT-001,JST-ADAPT-002,JST-ADAPT-003,JST-ADAPT-004     | Replay-equivalent searchable projection with freshness              | Implement adapters and search                               | -                 |
| M6.1        | Planned     | M5        | JST-ADAPT-001,JST-ADAPT-002                                 | Ordered committed reader and durable replay-safe checkpoints        | Implement runner                                            | -                 |
| M6.2        | Planned     | M6.1      | JST-ADAPT-003,JST-ADAPT-004                                 | Tantivy crash/rebuild/generation and revocation tests               | Implement projection and UI                                 | -                 |
| M8a         | Planned     | M6        | JST-COMPAT-003,JST-SEC-005                                  | Pinned release, restore, security, performance and cold UI evidence | Harden first release                                        | -                 |
| M8a.1       | Planned     | M6        | JST-COMPAT-003                                              | Local/shared examples, diagnostics, restore and request budgets     | Run release acceptance                                      | -                 |
| M7          | Deferred    | M8a       | JST-ADAPT-005                                               | Complete read-only replica through applied position                 | After first release                                         | -                 |
| M8b         | Deferred    | M8a       | JST-COMPAT-004                                              | Verified lossless import with source safety                         | Separate migration release                                  | -                 |

## Evidence ledger

Entries record actual runs. Pending entries do not establish completion. Files
and named tests are checked by the progress gate.

```json
[
  {
    "id": "E-BOOT-001",
    "tasks": ["Bootstrap", "Bootstrap.1"],
    "date": "2026-09-26",
    "kind": "governance",
    "files": [
      "AGENTS.md",
      "Cargo.toml",
      "package.json",
      "spec-validator.config.mjs",
      "scripts/check-progress.mjs",
      "scripts/check-boundaries.mjs",
      "scripts/governance.test.mjs",
      "docs/implementation-plan.md"
    ],
    "tests": [
      "scripts/governance.test.mjs#rejects_missing_evidence",
      "scripts/governance.test.mjs#rejects_completed_parent_with_incomplete_child"
    ],
    "runs": [
      {
        "command": "node --test scripts/governance.test.mjs",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: test file absent; no test cases executed."
      },
      {
        "command": "pnpm spec:check",
        "result": "fail",
        "failures": 1,
        "note": "Rust linker could not use unaccepted Xcode license; resolved with installed Command Line Tools, without changing system configuration."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "7 lanes; 56 requirements; 14 governance tests; Rust skeleton has 0 product tests."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "14 governance tests; 0 product tests."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1; jj 0.45.1; spec-validator registry ^0.1.0",
    "formats": "No production format introduced."
  },
  {
    "id": "E-M0-001",
    "tasks": ["M0", "M0.1", "M0.3"],
    "date": "2026-09-26",
    "kind": "real-rustfs",
    "files": [
      "scripts/rustfs-contract.py",
      "docs/codec-coverage.md",
      "docs/upstream-lock.md",
      "docs/evidence/m0-rustfs-single-node.json",
      "package.json",
      "spec-validator.config.mjs"
    ],
    "tests": [
      "scripts/rustfs-contract.py#test_signatures_bind_conditions_and_bytes",
      "scripts/rustfs-contract.py#test_rejects_more_than_one_winner",
      "scripts/rustfs-contract.py#test_unknown_is_not_a_precondition_failure"
    ],
    "runs": [
      {
        "command": "python3 scripts/rustfs-contract.py --self-test",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: script absent."
      },
      {
        "command": "python3 scripts/rustfs-contract.py --self-test",
        "result": "pass",
        "failures": 0,
        "note": "3 tests"
      },
      {
        "command": "python3 scripts/rustfs-contract.py --run --output .scratch/rustfs-initial.json",
        "result": "pass",
        "failures": 0,
        "note": "7 probes; 20 rounds x 16 contenders for create and CAS; owned container and volume removed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "jj 0.45.1; RustFS sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f; reference SHAs in upstream-lock",
    "formats": "Profile defined; no production persistence implemented."
  },
  {
    "id": "E-M0-002",
    "tasks": ["M0", "M0.2"],
    "date": "2026-09-26",
    "kind": "compiled-native-integration",
    "files": [
      "Cargo.toml",
      "Cargo.lock",
      "crates/jj-store-cli/examples/upstream_custom_backend.rs",
      "crates/jj-store-jj/tests/compatibility.rs",
      "crates/jj-store-jj/proto/probe.proto",
      "fixtures/native-profile/m0-hashes.json",
      "scripts/smoke-upstream-cli.py",
      "docs/licenses/NOTICE.md",
      "scripts/rustfs-contract.py",
      "docs/evidence/m0-rustfs-cold.json"
    ],
    "tests": [
      "crates/jj-store-jj/tests/compatibility.rs#pinned_identity_roots_and_predecessor_presence",
      "crates/jj-store-jj/tests/compatibility.rs#explicit_bounded_reads_preserve_divergent_heads",
      "crates/jj-store-jj/tests/compatibility.rs#native_factory_preserves_request_context_across_rpc"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test compatibility",
        "result": "fail",
        "failures": 1,
        "note": "Before crate creation: package absent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test compatibility",
        "result": "fail",
        "failures": 1,
        "note": "2 passed; initial upstream hash fixture deliberately empty before recording pinned reference vectors."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test compatibility",
        "result": "pass",
        "failures": 0,
        "note": "3 tests; reference hash fixture established, request-context RPC and read-only divergent operation probes pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build -p jj-store-cli --example upstream_custom_backend",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "Isolated upstream_custom_backend --help/init-jit/describe/log/op log/evolog smoke; reproduced by scripts/smoke-upstream-cli.py",
        "result": "pass",
        "failures": 0,
        "note": "6 commands; upstream Git-backed extension proof only."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "python3 scripts/smoke-upstream-cli.py",
        "result": "pass",
        "failures": 0,
        "note": "6 commands"
      },
      {
        "command": "python3 scripts/rustfs-contract.py --run --output .scratch/m0-rustfs-cold.json",
        "result": "pass",
        "failures": 0,
        "note": "8 real probes, including truncated body; fresh disposable container/volume."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "jj-cli/jj-lib 0.45.1, Rust 1.97.1, tonic 0.14.6, protobuf compiler pinned in Cargo.lock",
    "formats": "Test-only probe protobuf; native profile hash vectors; production wire format not implemented."
  },
  {
    "id": "E-M1-001",
    "tasks": ["M1", "M1.1"],
    "date": "2026-09-27",
    "kind": "native-codec",
    "files": [
      "Cargo.lock",
      "crates/jj-store-model/Cargo.toml",
      "crates/jj-store-model/src/lib.rs",
      "crates/jj-store-model/src/ids.rs",
      "crates/jj-store-model/src/envelope.rs",
      "crates/jj-store-model/tests/framing.rs",
      "crates/jj-store-jj/Cargo.toml",
      "crates/jj-store-jj/src/lib.rs",
      "crates/jj-store-jj/src/codec.rs",
      "crates/jj-store-jj/src/roots.rs",
      "crates/jj-store-jj/tests/native_codec.rs",
      "fixtures/native-profile/m1-hashes.json",
      "fixtures/native-profile/v1-envelopes.json"
    ],
    "tests": [
      "crates/jj-store-jj/tests/native_codec.rs#all_native_fields_roundtrip_without_identity_loss",
      "crates/jj-store-jj/tests/native_codec.rs#codec_ids_match_pinned_simple_stores",
      "crates/jj-store-jj/tests/native_codec.rs#frozen_v1_envelopes_remain_readable",
      "crates/jj-store-jj/tests/native_codec.rs#malformed_native_structures_never_panic",
      "crates/jj-store-model/tests/framing.rs#envelope_rejects_corruption_truncation_wrong_key_and_limits"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_codec",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: native_codec target absent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-jj",
        "result": "fail",
        "failures": 5,
        "note": "Initial compilation: five generic-array conversions required explicit Into."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_codec",
        "result": "fail",
        "failures": 1,
        "note": "5 passed; initial golden vector deliberately empty before recording pinned reference outputs."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_codec",
        "result": "pass",
        "failures": 0,
        "note": "8 tests covering all seven stored kinds, pinned upstream comparison, virtual roots, frozen V1 bytes and malformed inputs."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-model --test framing",
        "result": "pass",
        "failures": 0,
        "note": "2 tests; every single-byte envelope corruption and truncation rejected."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "fail",
        "failures": 2,
        "note": "Initial lint run: unused import and comparison-to-empty; both corrected."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1; jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5",
    "formats": "V1 envelope (113-byte header); profile 1; explicit canonical JSON metadata, raw file/symlink payloads; separate SHA-256 envelope digest and BLAKE2b native identities."
  },
  {
    "id": "E-M1-002",
    "tasks": ["M1", "M1.2"],
    "date": "2026-09-27",
    "kind": "real-rustfs",
    "files": [
      "Cargo.toml",
      "Cargo.lock",
      "crates/jj-store-object/Cargo.toml",
      "crates/jj-store-object/src/lib.rs",
      "crates/jj-store-object/src/staging.rs",
      "crates/jj-store-object/src/memory.rs",
      "crates/jj-store-object/src/s3.rs",
      "crates/jj-store-object/tests/contract.rs",
      "crates/jj-store-object/tests/sigv4.txt",
      "crates/jj-store-jj/src/objects.rs",
      "crates/jj-store-jj/tests/native_objects.rs",
      "crates/jj-store-jj/tests/rustfs.rs",
      "scripts/rustfs-contract.py",
      "docs/evidence/m1-native-rustfs.json"
    ],
    "tests": [
      "crates/jj-store-object/tests/contract.rs#conditional_create_and_cas_have_one_winner",
      "crates/jj-store-object/tests/contract.rs#unknown_before_and_after_apply_are_distinct_from_conflicts",
      "crates/jj-store-object/src/s3.rs#sigv4_matches_independent_python_vector",
      "crates/jj-store-object/src/s3.rs#streaming_reader_rejects_short_and_excess_bodies",
      "crates/jj-store-jj/tests/native_objects.rs#streamed_binary_blobs_match_codec_and_resolve_lost_acknowledgements",
      "crates/jj-store-jj/tests/native_objects.rs#streamed_reader_rejects_late_corruption_truncation_and_forged_identity",
      "crates/jj-store-jj/tests/rustfs.rs#real_rustfs_transport_and_native_blob_contract"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-object --test contract",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: object crate absent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_objects",
        "result": "fail",
        "failures": 1,
        "note": "Streamed binary fixture aborted with async stack overflow; moved bounded buffers to heap."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_objects",
        "result": "pass",
        "failures": 0,
        "note": "4 tests, including late-stream corruption and forged native identity."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-object",
        "result": "pass",
        "failures": 0,
        "note": "2 internal + 4 contract tests."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "fail",
        "failures": 1,
        "note": "Duplicate test import removed; no lint suppression."
      },
      {
        "command": "python3 scripts/rustfs-contract.py --run --rust-client --output .scratch/m1-rustfs.json",
        "result": "pass",
        "failures": 0,
        "note": "9 real probes including 20x16 Python create/CAS races, process kill/restart and 8x8 independent Rust client races; compiled native 8 MiB file and dropped real acknowledgement. Owned container/volume cleaned up."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "jj 0.45.1; Rust 1.97.1; reqwest 0.12.28; RustFS sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f; single-node independent connections only",
    "formats": "Existing V1 native envelope unchanged; repository-local typed keys; opaque conditional version tokens. Uploads use at most two full private disk spools plus bounded 64 KiB buffers; configured single PUT cap <= 5 GiB."
  },
  {
    "id": "E-M2-001",
    "tasks": ["M2", "M2.1"],
    "date": "2026-09-27",
    "kind": "native-dependency-inventory",
    "files": [
      "Cargo.toml",
      "Cargo.lock",
      "crates/jj-store-wal/Cargo.toml",
      "crates/jj-store-wal/src/lib.rs",
      "crates/jj-store-wal/src/inventory.rs",
      "crates/jj-store-wal/tests/inventory.rs",
      "crates/jj-store-jj/Cargo.toml",
      "crates/jj-store-jj/src/dependencies.rs",
      "crates/jj-store-jj/src/objects.rs",
      "crates/jj-store-jj/tests/native_inventory.rs",
      "docs/evidence/m2-inventory-native-rustfs.json"
    ],
    "tests": [
      "crates/jj-store-wal/tests/inventory.rs#complete_dag_closure_retains_history_and_reuses_verified_base",
      "crates/jj-store-wal/tests/inventory.rs#missing_dependencies_cycles_wrong_keys_and_budgets_fail_closed",
      "crates/jj-store-wal/tests/inventory.rs#verified_bases_cannot_cross_storage_sources_even_with_same_repository_id",
      "crates/jj-store-wal/tests/inventory.rs#deadline_cancels_a_stalled_source",
      "crates/jj-store-jj/tests/native_inventory.rs#inventory_includes_invisible_evolution_copies_and_every_reference_term",
      "crates/jj-store-jj/tests/native_inventory.rs#missing_or_corrupt_native_dependencies_never_certify_inventory"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test inventory",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: WAL crate absent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test inventory",
        "result": "pass",
        "failures": 0,
        "note": "4 tests: closure, incremental retention, cycles/missing objects/resource limits, cancellation and source-bound bases."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_inventory",
        "result": "pass",
        "failures": 0,
        "note": "3 tests: complete seven-kind native closure, all view/evolution references, copies, external submodules and missing/corrupt objects."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "python3 scripts/rustfs-contract.py --run --rust-client --output .scratch/m2-native-rustfs.json",
        "result": "pass",
        "failures": 0,
        "note": "Fresh single-node pinned container: all 9 probes passed; native metadata/file transport remains conformant after inventory-source integration. No publication claim. Owned container and volume cleaned up."
      }
    ],
    "result": "pending",
    "remaining": [
      "Attempt-unique WAL candidates and manifest CAS are not implemented.",
      "Durable receipts, unknown-outcome recovery and bounded persistent checkpoint/catalog formats remain M2.2/M2.3."
    ],
    "versions": "jj-lib 0.45.1; Rust 1.97.1",
    "formats": "No persisted format change. Process-local verified inventories retain the full dependency closure; source capabilities are not serialized or used as publication authority."
  }
]
```
