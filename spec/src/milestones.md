# Implementation progress

This is the single status ledger. Done requires original-gate evidence; fake or
analog fixtures cannot satisfy a real-storage gate. Every implementation slice
adds evidence here in its enclosing Jujutsu change.

## Tasks and gates

| ID          | Status      | Depends   | Requirements                                                            | Gate                                                                | Next                                             | Evidence                                              |
| ----------- | ----------- | --------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------- | ------------------------------------------------ | ----------------------------------------------------- |
| Bootstrap   | Done        | -         | JST-GOV-001,JST-GOV-002,JST-GOV-003,JST-GOV-004,JST-GOV-005,JST-GOV-006 | Spec, ledger, skeleton, gates and independent commit                | Maintain governance each slice                   | E-BOOT-001,E-BOOT-002                                 |
| Bootstrap.1 | Done        | -         | JST-GOV-001,JST-GOV-004,JST-GOV-006                                     | Reference checksum, requirements, progress checker negative tests   | Governance maintained each slice                 | E-BOOT-001,E-BOOT-002                                 |
| M0          | Done        | Bootstrap | JST-COMPAT-001,JST-COMPAT-002                                           | Compiled extension and exact RustFS contract qualified              | Maintain pinned compatibility probes             | E-M0-001,E-M0-002                                     |
| M0.1        | Done        | Bootstrap | JST-COMPAT-001                                                          | Pin references/licenses and complete trait/codec matrix             | Re-audit on upstream upgrades                    | E-M0-001                                              |
| M0.2        | Done        | M0.1      | JST-JJ-001,JST-JJ-002,JST-ID-002                                        | Compile custom CLI/RPC context and query spikes; golden IDs         | Retain probes across upstream changes            | E-M0-002                                              |
| M0.3        | Done        | M0.1      | JST-OBJ-002                                                             | Real image create/CAS/range/restart/fault qualification             | Requalify topology or image changes              | E-M0-001                                              |
| M1          | Done        | M0        | JST-ID-001,JST-ID-003,JST-OBJ-001,JST-OBJ-003                           | Complete lossless objects and streamed storage                      | Maintain codec and storage conformance           | E-M1-001,E-M1-002                                     |
| M1.1        | Done        | M0        | JST-ID-003,JST-ID-004,JST-ID-005                                        | All native variants and golden identities                           | Retain V1 fixtures across codec changes          | E-M1-001                                              |
| M1.2        | Done        | M0        | JST-OBJ-001,JST-OBJ-003,JST-OBJ-004                                     | Fault fake and real streamed immutable object transport             | Retain real RustFS and fault-store conformance   | E-M1-002                                              |
| M2          | Done        | M1        | JST-PUB-001,JST-PUB-006                                                 | Publication fault matrix and cold replay pass                       | Retain publication and cold recovery conformance | E-M2-001,E-M2-002                                     |
| M2.1        | Done        | M1        | JST-PUB-001,JST-PUB-002                                                 | Complete inventory and unique candidate CAS                         | Retain publication and cold recovery conformance | E-M2-001,E-M2-002                                     |
| M2.2        | Done        | M2.1      | JST-PUB-003,JST-PUB-004,JST-PUB-005                                     | Retry receipts and unknown outcome recovery                         | Retain publication and cold recovery conformance | E-M2-002                                              |
| M2.3        | Done        | M2.2      | JST-PUB-006                                                             | Checkpoint equivalence and cache-free restart                       | Retain publication and cold recovery conformance | E-M2-002                                              |
| M3          | In progress | M2        | JST-JJ-001,JST-JJ-002,JST-CLI-001,JST-SEC-001                           | Native CLI/server vertical slice with authentication                | Integrate server creation and administration     | E-M3-001,E-M3-002,E-M3-003,E-M3-004,E-M3-005,E-M3-006 |
| M3.1        | Partial     | M2        | JST-SEC-001,JST-SEC-002,JST-SEC-003,JST-SEC-004                         | Revocable tokens and transport authorization                        | Integrate server creation and administration     | E-M3-001,E-M3-004,E-M3-005,E-M3-006                   |
| M3.2        | Partial     | M3.1      | JST-JJ-001,JST-JJ-004,JST-JJ-005                                        | Create edit describe log restart op-log evolog; multi-workspace     | Integrate server creation and administration     | E-M3-002,E-M3-003,E-M3-004,E-M3-005,E-M3-006          |
| M3.3        | Partial     | M3.2      | JST-CLI-001,JST-CLI-002,JST-CLI-003,JST-CLI-004                         | Custom command presentation and capability parity                   | Complete command family and semantic parity      | E-M3-006                                              |
| M4          | Planned     | M3        | JST-API-001,JST-API-005                                                 | Structured reads and basic served UI match native semantics         | Implement read surface                           | -                                                     |
| M4.1        | Planned     | M3        | JST-API-001,JST-API-002,JST-JJ-003                                      | Pinned three-graph reads with zero GET publications                 | Implement bounded graph sessions                 | -                                                     |
| M4.2        | Planned     | M4.1      | JST-API-003,JST-API-004,JST-API-005                                     | SSE, safe UI, generated contracts and cold interaction tests        | Build Svelte UI                                  | -                                                     |
| M5          | Planned     | M4        | JST-MUT-001,JST-MUT-002,JST-MUT-005                                     | Authenticated typed commands and shared RPC policy                  | Implement commands                               | -                                                     |
| M5.1        | Planned     | M4        | JST-MUT-001,JST-MUT-002,JST-MUT-005                                     | Basic changes/refs/reconcile; preview receipt and semantic policy   | Implement command transaction facade             | -                                                     |
| M5.2        | Planned     | M5.1      | JST-MUT-003                                                             | Full abandon descendant bookmark workspace/evolution parity         | Implement abandon                                | -                                                     |
| M5.3        | Planned     | M5.1      | JST-MUT-004                                                             | Atomic stale-safe resolution with native conflict variants          | Implement resolve                                | -                                                     |
| M6          | Planned     | M5        | JST-ADAPT-001,JST-ADAPT-002,JST-ADAPT-003,JST-ADAPT-004                 | Replay-equivalent searchable projection with freshness              | Implement adapters and search                    | -                                                     |
| M6.1        | Planned     | M5        | JST-ADAPT-001,JST-ADAPT-002                                             | Ordered committed reader and durable replay-safe checkpoints        | Implement runner                                 | -                                                     |
| M6.2        | Planned     | M6.1      | JST-ADAPT-003,JST-ADAPT-004                                             | Tantivy crash/rebuild/generation and revocation tests               | Implement projection and UI                      | -                                                     |
| M8a         | Planned     | M6        | JST-COMPAT-003,JST-SEC-005                                              | Pinned release, restore, security, performance and cold UI evidence | Harden first release                             | -                                                     |
| M8a.1       | Planned     | M6        | JST-COMPAT-003                                                          | Local/shared examples, diagnostics, restore and request budgets     | Run release acceptance                           | -                                                     |
| M7          | Deferred    | M8a       | JST-ADAPT-005                                                           | Complete read-only replica through applied position                 | After first release                              | -                                                     |
| M8b         | Deferred    | M8a       | JST-COMPAT-004                                                          | Verified lossless import with source safety                         | Separate migration release                       | -                                                     |

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
  },
  {
    "id": "E-M2-002",
    "tasks": ["M2", "M2.1", "M2.2", "M2.3"],
    "date": "2026-09-27",
    "kind": "manifest-publication-recovery",
    "files": [
      "Cargo.lock",
      "crates/jj-store-jj/tests/rustfs.rs",
      "crates/jj-store-model/src/ids.rs",
      "crates/jj-store-wal/Cargo.toml",
      "crates/jj-store-wal/src/catalog.rs",
      "crates/jj-store-wal/src/inventory.rs",
      "crates/jj-store-wal/src/lib.rs",
      "crates/jj-store-wal/src/publication.rs",
      "crates/jj-store-wal/src/records.rs",
      "crates/jj-store-wal/tests/publication.rs",
      "scripts/rustfs-contract.py",
      "docs/evidence/m2-publication-rustfs.json",
      "spec/src/verification.md"
    ],
    "tests": [
      "crates/jj-store-wal/tests/publication.rs#atomic_publication_retains_receipts_through_restart_and_checkpoint",
      "crates/jj-store-wal/tests/publication.rs#every_staging_or_manifest_fault_preserves_atomic_visibility_and_retry_identity",
      "crates/jj-store-wal/tests/publication.rs#competing_candidates_preserve_siblings_and_only_selected_records_are_committed",
      "crates/jj-store-wal/tests/publication.rs#delayed_duplicate_cannot_resurrect_superseded_heads",
      "crates/jj-store-wal/tests/publication.rs#lost_acknowledgement_resolves_from_history_after_another_publication",
      "crates/jj-store-wal/tests/publication.rs#strict_stale_intent_reuse_and_missing_dependencies_publish_nothing",
      "crates/jj-store-wal/tests/publication.rs#checkpoint_race_preserves_newer_tail_receipts_and_inventory",
      "crates/jj-store-wal/tests/publication.rs#settings_are_selected_atomically_and_initialization_is_idempotent",
      "crates/jj-store-wal/tests/publication.rs#persistent_catalog_splits_are_bounded_and_old_roots_remain_readable",
      "crates/jj-store-wal/tests/publication.rs#cold_recovery_rejects_missing_native_data_and_control_corruption",
      "crates/jj-store-wal/tests/publication.rs#deterministic_publication_sequences_replay_with_original_results",
      "crates/jj-store-wal/tests/publication.rs#authorization_is_rechecked_after_cas_conflict_and_before_receipt_replay",
      "crates/jj-store-wal/tests/publication.rs#semantic_denial_stages_no_publication_records",
      "crates/jj-store-wal/tests/publication.rs#cold_recovery_verifies_retained_historical_roots_and_settings",
      "crates/jj-store-wal/tests/publication.rs#cancellation_preserves_unknown_identity_before_and_after_manifest_application",
      "crates/jj-store-jj/tests/rustfs.rs#real_rustfs_manifest_publication_and_cold_recovery"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: publication target absent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication cold_recovery_verifies_retained_historical_roots_and_settings",
        "result": "fail",
        "failures": 1,
        "note": "Review regression reproduced accepted corruption in a retained historical inventory root."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication",
        "result": "pass",
        "failures": 0,
        "note": "15 publication, fault, race, randomized replay, cancellation, guard and recovery tests. Historical-root regression now passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --rust-client --output .scratch/m2-publication-rustfs-final.json",
        "result": "pass",
        "failures": 0,
        "note": "10 real probes; 19 accepted native publications with concurrent siblings, randomized head updates, unknown acknowledgements, late receipts and checkpoints. A fresh process after container kill/restart verifies all retained native and control data. Owned services cleaned up."
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
        "failures": 0,
        "note": "45 Rust tests, 14 governance tests and 3 harness self-tests; 2 external RustFS tests run separately by the real harness."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm format:check",
        "result": "fail",
        "failures": 2,
        "note": "Two canonical Markdown files required formatting; corrected."
      },
      {
        "command": "pnpm check:progress",
        "result": "fail",
        "failures": 4,
        "note": "Ledger status-edit script matched four dependency cells; restored original requirement IDs before commit."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm check:progress",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:build",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "jj 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1; pinned single-node RustFS.",
    "formats": "New V1 canonical repository-bound JSON control envelopes, SHA-256 references; attempt-unique WAL keys; bounded radix catalogs; manifest revision distinct from publication sequence; native V1 envelope unchanged."
  },
  {
    "id": "E-M3-001",
    "tasks": ["M3", "M3.1"],
    "date": "2026-09-27",
    "kind": "authentication-foundation",
    "files": [
      "Cargo.lock",
      "Cargo.toml",
      "crates/jj-store-model/src/ids.rs",
      "crates/jj-store-service/Cargo.toml",
      "crates/jj-store-service/src/auth.rs",
      "crates/jj-store-service/src/guard.rs",
      "crates/jj-store-service/src/lib.rs",
      "crates/jj-store-service/tests/authorization.rs",
      "crates/jj-store-wal/Cargo.toml",
      "crates/jj-store-wal/src/publication.rs",
      "crates/jj-store-wal/tests/publication.rs",
      "docs/evidence/m3-auth-storage-rustfs.json"
    ],
    "tests": [
      "crates/jj-store-service/tests/authorization.rs#tokens_are_secret_scope_bound_revocable_and_expiring",
      "crates/jj-store-service/tests/authorization.rs#operator_capability_never_implies_repository_access",
      "crates/jj-store-service/tests/authorization.rs#malformed_credentials_policies_and_limits_fail_closed",
      "crates/jj-store-service/tests/authorization.rs#listener_configuration_rejects_insecure_shared_or_implicit_development_access",
      "crates/jj-store-service/tests/authorization.rs#actor_permission_expiry_and_semantic_denial_precede_dependency_reads",
      "crates/jj-store-service/tests/authorization.rs#revocation_wins_publication_race_and_blocks_receipt_replay",
      "crates/jj-store-service/tests/authorization.rs#replacement_policy_must_retain_scope_and_can_be_read_after_restart",
      "crates/jj-store-wal/tests/publication.rs#cancellation_preserves_unknown_identity_before_and_after_manifest_application"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service --test authorization",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: service crate absent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service --test authorization",
        "result": "pass",
        "failures": 0,
        "note": "7 tests: token scope/permissions/verifiers/expiry/revocation, limits/malformed inputs, listener configuration and publication/receipt authorization races."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "fail",
        "failures": 1,
        "note": "51 Rust tests passed; cancellation fixture timed out during staging under concurrent compiler load, before its intended after-CAS fault."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication cancellation_preserves_unknown_identity_before_and_after_manifest_application",
        "result": "pass",
        "failures": 0,
        "note": "Synchronized with the exact injected before/after-CAS boundary before advancing a controlled Tokio clock; both original publication-state assertions retained."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --rust-client --output .scratch/m3-auth-storage-rustfs.json",
        "result": "pass",
        "failures": 0,
        "note": "All 10 storage/publication probes pass after manifest-reader extraction and token-ID addition. This is storage compatibility evidence, not deployed transport-authentication evidence."
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
        "failures": 0,
        "note": "52 Rust tests, 14 governance tests and 3 harness self-tests pass after the deterministic cancellation fixture fix. The two external RustFS tests ran separately."
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
    "result": "pending",
    "remaining": [
      "HTTP/RPC authentication middleware and production native semantic policy are not connected.",
      "OS credential integration, operator administration and server TLS enforcement remain M3."
    ],
    "versions": "Rust 1.97.1; jj 0.45.1; SHA-256, subtle constant-time comparison, zeroize secret buffers.",
    "formats": "V1 repository/operator policy selected as manifest settings; 128 credentials and 64 KiB limit; jst1 token carries 128-bit ID and 256-bit secret; only domain/scope-bound verifiers persisted. No native/envelope/control-record format change."
  },
  {
    "id": "E-M3-002",
    "tasks": ["M3", "M3.2"],
    "date": "2026-09-27",
    "kind": "native-operation-traits",
    "files": [
      "crates/jj-store-jj/src/lib.rs",
      "crates/jj-store-jj/src/operation_stores.rs",
      "crates/jj-store-jj/tests/native_operations.rs"
    ],
    "tests": [
      "crates/jj-store-jj/tests/native_operations.rs#native_operation_traits_preserve_identity_fields_and_virtual_roots",
      "crates/jj-store-jj/tests/native_operations.rs#staged_prefixes_are_session_local_and_reads_publish_nothing",
      "crates/jj-store-jj/tests/native_operations.rs#concurrent_native_head_updates_keep_siblings_and_receipt_retries",
      "crates/jj-store-jj/tests/native_operations.rs#failed_upload_and_revocation_never_report_native_success",
      "crates/jj-store-jj/tests/native_operations.rs#prefix_queries_fail_on_budget_or_deadline_instead_of_partial_matches",
      "crates/jj-store-jj/tests/native_operations.rs#invalid_heads_and_invalid_id_lengths_fail_before_publication",
      "crates/jj-store-jj/tests/native_operations.rs#staged_operation_limit_preserves_identical_write_retry"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_operations",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: native_operations target absent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_operations",
        "result": "pass",
        "failures": 0,
        "note": "6 tests over real pinned OpStore/OpHeadsStore traits, checked native objects and the manifest coordinator using a deterministic test authority."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_operations staged_operation_limit_preserves_identical_write_retry",
        "result": "fail",
        "failures": 1,
        "note": "Review regression: a full staging budget rejected a repeated identical operation upload; fixed to preserve idempotency without admitting a new distinct operation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_operations",
        "result": "pass",
        "failures": 0,
        "note": "7 native operation trait tests, including identical retry at the staging limit."
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
        "failures": 0,
        "note": "59 Rust tests, 14 governance tests and 3 storage-harness self-tests. 2 external RustFS tests remain explicitly ignored in the normal lane; this slice does not claim live transport integration."
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
    "result": "pending",
    "remaining": [
      "Production Backend, StoreFactories, authenticated authority implementation and native RPC are still required.",
      "Independent physical workspaces and full CLI lifecycle acceptance remain unimplemented."
    ],
    "versions": "jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1",
    "formats": "No storage format changes. Owned operation-authority interface requires current authorization and guarded durable publication; bounded session-local staging enumeration is not committed authority."
  },
  {
    "id": "E-BOOT-002",
    "tasks": ["Bootstrap", "Bootstrap.1"],
    "date": "2026-09-27",
    "kind": "rustfmt-governance",
    "files": [
      "README.md",
      "package.json",
      "rustfmt.toml",
      "scripts/check-progress.mjs",
      "spec/src/architecture.md",
      "spec/src/spec-governance.md",
      "spec/src/verification.md",
      "spec-validator.config.mjs"
    ],
    "tests": [],
    "runs": [
      {
        "command": "rustup component list --installed",
        "result": "pass",
        "failures": 0,
        "note": "rustfmt was already installed by the pinned 1.97.1 toolchain; pnpm check already invoked cargo fmt directly."
      },
      {
        "command": "pnpm format:rust:check",
        "result": "fail",
        "failures": 1,
        "note": "Before change: dedicated Rust formatting check command absent; spec:check and shared format commands did not invoke rustfmt."
      },
      {
        "command": "pnpm format:rust:check",
        "result": "pass",
        "failures": 0,
        "note": "All workspace Rust files already conform to tracked edition/style edition 2024 and Unix newline settings; no Rust source rewrites."
      },
      {
        "command": "pnpm format",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "9 lanes, including explicit rustfmt; 57 canonical requirements."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "59 Rust tests, 14 governance tests and 3 harness self-tests passed; external RustFS tests remain opt-in and are unrelated to formatting."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1; rustfmt 1.9.0-stable (8bab26f4f6 2026-07-14)",
    "formats": "No storage/API changes. New tracked rustfmt.toml; Rust formatter aliases and normal/specification/shared-format integration."
  },
  {
    "id": "E-M3-003",
    "tasks": ["M3", "M3.2"],
    "date": "2026-09-27",
    "kind": "native-commit-backend",
    "files": [
      "Cargo.lock",
      "crates/jj-store-jj/Cargo.toml",
      "crates/jj-store-jj/src/backend.rs",
      "crates/jj-store-jj/src/copy_query.rs",
      "crates/jj-store-jj/src/lib.rs",
      "crates/jj-store-jj/tests/native_backend.rs",
      "spec/src/jj-integration.md",
      "spec/src/architecture.md"
    ],
    "tests": [
      "crates/jj-store-jj/tests/native_backend.rs#native_backend_roundtrips_roots_full_trees_commits_and_signatures",
      "crates/jj-store-jj/tests/native_backend.rs#file_stream_survives_backend_drop_and_rejects_unknown_corrupt_or_revoked_reads",
      "crates/jj-store-jj/tests/native_backend.rs#copy_inventory_is_session_local_complete_deterministic_and_reverse_topological",
      "crates/jj-store-jj/tests/native_backend.rs#copy_records_follow_renames_edits_dag_ranges_and_path_filters",
      "crates/jj-store-jj/tests/native_backend.rs#copy_records_cover_merges_multiple_sources_and_intra_commit_chains",
      "crates/jj-store-jj/tests/native_backend.rs#conflicted_copy_targets_preserve_positive_alternatives_and_nested_paths",
      "crates/jj-store-jj/tests/native_backend.rs#copy_queries_fail_explicitly_on_limits_deadlines_missing_data_and_revocation",
      "crates/jj-store-jj/tests/native_backend.rs#staging_limit_allows_identical_copy_retry_without_accepting_new_history",
      "crates/jj-store-jj/tests/native_backend.rs#signing_failure_and_unacknowledged_metadata_leave_no_successful_object",
      "crates/jj-store-jj/tests/native_backend.rs#record_stream_rechecks_revocation_between_results_and_rejects_bad_copy_paths",
      "crates/jj-store-jj/tests/native_backend.rs#canceled_tree_terms_do_not_emit_deleted_copy_records"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: native_backend target absent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-jj",
        "result": "fail",
        "failures": 2,
        "note": "Initial integration: ObjectKey does not implement Display; RepoPathBuf comparison requires borrowed RepoPath. Both corrected."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend",
        "result": "fail",
        "failures": 1,
        "note": "Test compilation: pinned RepoPathComponent uses as_internal_str rather than as_str; corrected."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend",
        "result": "pass",
        "failures": 0,
        "note": "8 focused tests passed before adding signing-failure and mid-stream-revocation coverage."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend",
        "result": "pass",
        "failures": 0,
        "note": "10 focused tests passed, including signing failure and revocation between streamed records."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "Initial 9-lane pass before review found canceled tree-term edge case."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend canceled_tree_terms_do_not_emit_deleted_copy_records",
        "result": "fail",
        "failures": 1,
        "note": "Review regression: considering only positive tree terms emitted a copy for a file removed by cancellation against the base."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend canceled_tree_terms_do_not_emit_deleted_copy_records",
        "result": "pass",
        "failures": 0,
        "note": "Full signed tree terms now simplify at each path using native Merge; the deleted file produces no copy record."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend",
        "result": "pass",
        "failures": 0,
        "note": "11 backend tests pass after native per-path merge simplification. Includes complete objects/signatures, streamed binary reads, scope/revocation, copy DAGs/chains/conflicts, unknown outcomes and resource limits."
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
        "note": "70 Rust tests, 14 governance tests and 3 storage-harness self-tests pass. Two external RustFS tests remain opt-in. This slice validates native trait integration using the fault store, not live CLI/server acceptance."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "Final nine lanes pass after merge-cancellation regression fix, including rustfmt, specification mapping, tests, governance and mdBook."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "Production authenticated authority, StoreFactories and remote RPC still required.",
      "Full CLI lifecycle, independent workspaces and deployed transport-authentication acceptance remain M3."
    ],
    "versions": "jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1",
    "formats": "Native V1 objects unchanged. New signatures cover domain-separated V1 unsigned commit payload; existing signature data remains exact. Copy-record and related-copy queries are derived bounded results, not publication state."
  },
  {
    "id": "E-M3-004",
    "tasks": ["M3", "M3.1", "M3.2"],
    "date": "2026-09-27",
    "kind": "authenticated-native-lifecycle",
    "files": [
      "Cargo.lock",
      "Cargo.toml",
      "crates/jj-store-cli/Cargo.toml",
      "crates/jj-store-cli/tests/native_lifecycle.rs",
      "crates/jj-store-jj/Cargo.toml",
      "crates/jj-store-jj/src/backend.rs",
      "crates/jj-store-jj/src/factories.rs",
      "crates/jj-store-jj/src/lib.rs",
      "crates/jj-store-jj/src/operation_stores.rs",
      "crates/jj-store-jj/src/policy.rs",
      "crates/jj-store-jj/src/runtime.rs",
      "crates/jj-store-jj/tests/native_operations.rs",
      "crates/jj-store-service/Cargo.toml",
      "crates/jj-store-service/src/guard.rs",
      "crates/jj-store-service/src/lib.rs",
      "crates/jj-store-service/src/native.rs",
      "crates/jj-store-wal/src/publication.rs",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/jj-integration.md",
      "spec/src/mutations.md",
      "spec/src/publication.md",
      "spec/src/security.md",
      "docs/evidence/m3-native-session-rustfs.json",
      "spec/src/compatibility.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle.rs#authenticated_native_workspace_edit_describe_reopen_and_cache_rebuild",
      "crates/jj-store-cli/tests/native_lifecycle.rs#independent_workspaces_preserve_concurrent_operations_and_unsnapshotted_edits",
      "crates/jj-store-cli/tests/native_lifecycle.rs#transition_identity_survives_session_restart_and_late_duplicates",
      "crates/jj-store-cli/tests/native_lifecycle.rs#protected_history_rejects_hidden_rewrites_without_predecessor_metadata",
      "crates/jj-store-cli/tests/native_lifecycle.rs#protected_bookmark_also_protects_its_commit_identity",
      "crates/jj-store-cli/tests/native_lifecycle.rs#committed_superseded_operation_cannot_be_resurrected_under_a_new_request",
      "crates/jj-store-cli/tests/native_lifecycle.rs#current_permissions_revocation_and_repository_binding_cover_native_access",
      "crates/jj-store-cli/tests/native_lifecycle.rs#concurrent_workspace_name_claims_cannot_alias_physical_clients",
      "crates/jj-store-cli/tests/native_lifecycle.rs#malformed_native_publications_stale_strict_intents_and_unknown_retries",
      "crates/jj-store-cli/tests/native_lifecycle.rs#protection_can_freeze_multiple_existing_divergent_incarnations"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service --test native_session",
        "result": "fail",
        "failures": 1,
        "note": "Initial service test target absent; native lifecycle tests belong in the CLI integration boundary because they import upstream JJ types."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: native_lifecycle target absent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check --workspace",
        "result": "fail",
        "failures": 3,
        "note": "Initial pinned-API integration diagnostics: workspace-name import, working-copy factory visibility and default-store factory constructor; corrected."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check --workspace",
        "result": "fail",
        "failures": 1,
        "note": "Corrected default factory function name to default_backend_factories."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle",
        "result": "fail",
        "failures": 4,
        "note": "Initial lifecycle fixture compilation used async syntax for the synchronous get_commit method and incorrect file-ID borrow; corrected."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle",
        "result": "fail",
        "failures": 2,
        "note": "Physical snapshot tests failed on upstream worker threads without a Tokio reactor; per-poll runtime context added to native traits and returned file/copy streams."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle",
        "result": "fail",
        "failures": 2,
        "note": "Runtime bridge required explicit file-reader and coordination-lock trait-object coercions; corrected."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle",
        "result": "pass",
        "failures": 0,
        "note": "Original three lifecycle tests pass after runtime bridge, including physical file snapshot, independent workspaces, reopen and cache rebuild."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle protected_bookmark_also_protects_its_commit_identity",
        "result": "fail",
        "failures": 3,
        "note": "New policy test compilation used invalid borrowed name conversions; corrected to pinned RefName/WorkspaceName constructors."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle protected_bookmark_also_protects_its_commit_identity",
        "result": "fail",
        "failures": 1,
        "note": "Protected bookmark identity could be rewritten while its target was retained."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle committed_superseded_operation_cannot_be_resurrected_under_a_new_request",
        "result": "fail",
        "failures": 1,
        "note": "Fresh request IDs could resurrect an already superseded committed operation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle concurrent_workspace_name_claims_cannot_alias_physical_clients",
        "result": "fail",
        "failures": 1,
        "note": "Concurrent clients could introduce the same logical workspace name from a stale parent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle protected_bookmark_also_protects_its_commit_identity",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle committed_superseded_operation_cannot_be_resurrected_under_a_new_request",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle concurrent_workspace_name_claims_cannot_alias_physical_clients",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle",
        "result": "pass",
        "failures": 0,
        "note": "Nine lifecycle/security/retry tests pass. Semantic graph caches share immutable values to avoid cloning large native metadata per traversal."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "fail",
        "failures": 2,
        "note": "Two fixture style diagnostics (cloned reference slice and needless borrow); corrected without changing assertions."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy --workspace --all-targets --locked -- -D warnings",
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
        "failures": 0,
        "note": "79 Rust tests, 14 governance tests and 3 storage harness self-tests. No failures. Two real RustFS tests run separately through the isolated harness."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --rust-client --output .scratch/m3-native-session-rustfs.json",
        "result": "pass",
        "failures": 0,
        "note": "All ten probes pass against pinned single-node RustFS; actual Rust client races, dropped responses and process-kill/fresh-process cache-free publication recovery retained. Owned container and volume cleaned up. This validates storage/publication conformance, not deployed authentication."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "Nine lanes pass before final divergent-protection regression."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle protection_can_freeze_multiple_existing_divergent_incarnations",
        "result": "fail",
        "failures": 1,
        "note": "Final review: one protected commit ID per change ID incorrectly rejected unchanged history with multiple protected divergent incarnations."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle protection_can_freeze_multiple_existing_divergent_incarnations",
        "result": "pass",
        "failures": 0,
        "note": "Protected change IDs now retain the set of exact protected incarnations; existing divergence remains readable and publishable without permitting a new rewrite."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle",
        "result": "pass",
        "failures": 0,
        "note": "Final ten native lifecycle, authentication, semantic-policy, retry and concurrency tests pass."
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
        "note": "Final 80 Rust tests, 14 governance tests and 3 harness self-tests pass. Two external RustFS tests were exercised by the recorded real-storage lane."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All nine lanes, including rustfmt, governed progress, package boundaries, tests, mapped spec-first and mdBook, pass after final regression fix."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "Native RPC, server executable, CLI commands and OS credential storage are not yet integrated.",
      "Library lifecycle evidence is not full executable CLI/server acceptance or browser/API parity.",
      "Trusted protected-history rules are constructor configuration; server configuration and transport administration remain to be connected."
    ],
    "versions": "jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1",
    "formats": "Native objects, WAL, manifests and token-policy V1 unchanged. Credential-free client cache binding format jj-store-client-cache-v1 / jj-native-0.45.1-v1. Native request/publication IDs are domain-separated hashes of repository, principal and exact head transition; per-poll runtime binding is process-local."
  },
  {
    "id": "E-M3-005",
    "tasks": ["M3", "M3.1", "M3.2"],
    "date": "2026-09-27",
    "kind": "native-rpc",
    "files": [
      "crates/jj-store-rpc/Cargo.toml",
      "crates/jj-store-rpc/proto/native.proto",
      "crates/jj-store-rpc/src/client.rs",
      "crates/jj-store-rpc/src/server.rs",
      "crates/jj-store-service/src/transport.rs",
      "crates/jj-store-jj/src/objects.rs",
      "crates/jj-store-cli/src/remote.rs",
      "crates/jj-store-cli/tests/native_rpc.rs",
      "scripts/check-boundaries.mjs",
      "scripts/governance.test.mjs",
      "Cargo.toml",
      "Cargo.lock"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_rpc.rs#remote_native_factories_preserve_workspace_and_history_after_reconnect",
      "crates/jj-store-cli/tests/native_rpc.rs#rpc_authorization_and_native_validation_precede_immutable_storage",
      "crates/jj-store-cli/tests/native_rpc.rs#rpc_guarded_publication_preserves_receipts_siblings_and_protection",
      "crates/jj-store-cli/tests/native_rpc.rs#rpc_stream_protocol_deadlines_and_permits_are_bounded",
      "crates/jj-store-cli/tests/native_rpc.rs#rpc_requires_credentials_and_explicit_secure_transport_configuration",
      "crates/jj-store-cli/tests/native_rpc.rs#rpc_tls_checks_trust_and_server_hostname",
      "crates/jj-store-cli/tests/native_rpc.rs#rpc_download_rechecks_revocation_and_releases_permits_on_cancellation"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_rpc",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: no native_rpc target existed; no test cases executed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check --workspace",
        "result": "fail",
        "failures": 2,
        "note": "Initial compile: streamed reader error type ambiguity and async upload Send lifetime error. Subsequent compile attempts isolated the remaining lifetime error; solved with an owned bounded upload producer and explicit stream error type."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_rpc",
        "result": "pass",
        "failures": 0,
        "note": "Five loopback gRPC acceptance tests passed before TLS and mid-stream revocation coverage."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_rpc",
        "result": "pass",
        "failures": 0,
        "note": "Seven loopback tests pass, including authenticated native workspace binary snapshot/describe, real server reconnection and fresh cache/history recovery, immutable validation, current revocation, crafted publication protection, sibling heads, receipt retries, deadlines/cancellation and real TLS trust/hostname verification. Memory storage is not RustFS conformance."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "fail",
        "failures": 1,
        "note": "Clippy found one redundant field-name initialization, reported for library and test targets; fixed without changing behavior."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Clippy all targets, Rust formatting, specification, seven package boundaries and progress bookkeeping pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "87 Rust tests, 14 governance tests and 3 storage-harness self-tests pass. Two external RustFS tests remain ignored in this lane; no deployment conformance claim for RPC."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All nine lanes pass, including rustfmt, mapped spec-first, mdBook and the seven native RPC tests."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "Executable CLI/server command, configuration and operator/token administration remain.",
      "OS credential integration and the full command presentation/capability matrix remain.",
      "M3 remains incomplete; RPC library tests do not establish executable or deployment acceptance."
    ],
    "versions": "jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1; tonic/prost 0.14",
    "formats": "Introduces jj_store.native.v1 protobuf RPC with typed immutable objects and guarded head transitions. Native codecs, WAL, manifest, token policy and local cache formats unchanged. No raw control-key access. TLS is required except explicitly enabled literal loopback."
  },
  {
    "id": "E-M3-006",
    "tasks": ["M3", "M3.1", "M3.2", "M3.3"],
    "date": "2026-09-27",
    "kind": "executable-native-cli",
    "files": [
      "Cargo.lock",
      "crates/jj-store-cli/Cargo.toml",
      "crates/jj-store-cli/build.rs",
      "crates/jj-store-cli/src/binding.rs",
      "crates/jj-store-cli/src/commands.rs",
      "crates/jj-store-cli/src/credentials.rs",
      "crates/jj-store-cli/src/lib.rs",
      "crates/jj-store-cli/src/main.rs",
      "crates/jj-store-cli/src/presentation.rs",
      "crates/jj-store-cli/tests/executable_native.rs",
      "crates/jj-store-cli/tests/fixtures/terminal.py",
      "docs/cli/attach.md",
      "docs/cli/capabilities.md",
      "docs/cli/commands.md",
      "docs/cli/credentials.md",
      "docs/cli/doctor.md",
      "docs/cli/pagination.md",
      "docs/cli/status.md",
      "docs/cli/store.md",
      "docs/cli/version.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/jj-integration.md",
      "spec/src/security.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/executable_native.rs#executable_native_create_edit_describe_log_restart_and_evolution",
      "crates/jj-store-cli/tests/executable_native.rs#executable_workspaces_keep_local_edits_and_reject_credential_or_binding_mismatch",
      "crates/jj-store-cli/tests/executable_native.rs#custom_help_completion_build_info_and_ordinary_git_commands",
      "crates/jj-store-cli/tests/executable_native.rs#custom_color_json_and_metadata_validation_do_not_contact_or_leak_credentials",
      "crates/jj-store-cli/tests/executable_native.rs#os_credential_facility_roundtrip",
      "crates/jj-store-cli/tests/executable_native.rs#custom_terminal_paging_and_color_policy",
      "crates/jj-store-cli/tests/executable_native.rs#attach_rejects_read_only_access_before_creating_local_state",
      "crates/jj-store-cli/tests/executable_native.rs#executable_login_logout_uses_os_credentials_across_processes"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: executable_native target absent; no cases executed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli",
        "result": "fail",
        "failures": 1,
        "note": "Initial compile used GlobalArgs.no_pager instead of GlobalArgs.early_args.no_pager; corrected."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native",
        "result": "fail",
        "failures": 1,
        "note": "Initial three tests: two passed, one help-rendering assertion failed because wrapped Markdown code spans retained backticks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native custom_help_completion_build_info_and_ordinary_git_commands",
        "result": "pass",
        "failures": 0,
        "note": "Exact rendering regression passes after handling split markers."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native attach_rejects_read_only_access_before_creating_local_state",
        "result": "fail",
        "failures": 1,
        "note": "Denied attachment created a physical directory before authorization."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native attach_rejects_read_only_access_before_creating_local_state",
        "result": "pass",
        "failures": 0,
        "note": "Write authorization now precedes local filesystem changes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native custom_terminal_paging_and_color_policy",
        "result": "fail",
        "failures": 1,
        "note": "Missing shell-wrapped pager lost the result instead of falling back to stdout."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native custom_terminal_paging_and_color_policy",
        "result": "pass",
        "failures": 0,
        "note": "Direct pager process with quoted arguments retains output on spawn failure."
      },
      {
        "command": "env -u JJ_STORE_TOKEN DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native -- --ignored",
        "result": "pass",
        "failures": 0,
        "note": "Both real macOS OS-facility tests pass: direct roundtrip and separate-process login/status/logout. Generated credential entries deleted."
      },
      {
        "command": "pnpm format:rust:check",
        "result": "fail",
        "failures": 1,
        "note": "Formatting drift in final pager refactor; formatter gate rejected it before cargo checks."
      },
      {
        "command": "pnpm format:rust && pnpm format:rust:check",
        "result": "pass",
        "failures": 0,
        "note": "Workspace rustfmt applied and exact formatting gate passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native",
        "result": "pass",
        "failures": 0,
        "note": "Final six normal executable tests pass after pager refactor; two OS-facility tests have separate explicit passing evidence."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Clippy all targets, rustfmt, specification, package boundaries and progress checks pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "93 Rust tests, 14 governance tests and 3 storage-harness self-tests pass. Four explicitly ignored external tests: two RustFS tests and two OS-facility tests; OS tests passed in the separate recorded lane."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Development jj executable built; installed jj unchanged."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All nine lanes pass, including rustfmt, mapped spec-first, mdBook and executable acceptance."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "Standalone server run/configuration/diagnostics, operator provisioning, repository creation, token administration and serve-local remain.",
      "Attachment recovery after uncertain publication, full ordinary-command semantic parity and real executable RustFS acceptance remain M3 gates.",
      "Loopback executable tests use the production RPC/service with memory storage; they do not establish RustFS deployment conformance.",
      "macOS OS credential integration is verified; Linux and Windows platform acceptance remains release hardening."
    ],
    "versions": "jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1; keyring 3.6.3; jj-nostr command reference ab864f64",
    "formats": "Introduces credential-free connection record format 1 pinned to jj-native-0.45.1-v1. OS account keys bind canonical endpoint and repository. Native objects, WAL, manifest, RPC and token-policy V1 unchanged."
  }
]
```
