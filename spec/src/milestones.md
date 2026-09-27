# Implementation progress

This is the single status ledger. Done requires original-gate evidence; fake or
analog fixtures cannot satisfy a real-storage gate. Every implementation slice
adds evidence here in its enclosing Jujutsu change.

## Tasks and gates

| ID          | Status      | Depends     | Requirements                                                            | Gate                                                                                                                | Next                                                     | Evidence                                                                                                                                                                                        |
| ----------- | ----------- | ----------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bootstrap   | Done        | -           | JST-GOV-001,JST-GOV-002,JST-GOV-003,JST-GOV-004,JST-GOV-005,JST-GOV-006 | Spec, ledger, skeleton, gates and independent commit                                                                | Maintain governance each slice                           | E-BOOT-001,E-BOOT-002                                                                                                                                                                           |
| Bootstrap.1 | Done        | -           | JST-GOV-001,JST-GOV-004,JST-GOV-006                                     | Reference checksum, requirements, progress checker negative tests                                                   | Governance maintained each slice                         | E-BOOT-001,E-BOOT-002                                                                                                                                                                           |
| M0          | Done        | Bootstrap   | JST-COMPAT-001,JST-COMPAT-002                                           | Compiled extension and exact RustFS contract qualified                                                              | Maintain pinned compatibility probes                     | E-M0-001,E-M0-002                                                                                                                                                                               |
| M0.1        | Done        | Bootstrap   | JST-COMPAT-001                                                          | Pin references/licenses and complete trait/codec matrix                                                             | Re-audit on upstream upgrades                            | E-M0-001                                                                                                                                                                                        |
| M0.2        | Done        | M0.1        | JST-JJ-001,JST-JJ-002,JST-ID-002                                        | Compile custom CLI/RPC context and query spikes; golden IDs                                                         | Retain probes across upstream changes                    | E-M0-002                                                                                                                                                                                        |
| M0.3        | Done        | M0.1        | JST-OBJ-002                                                             | Real image create/CAS/range/restart/fault qualification                                                             | Requalify topology or image changes                      | E-M0-001                                                                                                                                                                                        |
| M1          | Done        | M0          | JST-ID-001,JST-ID-003,JST-OBJ-001,JST-OBJ-003                           | Complete lossless objects and streamed storage                                                                      | Maintain codec and storage conformance                   | E-M1-001,E-M1-002                                                                                                                                                                               |
| M1.1        | Done        | M0          | JST-ID-003,JST-ID-004,JST-ID-005                                        | All native variants and golden identities                                                                           | Retain V1 fixtures across codec changes                  | E-M1-001                                                                                                                                                                                        |
| M1.2        | Done        | M0          | JST-OBJ-001,JST-OBJ-003,JST-OBJ-004                                     | Fault fake and real streamed immutable object transport                                                             | Retain real RustFS and fault-store conformance           | E-M1-002                                                                                                                                                                                        |
| M2          | Done        | M1          | JST-PUB-001,JST-PUB-006                                                 | Publication fault matrix and cold replay pass                                                                       | Retain publication and cold recovery conformance         | E-M2-001,E-M2-002                                                                                                                                                                               |
| M2.1        | Done        | M1          | JST-PUB-001,JST-PUB-002                                                 | Complete inventory and unique candidate CAS                                                                         | Retain publication and cold recovery conformance         | E-M2-001,E-M2-002                                                                                                                                                                               |
| M2.2        | Done        | M2.1        | JST-PUB-003,JST-PUB-004,JST-PUB-005                                     | Retry receipts and unknown outcome recovery                                                                         | Retain publication and cold recovery conformance         | E-M2-002                                                                                                                                                                                        |
| M2.3        | Done        | M2.2        | JST-PUB-006                                                             | Checkpoint equivalence and cache-free restart                                                                       | Retain publication and cold recovery conformance         | E-M2-002                                                                                                                                                                                        |
| M3          | Done        | M2          | JST-JJ-001,JST-JJ-002,JST-CLI-001,JST-SEC-001                           | Native CLI/server vertical slice with authentication                                                                | Implement M4 pinned read sessions                        | E-M3-001,E-M3-002,E-M3-003,E-M3-004,E-M3-005,E-M3-006,E-M3-007,E-M3-008,E-M3-009,E-M3-010,E-M3-011,E-M3-012,E-M3-013,E-M3-014,E-M3-015                                                          |
| M3.1        | Done        | M2          | JST-SEC-001,JST-SEC-002,JST-SEC-003,JST-SEC-004                         | Revocable tokens and transport authorization                                                                        | Retain native slice acceptance                           | E-M3-001,E-M3-004,E-M3-005,E-M3-006,E-M3-007,E-M3-008,E-M3-009,E-M3-011,E-M3-012,E-M3-014                                                                                                       |
| M3.2        | Done        | M3.1        | JST-JJ-001,JST-JJ-004,JST-JJ-005                                        | Create edit describe log restart op-log evolog; multi-workspace                                                     | Retain native slice acceptance                           | E-M3-002,E-M3-003,E-M3-004,E-M3-005,E-M3-006,E-M3-008,E-M3-009,E-M3-010,E-M3-011,E-M3-013,E-M3-014                                                                                              |
| M3.3        | Done        | M3.2        | JST-CLI-001,JST-CLI-002,JST-CLI-003,JST-CLI-004                         | Custom command presentation and capability parity                                                                   | Retain native slice acceptance                           | E-M3-006,E-M3-009,E-M3-010,E-M3-011,E-M3-012,E-M3-013,E-M3-014,E-M3-015                                                                                                                         |
| M4          | In progress | M3          | JST-API-001,JST-API-005                                                 | Structured reads and served UI match native semantics                                                               | Complete M4.10-M4.14 and qualify real SSE deployment     | E-M4-001,E-M4-002,E-M4-003,E-M4-004,E-M4-005,E-M4-006,E-M4-007,E-M4-008,E-M4-009,E-M4-010,E-M4-011,E-M4-012,E-M4-013,E-M4-014,E-M4-015,E-M4-016,E-M4-017,E-M4-018,E-SCOPE-001,E-UI-007,E-UI-008 |
| M4.1        | Done        | M3          | JST-API-001,JST-API-002,JST-JJ-003                                      | Pinned three-graph reads with zero GET publications                                                                 | Retain pinned read conformance                           | E-M4-001,E-M4-002,E-M4-003,E-M4-004,E-M4-005,E-M4-006,E-M4-007,E-M4-008,E-M4-009,E-M4-010,E-M4-011,E-M4-012,E-M4-013,E-M4-014                                                                   |
| M4.2        | In progress | M4.1        | JST-API-003,JST-API-006                                                 | Committed SSE and generated API contract deployment qualification                                                   | Qualify real RustFS/TLS SSE                              | E-M4-015,E-M4-016,E-M4-017,E-SCOPE-001                                                                                                                                                          |
| M4.3        | In progress | M4.1,M5,M6  | JST-API-004,JST-API-005                                                 | Complete browsing integration, then mutations/conflicts/search and deployed browser acceptance                      | Implement browsing M4.11-M4.14; retain write/search gaps | E-SCOPE-001,E-UI-007,E-UI-008,E-M4-018                                                                                                                                                          |
| M4.4        | Done        | Bootstrap   | JST-UI-001                                                              | Isolated catalog types, interaction/a11y and static build                                                           | Implement shell stories                                  | E-UI-001                                                                                                                                                                                        |
| M4.5        | Done        | M4.4        | JST-UI-002                                                              | Community shell mechanics and navigation acceptance                                                                 | Implement repository fixture views                       | E-UI-002                                                                                                                                                                                        |
| M4.6        | Done        | M4.5        | JST-UI-003                                                              | Native fixture graph/file/diff drill-down and accessibility                                                         | Visual review; live UI remains deferred                  | E-UI-003                                                                                                                                                                                        |
| M4.7        | Done        | M4.6        | JST-UI-004                                                              | Complete in-memory application, snapshot file browser and profile footer                                            | Retain fixture acceptance; live UI remains deferred      | E-UI-004                                                                                                                                                                                        |
| M4.8        | Done        | M4.7        | JST-UI-002,JST-UI-003,JST-UI-004                                        | Reviewed shell/card/header styling and directory navigation pass interaction, accessibility and cold catalog checks | Visual review pending; live UI remains deferred          | E-UI-005,E-UI-006,E-UI-009,E-UI-010                                                                                                                                                             |
| M4.9        | Done        | M4.8        | JST-ARCH-004,JST-UI-005                                                 | Shared production components/controller and no-growth architecture checks                                           | Retain source ownership and proceed to bounded APIs      | E-UI-007,E-UI-008                                                                                                                                                                               |
| M4.10       | Done        | M4.9        | JST-API-001,JST-API-002,JST-API-006                                     | Bounded bookmark, directory metadata and operation-change contracts                                                 | Implement M4.11 authenticated browser data/controller    | E-M4-018                                                                                                                                                                                        |
| M4.11       | Planned     | M4.10       | JST-API-004,JST-API-005,JST-UI-005                                      | Authenticated cached browser implements all browsing screens                                                        | Implement HTTP data source and production shell          | -                                                                                                                                                                                               |
| M4.12       | Planned     | M4.11       | JST-API-002,JST-API-003,JST-UI-005                                      | Hash restoration, Back/Forward and explicit SSE refresh preserve snapshots                                          | Implement after real browsing controller                 | -                                                                                                                                                                                               |
| M4.13       | Planned     | M4.11       | JST-API-002,JST-UI-006                                                  | Large collections are cursor-paged/virtualized and every resizable region persists                                  | Extend Design Core FileListing and integrate release     | -                                                                                                                                                                                               |
| M4.14       | Planned     | M4.12,M4.13 | JST-API-003,JST-API-004,JST-API-005                                     | Cold built browser passes captured and real RustFS-backed native repository acceptance                              | Run after browsing, navigation and layout complete       | -                                                                                                                                                                                               |
| M5          | Done        | M4.1        | JST-MUT-001,JST-MUT-002,JST-MUT-005                                     | Authenticated typed commands and shared RPC policy                                                                  | Retain mutation conformance; proceed to M6               | E-M5-001,E-SCOPE-001,E-M5-002,E-M5-003,E-M5-004,E-M5-005,E-M5-006,E-M5-007,E-M5-008                                                                                                             |
| M5.1        | Done        | M4.1        | JST-MUT-001,JST-MUT-002,JST-MUT-005                                     | Basic changes/refs/reconcile; preview receipt and semantic policy                                                   | Retain basic mutation acceptance                         | E-M5-001,E-SCOPE-001,E-M5-002,E-M5-003,E-M5-004                                                                                                                                                 |
| M5.2        | Done        | M5.1        | JST-MUT-003                                                             | Full abandon descendant bookmark workspace/evolution parity                                                         | Retain abandon acceptance                                | E-M5-005                                                                                                                                                                                        |
| M5.3        | Done        | M5.1        | JST-MUT-004                                                             | Atomic stale-safe resolution with native conflict variants                                                          | Retain mutation conformance; proceed to M6               | E-M5-006,E-M5-007,E-M5-008                                                                                                                                                                      |
| M6          | In progress | M5          | JST-ADAPT-001,JST-ADAPT-002,JST-ADAPT-003,JST-ADAPT-004                 | Replay-equivalent searchable projection with freshness                                                              | Integrate snapshot export/restore and qualify deployment | E-M6-001,E-M6-002,E-M6-003,E-M6-004,E-M6-005,E-M6-006,E-M6-007,E-M6-008,E-M6-009                                                                                                                |
| M6.1        | Done        | M5          | JST-ADAPT-001,JST-ADAPT-002                                             | Ordered committed reader and durable replay-safe checkpoints                                                        | Implement Tantivy projection and search                  | E-M6-001                                                                                                                                                                                        |
| M6.2        | In progress | M6.1        | JST-ADAPT-003,JST-ADAPT-004                                             | Tantivy crash/rebuild/generation and revocation tests                                                               | Integrate snapshot export/restore and qualify deployment | E-SCOPE-001,E-M6-002,E-M6-003,E-M6-004,E-M6-005,E-M6-006,E-M6-007,E-M6-008,E-M6-009                                                                                                             |
| M8a         | Planned     | M6,M4.3     | JST-COMPAT-003,JST-SEC-005                                              | Pinned release, restore, security, performance and cold UI evidence                                                 | Complete backend hardening; retain deferred UI gate      | E-SCOPE-001                                                                                                                                                                                     |
| M8a.1       | Planned     | M6          | JST-COMPAT-003,JST-SEC-005                                              | Local/shared examples, diagnostics, restore and request budgets                                                     | Run non-UI release acceptance                            | E-SCOPE-001                                                                                                                                                                                     |
| M7          | Deferred    | M8a         | JST-ADAPT-005                                                           | Complete read-only replica through applied position                                                                 | After first release                                      | -                                                                                                                                                                                               |
| M8b         | Deferred    | M8a         | JST-COMPAT-004                                                          | Verified lossless import with source safety                                                                         | Separate migration release                               | -                                                                                                                                                                                               |

The user reopened production browsing integration on 27 September 2026 after
accepting the Storybook application. M4.9–M4.14 now own promotion into the
served browser, missing bounded read contracts, cached controller state,
hash/SSE navigation, virtualization/layout persistence and cold real-repository
acceptance. Mutation, conflict-editing and search controls remain later M4.3
gaps. Stories remain visual-pending.

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
  },
  {
    "id": "E-M3-007",
    "tasks": ["M3", "M3.1"],
    "date": "2026-09-27",
    "kind": "typed-administration",
    "files": [
      "crates/jj-store-service/src/administration.rs",
      "crates/jj-store-service/src/auth.rs",
      "crates/jj-store-service/src/lib.rs",
      "crates/jj-store-service/src/native.rs",
      "crates/jj-store-service/tests/administration.rs",
      "crates/jj-store-wal/src/publication.rs",
      "crates/jj-store-wal/tests/publication.rs",
      "spec/src/mutations.md",
      "spec/src/publication.md",
      "spec/src/security.md"
    ],
    "tests": [
      "crates/jj-store-service/tests/administration.rs#creation_requires_fresh_operator_permission_before_storage_and_is_retryable",
      "crates/jj-store-service/tests/administration.rs#token_grants_lists_and_revoke_are_admin_scoped_and_never_persist_secrets",
      "crates/jj-store-service/tests/administration.rs#token_retries_recover_original_receipts_after_later_changes_and_restart",
      "crates/jj-store-service/tests/administration.rs#stale_concurrent_token_mutations_cannot_restore_revoked_grants",
      "crates/jj-store-service/tests/administration.rs#token_action_identity_and_current_authorization_apply_to_receipt_queries",
      "crates/jj-store-service/tests/administration.rs#client_generated_grants_are_exact_idempotent_and_reject_token_id_collisions",
      "crates/jj-store-service/tests/administration.rs#unknown_token_publication_resolves_by_original_request_without_issuing_again",
      "crates/jj-store-wal/tests/publication.rs#historical_settings_bind_selected_snapshot_and_survive_checkpoint"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service --test administration",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: administration target absent; no test cases executed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service --test administration",
        "result": "fail",
        "failures": 1,
        "note": "Initial fixture compile had ambiguous str::contains argument; corrected explicit string slice."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service --test administration",
        "result": "pass",
        "failures": 0,
        "note": "Initial six administration tests pass before unknown-outcome and checkpoint extensions."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication historical_settings_bind_selected_snapshot_and_survive_checkpoint",
        "result": "pass",
        "failures": 0,
        "note": "Historical policy lookup verifies exact selected snapshots across checkpoint/restart and fails on corruption."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service --test administration",
        "result": "pass",
        "failures": 0,
        "note": "All seven tests pass, including explicit unknown-outcome recovery, late receipt replay after checkpoint/restart, secret-free persistence and a concurrent revocation race."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Clippy all targets, rustfmt, specification, boundaries and progress pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "101 Rust tests, 14 governance tests and 3 storage-harness self-tests pass. Four external tests remain explicitly ignored in this lane."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Workspace and development executable build pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All nine specification lanes pass, including full Rust tests, mapped spec-first and mdBook."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "Service administration is not yet exposed through RPC or server commands. Operator file provisioning and refresh wiring remain.",
      "M3 executable server creation, configuration, diagnostics, serve-local, attach recovery and complete ordinary-command parity remain.",
      "Administration fixtures use memory storage and do not establish real RustFS deployment conformance."
    ],
    "versions": "jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1",
    "formats": "No persisted format change: token policy, native objects, WAL, manifests and receipts remain V1. Token administration derives deterministic publication IDs and encodes secret-free typed results; historical settings lookup follows selected catalogs."
  },
  {
    "id": "E-M3-008",
    "tasks": ["M3", "M3.1", "M3.2"],
    "date": "2026-09-27",
    "kind": "typed-administration-rpc",
    "files": [
      "Cargo.lock",
      "crates/jj-store-cli/tests/administration_rpc.rs",
      "crates/jj-store-rpc/Cargo.toml",
      "crates/jj-store-rpc/build.rs",
      "crates/jj-store-rpc/proto/native.proto",
      "crates/jj-store-rpc/src/administration.rs",
      "crates/jj-store-rpc/src/client.rs",
      "crates/jj-store-rpc/src/lib.rs",
      "crates/jj-store-rpc/src/server.rs",
      "crates/jj-store-service/src/administration.rs",
      "crates/jj-store-service/src/native.rs",
      "crates/jj-store-service/src/transport.rs",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/jj-integration.md",
      "spec/src/mutations.md",
      "spec/src/security.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/administration_rpc.rs#rpc_created_repositories_attach_with_real_cli_and_survive_host_restart",
      "crates/jj-store-cli/tests/administration_rpc.rs#rpc_administration_separates_operator_admin_and_cross_repository_access",
      "crates/jj-store-cli/tests/administration_rpc.rs#rpc_token_retries_and_competing_snapshots_preserve_accepted_policy",
      "crates/jj-store-cli/tests/administration_rpc.rs#rpc_rejects_malformed_admin_requests_without_publication_and_redacts_secrets",
      "crates/jj-store-rpc/src/administration.rs#token_receipt_validation_binds_action_request_and_exact_snapshot",
      "crates/jj-store-rpc/src/administration.rs#admin_decoding_clears_credentials_and_rejects_unbounded_or_duplicate_results"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: administration_rpc target absent; no cases executed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check --workspace",
        "result": "fail",
        "failures": 2,
        "note": "Initial compile produced two diagnostics for one ambiguous Result error type in optional receipt conversion. Focused test compilation reproduced the same two diagnostics. Fixed explicit tonic Status type."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc",
        "result": "fail",
        "failures": 2,
        "note": "Same receipt-conversion compile ambiguity, before explicit Status annotation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc",
        "result": "pass",
        "failures": 0,
        "note": "Four loopback acceptance tests pass: typed creation to real CLI attachment/describe/restart, permission and cross-repository denial, competing stale updates with durable receipts, malformed requests and secret redaction."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-rpc --lib",
        "result": "pass",
        "failures": 0,
        "note": "Two protocol validation tests pass: exact receipt action/request/snapshot binding, plaintext clearing and response bounds."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Clippy all targets, rustfmt, specification, dependency boundaries and progress pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "107 Rust tests, 14 governance tests and 3 storage-harness self-tests pass. Four external tests remain explicitly ignored in this lane."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Workspace and custom development jj executable build pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All nine specification lanes pass, including full Rust tests, mapped spec-first, rustfmt and mdBook."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "Standalone server commands, durable operator provisioning/configuration, init and serve-local remain.",
      "Real executable RustFS deployment, attachment recovery, full CLI semantic parity and release platform acceptance remain gated.",
      "Loopback production RPC uses memory storage in these tests; no additional deployment-conformance claim."
    ],
    "versions": "jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; Rust 1.97.1; tonic/prost 0.14",
    "formats": "Additive native RPC V1 creation and typed token list/apply/status messages. No native, token-policy, WAL, manifest or receipt storage-format changes. Credential-bearing protobuf Debug is redacted and plaintext fields are zeroized on drop."
  },
  {
    "id": "E-M3-009",
    "tasks": ["M3", "M3.1", "M3.2", "M3.3"],
    "date": "2026-09-27",
    "kind": "standalone-native-server",
    "files": [
      "Cargo.lock",
      "Cargo.toml",
      "crates/jj-store-cli/Cargo.toml",
      "crates/jj-store-cli/src/commands.rs",
      "crates/jj-store-cli/src/credentials.rs",
      "crates/jj-store-cli/src/lib.rs",
      "crates/jj-store-cli/src/presentation.rs",
      "crates/jj-store-cli/src/server_commands.rs",
      "crates/jj-store-cli/src/server_main.rs",
      "crates/jj-store-cli/tests/fixtures/terminal.py",
      "crates/jj-store-cli/tests/server_executable.rs",
      "crates/jj-store-server/Cargo.toml",
      "crates/jj-store-server/src/config.rs",
      "crates/jj-store-server/src/lib.rs",
      "crates/jj-store-server/src/policy.rs",
      "crates/jj-store-server/src/process.rs",
      "crates/jj-store-server/tests/configuration.rs",
      "docs/cli/serve-local.md",
      "docs/cli/server-commands.md",
      "docs/cli/server-config.md",
      "docs/cli/server-doctor.md",
      "docs/cli/server-operator.md",
      "docs/cli/server-run.md",
      "docs/cli/server.md",
      "docs/evidence/m3-server-rustfs.json",
      "scripts/check-boundaries.mjs",
      "scripts/governance.test.mjs",
      "scripts/rustfs-contract.py",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/security.md",
      "spec/src/spec-governance.md",
      "spec-validator.config.mjs"
    ],
    "tests": [
      "crates/jj-store-server/tests/configuration.rs#configuration_is_bounded_strict_and_resolves_paths_without_creating_state",
      "crates/jj-store-server/tests/configuration.rs#insecure_shared_endpoints_credentials_and_checkout_control_paths_are_rejected",
      "crates/jj-store-server/tests/configuration.rs#operator_policy_persists_only_verifiers_and_reload_observes_revocation",
      "crates/jj-store-server/tests/configuration.rs#operator_updates_serialize_and_state_locks_release_with_the_owner",
      "crates/jj-store-server/tests/configuration.rs#configuration_and_policy_symlinks_cannot_bypass_private_path_rules",
      "crates/jj-store-server/tests/configuration.rs#programmatic_configuration_cannot_bypass_private_path_validation",
      "crates/jj-store-cli/tests/server_executable.rs#standalone_help_configuration_completions_and_build_info_are_consistent",
      "crates/jj-store-cli/tests/server_executable.rs#operator_commands_provision_list_and_revoke_without_exposing_credentials",
      "crates/jj-store-cli/tests/server_executable.rs#cold_server_and_serve_local_hold_state_lock_and_stop_on_termination",
      "crates/jj-store-cli/tests/server_executable.rs#server_rejects_missing_credentials_invalid_tls_and_shared_plaintext_before_listening",
      "crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation",
      "crates/jj-store-cli/tests/server_executable.rs#standalone_terminal_paging_color_and_json_contract",
      "crates/jj-store-cli/src/credentials.rs#competing_operator_provision_cannot_replace_the_first_credential",
      "scripts/governance.test.mjs#server_composition_cannot_depend_on_cli_or_native_upstream"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: package absent; zero cases executed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration",
        "result": "pass",
        "failures": 0,
        "note": "Initial five configuration cases pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration programmatic_configuration_cannot_bypass_private_path_validation",
        "result": "fail",
        "failures": 1,
        "note": "Regression showed directly constructed configuration could bypass private control-path checks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration programmatic_configuration_cannot_bypass_private_path_validation",
        "result": "pass",
        "failures": 0,
        "note": "Config::validate enforces the same boundary for file-loaded and programmatic configuration."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli",
        "result": "fail",
        "failures": 4,
        "note": "Four CommandError conversion diagnostics; followed by one remaining Ui conversion diagnostic in executable test compilation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable",
        "result": "fail",
        "failures": 1,
        "note": "Initial compilation after partial repair: one remaining Ui conversion diagnostic."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable",
        "result": "fail",
        "failures": 1,
        "note": "Three cases pass; serve-local fails because custom --config conflicts with upstream global String argument."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_server_and_serve_local_hold_state_lock_and_stop_on_termination",
        "result": "fail",
        "failures": 1,
        "note": "First targeted rerun still had the old --config fixture flag."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_server_and_serve_local_hold_state_lock_and_stop_on_termination",
        "result": "fail",
        "failures": 1,
        "note": "After flag fix, upstream cleanup_guard signal handler exits status 1. Diagnostic rerun reproduced one failure. Unix handoff to paired binary fixes lifecycle ownership."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_server_and_serve_local_hold_state_lock_and_stop_on_termination",
        "result": "pass",
        "failures": 0,
        "note": "Exact cold process/state lock/termination regression passes after Unix exec handoff."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output .scratch/m3-server-rustfs.json",
        "result": "fail",
        "failures": 1,
        "note": "Eight storage probes pass; one executable fixture fails: Invalid evolog template keyword description."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output .scratch/m3-server-rustfs-after.json",
        "result": "fail",
        "failures": 1,
        "note": "Eight storage probes pass; one executable fixture fails: File show path was relative to fixture cwd, outside selected repository."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output .scratch/m3-server-rustfs-final.json",
        "result": "pass",
        "failures": 0,
        "note": "All nine probes pass after correcting pinned evolog template and repository-relative file argument. Server/RustFS kill, empty process/index state, history, binary content and live revocation pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --lib competing_operator_provision_cannot_replace_the_first_credential",
        "result": "fail",
        "failures": 3,
        "note": "Regression before lock implementation: three unresolved function diagnostics, zero cases executed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --lib competing_operator_provision_cannot_replace_the_first_credential",
        "result": "pass",
        "failures": 0,
        "note": "A competing same-endpoint provision cannot reach OS credential mutation; lock releases after completion."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable standalone_terminal_paging_color_and_json_contract",
        "result": "pass",
        "failures": 0,
        "note": "Standalone executable passes owned PTY paging/color/no-pager/JSON/fallback checks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration",
        "result": "pass",
        "failures": 0,
        "note": "All six configuration/policy tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable",
        "result": "pass",
        "failures": 0,
        "note": "All five executable tests pass; real RustFS case ignored in this normal lane."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native",
        "result": "pass",
        "failures": 0,
        "note": "Six cases pass; two OS-facility tests ignored in normal lane. Shared terminal fixture remains compatible."
      },
      {
        "command": "node --test scripts/governance.test.mjs",
        "result": "pass",
        "failures": 0,
        "note": "15 cases pass, including new server composition boundary regression."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output docs/evidence/m3-server-rustfs.json",
        "result": "pass",
        "failures": 0,
        "note": "Nine real-storage probes pass on the final implementation. Immutable report retained; no cleanup errors."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "fail",
        "failures": 1,
        "note": "One Clippy collapsible-if diagnostic in server policy lock file validation. Flattened the conditional without changing behavior."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Specification, boundaries, progress, rustfmt and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "119 Rust tests, 15 governance tests and 3 storage-harness self-tests pass. Five external tests are ignored in the normal lane; server RustFS acceptance ran separately."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Workspace and both development executables build."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All nine lanes pass, including full Rust suite, rustfmt, mapped spec-first and mdBook."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Init and repository token CLI commands, recoverable attachment and complete ordinary-command semantic parity remain.",
      "Shared TLS deployment and non-Unix serve-local handoff qualification remain release gates."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; pinned RustFS digest in retained m3-server-rustfs report.",
    "formats": "New server TOML configuration V1. Existing native object, token policy, WAL, manifest, receipt and RPC V1 formats unchanged. Operator policy files contain verifiers only; credentials stay in OS facility or environment."
  },
  {
    "id": "E-M3-010",
    "tasks": ["M3", "M3.2", "M3.3"],
    "date": "2026-09-27",
    "kind": "resumable-native-attachment",
    "files": [
      "crates/jj-store-cli/Cargo.toml",
      "crates/jj-store-cli/src/attachment.rs",
      "crates/jj-store-cli/src/commands.rs",
      "crates/jj-store-cli/src/lib.rs",
      "crates/jj-store-cli/tests/attachment_recovery.rs",
      "docs/cli/attach.md",
      "docs/cli/commands.md",
      "docs/evidence/m3-attachment-rustfs.json",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/attachment_recovery.rs#accepted_attachment_retries_original_operation_after_lost_ack_and_later_publication",
      "crates/jj-store-cli/tests/attachment_recovery.rs#prepared_attachment_survives_cache_loss_and_rejects_changed_retry_identity",
      "crates/jj-store-cli/tests/attachment_recovery.rs#revoked_attachment_cannot_replay_receipt_or_expose_physical_metadata",
      "crates/jj-store-cli/tests/attachment_recovery.rs#recovery_preserves_unowned_physical_metadata_stage"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: test target absent; zero cases executed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli",
        "result": "fail",
        "failures": 1,
        "note": "Workspace-name comparison used RefSymbol against String; changed to WorkspaceName accessor."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery",
        "result": "fail",
        "failures": 1,
        "note": "Follow-up compile used a nonexistent RefSymbol::as_str; fixed to WorkspaceName::as_str."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery",
        "result": "fail",
        "failures": 2,
        "note": "Initial two cases lacked operation.hostname in their constructed test settings."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery",
        "result": "fail",
        "failures": 2,
        "note": "After hostname correction the same two cases lacked signing.behavior; the expanded four-case target reproduced four fixture failures. Fixed by using the upstream default settings stack."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery",
        "result": "fail",
        "failures": 4,
        "note": "Expanded target still used incomplete fixture settings before default-stack correction."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery",
        "result": "fail",
        "failures": 2,
        "note": "Two pass; two completed-retry paths fail because upstream new-workspace settings helper assumes .jj/repo is a directory. Completed retries now reuse settings without that initializer."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery",
        "result": "pass",
        "failures": 0,
        "note": "Four cases pass: original-operation receipt retry after later publication, cache-loss recovery and changed-intent/lock rejection, revoked receipt denial, and preservation of an unowned physical stage."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native --test administration_rpc --test server_executable",
        "result": "pass",
        "failures": 0,
        "note": "15 cases pass; three external cases ignored in this focused lane."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output docs/evidence/m3-attachment-rustfs.json",
        "result": "pass",
        "failures": 0,
        "note": "Nine pinned RustFS probes pass; actual server/CLI cold recovery and revocation retained. No cleanup errors."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "All-target Clippy, rustfmt, specification, boundaries and progress pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "123 Rust tests, 15 governance tests and 3 storage-harness self-tests pass. Five external tests ignored in normal lane."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Workspace and both development executables build."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All nine lanes pass including full Rust suite, mapped spec-first, rustfmt and mdBook."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Repository init and token administration CLI commands and complete semantic parity remain.",
      "Shared TLS/platform qualification remain."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; RustFS image retained in report.",
    "formats": "Introduces local secret-free attachment journal V1 holding credential ID, connection, paths and exact unpublished operation/parent IDs. Native object, authority, manifest, receipt and RPC formats unchanged."
  },
  {
    "id": "E-M3-011",
    "tasks": ["M3", "M3.1", "M3.2", "M3.3"],
    "date": "2026-09-27",
    "kind": "durable-cli-initialization",
    "files": [
      "crates/jj-store-cli/src/attachment.rs",
      "crates/jj-store-cli/src/commands.rs",
      "crates/jj-store-cli/src/credentials.rs",
      "crates/jj-store-cli/src/initialization.rs",
      "crates/jj-store-cli/src/lib.rs",
      "crates/jj-store-cli/tests/attachment_recovery.rs",
      "crates/jj-store-cli/tests/executable_native.rs",
      "crates/jj-store-cli/tests/server_executable.rs",
      "docs/cli/commands.md",
      "docs/cli/init.md",
      "docs/cli/store.md",
      "docs/evidence/m3-init-rustfs.json",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/security.md",
      "README.md",
      "spec/src/architecture.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/attachment_recovery.rs#executable_init_reuses_durable_identity_after_denial_and_lost_creation_ack",
      "crates/jj-store-cli/tests/attachment_recovery.rs#executable_init_rejects_changed_epoch_or_credential_and_preserves_existing_files",
      "crates/jj-store-cli/tests/executable_native.rs#os_credential_initial_admin_provisioning_is_repeatable",
      "crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation"
    ],
    "runs": [
      {
        "command": "target/debug/jj store init --help",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: unrecognized init subcommand; exit 2."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli",
        "result": "pass",
        "failures": 0,
        "note": "Initial implementation compiles."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery executable_init",
        "result": "pass",
        "failures": 0,
        "note": "Two executable cases pass: denied creation then same-ID retry, lost acknowledgement after operator revocation, existing source preservation, changed epoch/credential denial and native history after init."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test attachment_recovery",
        "result": "pass",
        "failures": 0,
        "note": "All six init/attachment cases pass after shared helper extraction."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output docs/evidence/m3-init-rustfs.json",
        "result": "pass",
        "failures": 0,
        "note": "Nine real-storage probes pass using actual jj store init followed by exact repeat, CLI edits, server/storage kill, cache deletion, history and revocation. No cleanup errors."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native os_credential_initial_admin_provisioning_is_repeatable -- --ignored",
        "result": "pass",
        "failures": 0,
        "note": "One real OS-facility case passes: random initial-admin account is generated once, reused, read and cleaned up."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "All-target Clippy, rustfmt, specification, boundaries and progress pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "125 Rust tests, 15 governance tests and 3 storage-harness self-tests pass. Six external cases remain ignored in the normal lane; real RustFS and initial-admin OS-facility acceptance ran separately."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Workspace and both development executables build."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All nine lanes pass including full Rust suite, rustfmt, mapped spec-first and mdBook."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Repository token administration CLI commands and complete ordinary-command semantic parity remain.",
      "Shared TLS/platform qualification remain."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; pinned RustFS image in retained report.",
    "formats": "New local secret-free creation journal V1 preserves generated repository ID, epoch, credential ID, connection and attachment paths. Initial credentials are durable separately in the OS facility or automation environment. Authority/object/manifest/receipt/RPC formats unchanged."
  },
  {
    "id": "E-M3-012",
    "tasks": ["M3", "M3.1", "M3.3"],
    "date": "2026-09-27",
    "kind": "typed-token-cli",
    "files": [
      "crates/jj-store-cli/src/commands.rs",
      "crates/jj-store-cli/src/lib.rs",
      "crates/jj-store-cli/src/server_commands.rs",
      "crates/jj-store-cli/src/token_commands.rs",
      "crates/jj-store-cli/tests/administration_rpc.rs",
      "crates/jj-store-cli/tests/server_executable.rs",
      "docs/cli/commands.md",
      "docs/cli/server-commands.md",
      "docs/cli/server-operator.md",
      "docs/cli/tokens.md",
      "docs/evidence/m3-token-cli-rustfs.json",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/security.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/administration_rpc.rs#executable_token_commands_share_receipts_and_never_expose_credentials",
      "crates/jj-store-cli/tests/administration_rpc.rs#executable_token_retry_rejects_changed_intent_stale_snapshot_and_wrong_authority",
      "crates/jj-store-cli/tests/administration_rpc.rs#executable_token_admin_rotation_and_bounded_stdin_preserve_request_identity",
      "crates/jj-store-cli/tests/administration_rpc.rs#executable_token_help_is_authored_for_both_families",
      "crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation"
    ],
    "runs": [
      {
        "command": "target/debug/jj store token --help",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: token subcommand absent; exit 2."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli",
        "result": "fail",
        "failures": 2,
        "note": "Initial compilation found missing Display implementations for WalPosition and upstream CommandError. Use the position value and a shared owned connection error boundary."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli",
        "result": "pass",
        "failures": 0,
        "note": "Shared command implementation compiles after the corrections."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc executable_token",
        "result": "pass",
        "failures": 0,
        "note": "Four new executable acceptance tests pass: both command families, stable/late receipts, stale and changed intents, permission/actor isolation, same-principal admin rotation, bounded stdin, private paths and authored help."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc --test server_executable",
        "result": "pass",
        "failures": 0,
        "note": "13 cases pass; one real-storage case ignored in this focused lane."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output docs/evidence/m3-token-cli-rustfs.json",
        "result": "pass",
        "failures": 0,
        "note": "Nine real RustFS probes pass. Actual CLI grant, standalone revoke, late grant replay and live access denial are verified after cold recovery. Owned resources cleaned without errors."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "All-target Clippy, rustfmt, boundaries, progress and specification pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "129 Rust tests, 15 governance tests and 3 storage-harness self-tests pass; six external cases ignored in the normal lane."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Both development binaries and the workspace build."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All nine lanes pass including Rust tests, rustfmt, mapped spec-first and mdBook."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "Complete ordinary-command semantic parity and shared TLS/platform qualification remain."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; pinned RustFS digest in report.",
    "formats": "Local token-request journal V1 contains connection, actor, request ID, exact snapshot and intent; grant secrets are represented only by public ID and domain-separated fingerprint. Stored authority and native RPC formats unchanged."
  },
  {
    "id": "E-M3-013",
    "tasks": ["M3", "M3.2", "M3.3"],
    "date": "2026-09-27",
    "kind": "upstream-executable-semantic-parity",
    "files": [
      "crates/jj-store-cli/src/commands.rs",
      "crates/jj-store-cli/tests/executable_native.rs",
      "docs/cli/capabilities.md",
      "spec/src/cli.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/executable_native.rs#native_cli_semantic_rebase_squash_split_abandon_and_undo_match_upstream",
      "crates/jj-store-cli/tests/executable_native.rs#native_cli_semantic_multiway_conflict_snapshot_resolution_matches_upstream",
      "crates/jj-store-cli/tests/executable_native.rs#native_cli_semantic_divergence_conflicted_bookmarks_and_operation_reconciliation",
      "crates/jj-store-cli/tests/executable_native.rs#native_cli_semantic_binary_symlink_executable_and_copied_files_match_upstream",
      "crates/jj-store-cli/tests/executable_native.rs#native_cli_semantic_ssh_sign_resign_and_unsign_preserve_verification",
      "crates/jj-store-cli/tests/executable_native.rs#native_cli_semantic_git_transport_and_gc_fail_without_publication"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native native_cli_semantic",
        "result": "pass",
        "failures": 0,
        "note": "Before new fixtures: zero matching cases. This baseline does not establish semantic acceptance."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native native_cli_semantic",
        "result": "fail",
        "failures": 2,
        "note": "Initial three cases: one pass, two fixture failures. Graph comparison included Git-only @git refs; abandon assertions assumed a selected revision that the revset did not match."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native native_cli_semantic",
        "result": "fail",
        "failures": 2,
        "note": "After local-bookmark comparison: multiway conflict passes; two selectors still resolve to no revisions."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native native_cli_semantic",
        "result": "fail",
        "failures": 2,
        "note": "Quoted description selectors still match nothing. Pinned 0.45.1 uses glob matching against full multiline descriptions, so selectors must use subject or an explicit substring pattern."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native native_cli_semantic",
        "result": "fail",
        "failures": 3,
        "note": "Expanded six cases: three pass; the two selector mistakes remain and chmod incorrectly used POSIX -x. Upstream requires n/normal."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native native_cli_semantic",
        "result": "pass",
        "failures": 0,
        "note": "All six scenarios pass after using first-line subject selectors and pinned chmod syntax. Comparisons include ordered parent descriptions/counts, local bookmarks, conflicts, file content, evolution and protected rejection. SSH signatures verify before and after re-signing."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "fail",
        "failures": 1,
        "note": "Executable suite: 11 passed / 1 failed / 3 ignored. Old rebase capability assertion still expected unqualified after direct parity acceptance."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 2,
        "note": "Executable suite: 10 passed / 2 failed / 3 ignored. Same old capability assertion plus ephemeral-port rebind collision during in-process service restart. Fixture now retains a socket reservation across service instances; standalone-process tests retain cold TCP acceptance."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test executable_native",
        "result": "pass",
        "failures": 0,
        "note": "All 12 normal executable cases pass, with 3 OS-credential cases explicitly ignored; includes all six semantic cases and both repaired fixtures."
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
        "note": "135 Rust cases pass / 6 external cases ignored; 15 governance and 3 storage-harness tests pass."
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
        "note": "All 9 lanes pass, including rustfmt, Rust tests, spec-first and mdBook."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "Standalone shared TLS acceptance and release platform qualification remain."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; current Unix host and disposable SSH Ed25519 keys.",
    "formats": "No persistence or protocol format changes. The JSON capability matrix now advertises commands with direct semantic acceptance. Reference-only Git tracking annotations are excluded from native/local bookmark comparisons."
  },
  {
    "id": "E-M3-014",
    "tasks": ["M3", "M3.1", "M3.2", "M3.3"],
    "date": "2026-09-27",
    "kind": "standalone-tls-deployment",
    "files": [
      "crates/jj-store-cli/tests/server_executable.rs",
      "scripts/rustfs-contract.py",
      "spec/src/cli.md",
      "spec/src/security.md",
      "spec/src/compatibility.md",
      "crates/jj-store-cli/src/commands.rs",
      "docs/cli/capabilities.md",
      "docs/evidence/m3-tls-server-rustfs.json",
      "docs/evidence/m3-gate-audit.md",
      "spec/src/verification.md",
      "spec/src/spec-governance.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/server_executable.rs#cold_tls_server_checks_trust_hostname_and_plaintext",
      "crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_checks_trust_hostname_and_plaintext",
        "result": "pass",
        "failures": 0,
        "note": "Before fixture: zero matching tests; no positive standalone TLS acceptance."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_checks_trust_hostname_and_plaintext",
        "result": "fail",
        "failures": 1,
        "note": "New negative plaintext assertion stopped at TCP/channel establishment. Tonic can return before the TLS peer rejects the HTTP/2 preface; fixture must drive an RPC and assert transport failure."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_checks_trust_hostname_and_plaintext",
        "result": "fail",
        "failures": 1,
        "note": "Plaintext RPC rejected as transport Cancelled rather than Unavailable. Both are valid pre-application connection failures; fixture accepts those two transport categories and excludes application authorization/errors."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_checks_trust_hostname_and_plaintext",
        "result": "pass",
        "failures": 0,
        "note": "1 passed. Explicit trust succeeds; untrusted certificate, wrong hostname and plaintext RPC are rejected. Fresh standalone server starts and terminates."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --server --output docs/evidence/m3-tls-server-rustfs.json",
        "result": "pass",
        "failures": 0,
        "note": "All 9 probes pass. Actual CLI init/attach, token administration and cache-free process/RustFS recovery use TLS with generated localhost certificate and disabled development override; owned resources removed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 1,
        "note": "Regular test lane passed, but repeated TLS negative test returned Unknown with a local tonic transport/h2 FRAME_SIZE_ERROR source. Numeric status depends on timing. Final fixture asserts a local tonic transport error source (not a decoded application status) instead."
      },
      {
        "command": "pnpm format:check",
        "result": "fail",
        "failures": 2,
        "note": "Prettier found line wrapping in canonical CLI and security chapters; Rust formatting passed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_checks_trust_hostname_and_plaintext",
        "result": "pass",
        "failures": 0,
        "note": "5 consecutive runs pass after asserting the local tonic transport error source, independently of numeric h2/gRPC status."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 1,
        "note": "All functional lanes and mdBook passed; mapped spec-first detected missing governance chapter update for the RustFS harness change. Added explicit TLS report/transport scope guidance."
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
        "note": "136 Rust cases pass / 6 external cases ignored; 15 governance and 3 storage-harness tests pass."
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
        "note": "All 9 lanes pass after the governance mapping update, including repeated full Rust suite, rustfmt, mdBook and spec-first."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; generated localhost TLS certificate; pinned RustFS image.",
    "formats": "No persistence, RPC or application format changes. M3 native CLI/server gate qualified on current Unix host and pinned single-node TLS topology; M4 bounded reads/HTTP/UI, M5 typed application mutations, M6 search and M8a deployment/platform release hardening remain open."
  },
  {
    "id": "E-M4-001",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "committed-read-snapshot-foundation",
    "files": [
      "crates/jj-store-service/src/reads.rs",
      "crates/jj-store-service/src/lib.rs",
      "crates/jj-store-cli/tests/native_lifecycle.rs",
      "spec/src/api.md",
      "spec/src/mutations.md",
      "spec/src/cli.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle.rs#read_session_pins_history_and_reports_divergent_heads_without_publication",
      "crates/jj-store-cli/tests/native_lifecycle.rs#read_session_excludes_staged_operations_and_rechecks_revocation",
      "crates/jj-store-cli/tests/native_lifecycle.rs#read_session_enforces_inventory_and_elapsed_budgets",
      "crates/jj-store-cli/tests/native_lifecycle.rs#read_session_rejects_replaced_epoch_without_rebinding"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle read_session_",
        "result": "pass",
        "failures": 0,
        "note": "Before implementation: zero matching cases, not acceptance evidence."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle read_session_",
        "result": "pass",
        "failures": 0,
        "note": "Initial 3 cases pass: exact historical selection, explicit sibling-head response, zero read publication, immutable membership, staged rejection, current revocation and resource budgets. Epoch fault case added next."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle read_session_",
        "result": "fail",
        "failures": 1,
        "note": "Epoch fault fixture failed compilation: storage condition variant is Matches, not Match. No tests executed in this run."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle read_session_",
        "result": "pass",
        "failures": 0,
        "note": "All 4 cases pass, including explicit epoch replacement rejection."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle read_session_pins_history_and_reports_divergent_heads_without_publication",
        "result": "fail",
        "failures": 1,
        "note": "Additional deadline regression: an immediately ready multi-head response escaped a one-nanosecond budget. Pinned Tokio polls the future before its timer. Guard now checks elapsed time before and after polling as well as using timeout_at."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle read_session_pins_history_and_reports_divergent_heads_without_publication",
        "result": "pass",
        "failures": 0,
        "note": "Exact immediate-ready deadline regression passes after guarding both elapsed boundaries."
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
        "note": "140 Rust tests pass / 6 external cases ignored, including all 4 read-session cases; 15 governance tests and 3 storage-harness self-tests pass."
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
        "note": "All 9 lanes pass including repeated full Rust suite, rustfmt, mdBook and mapped spec-first."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "M4.1 still needs native graph/index sessions, bounded revsets, objects/files/diffs and signed snapshot cursors; M4.2 retains HTTP, SSE, generated contracts and UI."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; memory-store native transactions (not deployment conformance).",
    "formats": "Owned read snapshot DTO V1 records repo/epoch, manifest revision, publication position, heads and selected operation. It is informational only; the read capability has private construction and no mutation API. No authority or native RPC format changes."
  },
  {
    "id": "E-M4-002",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "native-three-graph-query-foundation",
    "files": [
      "Cargo.lock",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs",
      "crates/jj-store-cli/tests/native_lifecycle.rs",
      "crates/jj-store-jj/Cargo.toml",
      "crates/jj-store-jj/src/lib.rs",
      "crates/jj-store-jj/src/query/access.rs",
      "crates/jj-store-jj/src/query/graphs.rs",
      "crates/jj-store-jj/src/query/mod.rs",
      "crates/jj-store-service/src/native.rs",
      "crates/jj-store-service/src/reads.rs",
      "spec/src/api.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/jj-integration.md",
      "spec/src/mutations.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_three_histories_filtered_edges_and_snapshot_pagination",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_divergent_operations_bookmarks_and_explicit_workspace",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_rebuild_limits_staged_rejection_and_current_revocation",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_marks_absent_evolution_data_and_rejects_missing_continuations"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_",
        "result": "pass",
        "failures": 0,
        "note": "Before implementation: zero matching cases, not acceptance evidence."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_",
        "result": "fail",
        "failures": 1,
        "note": "Initial 3 cases: 2 pass / 1 fixture assertion failure. The native symbol resolver rejects a staged commit ID as Invalid before reading it; explicit evolution entry rejects the same staged ID as NotCommitted. Assertions now retain both distinctions."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_",
        "result": "fail",
        "failures": 1,
        "note": "Expanded fixture compile failure: native View.head_ids is a HashSet rather than BTreeSet. One unused-import warning also removed. No cases ran."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_",
        "result": "pass",
        "failures": 0,
        "note": "All 5 graph cases pass: explicit pinned history, filtered graph edges, hidden predecessors, divergent bookmarks, cache deletion/rebuild, limits, current revocation, worker cancellation and authorization deadlines."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-cli --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_",
        "result": "fail",
        "failures": 3,
        "note": "Added full operation-metadata assertions expose 3 missing DTO fields at compilation: is_snapshot, workspace and attributes. Fields are now mapped from native operation metadata. No cases ran."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_",
        "result": "pass",
        "failures": 0,
        "note": "All 5 cases pass after metadata mapping, including corrupt cache backend rejection and explicit cache removal/rebuild."
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
        "note": "145 Rust tests pass / 6 external cases ignored; 15 governance and 3 storage-harness tests pass."
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
        "note": "All 9 lanes pass, including the repeated full Rust suite, rustfmt, mdBook and mapped spec-first."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "M4 retains signed snapshot-bound transport cursors, native object/tree/file/diff and operation-diff queries, HTTP/SSE/generated contracts and browser acceptance.",
      "Current index-size, revset admission, I/O, output and worker limits are tested; synchronous native CPU algorithms are not preemptible by the async deadline. Adversarial CPU/memory qualification, index corruption recovery and large-index rebuild tasks/status remain M4 work. Per-operation cache retention/eviction requires operational qualification."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; in-memory store and native transactions, not deployment conformance.",
    "formats": "Owned graph DTO V1: native commit, operation and evolution nodes with ordered parents, filtered edges, annotations and absent-versus-empty predecessor metadata. Internal continuation is the last native node ID; signing/request binding remains separate. Cache key includes upstream/profile/repository/epoch/operation. No native persistence or RPC format change."
  },
  {
    "id": "E-M4-003",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "signed-snapshot-bound-graph-continuations",
    "files": [
      "Cargo.lock",
      "crates/jj-store-cli/tests/native_lifecycle/pagination.rs",
      "crates/jj-store-cli/tests/native_lifecycle.rs",
      "crates/jj-store-service/Cargo.toml",
      "crates/jj-store-service/src/lib.rs",
      "crates/jj-store-service/src/pagination.rs",
      "crates/jj-store-service/src/reads.rs",
      "spec/src/api.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/mutations.md",
      "spec/src/security.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/pagination.rs#graph_cursor_retains_exact_snapshot_through_publication_checkpoint_and_replay",
      "crates/jj-store-cli/tests/native_lifecycle/pagination.rs#graph_cursor_rejects_tampering_changed_context_and_current_revocation",
      "crates/jj-store-cli/tests/native_lifecycle/pagination.rs#graph_cursor_expiry_restart_capacity_and_epoch_changes_require_reselection",
      "crates/jj-store-cli/tests/native_lifecycle/pagination.rs#graph_cursor_renews_request_deadline_keeps_head_selection_explicit_and_pages_evolution",
      "crates/jj-store-service/src/pagination.rs#graph_cursor_rejects_immediately_ready_results_after_deadline"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cursor_",
        "result": "pass",
        "failures": 0,
        "note": "Before implementation: zero matching cases, not acceptance evidence."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cursor_",
        "result": "fail",
        "failures": 2,
        "note": "Fixture compilation produced 2 errors from one StagedUpload::from_bytes call: missing byte limit and missing await. No tests ran."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cursor_",
        "result": "pass",
        "failures": 0,
        "note": "Initial 3 native pagination cases pass: snapshot replay across publications/checkpoint, changed-input/signature rejection, current revocation, expiry, registry budgets, restart and epoch replacement."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli -p jj-store-service graph_cursor_",
        "result": "pass",
        "failures": 0,
        "note": "Expanded 4 native cases plus 1 deadline unit case pass. Adds valid cross-repository credential rejection, all log-context bindings, explicit sibling head selection, terminal pages consuming no pin slot, evolution pagination and new request deadlines for retained snapshots."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cursor_",
        "result": "pass",
        "failures": 0,
        "note": "Exact narrow target rerun: all 4 native cursor cases pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Includes rustfmt, Clippy, progress and boundary checks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "150 Rust tests pass / 6 external cases ignored; 15 governance and 3 storage-harness tests pass."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rust and Prettier both pass."
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
        "note": "All 9 lanes pass, including the repeated full Rust suite, rustfmt, progress/boundary checks, mdBook and mapped spec-first."
      }
    ],
    "result": "pending",
    "remaining": [
      "Cursor service is not yet exposed through HTTP or browser routes. M4 still needs native objects/files/trees/diffs, operation diffs, HTTP/SSE/contracts/browser and production query CPU/index/cache qualification.",
      "Cursor pins expire and reset after restart. A future multi-instance deployment requires sticky routing or a separately qualified shared pin design."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; HMAC-SHA256 and base64url; native in-memory transaction fixtures, not deployment evidence.",
    "formats": "Application graph cursor V1 binds process instance, pin, repo/epoch, op, publication position/revision, native sort profile, full query context, page size and last node. Registry holds immutable snapshot/inventory pins without credentials; current authorization and epoch are checked on resume. No native object, authority or RPC format change."
  },
  {
    "id": "E-M4-004",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "native-object-tree-and-verified-content-reads",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/content.rs",
      "crates/jj-store-cli/tests/native_lifecycle.rs",
      "crates/jj-store-jj/src/codec.rs",
      "crates/jj-store-jj/src/lib.rs",
      "crates/jj-store-jj/src/native_dto.rs",
      "crates/jj-store-jj/src/query/content/trees.rs",
      "crates/jj-store-jj/src/query/content.rs",
      "crates/jj-store-jj/src/query/mod.rs",
      "crates/jj-store-object/src/staging.rs",
      "crates/jj-store-object/tests/contract.rs",
      "crates/jj-store-service/src/reads.rs",
      "spec/src/api.md",
      "spec/src/cli.md",
      "spec/src/identity.md",
      "spec/src/jj-integration.md",
      "spec/src/mutations.md",
      "spec/src/object-store.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_preserves_every_native_object_field_and_virtual_roots",
      "crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_lists_native_directory_entries_with_paths_and_pagination",
      "crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_verifies_binary_ranges_before_streaming_and_retains_permits",
      "crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_rechecks_revocation_and_limits_and_rejects_staged_objects",
      "crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_rejects_corruption_outside_requested_range_and_missing_committed_objects",
      "crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_stream_deadline_and_merged_tree_conflicts_preserve_native_terms",
      "crates/jj-store-cli/tests/native_lifecycle/content.rs#content_read_canceled_chunk_cannot_resume_with_skipped_bytes",
      "crates/jj-store-object/tests/contract.rs#private_spool_ranges_seek_and_end_at_exact_boundaries",
      "crates/jj-store-jj/tests/native_codec.rs#all_native_fields_roundtrip_without_identity_loss"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_",
        "result": "pass",
        "failures": 0,
        "note": "Before implementation: zero matching cases, not acceptance evidence."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli",
        "result": "fail",
        "failures": 1,
        "note": "Compile error: ConflictLabels does not implement From<Merge<String>>. Used the pinned Commit::tree() facade; removed one obsolete Deserialize import warning."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-cli",
        "result": "fail",
        "failures": 1,
        "note": "Compile error: native merged value borrowed the current tree across reassignment. Dropped the inspected value before loading the next directory."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_",
        "result": "fail",
        "failures": 2,
        "note": "Fixture compilation produced 2 type-inference diagnostics for redundant Into on a generic path constructor; no cases ran. Imports reserved for expanded fixtures initially emitted 3 warnings."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_",
        "result": "fail",
        "failures": 3,
        "note": "All 3 fixtures were denied publication because they named protected root as a legacy predecessor. The fixture now stages a real invisible predecessor; production root protection is unchanged."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_",
        "result": "fail",
        "failures": 1,
        "note": "2 cases pass / 1 path-validation failure: upstream internal RepoPath permits parent components as object names. Application traversal now explicitly rejects dot, parent and NUL components without normalization."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_",
        "result": "pass",
        "failures": 0,
        "note": "Expanded 6 cases pass: all native metadata and roots, binary ranges and permits, native five-term merge/deletion/directory semantics, staged/later-publication rejection, limits, deadlines, current revocation, corruption beyond the requested range and missing committed history."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_",
        "result": "pass",
        "failures": 0,
        "note": "All 7 cases pass including cancellation after chunk I/O but before final authorization. Reusing a canceled stream fails explicitly rather than skipping bytes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-object --test contract private_spool_",
        "result": "pass",
        "failures": 0,
        "note": "Before range fixture: zero matching cases, not acceptance evidence."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-object --test contract private_spool_",
        "result": "pass",
        "failures": 0,
        "note": "1 range fixture passes, covering first/last bytes, 64 KiB crossings, whole file, exact EOF and invalid ranges."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_codec",
        "result": "pass",
        "failures": 0,
        "note": "All 8 codec fixtures pass, including frozen V1 envelopes and upstream identities."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "57 requirements, boundaries, progress, rustfmt and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "158 Rust tests pass; 6 external acceptance cases intentionally ignored by the default lane. 15 governance and 3 storage-harness tests pass."
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
        "note": "All 9 lanes pass, including rustfmt, repeated Rust tests, mdBook and mapped spec-first checks."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "M4 retains signed directory continuations, structured diffs/conflict descriptors and operation diffs, HTTP JSON/media/SSE, generated contracts and browser integration.",
      "Production query CPU/memory/index/cache qualification and cold rebuild task/status remain open. Current content bounds are library acceptance evidence, not a deployment performance claim."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 / 7c41cdeb16b6b321c64e789a966b6adf723816a5; production native/service code over disposable in-memory storage, not RustFS conformance.",
    "formats": "Owned native metadata V1 schemas exported without changing persisted field order, payload bytes or native identity. Object wrappers carry version/profile/key; directory pages expose native terms and internal name continuations. Files use complete native/envelope verification into bounded private spools and authorized 64 KiB chunks; ranges seek within verified spools. No authority or native RPC format change."
  },
  {
    "id": "E-M4-005",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "signed-directory-snapshot-pagination",
    "files": [
      "crates/jj-store-service/src/pagination.rs",
      "crates/jj-store-service/src/pagination/cursors.rs",
      "crates/jj-store-service/src/pagination/directory.rs",
      "crates/jj-store-cli/tests/native_lifecycle/content.rs",
      "crates/jj-store-cli/tests/native_lifecycle/directory.rs",
      "spec/src/api.md",
      "spec/src/mutations.md",
      "spec/src/cli.md",
      "spec/src/security.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/directory.rs#directory_cursor_retains_snapshot_through_writes_checkpoint_and_retries",
      "crates/jj-store-cli/tests/native_lifecycle/directory.rs#directory_cursor_binds_all_inputs_and_rechecks_revocation_before_lookup",
      "crates/jj-store-cli/tests/native_lifecycle/directory.rs#directory_cursor_shares_graph_capacity_and_rejects_cross_query_restart_and_expiry",
      "crates/jj-store-cli/tests/native_lifecycle/directory.rs#directory_cursor_keeps_head_selection_explicit_and_rejects_invalid_paths",
      "crates/jj-store-cli/tests/native_lifecycle/directory.rs#directory_cursor_oversized_name_fails_without_consuming_snapshot_capacity",
      "crates/jj-store-cli/tests/native_lifecycle/pagination.rs#graph_cursor_rejects_tampering_changed_context_and_current_revocation"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle directory_cursor_",
        "result": "pass",
        "failures": 0,
        "note": "Before implementation: zero matching cases, not acceptance evidence."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service",
        "result": "fail",
        "failures": 1,
        "note": "Compile diagnostic: GraphExecutor has no Debug implementation. The graph pager now explicitly formats only its redacted cursor registry."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle directory_cursor_",
        "result": "pass",
        "failures": 0,
        "note": "First 4 cases pass: exact snapshot and retry through writes/checkpoint, input tampering and revocation, shared graph capacity and cross-query rejection, expiry/restart/epoch, explicit multi-head selection and invalid paths."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle directory_cursor_",
        "result": "pass",
        "failures": 0,
        "note": "All 5 cases pass including oversized continuation rejection without leaked snapshot capacity."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cursor_",
        "result": "pass",
        "failures": 0,
        "note": "All 4 existing graph cursor regressions pass after shared registry extraction."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "57 requirements, boundaries, ledger, rustfmt and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "163 Rust tests pass / 6 external cases intentionally ignored. 15 governance and 3 storage-harness tests pass."
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
        "note": "All 9 lanes pass including 163 Rust tests, rustfmt, mdBook and spec-first."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "Structured diffs/conflict descriptors and operation diffs, resource qualification, HTTP JSON/media/SSE, generated contracts and browser integration remain M4 work."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1. Production service/native queries over disposable in-memory stores, not RustFS or browser evidence.",
    "formats": "Shared process-local V1 read cursor prefix jsr1 and domain bind traversal kind, snapshot, query, page size and typed continuation. Replaces unreleased jsg1 graph tokens; retained pins remain ephemeral across restarts. No native object, authority or RPC format change."
  },
  {
    "id": "E-M4-006",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "native-operation-view-and-visibility-differences",
    "files": [
      "crates/jj-store-jj/src/query/content/operations.rs",
      "crates/jj-store-jj/src/query/content.rs",
      "crates/jj-store-service/src/reads.rs",
      "crates/jj-store-cli/tests/native_lifecycle/operation_diff.rs",
      "crates/jj-store-cli/tests/native_lifecycle/content.rs",
      "spec/src/api.md",
      "spec/src/jj-integration.md",
      "spec/src/cli.md",
      "spec/src/mutations.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/operation_diff.rs#operation_diff_preserves_every_view_field_and_excludes_hidden_predecessors",
      "crates/jj-store-cli/tests/native_lifecycle/operation_diff.rs#operation_diff_matches_native_revsets_after_rewrite_and_abandon",
      "crates/jj-store-cli/tests/native_lifecycle/operation_diff.rs#operation_diff_merge_parent_selection_and_root_are_explicit",
      "crates/jj-store-cli/tests/native_lifecycle/operation_diff.rs#operation_diff_rejects_limits_staged_later_and_revoked_reads",
      "crates/jj-store-cli/tests/native_lifecycle/operation_diff.rs#operation_diff_rejects_missing_and_corrupt_visible_history"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle operation_diff_",
        "result": "pass",
        "failures": 0,
        "note": "Before implementation: zero matching cases, not acceptance evidence."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service",
        "result": "fail",
        "failures": 1,
        "note": "Compile diagnostic: upstream ObjectId trait needed for CommitId::as_bytes. Added the trait import at the JJ integration boundary."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle operation_diff_",
        "result": "fail",
        "failures": 1,
        "note": "Fixture compile diagnostic: pinned Revset exposes an async stream, not iter(). The upstream parity fixture now collects that native stream."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle operation_diff_",
        "result": "pass",
        "failures": 0,
        "note": "Initial 5 cases pass: full-field deltas, native revset parity, parent selection, budgets, membership, revocation, missing/corrupt commit history."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle operation_diff_",
        "result": "pass",
        "failures": 0,
        "note": "Expanded 5 cases pass: native child creation distinguishes head changes from visibility, parent abandonment rebases descendants and bookmark/workspace mappings, and a merge yields distinct differences for its two parents."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "57 requirements, boundaries, progress, rustfmt and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "168 Rust tests pass / 6 external cases intentionally ignored. 15 governance and 3 storage-harness tests pass."
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
        "note": "All 9 lanes pass, including 168 Rust tests, rustfmt, mdBook and mapped spec-first checks."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0
      }
    ],
    "result": "pending",
    "remaining": [
      "Structured content diffs/conflict descriptors, query resource qualification, HTTP JSON/media/SSE, generated contracts and browser integration remain M4 work."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1. Native transactions and production reads over disposable in-memory stores; no deployment/performance claim.",
    "formats": "Version 1 operation-diff DTO distinguishes root/parent-selection/difference, complete view-field deltas, deterministic native-ID visibility deltas and absent-versus-empty predecessor maps. No authority, persistence or native RPC format change."
  },
  {
    "id": "E-M4-007",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "native-content-diffs-and-conflict-descriptors",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/conflicts.rs",
      "crates/jj-store-cli/tests/native_lifecycle/content.rs",
      "crates/jj-store-cli/tests/native_lifecycle/diff.rs",
      "crates/jj-store-jj/src/copy_query.rs",
      "crates/jj-store-jj/src/query/content/conflicts.rs",
      "crates/jj-store-jj/src/query/content/diff.rs",
      "crates/jj-store-jj/src/query/content/diff_copies.rs",
      "crates/jj-store-jj/src/query/content/native.rs",
      "crates/jj-store-jj/src/query/content/trees.rs",
      "crates/jj-store-jj/src/query/content.rs",
      "crates/jj-store-jj/tests/native_backend.rs",
      "crates/jj-store-model/src/ids.rs",
      "crates/jj-store-service/src/reads.rs",
      "spec/src/api.md",
      "spec/src/cli.md",
      "spec/src/identity.md",
      "spec/src/jj-integration.md",
      "spec/src/mutations.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/diff.rs#content_diff_native_lines_modes_binary_symlinks_submodules_and_prefixes",
      "crates/jj-store-cli/tests/native_lifecycle/diff.rs#content_diff_copy_and_rename_sources_match_pinned_native_stream",
      "crates/jj-store-cli/tests/native_lifecycle/diff.rs#content_diff_bounds_stale_membership_corruption_and_revocation",
      "crates/jj-store-cli/tests/native_lifecycle/diff.rs#content_diff_multiple_copy_lineages_are_explicit_without_guessing_one_source",
      "crates/jj-store-cli/tests/native_lifecycle/diff.rs#content_diff_canceled_copy_terms_cannot_reappear_as_sources",
      "crates/jj-store-cli/tests/native_lifecycle/conflicts.rs#conflict_descriptor_preserves_native_terms_labels_paths_and_editor_capabilities",
      "crates/jj-store-cli/tests/native_lifecycle/conflicts.rs#conflict_descriptor_multisided_and_fingerprints_bind_every_input",
      "crates/jj-store-cli/tests/native_lifecycle/conflicts.rs#conflict_descriptor_limits_revocation_and_verified_content_fail_closed",
      "crates/jj-store-jj/src/query/content/conflicts.rs#conflict_fingerprint_v1_matches_independent_canonical_vector",
      "crates/jj-store-jj/tests/native_backend.rs#copy_placeholder_parents_are_boundaries_not_stored_history"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_",
        "result": "pass",
        "failures": 0,
        "note": "Before implementation: zero matching cases; not acceptance evidence."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_descriptor_",
        "result": "pass",
        "failures": 0,
        "note": "Before implementation: zero matching cases; not acceptance evidence."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service",
        "result": "fail",
        "failures": 1,
        "note": "Label mapping returned Option<&str> instead of owned Option<String>; fixed explicit dereference and owned conversion."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "cargo fmt --all",
        "result": "fail",
        "failures": 1,
        "note": "New fixture module declared before its file was created. Created the file before rerunning."
      },
      {
        "command": "cargo fmt --all",
        "result": "pass",
        "failures": 0
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_",
        "result": "fail",
        "failures": 5,
        "note": "Fixture compile diagnostics: generic collect and ambiguous BStr::as_ref; fixed explicit Vec and byte-slice types."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_",
        "result": "fail",
        "failures": 3,
        "note": "First executable run: 0 passed / 3 failed with Unavailable."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_native_lines -- --nocapture",
        "result": "fail",
        "failures": 1,
        "note": "Pinned experimental copy-history diff helper rejects valid placeholder copy IDs. Replaced with raw native tree deltas plus bounded owned lineage annotations; no synthetic history objects."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend copy_placeholder_parents_are_boundaries_not_stored_history",
        "result": "fail",
        "failures": 2,
        "note": "Initial fixture compile diagnostics: missing helper label argument."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend copy_placeholder_parents_are_boundaries_not_stored_history",
        "result": "fail",
        "failures": 1,
        "note": "Backend tried to read a placeholder parent as a stored object. Keep placeholder parents in returned histories while stopping traversal at these boundaries."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend copy_placeholder_parents_are_boundaries_not_stored_history",
        "result": "pass",
        "failures": 0,
        "note": "1 case passes; direct placeholder object lookup still explicitly errors."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_descriptor_",
        "result": "pass",
        "failures": 0,
        "note": "Initial 3 descriptor cases pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_",
        "result": "pass",
        "failures": 0,
        "note": "Initial 3 diff cases pass after placeholder handling and lineage lookup changes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_canceled_copy_terms_",
        "result": "fail",
        "failures": 1,
        "note": "Canceled positive tree term incorrectly reappeared as a copy source. Lookup now simplifies signed terms before testing source membership and charges quadratic comparison work."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_canceled_copy_terms_",
        "result": "pass",
        "failures": 0,
        "note": "Exact regression passes: canceled signed term cannot become a source."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_",
        "result": "fail",
        "failures": 1,
        "note": "Expanded cases: 4 passed / 1 failed because expected sorted path list omitted the newly added non-UTF8 fixture. Added its expected path; preserved Binary assertion."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_diff_",
        "result": "pass",
        "failures": 0,
        "note": "All 5 cases pass: native line parity, Unicode/CRLF/no-final-newline, empty/non-UTF8/binary, modes/symlinks/submodules, copy/rename and reverse parity, multiple lineages, canceled terms, limits, membership, corruption and revocation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_descriptor_",
        "result": "pass",
        "failures": 0,
        "note": "All 3 cases pass: native term/label parity, typed editor capabilities, 5-term merges, fingerprint binding, limits/revocation and verified content failure."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj conflict_fingerprint_v1_matches_independent_canonical_vector",
        "result": "pass",
        "failures": 0,
        "note": "1 golden case passes against independent Python canonical JSON/hashlib BLAKE2b digest."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj --test native_backend",
        "result": "pass",
        "failures": 0,
        "note": "All 12 backend cases pass after shared copy-query changes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle content_read_",
        "result": "pass",
        "failures": 0,
        "note": "All 7 existing content-read cases pass after shared native helper extraction."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "57 requirements, boundaries, ledger, rustfmt and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "178 Rust tests pass / 6 external cases intentionally ignored. 15 governance and 3 storage-harness tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Locked workspace build passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 9 lanes pass, including 178 Rust tests, rustfmt, mdBook and mapped spec-first checks."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Production query CPU/memory/index/cache qualification, HTTP JSON/media/SSE, generated contracts and browser integration remain M4 work. Typed conflict resolution is M5."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5. Native production reads over disposable in-memory storage; no deployment or performance claim.",
    "formats": "V1 structural diff DTO retains raw target terms separately from complete source annotations and optional native line hunks. V1 conflict descriptor carries ordered labeled terms, explicit editor support and a domain-separated ConflictFingerprint. No native object, authority or RPC persistence format change."
  },
  {
    "id": "E-M4-008",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "bounded-native-query-cache-retention-and-recovery",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs",
      "crates/jj-store-jj/src/factories.rs",
      "crates/jj-store-jj/src/query/cache.rs",
      "crates/jj-store-jj/src/query/mod.rs",
      "spec/src/api.md",
      "spec/src/cli.md",
      "spec/src/jj-integration.md",
      "crates/jj-store-cli/tests/administration_rpc.rs"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_corrupt_operation_link_rebuilds_without_publication",
      "crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_checksums_missing_files_and_receipts_rebuild_exact_snapshots",
      "crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_retention_evicts_lru_and_bounds_bytes_files_and_slots",
      "crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_recovery_rechecks_permission_and_serializes_independent_executors",
      "crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_recovery_never_follows_symlinks_or_repairs_authoritative_objects",
      "crates/jj-store-cli/tests/native_lifecycle/graph_cache.rs#graph_cache_bindings_separate_repository_and_epoch_for_identical_root_ids",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_three_histories_filtered_edges_and_snapshot_pagination",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_divergent_operations_bookmarks_and_explicit_workspace",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_rebuild_limits_staged_rejection_and_current_revocation",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_marks_absent_evolution_data_and_rejects_missing_continuations",
      "crates/jj-store-cli/tests/administration_rpc.rs#rpc_fixture_reserves_listener_across_restarts",
      "crates/jj-store-cli/tests/administration_rpc.rs#rpc_token_retries_and_competing_snapshots_preserve_accepted_policy"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cache_",
        "result": "pass",
        "failures": 0,
        "note": "Before new fixture: zero matching cases; not acceptance evidence."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cache_corrupt_operation_link_rebuilds_without_publication",
        "result": "fail",
        "failures": 1,
        "note": "Original regression fails: malformed native index operation link returns Unavailable instead of rebuilding the derived cache."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service",
        "result": "fail",
        "failures": 16,
        "note": "Initial cache module missed its std::fs import after removing the parent import; 16 diagnostics."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service",
        "result": "pass",
        "failures": 0,
        "note": "Service compile passes after local import correction."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cache_corrupt_operation_link_rebuilds_without_publication",
        "result": "pass",
        "failures": 0,
        "note": "Exact regression passes after receipt inventory and derived-file reconstruction."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_",
        "result": "pass",
        "failures": 0,
        "note": "All 5 original graph acceptance cases pass under bounded slot caching."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cache_",
        "result": "fail",
        "failures": 2,
        "note": "Two fixture compile diagnostics: unavailable NativeObjects::scope trait and nonexistent MemoryStore::corrupt helper. Fixture now uses ObjectKey::storage_path and a checked conditional store replacement."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cache_",
        "result": "pass",
        "failures": 0,
        "note": "Initial 5 cache cases pass: malformed links, checksum/missing-file/receipt/index-type damage, LRU/byte/file/slot limits, revocation, independent executors, interrupted-build cleanup, symlink isolation and authoritative-object errors."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cache_",
        "result": "pass",
        "failures": 0,
        "note": "Final 6 cases pass, including separate repository/epoch bindings for identical virtual roots and atomic local receipts."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_",
        "result": "pass",
        "failures": 0,
        "note": "All 5 existing graph cases pass, including retained namespace lock across caller cancellation until worker exit."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "57 requirements, boundaries, ledger, rustfmt and all-target Clippy pass before shared RPC fixture repair."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "fail",
        "failures": 1,
        "note": "Governance/storage self-tests pass. Administration RPC target: 7 passed / 1 failed. rpc_token_retries_and_competing_snapshots_preserve_accepted_policy hit AddrInUse at restart because the fixture released its listener reservation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc rpc_fixture_reserves_listener_across_restarts",
        "result": "fail",
        "failures": 1,
        "note": "Deterministic source regression proves another listener can acquire a stopped fixture endpoint. The fixture now owns one nonblocking listener and restarts using cloned handles."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc rpc_fixture_reserves_listener_across_restarts",
        "result": "pass",
        "failures": 0,
        "note": "Exact listener-reservation regression passes with retained socket ownership."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc rpc_token_retries_and_competing_snapshots_preserve_accepted_policy",
        "result": "pass",
        "failures": 0,
        "note": "Original failing retry/competing-snapshot case passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test administration_rpc",
        "result": "pass",
        "failures": 0,
        "note": "All 9 cases pass after the shared fixture change."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "57 requirements, boundaries, ledger, rustfmt and all-target Clippy pass after final changes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "185 Rust tests pass / 6 external cases intentionally ignored. 15 governance and 3 storage-harness tests pass; no unresolved failures."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Locked workspace build passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 9 lanes pass, including 185 Rust tests, rustfmt, mdBook and mapped spec-first checks."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Production CPU/memory/transient-build resource qualification and long rebuild task/status, then HTTP JSON/media/SSE, generated contracts and browser integration."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5. Native queries over disposable in-memory stores; no deployment/performance claim.",
    "formats": "Local query-cache-v1 slots replace unreleased per-operation paths under the pinned revision/profile. Receipts bind repository/epoch/operation and checksummed file inventories. Additive V1 graph cache diagnostics report reuse/build/rebuild, evictions and retained bytes/counts. No authority, native object or native RPC format change."
  },
  {
    "id": "E-M4-009",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "native-query-cancellation-and-release-resource-qualification",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs",
      "crates/jj-store-cli/tests/native_lifecycle/query_resources.rs",
      "crates/jj-store-jj/examples/query_qualification.rs",
      "crates/jj-store-jj/src/query/access.rs",
      "crates/jj-store-jj/src/query/content.rs",
      "crates/jj-store-jj/src/query/mod.rs",
      "crates/jj-store-jj/src/runtime.rs",
      "docs/evidence/m4-native-query-audit.md",
      "docs/evidence/m4-native-query-budget-before.json",
      "docs/evidence/m4-native-query-budget.json",
      "scripts/qualify-query-resources.py",
      "spec/src/api.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/jj-integration.md",
      "spec/src/spec-governance.md",
      "docs/evidence/m4-runtime-rustfs.json"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/query_resources.rs#query_cancel_drops_pending_graph_authorization_and_releases_cache",
      "crates/jj-store-cli/tests/native_lifecycle/query_resources.rs#query_cancel_drops_pending_content_authorization_and_releases_permit",
      "crates/jj-store-cli/tests/native_lifecycle/query_resources.rs#query_cancel_drops_remote_object_bodies_for_graphs_and_content",
      "crates/jj-store-jj/src/runtime.rs#native_runtime_future_progress_does_not_require_foreign_executor_cooperative_reset",
      "crates/jj-store-jj/src/runtime.rs#native_runtime_stream_progress_does_not_require_foreign_executor_cooperative_reset",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits",
      "crates/jj-store-jj/examples/query_qualification.rs#main",
      "crates/jj-store-jj/tests/rustfs.rs#real_rustfs_transport_and_native_blob_contract",
      "crates/jj-store-jj/tests/rustfs.rs#real_rustfs_manifest_publication_and_cold_recovery",
      "crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_cancel_",
        "result": "fail",
        "failures": 2,
        "note": "Before: 0 passed / 2 failed. Detached graph/content workers retained permits past the one-second cancellation check and produced secondary runtime-shutdown panics."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_cancel_",
        "result": "pass",
        "failures": 0,
        "note": "After request-drop cancellation: original 2 cases pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits",
        "result": "fail",
        "failures": 1,
        "note": "The old async-pause fixture expected Busy after the worker had correctly exited. Replaced only its first pause with a synchronous gate, retaining the exact permit/cache ownership assertion and independent deadline case."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_cancel_",
        "result": "fail",
        "failures": 1,
        "note": "One fixture compile diagnostic: non-binding synchronization guard. Bound the guard within its synchronous scope."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_cancel_",
        "result": "pass",
        "failures": 0,
        "note": "All 3 cases pass, including cancellation dropping actual pending remote response bodies. Temporary unused-guard warning subsequently corrected."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits",
        "result": "pass",
        "failures": 0,
        "note": "Exact permit/cache/deadline case passes with deterministic synchronous work."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo run --release --locked -p jj-store-jj --example query_qualification -- 50000",
        "result": "fail",
        "failures": 1,
        "note": "Before benchmark creation: example target absent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example query_qualification",
        "result": "fail",
        "failures": 1,
        "note": "One fixture compile diagnostic: native View head_ids is a HashSet, not BTreeSet. Fixed explicit collection; removed unused import."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example query_qualification",
        "result": "pass",
        "failures": 0,
        "note": "Release fixture compiles."
      },
      {
        "command": "python3 scripts/qualify-query-resources.py --binary target/release/examples/query_qualification --commits 50000 --output .scratch/m4-native-query-budget.json",
        "result": "fail",
        "failures": 1,
        "note": "Independent process timeout at 180.01 seconds / 177.47 CPU seconds; no complete query report. Live sampling located a Tokio cooperative-budget spin inside native synchronous metadata predicate reads. Immutable before report retained."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj runtime::tests",
        "result": "fail",
        "failures": 2,
        "note": "Both bounded-poll regressions fail: native future and stream yield Pending after exhausting Tokio cooperative budget under a foreign executor."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj runtime::tests",
        "result": "pass",
        "failures": 0,
        "note": "Both regressions pass after runtime boundary unconstrained polling; real pending oneshot I/O remains pending."
      },
      {
        "command": "python3 scripts/qualify-query-resources.py --binary target/release/examples/query_qualification --commits 50000 --output .scratch/m4-native-query-budget-after.json",
        "result": "pass",
        "failures": 0,
        "note": "First repaired release run passes: 4.68 seconds elapsed, 4.15 CPU seconds, 243204096 peak RSS bytes. All 8 native query cases pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example query_qualification",
        "result": "pass",
        "failures": 0,
        "note": "Final release fixture asserts exact row counts and records query strings and configured limits."
      },
      {
        "command": "python3 scripts/qualify-query-resources.py --binary target/release/examples/query_qualification --commits 50000 --output docs/evidence/m4-native-query-budget.json",
        "result": "pass",
        "failures": 0,
        "note": "Final report passes: 8 query cases plus 4 negative admission checks; 4.74 seconds elapsed, 4.13 CPU seconds, 246366208 peak RSS bytes. Binary/lock hashes and scope retained."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle",
        "result": "pass",
        "failures": 0,
        "note": "All 57 native lifecycle cases pass after cancellation and shared runtime fixes."
      },
      {
        "command": "python3 scripts/rustfs-contract.py --run --rust-client --server --output .scratch/m4-runtime-rustfs.json",
        "result": "pass",
        "failures": 0,
        "note": "All 11 real RustFS harness cases pass; compiled native transport, cache-free cold publication recovery and TLS server/CLI restart/revocation requalify the shared runtime adapter."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "57 requirements, boundaries, ledger, rustfmt and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "190 Rust tests pass / 6 external cases intentionally ignored; 15 governance and 3 storage-harness tests pass. No unresolved failures."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Locked workspace build passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 9 lanes pass, including 190 Rust tests, rustfmt, mdBook and mapped spec-first checks."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Adversarial merge/tree amplification and large content memory admission, controlled long-index rebuild tasks/status, HTTP JSON/media/SSE, generated contracts and browser integration. Fixed-width graph measurements do not qualify whole-server deployment budgets."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; Tokio 1.53.1; release measurement host and binary/lock checksums recorded in linked evidence.",
    "formats": "New owned Canceled query error; no native identity, envelope, publication, RPC or persisted authority format change. Qualification JSON is evidence only."
  },
  {
    "id": "E-M4-010",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "native-query-metadata-shape-admission",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/content.rs",
      "crates/jj-store-cli/tests/native_lifecycle/content_admission.rs",
      "crates/jj-store-jj/src/query/admission.rs",
      "crates/jj-store-jj/src/query/access.rs",
      "crates/jj-store-jj/src/query/content.rs",
      "crates/jj-store-jj/src/query/mod.rs",
      "crates/jj-store-jj/examples/query_qualification.rs",
      "crates/jj-store-jj/examples/content_qualification.rs",
      "scripts/qualify-query-resources.py",
      "docs/evidence/m4-query-metadata-budget.json",
      "docs/evidence/m4-content-shape-budget.json",
      "docs/evidence/m4-query-admission-audit.md",
      "spec/src/api.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/jj-integration.md",
      "spec/src/spec-governance.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/content_admission.rs#query_shape_rejects_commit_merge_amplification_without_narrowing_persistence",
      "crates/jj-store-cli/tests/native_lifecycle/content_admission.rs#query_shape_rejects_reference_merge_amplification",
      "crates/jj-store-cli/tests/native_lifecycle/content_admission.rs#query_shape_rejects_dense_trees_before_native_expansion",
      "crates/jj-store-cli/tests/native_lifecycle/content_admission.rs#query_shape_node_admission_is_request_local_and_preserves_file_streaming",
      "crates/jj-store-jj/src/query/admission.rs#query_shape_bounds_every_persisted_reference_and_edge_collection",
      "crates/jj-store-jj/src/query/admission.rs#query_shape_node_limit_is_cumulative_and_counts_keys_nulls_and_empty_values",
      "crates/jj-store-jj/src/query/admission.rs#query_shape_rejects_corruption_depth_and_cancellation_without_collections",
      "crates/jj-store-jj/src/query/admission.rs#query_shape_recognizes_escaped_fields_and_rejects_trailing_input",
      "crates/jj-store-jj/examples/content_qualification.rs#main",
      "crates/jj-store-jj/examples/query_qualification.rs#main"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_shape_",
        "result": "fail",
        "failures": 3,
        "note": "Before: all 3 regressions fail because oversized merges, reference conflicts and dense trees are accepted."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_shape_",
        "result": "pass",
        "failures": 0,
        "note": "After admission: original 3 regressions pass; published native objects still round-trip losslessly."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj query_shape_",
        "result": "fail",
        "failures": 1,
        "note": "One unit-fixture compile error: ObjectKey::from_parts needs the kind byte, not ObjectKind. Fixed explicit conversion."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj query_shape_",
        "result": "pass",
        "failures": 0,
        "note": "All 4 scanner cases pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example query_qualification",
        "result": "pass",
        "failures": 0,
        "note": "Release graph fixture builds with explicit metadata limits/counters."
      },
      {
        "command": "python3 scripts/qualify-query-resources.py --binary target/release/examples/query_qualification --commits 50000 --output .scratch/m4-shape-graph-budget.json",
        "result": "pass",
        "failures": 0,
        "note": "8 queries plus 4 negative admission cases; 5.88 seconds elapsed, 5.16 CPU seconds, 278675456 peak RSS bytes; at most 2221020 metadata nodes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example content_qualification",
        "result": "pass",
        "failures": 0,
        "note": "Release content fixture builds."
      },
      {
        "command": "python3 scripts/qualify-query-resources.py --scenario content --binary target/release/examples/content_qualification --output .scratch/m4-shape-content-budget.json",
        "result": "pass",
        "failures": 0,
        "note": "8 content boundary cases pass; 7.70 seconds elapsed, 7.36 CPU seconds, 111181824 peak RSS bytes. Dense/merge pages, 256 MiB complete verification and bounded streams/ranges, negative shape/blob budgets."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle",
        "result": "pass",
        "failures": 0,
        "note": "60 native lifecycle cases passed before adding the request-local node/file-streaming case."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_shape_",
        "result": "pass",
        "failures": 0,
        "note": "All 4 final shape acceptance cases pass; original 3 regressions plus request-local node accounting/file streaming."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "57 requirements, package boundaries, progress ledger, rustfmt and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "198 Rust tests pass / 6 external cases intentionally ignored; includes all 61 native lifecycle cases. 15 governance and 3 storage-harness tests pass. No unresolved failures."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Locked workspace build passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 9 lanes pass, including 198 Rust tests, rustfmt, mdBook and mapped spec-first checks."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Derived merge and recursive tree amplification, native file-content predicates and aggregate worker budgets; controlled long rebuild tasks/status, HTTP JSON/media/SSE, generated contracts and browser integration."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; release host and binary/lock hashes recorded in linked evidence.",
    "formats": "Additive owned query metadata-node statistics and trusted configuration limits. No native identity, envelope, authority or RPC format change."
  },
  {
    "id": "E-M4-011",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "native-revset-predicate-admission-and-reader-runtime",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/content.rs",
      "crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs",
      "crates/jj-store-jj/src/backend.rs",
      "crates/jj-store-jj/src/query/access.rs",
      "crates/jj-store-jj/src/query/content.rs",
      "crates/jj-store-jj/src/query/graphs.rs",
      "crates/jj-store-jj/src/query/mod.rs",
      "crates/jj-store-jj/src/query/predicates.rs",
      "scripts/qualify-query-resources.py",
      "spec/src/api.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/jj-integration.md",
      "spec/src/spec-governance.md",
      "docs/evidence/m4-native-predicate-audit.md",
      "docs/evidence/m4-native-predicate-budget.json",
      "docs/evidence/m4-native-reader-rustfs.json"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_bound_file_bytes_before_materialization",
      "crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_bound_line_comparison_work_before_evaluation",
      "crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_bound_derived_merge_terms_without_limiting_plain_log",
      "crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_preserve_admitted_file_and_recursive_merge_semantics",
      "crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_reject_recursive_criss_cross_expansion",
      "crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_bound_recursive_tree_depth_for_graphs_and_content",
      "crates/jj-store-cli/tests/native_lifecycle/query_predicates.rs#native_predicates_admit_boundary_files_and_charge_repeated_tree_paths",
      "crates/jj-store-jj/src/backend.rs#native_file_reader_progress_does_not_require_foreign_executor_cooperative_reset",
      "crates/jj-store-jj/tests/rustfs.rs#real_rustfs_transport_and_native_blob_contract",
      "crates/jj-store-jj/tests/rustfs.rs#real_rustfs_manifest_publication_and_cold_recovery",
      "crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_",
        "result": "fail",
        "failures": 1,
        "note": "Initial fixture compile error: CommittedRead.graph requires an Arc receiver; corrected helper type."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_",
        "result": "fail",
        "failures": 3,
        "note": "Before implementation: 2 real regressions accept oversized file bytes/lines. The third merge fixture returned Invalid because its path predicate omitted logical workspace context; this result was not accepted as merge-limit evidence."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_",
        "result": "fail",
        "failures": 1,
        "note": "After byte/line guards: 2 pass / merge case still Invalid. Switching to files(all()) did not remove native workspace requirements."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_bound_derived_merge_terms_without_limiting_plain_log",
        "result": "fail",
        "failures": 1,
        "note": "First temporary preflight-disabled probe still lacked workspace context; Invalid, not valid baseline evidence. Source restored afterward."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_",
        "result": "fail",
        "failures": 3,
        "note": "Expanded suite before workspace fixture correction: 2 pass / 3 Invalid failures."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_bound_derived_merge_terms_without_limiting_plain_log",
        "result": "fail",
        "failures": 1,
        "note": "Corrected fixture supplies a published logical workspace. With only preflight temporarily disabled, the 34-parent input succeeds instead of Limit. This is the actual merge regression baseline; preflight restored afterward."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_",
        "result": "fail",
        "failures": 1,
        "note": "After workspace correction: 4 pass / 1 parity expectation fails. Native 3-by-2 criss-cross merge behavior differs from the fixture assumption. Replaced the assumption with direct pinned native evaluation, preserving exact result comparison."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_",
        "result": "fail",
        "failures": 2,
        "note": "Oracle fixture had 2 compile diagnostics: direct chrono dependency unavailable and missing expected binding. Used inferred native date type and corrected binding."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_",
        "result": "pass",
        "failures": 0,
        "note": "Five cases pass, including direct native oracle comparison across 4 histories and 4 predicates."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_bound_recursive_tree_depth_for_graphs_and_content",
        "result": "fail",
        "failures": 1,
        "note": "Before path admission, a 70-level tree succeeds through graph traversal instead of Limit."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_",
        "result": "pass",
        "failures": 0,
        "note": "Six cases pass after backend tree-path admission, including graph and content depth rejection."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_",
        "result": "fail",
        "failures": 1,
        "note": "Boundary fixture: 6 pass / 1 native sorted-tree assertion fails. Zero-padded generated entry names preserve canonical lexical ordering."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_",
        "result": "fail",
        "failures": 1,
        "note": "After fixture ordering repair: 6 cases pass, final repeated-file predicate spins under pollster. Live process and one-second sample prove FileReader/Tokio cooperative-budget exhaustion. Owned test process 16674 terminated after diagnosis; no deadline pass claimed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj native_file_reader_progress_does_not_require_foreign_executor_cooperative_reset",
        "result": "fail",
        "failures": 1,
        "note": "Bounded manual polling of a budget-consuming returned reader reproduces the runtime gap."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-jj native_file_reader_progress_does_not_require_foreign_executor_cooperative_reset",
        "result": "pass",
        "failures": 0,
        "note": "Exact regression passes after routing each reader poll through RuntimeContext.run; genuine Pending is preserved."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_predicates_",
        "result": "pass",
        "failures": 0,
        "note": "All 7 cases pass in 2.25 seconds after reader fix: accepted 256 KiB / 901-line input, exact 4 MiB repeated-path reads, over-budget rejections, recursive merges/depth and upstream parity."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test --release --locked -p jj-store-cli --test native_lifecycle --no-run --message-format=json-render-diagnostics",
        "result": "pass",
        "failures": 0,
        "note": "Release native test binary compiled, then rebuilt after the returned-reader repair and conservative temporary-label reservation."
      },
      {
        "command": "python3 scripts/qualify-query-resources.py --scenario predicates --binary /Users/stevejuma/code/lapismd/jj-store/target/release/deps/native_lifecycle-fa95bbe1873b7c06 --output docs/evidence/m4-native-predicate-budget.json",
        "result": "pass",
        "failures": 0,
        "note": "All 7 named acceptance cases pass in final release process: 4.10 seconds elapsed / 0.85 CPU seconds / 19169280 peak RSS bytes. Earlier repaired release process also passed (2.17 elapsed / 0.76 CPU / 19595264 RSS) before final label reservation."
      },
      {
        "command": "python3 scripts/rustfs-contract.py --run --rust-client --server --output .scratch/m4-native-reader-rustfs.json",
        "result": "pass",
        "failures": 0,
        "note": "All 11 real RustFS harness cases pass after the native returned-reader runtime fix, including native transport, cold publication recovery and TLS server/CLI restart/revocation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "57 requirements, package boundaries, progress, rustfmt and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "206 Rust tests pass / 6 external cases intentionally ignored. All 68 native lifecycle cases, 15 governance tests and 3 storage-harness self-tests pass; no unresolved failures."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Locked workspace build passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 9 lanes pass, including 206 Rust tests, rustfmt, mdBook and mapped spec-first checks."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Shared worker composition and controlled long cold-index tasks/status, then HTTP JSON/media/SSE, generated contracts and browser integration. Whole-server operational qualification remains M8a."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5. Predicate ancestry accounting mirrors the pinned recursive merge ordering without constructing merges; admitted evaluation remains upstream.",
    "formats": "Additive owned graph predicate limits and file-byte/line statistics. Optional native authority tree-path admission defaults to existing session permission. No native identity, envelope, publication or RPC format changes."
  },
  {
    "id": "E-M4-012",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "shared-native-query-workers",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/content.rs",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs",
      "crates/jj-store-cli/tests/native_lifecycle/query_workers.rs",
      "crates/jj-store-jj/src/query/content.rs",
      "crates/jj-store-jj/src/query/mod.rs",
      "crates/jj-store-jj/src/query/workers.rs",
      "spec/src/api.md",
      "spec/src/cli.md",
      "spec/src/jj-integration.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/query_workers.rs#query_workers_file_lifetime_limits_graphs_and_content_together",
      "crates/jj-store-cli/tests/native_lifecycle/query_workers.rs#query_workers_graph_and_content_cancel_release_shared_capacity",
      "crates/jj-store-cli/tests/native_lifecycle/query_workers.rs#query_workers_mixed_capacity_clones_and_errors_release_slots",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs#graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_workers_",
        "result": "fail",
        "failures": 2,
        "note": "Before: independent graph/content pools admit a competing route while a worker or file handle occupies the intended shared capacity. Both tests failed; no compile errors."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle query_workers_",
        "result": "pass",
        "failures": 0,
        "note": "Original 2 regressions pass after shared composition. Final expanded suite passes all 3 cases, including mixed capacity, clones, alternate cache roots and error cleanup."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_query_cancellation_retains_worker_permit_until_exit_and_deadlines_bound_waits",
        "result": "pass",
        "failures": 0,
        "note": "Synchronous canceled graph worker retains both shared content capacity and its cache lock until actual exit."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "57 requirements, boundaries, progress, rustfmt and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "209 Rust tests pass / 6 external cases intentionally ignored; all 71 native lifecycle cases, 15 governance tests and 3 storage-harness self-tests pass. No unresolved failures."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Locked workspace build passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 9 lanes pass, including Rust tests, rustfmt, mdBook and mapped spec-first checks."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Controlled long cold-index tasks/status; HTTP admission for snapshot capture and response delivery, JSON/media/SSE, generated contracts and browser integration. Shared native pool is a tested composition interface; whole-server concurrent resource qualification remains M8a."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5. Existing per-query shape/byte/work/deadline policies unchanged.",
    "formats": "Additive shared query worker composition and capacity/in-use diagnostics. No native identity, envelope, publication, HTTP or RPC format changes."
  },
  {
    "id": "E-M4-013",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "native-index-preparation-and-service-tasks",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/content.rs",
      "crates/jj-store-cli/tests/native_lifecycle/graphs.rs",
      "crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs",
      "crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs",
      "crates/jj-store-jj/examples/query_qualification.rs",
      "crates/jj-store-jj/src/query/mod.rs",
      "crates/jj-store-service/src/index_tasks.rs",
      "crates/jj-store-service/src/lib.rs",
      "crates/jj-store-service/src/pagination.rs",
      "crates/jj-store-service/src/reads.rs",
      "docs/evidence/m4-index-preparation-audit.md",
      "docs/evidence/m4-index-preparation-budget.json",
      "scripts/qualify-query-resources.py",
      "spec/src/api.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/jj-integration.md",
      "spec/src/mutations.md",
      "spec/src/spec-governance.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs#index_preparation_is_explicit_snapshot_bound_and_disposable",
      "crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs#index_preparation_cancels_pending_work_and_retains_shared_slot_until_exit",
      "crates/jj-store-cli/tests/native_lifecycle/index_preparation.rs#graph_cursor_rebuild_reports_original_snapshot_after_later_publication",
      "crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs#index_tasks_report_pinned_completion_restart_and_current_revocation",
      "crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs#index_tasks_deduplicate_bound_cancel_and_drop_running_jobs",
      "crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs#index_tasks_retention_expiry_and_failures_are_explicit",
      "crates/jj-store-cli/tests/native_lifecycle/index_tasks.rs#index_tasks_expiry_cancels_pending_storage_without_status_polling"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle index_preparation_",
        "result": "fail",
        "failures": 17,
        "note": "Before: 17 compile diagnostics identify absent explicit preparation, cancellation and inline-threshold interfaces. No tests ran; this is a missing-feature baseline."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle index_preparation_",
        "result": "pass",
        "failures": 0,
        "note": "Both native preparation fixtures pass after implementation: exact snapshot/cache loss/hard budget and cancellation/shared worker lifecycle."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle index_tasks_",
        "result": "fail",
        "failures": 1,
        "note": "Before service implementation: unresolved index_tasks import; no tests ran."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle index_tasks_",
        "result": "pass",
        "failures": 0,
        "note": "Initial three service task cases pass for ready/failed/canceled, retention, restart/revocation, coalescing, bounded execution and service drop."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cursor_rebuild_reports_original_snapshot_after_later_publication",
        "result": "fail",
        "failures": 1,
        "note": "Before pagination integration: rebuild error loses the original cursor snapshot after a later publication."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle graph_cursor_rebuild_reports_original_snapshot_after_later_publication",
        "result": "pass",
        "failures": 0,
        "note": "Exact regression passes with original complete snapshot in rebuild-required response and original cursor resume after preparation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle index_",
        "result": "pass",
        "failures": 0,
        "note": "All 7 preparation/task/cursor cases pass, including expiry without status polling."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle index_tasks_report_pinned_completion_restart_and_current_revocation",
        "result": "pass",
        "failures": 0,
        "note": "Expanded access fixture also rejects valid same-principal tokens for a different native repository on status and cancel."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --release --locked -p jj-store-jj --example query_qualification",
        "result": "pass",
        "failures": 0,
        "note": "Optimized native graph fixture built with explicit cold admission/preparation."
      },
      {
        "command": "python3 scripts/qualify-query-resources.py --scenario graphs --binary target/release/examples/query_qualification --output docs/evidence/m4-index-preparation-budget.json",
        "result": "pass",
        "failures": 0,
        "note": "Linear and wide-head histories each contain 50,000 commits. Cold admission, preparation and 8 prepared/warm graph reads pass. Whole process: 5.71 seconds elapsed, 5.22 CPU seconds, 296026112 peak RSS bytes. Scope excludes service, RustFS/network and browser costs."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "57 requirements, package boundaries, progress, rustfmt and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "216 Rust tests pass / 6 external cases intentionally ignored. All 78 native lifecycle cases, 15 governance tests and 3 storage-harness self-tests pass; no unresolved failures."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Locked workspace build passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 9 lanes pass, including 216 Rust tests, rustfmt, mdBook and mapped spec-first checks."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "HTTP composition and admission for snapshot capture/response delivery, JSON/media/SSE routes including index task status, generated contracts and browser integration. Whole-server deployment and aggregate performance qualification remains M8a."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5. Release binary and Cargo.lock hashes retained in the process report.",
    "formats": "Additive owned index-preparation report, process-local task status/error DTOs and snapshot-bearing pagination rebuild errors. Default inline threshold is 1000 retained commits; hard index/shape/work/byte budgets are unchanged. No native identity, envelope, publication, local cache receipt or RPC format change."
  },
  {
    "id": "E-M4-014",
    "tasks": ["M4", "M4.1"],
    "date": "2026-09-27",
    "kind": "authenticated-http-reads-and-shared-tls-listener",
    "files": [
      "Cargo.lock",
      "Cargo.toml",
      "crates/jj-store-cli/Cargo.toml",
      "crates/jj-store-cli/tests/native_lifecycle/content.rs",
      "crates/jj-store-cli/tests/native_lifecycle/http_reads.rs",
      "crates/jj-store-cli/tests/server_executable.rs",
      "crates/jj-store-http/Cargo.toml",
      "crates/jj-store-http/src/body.rs",
      "crates/jj-store-http/src/error.rs",
      "crates/jj-store-http/src/lib.rs",
      "crates/jj-store-http/src/routes.rs",
      "crates/jj-store-jj/src/query/content.rs",
      "crates/jj-store-server/Cargo.toml",
      "crates/jj-store-server/src/lib.rs",
      "crates/jj-store-server/src/listener.rs",
      "crates/jj-store-server/src/process.rs",
      "crates/jj-store-service/src/reads.rs",
      "crates/jj-store-service/src/transport.rs",
      "docs/evidence/m4-http-read-gate.md",
      "docs/evidence/m4-http-rustfs.json",
      "scripts/check-boundaries.mjs",
      "scripts/governance.test.mjs",
      "spec/src/api.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/jj-integration.md",
      "spec/src/milestones.md",
      "spec/src/mutations.md",
      "spec/src/security.md",
      "spec/src/spec-governance.md",
      "spec/src/verification.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_match_native_snapshots_objects_graphs_and_differences_without_publication",
      "crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_authentication_queries_and_binary_ranges_fail_closed",
      "crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_revocation_stops_inflight_media_and_credentials_never_become_query_data",
      "crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_unpolled_response_expiry_releases_transport_and_native_slots",
      "crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_index_task_and_signed_cursor_preserve_snapshot_across_publication",
      "crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_range_selection_cannot_reset_consumed_or_preselected_files",
      "crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_reads_reject_unframed_request_content",
      "crates/jj-store-server/src/listener.rs#listener_idle_deadline_releases_owned_connection_slot",
      "crates/jj-store-server/src/listener.rs#listener_connection_capacity_is_retained_until_drop",
      "crates/jj-store-cli/tests/server_executable.rs#cold_tls_http_reads_support_http1_and_http2",
      "crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation",
      "scripts/governance.test.mjs#http_consumes_owned_service_contracts_without_storage_or_upstream",
      "scripts/governance.test.mjs#server_composition_cannot_depend_on_cli_or_native_upstream"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_reads_",
        "result": "fail",
        "failures": 9,
        "note": "Before HTTP implementation: missing crate/Axum/Tower imports produced 9 compile diagnostics; no tests ran."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_reads_",
        "result": "fail",
        "failures": 3,
        "note": "Initial implementation: 3 Send-future compile errors and 1 unused import warning; fixed an owned VerifiedFile info future and middleware request borrow scope."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_reads_",
        "result": "pass",
        "failures": 0,
        "note": "Initial 2 route fixtures passed, then expanded 6 cases passed including range, revocation, cursor/task and unpolled response expiry."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_http_reads_support_http1_and_http2",
        "result": "fail",
        "failures": 1,
        "note": "Before production integration: HTTP/1.1 TLS client rejected with NoApplicationProtocol; one failed test."
      },
      {
        "command": "pnpm test:governance",
        "result": "fail",
        "failures": 1,
        "note": "New allowed server-to-HTTP composition assertion failed before boundary allowlist update. HTTP direct storage/native import rejection passed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_",
        "result": "pass",
        "failures": 0,
        "note": "Both HTTP/1.1 + HTTP/2 and native certificate/hostname/plaintext TLS fixtures passed after shared listener integration."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_reads_reject_unframed_request_content",
        "result": "fail",
        "failures": 1,
        "note": "Unexpected streamed GET body without framing headers returned 200 instead of 400. Fixed bounded EOF verification before bodyless route execution."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_reads_",
        "result": "fail",
        "failures": 1,
        "note": "Intermediate compile diagnostic: HeaderMap has no get_all_mut; changed sensitive-header marking to iter_mut. No tests ran."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_reads_",
        "result": "pass",
        "failures": 0,
        "note": "All 7 focused HTTP fixtures passed, including the exact unframed-body regression."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server listener_",
        "result": "pass",
        "failures": 0,
        "note": "2 listener fixtures pass: shared connection admission and activity/idle deadline retaining then releasing the socket permit."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable",
        "result": "pass",
        "failures": 0,
        "note": "7 cold executable tests pass / 1 external RustFS case ignored; trust, protocols, state lock, commands and shutdown remain intact."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools python3 scripts/rustfs-contract.py --run --rust-client --server --output .scratch/m4-http-rustfs.json",
        "result": "pass",
        "failures": 0,
        "note": "All 11 pinned single-node harness cases pass, including real HTTP/CLI fixed-operation parity, unchanged GET manifest snapshot, ranges, process/storage kills, removed caches and revoked HTTP access. Report retained in docs/evidence/m4-http-rustfs.json."
      },
      {
        "command": "pnpm test:governance",
        "result": "pass",
        "failures": 0,
        "note": "All 16 cases pass, including corrected allowed composition and forbidden direct HTTP storage/native dependencies."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "fail",
        "failures": 1,
        "note": "One Clippy manual_option_zip diagnostic, repeated for library/test targets. Replaced nested option mapping with zip; no behavior or assertion weakened."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "57 requirements, 9 package boundaries, progress, rustfmt and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "fail",
        "failures": 1,
        "note": "Environment failure before Rust tests: attachment_recovery linker write failed with errno=28 (disk full). Governance 16/16 and harness self-tests 3/3 passed. No product test failure was observed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "fail",
        "failures": 2,
        "note": "Environment failure: rustc LLVM output could not write (disk full) for jj-cli and jj-store-cli. Removed only jj-store target/debug/incremental generated cache (14 GiB); no source, release artifacts, credentials or other checkout files removed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "226 Rust tests pass / 6 external tests ignored, including 85 native lifecycle tests and 7 cold executable server tests. Governance 16/16, storage harness self-tests 3/3 pass after generated-cache cleanup. No unresolved product failures."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Locked workspace build passes after generated-cache cleanup."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 9 lanes pass, including 226 Rust tests / 6 external ignored, governance, rustfmt, mdBook and mapped spec-first checks. M4.1 read gate is complete; M4.2 SSE/generated contracts/browser, M5, M6 and M8a remain separately unfinished."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; Axum 0.8.9, tonic 0.14.6, tokio-rustls 0.26.5; locked serde_path_to_error 0.1.20.",
    "formats": "Additive HTTP V1 authenticated JSON/media/index-task routes and stable error envelopes, one-time verified spool range selection; dual HTTP/1.1 and h2 ALPN. Native persistence, publication and RPC wire formats unchanged. SSE, mutations, search and UI are not advertised."
  },
  {
    "id": "E-M4-015",
    "tasks": ["M4", "M4.2"],
    "date": "2026-09-27",
    "kind": "committed-event-replay-and-authenticated-sse",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/events.rs",
      "crates/jj-store-cli/tests/native_lifecycle/http_events.rs",
      "crates/jj-store-cli/tests/native_lifecycle.rs",
      "crates/jj-store-cli/tests/server_executable.rs",
      "crates/jj-store-http/Cargo.toml",
      "crates/jj-store-http/src/error.rs",
      "crates/jj-store-http/src/lib.rs",
      "crates/jj-store-http/src/routes.rs",
      "crates/jj-store-service/src/administration.rs",
      "crates/jj-store-service/src/events.rs",
      "crates/jj-store-service/src/lib.rs",
      "crates/jj-store-service/src/native.rs",
      "crates/jj-store-service/src/transport.rs",
      "docs/evidence/m4-committed-events.md",
      "spec/src/api.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/milestones.md",
      "spec/src/mutations.md",
      "spec/src/security.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/events.rs#events_replay_only_committed_positions_across_restart_and_checkpoint",
      "crates/jj-store-cli/tests/native_lifecycle/events.rs#events_reject_foreign_cursors_and_reset_expired_future_and_replaced_epochs",
      "crates/jj-store-cli/tests/native_lifecycle/events.rs#events_reauthorize_queued_records_after_revocation",
      "crates/jj-store-cli/tests/native_lifecycle/events.rs#events_local_publication_wakes_shared_host_without_waiting_for_poll",
      "crates/jj-store-cli/tests/native_lifecycle/events.rs#events_corrupt_selected_record_fails_without_skipping_or_resetting",
      "crates/jj-store-cli/tests/native_lifecycle/events.rs#events_epoch_replacement_resets_before_queued_delivery",
      "crates/jj-store-cli/tests/native_lifecycle/events.rs#events_storage_deadlines_close_without_advancing_resume_position",
      "crates/jj-store-cli/tests/native_lifecycle/events.rs#events_lagged_wake_hints_replay_every_selected_record_in_bounded_pages",
      "crates/jj-store-cli/tests/native_lifecycle/http_events.rs#http_events_resume_restart_and_reset_use_selected_publications",
      "crates/jj-store-cli/tests/native_lifecycle/http_events.rs#http_events_have_separate_admission_and_expire_unpolled_streams",
      "crates/jj-store-cli/tests/native_lifecycle/http_events.rs#http_events_validate_resume_headers_and_reset_without_replaying_ahead",
      "crates/jj-store-cli/tests/native_lifecycle/http_events.rs#http_events_revocation_terminates_queued_delivery",
      "crates/jj-store-cli/tests/server_executable.rs#real_rustfs_server_cli_restart_and_revocation"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle events_",
        "result": "fail",
        "failures": 3,
        "note": "Before service implementation: 3 compile diagnostics for absent events module/types; no tests ran."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle events_",
        "result": "pass",
        "failures": 0,
        "note": "Initial 3 service fixtures pass: selected replay/restart/checkpoint, explicit cursor resets, queued-event revocation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_events_",
        "result": "fail",
        "failures": 1,
        "note": "Before HTTP route implementation: /events returned 404 instead of 200."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle events_",
        "result": "pass",
        "failures": 0,
        "note": "Expanded 10 service/HTTP cases pass, including local wake, selected-record corruption, live epoch reset, separate stream admission and unpolled stream expiry."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle events_",
        "result": "fail",
        "failures": 3,
        "note": "Test helper initially assumed an ObjectStore delete API; 3 compile diagnostics. Removed those nonexistent methods from the fixture adapter, retaining the real trait contract."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle events_",
        "result": "pass",
        "failures": 0,
        "note": "All 11 service/HTTP cases pass including bounded stalled storage and retry from the original delivered cursor."
      },
      {
        "command": "pnpm check:progress",
        "result": "fail",
        "failures": 2,
        "note": "Expected bookkeeping diagnostics before this entry existed: missing same-change ledger update and missing new/updated evidence entry."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle events_",
        "result": "pass",
        "failures": 0,
        "note": "All 12 event cases pass, including lagged 64-slot wake hints replaying 70 selected publications through two-record pages."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "57 requirements, package boundaries, updated ledger, rustfmt and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "238 Rust tests passed / 6 external cases ignored. Includes 97 native lifecycle tests. Governance 16/16 and storage harness self-tests 3/3 passed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "fail",
        "failures": 1,
        "note": "Production-only compile error: Axum MatchedPath was unavailable without the matched-path feature; dev dependencies had enabled it for tests. Added the explicit production feature without changing assertions."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Production workspace build passes after explicitly enabling Axum matched-path."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 9 lanes pass after the production feature fix, including 238 Rust tests / 6 external ignored, rustfmt, governance, mdBook and mapped spec-first checks. Real SSE deployment remains unverified while Docker restart approval is pending."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Docker Desktop logs confirm the engine stopped on disk-full error. Repository-generated cache cleanup restored free space, but shared-engine restart needs user approval under AGENTS.md. The isolated harness remains live at docker volume creation; SSE real TLS/RustFS restart/revocation evidence is pending, not passed. Generated OpenAPI/client contracts and browser remain M4.2; M5/M6/M8a unfinished."
    ],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; existing pinned Axum/tonic/Tokio unchanged.",
    "formats": "Additive authenticated SSE route with V1 repository/epoch/position IDs, ready/changed/reset events and authorized heartbeat comments. Bounded process-local wake hints and independent SSE admission. Native persistence, manifest/WAL/receipt, RPC wire and existing read cursor formats unchanged."
  },
  {
    "id": "E-M4-016",
    "tasks": ["M4", "M4.2"],
    "date": "2026-09-27",
    "kind": "generated-openapi-and-typescript-contracts",
    "files": [
      "Cargo.lock",
      "Cargo.toml",
      "README.md",
      "crates/jj-store-cli/tests/native_lifecycle/http_reads.rs",
      "crates/jj-store-http/Cargo.toml",
      "crates/jj-store-http/examples/openapi.rs",
      "crates/jj-store-http/src/contract.rs",
      "crates/jj-store-http/src/contract/tests.rs",
      "crates/jj-store-http/src/dto.rs",
      "crates/jj-store-http/src/error.rs",
      "crates/jj-store-http/src/lib.rs",
      "crates/jj-store-http/src/routes.rs",
      "crates/jj-store-jj/Cargo.toml",
      "crates/jj-store-jj/src/native_dto.rs",
      "crates/jj-store-jj/src/query/cache.rs",
      "crates/jj-store-jj/src/query/content.rs",
      "crates/jj-store-jj/src/query/content/conflicts.rs",
      "crates/jj-store-jj/src/query/content/diff.rs",
      "crates/jj-store-jj/src/query/content/operations.rs",
      "crates/jj-store-jj/src/query/content/trees.rs",
      "crates/jj-store-jj/src/query/mod.rs",
      "crates/jj-store-model/Cargo.toml",
      "crates/jj-store-model/src/ids.rs",
      "crates/jj-store-service/Cargo.toml",
      "crates/jj-store-service/src/events.rs",
      "crates/jj-store-service/src/index_tasks.rs",
      "crates/jj-store-service/src/reads.rs",
      "docs/evidence/m4-generated-contracts.md",
      "package.json",
      "pnpm-lock.yaml",
      "schemas/README.md",
      "schemas/openapi.json",
      "scripts/contracts-validation.mjs",
      "scripts/contracts.mjs",
      "scripts/contracts.test.mjs",
      "scripts/contracts.types.ts",
      "scripts/tsconfig.contracts.json",
      "spec-validator.config.mjs",
      "spec/src/api.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/identity.md",
      "spec/src/jj-integration.md",
      "spec/src/milestones.md",
      "spec/src/mutations.md",
      "spec/src/spec-governance.md",
      "spec/src/verification.md",
      "web/src/lib/api/generated.ts"
    ],
    "tests": [
      "crates/jj-store-http/src/contract/tests.rs#contract_distinguishes_optional_inputs_from_nullable_outputs",
      "crates/jj-store-http/src/contract/tests.rs#contract_preserves_native_shapes_and_hex_id_widths",
      "crates/jj-store-http/src/contract/tests.rs#contract_inventory_covers_security_methods_media_and_task_bodies",
      "crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_contract_capture_matches_registered_routes",
      "scripts/contracts.test.mjs#openapi_document_validates_without_external_references",
      "scripts/contracts.test.mjs#native_hex_rejects_short_uppercase_and_untyped_values",
      "scripts/contracts.test.mjs#frozen_native_shapes_preserve_nullability_and_tuples",
      "scripts/contracts.test.mjs#tagged_union_and_required_nullable_fields_fail_closed",
      "scripts/contracts.test.mjs#response_validation_rejects_missing_contracts_and_wrong_media"
    ],
    "runs": [
      {
        "command": "pnpm format:rust:check",
        "result": "pass",
        "failures": 0,
        "note": "Before change: pinned Rust 1.97.1 rustfmt policy and normal/spec integration verified; formatting passes."
      },
      {
        "command": "pnpm contracts:check",
        "result": "fail",
        "failures": 1,
        "note": "Before change: required generated contract checker command absent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-http --all-targets",
        "result": "fail",
        "failures": 1,
        "note": "Initial scaffolding: contract test module not yet added (E0583)."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --lib contract::tests --locked",
        "result": "fail",
        "failures": 3,
        "note": "Initial tests exposed omitted properties on the empty query schema. Generator now handles an empty deny-unknown-field object explicitly."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --lib contract::tests --locked",
        "result": "fail",
        "failures": 1,
        "note": "Fixture wrongly expected a named transparent RefTarget schema. Assertion now checks its actual inline array in NativeRemoteRef."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --lib contract::tests --locked",
        "result": "pass",
        "failures": 0,
        "note": "3 contract shape/inventory tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:check",
        "result": "fail",
        "failures": 1,
        "note": "Exact lifecycle filter omitted nested module names and selected zero tests. Checker rejected that run; corrected to content_acceptance::http_acceptance::http_contract_capture_matches_registered_routes."
      },
      {
        "command": "pnpm check:progress",
        "result": "fail",
        "failures": 2,
        "note": "Expected missing same-change ledger/update diagnostics before adding this evidence entry."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:generate",
        "result": "pass",
        "failures": 0,
        "note": "OpenAPI 3.1 validates without external references; generated JSON and TypeScript written deterministically."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:check",
        "result": "pass",
        "failures": 0,
        "note": "Generated artifacts match; 25 actual HTTP/SSE serialized responses conform. Five Node schema tests and TypeScript consumer acceptance pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "58 requirements, package boundaries, progress, rustfmt, regenerated contract conformance and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "242 Rust tests pass / 6 external tests ignored; 98 native lifecycle tests. 16 governance tests, 3 storage harness self-tests, 5 contract tests and TypeScript consumer checks pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Production workspace build succeeds with schema traits and typed capabilities/detail/error responses."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 10 lanes pass, including generated contract conformance, rustfmt, repeated full Rust tests, mdBook and mapped spec-first."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass after source, generated artifact, specification and ledger formatting."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; schemars 1.2.2; openapi-typescript 7.13.0; swagger-parser 13.1.0; Ajv 8.20.0; TypeScript 5.9.3.",
    "formats": "OpenAPI 3.1 / JSON Schema 2020-12 plus generated TypeScript read contracts. No native persistence, identity, RPC or HTTP wire-shape changes. Enclosing Jujutsu change is the version-control reference. M4.2 remains In progress for browser implementation and real SSE deployment qualification; E-M4-015 retains the pending shared Docker restart approval and live isolated harness. M5, M6 and M8a remain unfinished."
  },
  {
    "id": "E-M4-017",
    "tasks": ["M4", "M4.2"],
    "date": "2026-09-27",
    "kind": "implementation",
    "files": [
      ".gitignore",
      ".prettierignore",
      ".prettierrc.json",
      "Cargo.lock",
      "README.md",
      "crates/jj-store-cli/tests/server_executable.rs",
      "crates/jj-store-http/Cargo.toml",
      "crates/jj-store-http/examples/browser_preview.rs",
      "crates/jj-store-http/src/assets.rs",
      "crates/jj-store-http/src/contract.rs",
      "crates/jj-store-http/src/lib.rs",
      "crates/jj-store-http/tests/assets.rs",
      "crates/jj-store-server/Cargo.toml",
      "crates/jj-store-server/src/config.rs",
      "crates/jj-store-server/src/process.rs",
      "docs/browser.md",
      "docs/cli/server.md",
      "docs/evidence/m4-browser.md",
      "package.json",
      "pnpm-lock.yaml",
      "schemas/openapi.json",
      "scripts/browser-fixtures.mjs",
      "scripts/contracts.types.ts",
      "scripts/web-manifest.mjs",
      "spec/src/api.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/security.md",
      "spec/src/spec-governance.md",
      "spec/src/verification.md",
      "spec-validator.config.mjs",
      "web/index.html",
      "web/playwright.config.ts",
      "web/src/App.svelte",
      "web/src/lib/Graph.svelte",
      "web/src/lib/TextPreview.svelte",
      "web/src/lib/api/client.ts",
      "web/src/lib/api/generated.ts",
      "web/src/lib/graph.ts",
      "web/src/main.ts",
      "web/src/style.css",
      "web/src/tokens.css",
      "web/svelte.config.js",
      "web/tests/browser/reads.spec.ts",
      "web/tests/client.test.ts",
      "web/tsconfig.json",
      "web/vite.config.ts"
    ],
    "tests": [
      "web/tests/client.test.ts#empty_file_range_retries_a_bounded_full_read",
      "web/tests/client.test.ts#graph_rendering_bounds_nodes_and_edges_before_layout",
      "web/tests/browser/reads.spec.ts#native_graphs_files_and_source_rendering_are_accessible",
      "web/tests/browser/reads.spec.ts#event_reconnect_resumes_and_reset_requires_explicit_refresh",
      "web/tests/browser/reads.spec.ts#graph_and_directory_pages_retain_exact_snapshots_and_query_context",
      "web/tests/browser/reads.spec.ts#index_preparation_retries_the_original_query_after_context_edits",
      "crates/jj-store-http/tests/assets.rs#browser_assets_are_allowlisted_preloaded_and_sandboxed",
      "crates/jj-store-cli/tests/server_executable.rs#cold_tls_server_serves_only_the_configured_browser_bundle"
    ],
    "runs": [
      {
        "command": "pnpm web:check",
        "result": "fail",
        "failures": 1,
        "note": "Before browser implementation the web:check command did not exist."
      },
      {
        "command": "pnpm web:check",
        "result": "fail",
        "failures": 3,
        "note": "Initial implementation: recursive task result inference and two missing operation-diff discriminator errors."
      },
      {
        "command": "pnpm test:contracts",
        "result": "fail",
        "failures": 4,
        "note": "TypeScript consumer operationParent reproduced loss of reference-sibling discriminator; fixed schemars reference normalization and regenerated artifacts."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:generate",
        "result": "fail",
        "failures": 1,
        "note": "Initial RemoveRefSiblings construction failed E0603; corrected the non-exhaustive transform to Default construction."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser",
        "result": "fail",
        "failures": 4,
        "note": "Matching Playwright browser absent; installed chromium headless shell 1243."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser",
        "result": "fail",
        "failures": 4,
        "note": "Actual browser fetch called with Client receiver, preventing login. Default fetch now forwards through a closure."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep native_graphs_files_and_source_rendering_are_accessible",
        "result": "fail",
        "failures": 1,
        "note": "Binary file locator incorrectly included an aria-hidden marker; corrected to the actual accessible name bin file."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep native_graphs_files_and_source_rendering_are_accessible",
        "result": "fail",
        "failures": 1,
        "note": "Axe identified scrollable-region-focusable on operation metadata; labeled keyboard-focusable text regions fix this."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep native_graphs_files_and_source_rendering_are_accessible",
        "result": "pass",
        "failures": 0,
        "note": "Native views, source rendering, metadata, disconnect and axe pass under production asset CSP."
      },
      {
        "command": "pnpm web:check",
        "result": "fail",
        "failures": 3,
        "note": "TextPreview refactor initially missed three multiline pre tags, causing two parse diagnostics and a missing App export; corrected every tag."
      },
      {
        "command": "pnpm web:check",
        "result": "fail",
        "failures": 1,
        "note": "Retained query union needed an explicit record type; fixed without weakening generated DTO types."
      },
      {
        "command": "pnpm web:check",
        "result": "pass",
        "failures": 0,
        "note": "Svelte/TypeScript reports zero errors and zero warnings."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_serves_only_the_configured_browser_bundle --locked -- --exact",
        "result": "fail",
        "failures": 1,
        "note": "Unknown ordinary HTTP asset path inherited Tonic HTTP 200 fallback; expected 404."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_serves_only_the_configured_browser_bundle --locked -- --exact",
        "result": "pass",
        "failures": 0,
        "note": "Cold real TLS executable serves configured assets/CSP, denies unauthenticated reads, returns ordinary HTTP 404 and preserves unknown gRPC service status 12."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep event_reconnect_resumes_and_reset_requires_explicit_refresh",
        "result": "fail",
        "failures": 1,
        "note": "Extended regression confirmed explicit refresh after reset did not resubscribe."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep event_reconnect_resumes_and_reset_requires_explicit_refresh",
        "result": "pass",
        "failures": 0,
        "note": "Refresh aborts old feed and starts a fresh subscription; reconnect retains the fully parsed prior cursor before reset."
      },
      {
        "command": "pnpm web:test",
        "result": "fail",
        "failures": 1,
        "note": "Empty file range returned file_unavailable; retry only bytes */0 with a bounded authenticated full read."
      },
      {
        "command": "pnpm web:test",
        "result": "pass",
        "failures": 0,
        "note": "Eight focused client/layout tests pass, including empty files, lossless numbers, bounded graph rendering and revocation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser",
        "result": "pass",
        "failures": 0,
        "note": "Eight cold browser interactions pass before final reset/empty-file additions; includes axe desktop/mobile, pagination, index retry and stale file completion. Full final verification remains pending."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --test assets --locked",
        "result": "pass",
        "failures": 0,
        "note": "Three asset allowlist/preload, invalid-manifest and symlink tests pass."
      },
      {
        "command": "pnpm check:progress",
        "result": "fail",
        "failures": 2,
        "note": "Expected missing ledger update and evidence entry before recording E-M4-017."
      },
      {
        "command": "pnpm format:rust:check",
        "result": "fail",
        "failures": 1,
        "note": "Pinned rustfmt rejects the newly authored unformatted Rust sources; formatting is required before acceptance."
      },
      {
        "command": "pnpm format:rust:check",
        "result": "pass",
        "failures": 0,
        "note": "Pinned rustfmt now passes all Rust source, examples and tests after pnpm format."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "58 requirements, package boundaries, progress, rustfmt, 25 serialized contract responses, Svelte and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "246 Rust tests pass / 6 explicit external tests ignored; 98 native lifecycle cases, 16 governance tests, 3 storage-harness self-tests, 5 schema tests, TypeScript consumer checks and 8 browser-client unit tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep graph_and_directory_pages_retain_exact_snapshots_and_query_context",
        "result": "fail",
        "failures": 1,
        "note": "Extended regression showed snapshot bar using editable operation text rather than loaded snapshot identity."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep graph_and_directory_pages_retain_exact_snapshots_and_query_context",
        "result": "pass",
        "failures": 0,
        "note": "Loaded snapshot label remains exact while operation/revset inputs change. Continuations retain original operation, query/time and reject mixed snapshots for graph and directory pages."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Production Svelte assets and complete locked Rust workspace build pass; JS 75.63 kB / 27.95 kB gzip and CSS 7.87 kB / 2.42 kB gzip."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 1,
        "note": "Browser analysis/unit/eight cold interactions, contracts, Rust formatting/tests and mdBook pass; final spec-first rejected new docs/browser.md without an owning map."
      },
      {
        "command": "pnpm spec:first",
        "result": "fail",
        "failures": 1,
        "note": "Reproduced docs/browser.md missing chapter mapping. Mapped it to api.md and made governance table pipe escaping valid Markdown."
      },
      {
        "command": "pnpm spec:first",
        "result": "pass",
        "failures": 0,
        "note": "41 protected files and 7 owning canonical chapters pass."
      },
      {
        "command": "pnpm spec:validate",
        "result": "pass",
        "failures": 0,
        "note": "Updated map/table validate with 58 requirements and exactly one verification row per requirement."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 lanes pass on final implementation: eight cold browser interactions including axe and reset/refresh/snapshot-label regressions; eight client unit tests; 246 Rust tests with six external tests explicitly ignored; contract conformance, rustfmt, governance, mdBook and mapped spec-first."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Pinned cargo fmt --all -- --check and Prettier pass across authored Rust, Svelte, TypeScript, Markdown and tooling."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; Svelte 5.57.1; Vite 8.3.1; Playwright 1.63.0; Chromium headless shell 1243; axe 4.13.0; lossless-json 4.3.1.",
    "formats": "Browser asset manifest V1 added. Existing native, RPC and HTTP wire formats unchanged; OpenAPI reference siblings normalized to allOf for discriminator-preserving client generation. Enclosing Jujutsu change is the revision reference. M4.2 retains real RustFS/SSE deployment qualification; E-M4-015 records stopped shared Docker engine and pending restart approval. M5 mutations, M6 search and M8a hardening remain unfinished. Browser visual review remains visual-pending."
  },
  {
    "id": "E-M5-001",
    "tasks": ["M5", "M5.1"],
    "date": "2026-09-27",
    "kind": "native-command-preparation",
    "files": [
      "crates/jj-store-jj/src/commands/mod.rs",
      "crates/jj-store-jj/src/commands/access.rs",
      "crates/jj-store-jj/src/commands/execute.rs",
      "crates/jj-store-jj/src/lib.rs",
      "crates/jj-store-jj/src/query/access.rs",
      "crates/jj-store-jj/src/query/mod.rs",
      "crates/jj-store-jj/src/query/workers.rs",
      "crates/jj-store-service/src/commands.rs",
      "crates/jj-store-service/src/lib.rs",
      "crates/jj-store-cli/tests/native_lifecycle.rs",
      "crates/jj-store-cli/tests/native_lifecycle/commands.rs",
      "docs/evidence/m5-command-preparation.md",
      "spec/src/mutations.md",
      "spec/src/jj-integration.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/verification.md",
      "spec/src/milestones.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_describe_prepares_native_descendants_without_publication",
      "crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_new_merges_ordered_parents_and_moves_only_logical_workspace",
      "crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_bookmarks_compare_complete_conflicted_targets_and_publish_once",
      "crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_commands_reject_readers_stale_root_and_uncommitted_targets",
      "crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_preparation_limits_and_protected_publication_leave_authority_unchanged",
      "crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_cancellation_retains_shared_worker_until_native_exit",
      "crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_preparation_rejects_snapshot_changes_during_staging",
      "crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_preparation_rechecks_revocation_during_staging",
      "crates/jj-store-cli/tests/native_lifecycle/commands.rs#typed_commands_distinguish_hidden_noop_and_ambiguous_heads"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_describe_prepares_native_descendants_without_publication --locked -- --exact",
        "result": "fail",
        "failures": 4,
        "note": "Baseline compile: missing service commands module and prepare_command method; two test-only uses of MergedTree::id corrected to tree_ids."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_describe_prepares_native_descendants_without_publication --locked -- --exact",
        "result": "fail",
        "failures": 2,
        "note": "Native ref/workspace name types do not implement Display; use as_str().to_owned()."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_describe_prepares_native_descendants_without_publication --locked -- --exact",
        "result": "fail",
        "failures": 1,
        "note": "E0283 on ambiguous BTreeMap lookup; use typed native RefName."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_describe_prepares_native_descendants_without_publication --locked -- --exact",
        "result": "fail",
        "failures": 1,
        "note": "Executor incorrectly reused graph cache path validation; extracted pure limit validation while retaining graph absolute-path checks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_describe_prepares_native_descendants_without_publication --locked -- --exact",
        "result": "fail",
        "failures": 1,
        "note": "Author fixture assumed native blank fields remained blank; inspected pinned commit-builder policy, used explicit original identity/timestamp, and asserted native preservation. Describe uses pinned reparenting semantics."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance:: --locked",
        "result": "fail",
        "failures": 8,
        "note": "Expanded fixture inherited auth Result alias; qualify the command result type."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance:: --locked",
        "result": "pass",
        "failures": 0,
        "note": "Initial five native command fixtures passed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_new_merges_ordered_parents_and_moves_only_logical_workspace --locked -- --exact",
        "result": "fail",
        "failures": 1,
        "note": "Strengthened assertion exposed New being reported as a rewrite because native predecessor vectors were empty."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_new_merges_ordered_parents_and_moves_only_logical_workspace --locked -- --exact",
        "result": "pass",
        "failures": 0,
        "note": "Empty predecessor vectors now contribute only to created reports; native operation data is unchanged."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance:: --locked",
        "result": "pass",
        "failures": 0,
        "note": "Eight fixtures pass including shared worker cancellation, mid-flight snapshot changes and token revocation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance::typed_commands_distinguish_hidden_noop_and_ambiguous_heads --locked -- --exact",
        "result": "pass",
        "failures": 0,
        "note": "Hidden Describe is rejected; committed historical parent New works; no-op reports no effects; multiple heads are rejected without reconciliation."
      },
      {
        "command": "pnpm format:rust:check",
        "result": "fail",
        "failures": 1,
        "note": "Pinned rustfmt detects unformatted newly authored sources."
      },
      {
        "command": "pnpm format:rust:check",
        "result": "pass",
        "failures": 0,
        "note": "cargo fmt --all -- --check passes after pnpm format:rust."
      },
      {
        "command": "pnpm check:progress",
        "result": "fail",
        "failures": 2,
        "note": "Before recording E-M5-001: missing same-change ledger update and evidence entry."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle command_acceptance:: --locked",
        "result": "pass",
        "failures": 0,
        "note": "All nine command preparation acceptance tests pass after formatting; zero failures."
      },
      {
        "command": "pnpm check:progress",
        "result": "pass",
        "failures": 0,
        "note": "31 task references, same-change evidence and named tests pass. Bookkeeping does not complete M5."
      },
      {
        "command": "pnpm spec:first",
        "result": "pass",
        "failures": 0,
        "note": "12 protected implementation/report files and five owning chapters pass mapped spec-first."
      },
      {
        "command": "pnpm check:boundaries",
        "result": "pass",
        "failures": 0,
        "note": "All nine Rust package boundaries pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Specification, progress, rustfmt, 25 actual contract responses, five schema tests, Svelte and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "255 Rust tests pass, six external tests explicitly ignored; 107 native lifecycle tests, 16 governance tests, three storage-harness self-tests, five schema tests, TypeScript consumer checks and eight browser-client unit tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Production browser bundle and locked Rust workspace build pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 lanes pass, including eight cold Playwright interactions with axe, 255 Rust tests with six explicit external ignores, native DTO contracts, rustfmt, governance, mdBook and spec-first."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Pinned rustfmt and repository Prettier checks pass."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.",
    "formats": "No persistence, WAL, native RPC or public HTTP format changes. Internal owned command and impact-report DTOs introduced. Enclosing Jujutsu change is the revision reference. M5 remains In progress: durable preview/apply/status, reconciliation, abandon, conflict resolution and transport/browser integration remain. M4 real RustFS/SSE/browser qualification remains open in E-M4-015; no shared Docker restart approval has been received."
  },
  {
    "id": "E-SCOPE-001",
    "tasks": ["M4", "M4.2", "M4.3", "M5", "M5.1", "M6.2", "M8a", "M8a.1"],
    "date": "2026-09-27",
    "kind": "user-requested-scope-deferral",
    "files": [
      "AGENTS.md",
      "docs/implementation-plan.md",
      "spec/src/architecture.md",
      "spec/src/spec-governance.md",
      "spec/src/api.md",
      "spec/src/mutations.md",
      "spec/src/milestones.md"
    ],
    "tests": [],
    "runs": [
      {
        "command": "jj --no-pager st",
        "result": "pass",
        "failures": 0,
        "note": "Clean starting state after verified M5 preparation commit 464f05b5; no UI implementation changed."
      },
      {
        "command": "pnpm spec:validate",
        "result": "pass",
        "failures": 0,
        "note": "58 requirements and one verification row per requirement pass."
      },
      {
        "command": "pnpm spec:first",
        "result": "pass",
        "failures": 0,
        "note": "Scope update maps to architecture and governance chapters."
      },
      {
        "command": "pnpm check:progress",
        "result": "pass",
        "failures": 0,
        "note": "32 tasks with deferred UI separated from backend dependencies pass."
      },
      {
        "command": "pnpm spec:build",
        "result": "pass",
        "failures": 0,
        "note": "Canonical mdBook builds."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Pinned rustfmt and Prettier checks pass; no UI sources edited."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1; jj-cli/jj-lib 0.45.1; released spec-validator ^0.1.0 unchanged.",
    "formats": "No implementation, lockfile or storage/API format changes. User requested deferring further UI changes while continuing all other tasks. M4.3 preserves browser mutation/conflict/search and deployment acceptance; M4.2 retains SSE/API conformance; M5 depends on completed M4.1. Full UI release requirements remain uncompleted. Existing browser sources/tests are unchanged. Enclosing Jujutsu change is the revision reference."
  },
  {
    "id": "E-M5-002",
    "tasks": ["M5", "M5.1"],
    "date": "2026-09-27",
    "kind": "durable-native-command-service",
    "files": [
      "crates/jj-store-wal/src/publication.rs",
      "crates/jj-store-wal/src/publication/application.rs",
      "crates/jj-store-wal/tests/publication.rs",
      "crates/jj-store-wal/tests/publication/application.rs",
      "crates/jj-store-service/src/lib.rs",
      "crates/jj-store-service/src/mutations.rs",
      "crates/jj-store-jj/src/commands/mod.rs",
      "crates/jj-store-jj/src/query/mod.rs",
      "crates/jj-store-cli/tests/native_lifecycle.rs",
      "crates/jj-store-cli/tests/native_lifecycle/mutations.rs",
      "docs/evidence/m5-durable-commands.md",
      "spec/src/publication.md",
      "spec/src/mutations.md",
      "spec/src/jj-integration.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/verification.md",
      "spec/src/milestones.md"
    ],
    "tests": [
      "crates/jj-store-wal/tests/publication/application.rs#application_receipt_survives_regenerated_candidates_restart_and_checkpoint",
      "crates/jj-store-wal/tests/publication/application.rs#application_competing_candidates_return_only_the_manifest_selected_result",
      "crates/jj-store-wal/tests/publication/application.rs#application_unknown_outcomes_retain_identity_without_claiming_rejection",
      "crates/jj-store-wal/tests/publication/application.rs#application_identity_changes_conflict_and_native_digest_profile_is_unchanged",
      "crates/jj-store-wal/tests/publication/application.rs#application_guard_checks_staging_and_reauthorizes_receipt_replay",
      "crates/jj-store-wal/tests/publication/application.rs#application_receipt_lookup_rejects_selected_identity_position_and_index_corruption",
      "crates/jj-store-cli/tests/native_lifecycle/mutations.rs#typed_apply_returns_original_native_result_after_restart_and_later_publication",
      "crates/jj-store-cli/tests/native_lifecycle/mutations.rs#typed_mutations_reject_stale_intent_conflicts_readers_and_protected_previews",
      "crates/jj-store-cli/tests/native_lifecycle/mutations.rs#typed_mutation_status_is_principal_scoped_and_preview_does_not_authorize_apply",
      "crates/jj-store-cli/tests/native_lifecycle/mutations.rs#typed_identical_apply_races_return_one_result_during_preparation_or_manifest_cas",
      "crates/jj-store-cli/tests/native_lifecycle/mutations.rs#typed_apply_unknown_status_and_retry_recover_the_original_selected_result",
      "crates/jj-store-cli/tests/native_lifecycle/mutations.rs#typed_bookmark_receipts_replay_after_move_and_delete_without_resurrection"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication application_acceptance:: --locked",
        "result": "fail",
        "failures": 15,
        "note": "Acceptance fixture initially lacked ApplicationIntent, publish/status application methods and receipt-by-request lookup."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication application_acceptance:: --locked",
        "result": "pass",
        "failures": 0,
        "note": "First three candidate retry, race and unknown-outcome fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication application_acceptance:: --locked",
        "result": "pass",
        "failures": 0,
        "note": "Five fixtures pass after adding stable-input/native-profile and current-guard checks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle mutation_acceptance::typed_apply_returns_original_native_result_after_restart_and_later_publication --locked -- --exact",
        "result": "fail",
        "failures": 2,
        "note": "Native service fixture initially lacked the mutations module and typed result service."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle mutation_acceptance::typed_apply_returns_original_native_result_after_restart_and_later_publication --locked -- --exact",
        "result": "fail",
        "failures": 3,
        "note": "Service incorrectly referenced private WAL encoding helper; added service-owned bounded JSON encoding."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle mutation_acceptance::typed_apply_returns_original_native_result_after_restart_and_later_publication --locked -- --exact",
        "result": "pass",
        "failures": 0,
        "note": "Native empty-repository preview, New, Describe and retry after later publication/restart pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle mutation_acceptance:: --locked",
        "result": "pass",
        "failures": 0,
        "note": "Five native service fixtures pass, including stale/protected/reader/revocation checks and both preparation/CAS races."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication application_acceptance:: --locked",
        "result": "pass",
        "failures": 0,
        "note": "Six final WAL fixtures pass, including frozen independently calculated intent digest, zero-write preview checks and selected identity/position/index corruption."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle mutation_acceptance:: --locked",
        "result": "pass",
        "failures": 0,
        "note": "Six final native service fixtures pass, including checkpoint restart and late bookmark create/move receipts after deletion."
      },
      {
        "command": "pnpm check:progress",
        "result": "fail",
        "failures": 2,
        "note": "Before adding E-M5-002: missing same-change ledger update and evidence entry."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-wal -p jj-store-service --all-targets --locked -- -D warnings",
        "result": "fail",
        "failures": 2,
        "note": "Clippy found two large enum variants in Outcome/Status (352-byte receipt versus 16-byte request ID). Boxed receipt values without changing serialization."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-wal -p jj-store-service --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0,
        "note": "Exact owning-package all-target Clippy target passes after boxing retained results."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle mutation_acceptance:: --locked",
        "result": "pass",
        "failures": 0,
        "note": "All six final native service fixtures pass again after bounded enum layout repair."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt, workspace all-target Clippy, governance/progress/boundaries, unchanged generated contracts with 25 actual HTTP/SSE responses, five schema tests and Svelte analysis pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "267 Rust tests including 113 native lifecycle tests; six external tests explicitly ignored. Also 16 governance, three storage-harness, five schema and eight browser client tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Production browser assets and locked Rust workspace build pass; no browser source changes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 lanes pass, including eight cold Playwright interaction/accessibility tests, mdBook, spec-first, rustfmt and workspace checks."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Pinned rustfmt and Prettier pass."
      },
      {
        "command": "pnpm spec:first",
        "result": "pass",
        "failures": 0,
        "note": "11 protected files mapped to six canonical chapters."
      },
      {
        "command": "pnpm check:progress",
        "result": "pass",
        "failures": 0,
        "note": "32 task records validate; bookkeeping does not close remaining M5 gates."
      },
      {
        "command": "jj --no-pager diff --name-only -- web",
        "result": "pass",
        "failures": 0,
        "note": "Empty: no UI files changed."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.",
    "formats": "Adds application intent V1 and selected StoredResult V1 in existing WAL receipt result bytes. Existing native/token request digest profile, native object codecs, WAL envelopes, RPC and HTTP formats are unchanged. No UI files changed. Enclosing Jujutsu change is the revision reference. M5.1 retains HTTP exposure, capability registry and reconciliation; M5.2 abandon and M5.3 conflict resolution remain. M4.2 RustFS/SSE qualification awaits the stopped shared Docker engine; browser work is Deferred under M4.3 per user instruction."
  },
  {
    "id": "E-M5-003",
    "tasks": ["M5", "M5.1"],
    "date": "2026-09-27",
    "kind": "typed-http-mutation-api",
    "files": [
      "Cargo.lock",
      "README.md",
      "crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs",
      "crates/jj-store-cli/tests/native_lifecycle/http_reads.rs",
      "crates/jj-store-cli/tests/native_lifecycle.rs",
      "crates/jj-store-http/src/contract/tests.rs",
      "crates/jj-store-http/src/contract.rs",
      "crates/jj-store-http/src/dto.rs",
      "crates/jj-store-http/src/error.rs",
      "crates/jj-store-http/src/lib.rs",
      "crates/jj-store-http/src/mutations.rs",
      "crates/jj-store-http/src/routes.rs",
      "crates/jj-store-service/src/mutations.rs",
      "crates/jj-store-wal/Cargo.toml",
      "crates/jj-store-wal/src/publication.rs",
      "docs/evidence/m5-http-mutations.md",
      "schemas/README.md",
      "schemas/openapi.json",
      "scripts/contracts.mjs",
      "scripts/contracts.test.mjs",
      "scripts/contracts.types.ts",
      "spec/src/api.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/milestones.md",
      "spec/src/mutations.md",
      "spec/src/publication.md",
      "spec/src/spec-governance.md",
      "spec/src/verification.md",
      "web/src/lib/api/generated.ts"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_mutations_preview_apply_status_replay_and_contract_capture",
      "crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_mutations_authentication_strict_json_limits_and_stale_requests_publish_nothing",
      "crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_mutations_run_all_advertised_commands_and_late_bookmark_retries",
      "crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_mutations_enforce_protection_revocation_and_principal_scoped_status",
      "crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_mutations_small_result_budget_preserves_unknown_identity_and_recoverable_receipt",
      "crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_mutations_deadline_after_manifest_cas_retains_request_identity",
      "crates/jj-store-cli/tests/native_lifecycle/http_reads.rs#http_mutation_previews_share_native_worker_admission_with_file_reads",
      "crates/jj-store-http/src/contract/tests.rs#mutation_contracts_require_typed_intent_and_write_permission",
      "scripts/contracts.test.mjs#mutation_requests_are_strict_and_unknown_outcomes_retain_identity"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_mutation_acceptance:: --locked",
        "result": "fail",
        "failures": 2,
        "note": "Two initial acceptance tests returned 404 for absent mutation routes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_mutation_acceptance:: --locked",
        "result": "pass",
        "failures": 0,
        "note": "Both original route/authorization fixtures pass after implementation; expanded target subsequently passed six tests."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_mutation --locked",
        "result": "pass",
        "failures": 0,
        "note": "Seven native HTTP fixtures pass, including shared read/command admission and deadline after successful manifest CAS."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --locked",
        "result": "pass",
        "failures": 0,
        "note": "Four compiled contract tests and three existing asset tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:check",
        "result": "fail",
        "failures": 2,
        "note": "34 actual response schemas and six Node tests passed; formatter moved TypeScript directive off its intended property, yielding unused-directive and intentional invalid-command diagnostics. Moved directive to the property; assertion unchanged."
      },
      {
        "command": "pnpm check:progress",
        "result": "fail",
        "failures": 2,
        "note": "Expected missing same-change ledger/evidence errors before E-M5-003."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:check",
        "result": "pass",
        "failures": 0,
        "note": "Exact rerun passes after fixing TypeScript directive location: 34 actual HTTP/SSE responses, six schema tests and TypeScript consumer checks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-http -p jj-store-service -p jj-store-wal --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0,
        "note": "Owning-package Clippy passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "All normal lanes pass, including rustfmt, workspace all-target Clippy, boundaries, specification/progress, contracts and zero Svelte diagnostics."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "275 Rust tests pass including 120 native lifecycle tests, with six external tests explicitly ignored. 16 governance, three storage-harness, six schema and eight browser client tests also pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Production assets and Rust workspace build pass. Browser runtime bundle unchanged."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 lanes pass, including eight cold Playwright interaction/accessibility tests, mdBook, spec-first, rustfmt, contracts and workspace checks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Pinned rustfmt and Prettier pass."
      },
      {
        "command": "pnpm check:progress",
        "result": "pass",
        "failures": 0,
        "note": "32 tasks validate; this is bookkeeping, not completion of remaining mutation gates."
      },
      {
        "command": "pnpm spec:first",
        "result": "pass",
        "failures": 0,
        "note": "Mapped protected file/chapter gate passes."
      },
      {
        "command": "jj --no-pager diff --name-only -- web",
        "result": "pass",
        "failures": 0,
        "note": "Only web/src/lib/api/generated.ts changed; no browser behavior, component, test or visual changes."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.",
    "formats": "Adds V1 HTTP preview/apply/status and capability entries for five commands, generated OpenAPI/TypeScript and WAL Snapshot schema metadata. Adds existing schemars dependency to WAL lock entry; persisted native/WAL/intent/StoredResult formats unchanged. No browser feature or visual changes; generated TypeScript declaration is the sole web edit. Enclosing Jujutsu change is the revision reference. ReconcileOperations, Abandon and ResolveConflicts remain M5 gates. Real RustFS/TLS SSE qualification awaits stopped shared Docker; browser feature and deployment gates remain Deferred M4.3."
  },
  {
    "id": "E-M5-004",
    "tasks": ["M5", "M5.1"],
    "date": "2026-09-27",
    "kind": "explicit-native-reconciliation",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs",
      "crates/jj-store-cli/tests/native_lifecycle/reconciliation.rs",
      "crates/jj-store-cli/tests/native_lifecycle.rs",
      "crates/jj-store-http/src/contract/tests.rs",
      "crates/jj-store-http/src/dto.rs",
      "crates/jj-store-jj/src/commands/execute.rs",
      "crates/jj-store-jj/src/commands/mod.rs",
      "crates/jj-store-jj/src/commands/reconcile.rs",
      "crates/jj-store-service/src/commands.rs",
      "crates/jj-store-service/src/mutations.rs",
      "docs/evidence/m5-reconciliation.md",
      "schemas/openapi.json",
      "spec/src/api.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/jj-integration.md",
      "spec/src/milestones.md",
      "spec/src/mutations.md",
      "spec/src/verification.md",
      "web/src/lib/api/generated.ts"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/reconciliation.rs#typed_reconciliation_merges_native_heads_rebases_descendants_and_replays_receipt",
      "crates/jj-store-cli/tests/native_lifecycle/reconciliation.rs#typed_reconciliation_requires_exact_heads_and_explicit_current_base",
      "crates/jj-store-cli/tests/native_lifecycle/reconciliation.rs#typed_reconciliation_recursive_ancestors_and_conflicted_bookmarks_match_upstream",
      "crates/jj-store-cli/tests/native_lifecycle/reconciliation.rs#typed_reconciliation_resource_and_protection_failures_preserve_heads",
      "crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_reconciliation_preserves_get_heads_and_publishes_only_explicit_apply"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle reconciliation_acceptance:: --locked",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: Command::ReconcileOperations missing; acceptance did not compile."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle reconciliation_acceptance:: --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original two native reconciliation acceptance fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle reconciliation --locked",
        "result": "pass",
        "failures": 0,
        "note": "Five expanded fixtures pass: native rewrites/retry, strict context, recursive ancestors/conflicted bookmarks, protection/resource rejection and explicit HTTP application."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle reconciliation --locked",
        "result": "pass",
        "failures": 0,
        "note": "All five fixtures pass after extending recursive-ancestor acceptance to three current heads."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-jj -p jj-store-service -p jj-store-http --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0,
        "note": "Owning-package Clippy passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "All normal lanes pass: rustfmt, all-target Clippy, progress/boundaries/specification, 34 actual HTTP/SSE schema responses, six schema tests, TypeScript and zero Svelte diagnostics."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "280 Rust tests including 125 native lifecycle tests pass; six external tests explicitly ignored. Also 16 governance, three storage-harness, six schema and eight browser client tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Production assets and locked Rust workspace build pass; browser runtime bundle unchanged."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 lanes pass, including eight cold Playwright interaction/accessibility tests, mdBook, spec-first, rustfmt and existing workspace/contract lanes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Pinned rustfmt and Prettier pass."
      },
      {
        "command": "pnpm check:progress",
        "result": "pass",
        "failures": 0,
        "note": "32 tasks validate."
      },
      {
        "command": "pnpm spec:first",
        "result": "pass",
        "failures": 0,
        "note": "13 protected files mapped to six canonical chapters pass."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.",
    "formats": "Adds reconcile_operations with explicit current base to typed Command and generated HTTP contracts/capabilities. Reports, StoredResult V1, other canonical command digests, native codecs and WAL formats are unchanged. HeadsNormalized is the existing semantic kind. No browser behavior or visual changes; generated TypeScript only. Enclosing Jujutsu change is the revision reference. Abandon and ResolveConflicts remain M5 gates. Real RustFS/TLS qualification remains blocked by stopped shared Docker; M4.3 browser work stays Deferred. M5.1 basic command, preview/apply/status, receipt and reconciliation gate is complete; M5 remains In progress for abandon and conflict resolution."
  },
  {
    "id": "E-M5-005",
    "tasks": ["M5", "M5.2"],
    "date": "2026-09-27",
    "kind": "native-typed-abandon",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/abandon.rs",
      "crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs",
      "crates/jj-store-cli/tests/native_lifecycle.rs",
      "crates/jj-store-http/src/contract/tests.rs",
      "crates/jj-store-http/src/contract.rs",
      "crates/jj-store-http/src/dto.rs",
      "crates/jj-store-http/src/error.rs",
      "crates/jj-store-jj/src/commands/abandon.rs",
      "crates/jj-store-jj/src/commands/execute.rs",
      "crates/jj-store-jj/src/commands/mod.rs",
      "docs/evidence/m5-abandon.md",
      "docs/licenses/NOTICE.md",
      "schemas/openapi.json",
      "scripts/contracts.mjs",
      "spec/src/api.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/jj-integration.md",
      "spec/src/milestones.md",
      "spec/src/mutations.md",
      "spec/src/spec-governance.md",
      "spec/src/verification.md",
      "web/src/lib/api/generated.ts"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/abandon.rs#typed_abandon_preserves_native_descendant_bookmark_workspace_and_history_semantics",
      "crates/jj-store-cli/tests/native_lifecycle/abandon.rs#typed_abandon_adjacent_targets_and_hidden_noop_keep_complete_native_history",
      "crates/jj-store-cli/tests/native_lifecycle/abandon.rs#typed_abandon_merge_retains_ordered_parents_and_native_conflicted_bookmark_terms",
      "crates/jj-store-cli/tests/native_lifecycle/abandon.rs#typed_abandon_rejects_root_protected_unknown_and_invalid_batches_atomically",
      "crates/jj-store-cli/tests/native_lifecycle/abandon.rs#typed_abandon_report_extension_preserves_legacy_v1_canonical_receipt_bytes",
      "crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs#http_abandon_previews_native_effects_and_reports_hidden_noop_without_publication"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle abandon_acceptance:: --locked",
        "result": "fail",
        "failures": 4,
        "note": "Initial compile failure: absent Abandon variant and two report fields, plus fixture used nonexistent MergedTree::names. Repaired fixture to native entries iterator and implemented command/report."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle abandon_acceptance:: --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original four-option fixture passed; removed one unused-import warning."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle abandon_acceptance:: --locked",
        "result": "fail",
        "failures": 1,
        "note": "Expanded evolution fixture needed Arc<CommittedRead> for the graph method; corrected capability ownership without changing assertions."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle abandon_acceptance:: --locked",
        "result": "pass",
        "failures": 0,
        "note": "All five expanded native fixtures passed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_abandon --locked",
        "result": "pass",
        "failures": 0,
        "note": "Five final native fixtures pass with binary/mode/copy/symlink/submodule value preservation added."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_abandon --locked",
        "result": "pass",
        "failures": 0,
        "note": "HTTP preview/apply/status/retry and no-publication hidden no-op pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "All normal lanes pass, including rustfmt, workspace all-target Clippy, specification/progress/boundaries, 37 actual HTTP/SSE response schemas, six schema tests, generated TypeScript and zero Svelte diagnostics."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "286 Rust tests including 131 native lifecycle tests pass; six external tests explicitly ignored. Also 16 governance, three storage-harness, six schema and eight browser client tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Production assets and locked Rust workspace build pass. Browser runtime bundle unchanged."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 lanes pass, including eight cold Playwright interaction/accessibility tests, mdBook, spec-first, rustfmt and workspace/contract lanes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Pinned rustfmt and Prettier pass."
      },
      {
        "command": "pnpm check:progress",
        "result": "pass",
        "failures": 0,
        "note": "32 tasks validate."
      },
      {
        "command": "pnpm spec:first",
        "result": "pass",
        "failures": 0,
        "note": "Mapped protected-file/canonical-chapter gate passes."
      },
      {
        "command": "jj --no-pager diff --name-only -- web",
        "result": "pass",
        "failures": 0,
        "note": "Only the generated TypeScript API declaration changes; no UI feature, behavior or visual changes."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.",
    "formats": "Adds abandon typed command and capability, optional default-empty/empty-skipped abandoned and skipped_hidden report lists, and HTTP no_changes code. StoredResult V1 and existing report canonical bytes remain compatible; native codecs/WAL formats unchanged. No browser behavior/visual changes; generated TypeScript only. Enclosing Jujutsu change is the revision reference. ResolveConflicts remains M5.3; real RustFS/TLS SSE qualification awaits stopped shared Docker; M4.3 stays Deferred. M5.2 native abandon gate is complete; M5 remains In progress for conflict resolution."
  },
  {
    "id": "E-M5-006",
    "tasks": ["M5", "M5.3"],
    "date": "2026-09-27",
    "kind": "atomic-native-conflict-terms",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/conflicts.rs",
      "crates/jj-store-cli/tests/native_lifecycle/http_mutations.rs",
      "crates/jj-store-cli/tests/native_lifecycle/resolve.rs",
      "crates/jj-store-http/src/contract/tests.rs",
      "crates/jj-store-http/src/contract.rs",
      "crates/jj-store-http/src/dto.rs",
      "crates/jj-store-http/src/error.rs",
      "crates/jj-store-jj/src/commands/execute.rs",
      "crates/jj-store-jj/src/commands/mod.rs",
      "crates/jj-store-jj/src/commands/resolve.rs",
      "crates/jj-store-jj/src/query/content/conflicts.rs",
      "crates/jj-store-service/src/commands.rs",
      "docs/evidence/m5-resolve-terms.md",
      "schemas/openapi.json",
      "scripts/contracts.mjs",
      "spec/src/api.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/jj-integration.md",
      "spec/src/milestones.md",
      "spec/src/mutations.md",
      "spec/src/spec-governance.md",
      "spec/src/verification.md",
      "web/src/lib/api/generated.ts"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_native_terms_are_atomic_preserve_other_conflicts_and_retry_receipts",
      "crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_rejects_the_whole_stale_or_invalid_batch_before_staging",
      "crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_accepts_multisided_base_absent_and_directory_terms",
      "crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_rebases_descendants_updates_references_and_preserves_evolution",
      "crates/jj-store-cli/tests/native_lifecycle/resolve.rs#http_resolve_validates_atomic_fingerprints_permissions_and_selected_receipts",
      "crates/jj-store-jj/src/query/content/conflicts.rs#conflict_fingerprint_v1_matches_independent_canonical_vector",
      "crates/jj-store-cli/tests/native_lifecycle/abandon.rs#typed_abandon_report_extension_preserves_legacy_v1_canonical_receipt_bytes"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve --locked",
        "result": "fail",
        "failures": 1,
        "note": "Initial acceptance fails: resolve_conflicts is not a recognized typed command."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original native variant/partial resolution/retry test passes. Removed one unused-import warning."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve --locked",
        "result": "fail",
        "failures": 1,
        "note": "Expanded fixtures: 3 pass, 1 fixture fails codec validation because unlabeled five-term conflict supplied only one empty label. Corrected fixture to five empty labels, preserving native arity."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle resolve --locked",
        "result": "pass",
        "failures": 0,
        "note": "All 5 focused native/HTTP fixtures pass, including term variants, stale/invalid no-write batches, descendants, references, policy, revocation and durable retry."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve --locked",
        "result": "pass",
        "failures": 0,
        "note": "Final four native fixtures pass after explicitly testing an out-of-range term index independent of duplicate-path validation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle resolve --locked",
        "result": "pass",
        "failures": 0,
        "note": "All five fixtures pass after exporting owned resolution types through the service; fixture constructs selections through that public boundary."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Normal checks pass, including pinned rustfmt, Clippy, boundaries/progress/spec validation, 40 actual HTTP/SSE response schemas, six schema tests and zero Svelte diagnostics. Repeated after adding service-owned DTO exports."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "291 Rust tests pass, including 136 native lifecycle cases; six external tests explicitly ignored. Also 16 governance, three harness, six schema and eight browser client tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Production asset and locked Rust workspace builds pass, including final service export integration. Browser runtime bundle unchanged."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 lanes pass, including final Rust workspace tests, eight cold browser interaction/accessibility tests, rustfmt, contracts, mdBook and spec-first."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Pinned rustfmt and Prettier pass."
      },
      {
        "command": "jj --no-pager diff --name-only -- web",
        "result": "pass",
        "failures": 0,
        "note": "Only generated TypeScript API declarations change. No UI behavior, feature or visual changes."
      },
      {
        "command": "ps -p 54647,54802 -o pid,ppid,etime,stat,command",
        "result": "pass",
        "failures": 0,
        "note": "Existing isolated RustFS harness and Docker volume child still running at the previously recorded wait. No restart or duplicate harness launched; no new deployment result claimed."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.",
    "formats": "Adds resolve_conflicts typed variant with bounded paths, fingerprint, select_term/delete actions and stale_conflict HTTP code/capability. No native codec, WAL, StoredResult V1 or Prepared format changes; earlier receipts reencode unchanged. Only generated TypeScript changes under web; UI stays Deferred M4.3. Enclosing Jujutsu change is the revision reference. Authorized uploaded file replacements remain M5.3; real RustFS/TLS SSE qualification awaits the stopped shared Docker engine."
  },
  {
    "id": "E-M5-007",
    "tasks": ["M5", "M5.3"],
    "date": "2026-09-27",
    "kind": "authorized-conflict-uploads",
    "files": [
      "Cargo.lock",
      "crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs",
      "crates/jj-store-cli/tests/native_lifecycle/resolve.rs",
      "crates/jj-store-cli/tests/native_lifecycle/upload_faults.rs",
      "crates/jj-store-http/Cargo.toml",
      "crates/jj-store-http/src/contract.rs",
      "crates/jj-store-http/src/contract/tests.rs",
      "crates/jj-store-http/src/dto.rs",
      "crates/jj-store-http/src/error.rs",
      "crates/jj-store-http/src/lib.rs",
      "crates/jj-store-http/src/mutations.rs",
      "crates/jj-store-http/src/routes.rs",
      "crates/jj-store-http/src/uploads.rs",
      "crates/jj-store-jj/src/commands/execute.rs",
      "crates/jj-store-jj/src/commands/mod.rs",
      "crates/jj-store-jj/src/commands/resolve.rs",
      "crates/jj-store-model/src/ids.rs",
      "crates/jj-store-model/tests/framing.rs",
      "crates/jj-store-server/src/process.rs",
      "crates/jj-store-service/src/commands.rs",
      "crates/jj-store-service/src/lib.rs",
      "crates/jj-store-service/src/uploads.rs",
      "docs/evidence/m5-conflict-uploads.md",
      "schemas/openapi.json",
      "scripts/contracts.mjs",
      "scripts/contracts.test.mjs",
      "spec/src/api.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/identity.md",
      "spec/src/jj-integration.md",
      "spec/src/milestones.md",
      "spec/src/mutations.md",
      "spec/src/security.md",
      "spec/src/spec-governance.md",
      "web/src/lib/api/generated.ts"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs#conflict_upload_http_stages_verified_binary_without_publication_then_resolves",
      "crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs#conflict_upload_reuse_checks_principal_snapshot_path_and_verified_bytes",
      "crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs#conflict_upload_atomic_mixed_resolution_retains_copy_history_and_other_conflicts",
      "crates/jj-store-cli/tests/native_lifecycle/upload_faults.rs#conflict_upload_lost_ack_repeats_same_record_after_restart_and_stale_binding_rejects",
      "crates/jj-store-cli/tests/native_lifecycle/upload_faults.rs#conflict_upload_revocation_during_body_and_busy_deadline_reject_before_remote_writes",
      "crates/jj-store-cli/tests/native_lifecycle/upload_faults.rs#conflict_upload_http_bounds_bodies_and_authorizes_before_consumption",
      "crates/jj-store-service/src/uploads.rs#conflict_upload_record_v1_has_canonical_independent_digest_and_scope_binding",
      "crates/jj-store-http/src/contract/tests.rs#conflict_upload_contract_has_binary_body_exact_snapshot_and_write_permission"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_upload --locked",
        "result": "fail",
        "failures": 1,
        "note": "Initial HTTP acceptance fails: upload endpoint returns 404 unknown_route."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-service --locked",
        "result": "fail",
        "failures": 1,
        "note": "Initial implementation compile error: Repo trait import missing for native store access; fixed import."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-http",
        "result": "fail",
        "failures": 1,
        "note": "Same native Repo import compile failure before correction; lockfile updated for existing tokio-util streaming adapter."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_upload --locked",
        "result": "fail",
        "failures": 1,
        "note": "First integrated test aborts with a worker stack overflow. Replaced the new 64 KiB stack array in the async native verifier with a heap buffer; unchanged runtime stack configuration."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_upload --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original end-to-end upload/native-byte/atomic-resolution acceptance passes after heap-buffer correction."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_upload --locked",
        "result": "pass",
        "failures": 0,
        "note": "Expanded five fixtures pass; removed one unused fixture parameter warning."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_upload --locked",
        "result": "pass",
        "failures": 0,
        "note": "All six final fixtures pass, including 600,000-byte binary upload and actual HTTP preview/apply, empty/structural/copy resolution, corrupt/missing data, scoped handles, checkpoint receipt replay, lost acknowledgement, revocation, bounded/failed streams and admission deadlines."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service conflict_upload_record --locked",
        "result": "pass",
        "failures": 0,
        "note": "Canonical V1 record independent SHA-256 vector and scope-integrity cases pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "All-target Clippy, rustfmt, spec/boundaries/progress, generated contracts and 43 actual HTTP/SSE samples, schema tests and zero Svelte diagnostics pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "299 Rust tests pass, zero failures, six opt-in external tests ignored; governance, harness, generated-contract and existing browser unit checks pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Existing browser asset build and full locked Rust workspace build pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 specification lanes pass, including mdBook, spec-first, rustfmt and full locked Rust tests."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and all repository Prettier checks pass."
      },
      {
        "command": "pnpm check:progress",
        "result": "fail",
        "failures": 1,
        "note": "Evidence incorrectly marked pass while retaining the audited M5 report gap; restored pending evidence status. Product checks passed independently."
      },
      {
        "command": "pnpm check:progress",
        "result": "pass",
        "failures": 0,
        "note": "32 tasks validate with upload evidence pending only the explicit M5 report audit gap; all implementation checks already pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "M5 audit found a missing explicit remaining-conflict-path report; implement bounded preview/receipt reporting while preserving legacy V1 canonical receipt bytes before closing M5.3 or M5.",
      "Real RustFS/TLS deployment evidence and deferred browser controls remain separate gates."
    ],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.",
    "formats": "Adds ConflictUploadId SHA-256 record V1, immutable repository/epoch/principal staging namespace, binary HTTP upload endpoint/capabilities and uploaded_file resolution variant. Native/WAL/RPC codecs and existing StoredResult V1/Prepared bytes unchanged. Existing released tokio-util is added to HTTP and lockfile. Only generated web API declarations change; UI remains Deferred M4.3. Enclosing Jujutsu change is the revision reference. Real RustFS/TLS SSE qualification remains blocked by the stopped shared Docker engine."
  },
  {
    "id": "E-M5-008",
    "tasks": ["M5", "M5.3"],
    "date": "2026-09-27",
    "kind": "complete-conflict-reports",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs",
      "crates/jj-store-cli/tests/native_lifecycle/resolve.rs",
      "crates/jj-store-jj/src/commands/execute.rs",
      "crates/jj-store-jj/src/commands/mod.rs",
      "crates/jj-store-jj/src/commands/report.rs",
      "crates/jj-store-service/src/commands.rs",
      "crates/jj-store-service/src/mutations.rs",
      "docs/evidence/m5-conflict-reports.md",
      "schemas/openapi.json",
      "spec/src/api.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/jj-integration.md",
      "spec/src/milestones.md",
      "spec/src/mutations.md",
      "web/src/lib/api/generated.ts",
      "spec/src/verification.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_rebases_descendants_updates_references_and_preserves_evolution",
      "crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_accepts_multisided_base_absent_and_directory_terms",
      "crates/jj-store-cli/tests/native_lifecycle/resolve.rs#typed_resolve_report_limits_reject_without_publication_or_truncation",
      "crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs#conflict_upload_http_stages_verified_binary_without_publication_then_resolves",
      "crates/jj-store-cli/tests/native_lifecycle/conflict_uploads.rs#conflict_upload_atomic_mixed_resolution_retains_copy_history_and_other_conflicts",
      "crates/jj-store-service/src/mutations.rs#conflict_report_decodes_legacy_selected_v1_receipt_without_changing_bytes",
      "crates/jj-store-cli/tests/native_rpc.rs#rpc_guarded_publication_preserves_receipts_siblings_and_protection"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve_rebases_descendants_updates_references_and_preserves_evolution --locked",
        "result": "fail",
        "failures": 1,
        "note": "Regression confirms resolution preview omits exact target and remaining-conflict report."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve_rebases_descendants_updates_references_and_preserves_evolution --locked",
        "result": "pass",
        "failures": 0,
        "note": "Exact original regression passes; target and changed descendant report paths match native trees. Removed one unused import warning."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service conflict_report --locked",
        "result": "pass",
        "failures": 0,
        "note": "Frozen legacy V1 result field order reencodes byte-for-byte and passes selected receipt integrity decoder with optional report absent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle typed_resolve --locked",
        "result": "pass",
        "failures": 0,
        "note": "Five native resolution fixtures pass, including the original regression, explicit conflict-free results and output-budget rejection without publication."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle conflict_upload --locked",
        "result": "pass",
        "failures": 0,
        "note": "Six upload fixtures pass with explicit HTTP/native remaining-conflict reports and exact retained report replay after checkpoint/restart and staging loss."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:generate",
        "result": "pass",
        "failures": 0,
        "note": "OpenAPI and generated TypeScript declarations regenerated from owned Rust DTOs; browser source unchanged."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt, all-target Clippy, boundaries/progress, spec validation, 43 actual HTTP/SSE contract samples, schema checks and zero Svelte diagnostics pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "301 Rust tests pass, zero failures; six opt-in external tests ignored. Governance (16), harness (3), contract (6) and browser unit (8) tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Full locked Rust workspace and existing browser assets build successfully."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 lanes pass, including eight cold browser interaction/accessibility tests, all Rust tests, 43 actual HTTP/SSE schema samples, rustfmt, mdBook and spec-first."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Pinned rustfmt and repository-wide Prettier pass."
      },
      {
        "command": "pnpm spec:validate && pnpm spec:first && pnpm check:progress && pnpm spec:build",
        "result": "pass",
        "failures": 0,
        "note": "After the final M5 gate audit, all 58 requirements and 32 tasks validate; mapped spec-first and mdBook pass. Deferred UI and outstanding deployment gates remain unchanged."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.",
    "formats": "Adds optional absent-skipped ConflictResolutionReport to Prepared and generated HTTP/TypeScript contracts. Existing V1 receipt canonical bytes and native/WAL/RPC versions unchanged. Enclosing Jujutsu change is the revision reference. UI stays Deferred M4.3; RustFS/TLS deployment gate remains separate. E-M5-007 recorded the previously missing report; this audit closes that gap. M5 backend gates are complete independently of deferred M4.3 and outstanding external deployment qualification."
  },
  {
    "id": "E-M6-001",
    "tasks": ["M6", "M6.1"],
    "date": "2026-09-27",
    "kind": "adapter-delivery-checkpoints",
    "files": [
      "Cargo.lock",
      "Cargo.toml",
      "crates/jj-store-adapters/Cargo.toml",
      "crates/jj-store-adapters/src/context.rs",
      "crates/jj-store-adapters/src/lib.rs",
      "crates/jj-store-adapters/src/progress.rs",
      "crates/jj-store-adapters/src/runner.rs",
      "crates/jj-store-adapters/src/source.rs",
      "crates/jj-store-adapters/tests/delivery.rs",
      "crates/jj-store-adapters/tests/delivery/support.rs",
      "crates/jj-store-cli/Cargo.toml",
      "crates/jj-store-cli/tests/native_lifecycle.rs",
      "crates/jj-store-cli/tests/native_lifecycle/adapters.rs",
      "crates/jj-store-service/Cargo.toml",
      "crates/jj-store-service/src/adapter.rs",
      "crates/jj-store-service/src/lib.rs",
      "crates/jj-store-wal/src/publication.rs",
      "crates/jj-store-wal/src/publication/reader.rs",
      "crates/jj-store-wal/tests/publication.rs",
      "docs/evidence/m6-adapter-delivery.md",
      "scripts/check-boundaries.mjs",
      "spec/src/adapters.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/milestones.md",
      "spec/src/mutations.md",
      "spec/src/publication.md",
      "spec/src/security.md",
      "spec/src/spec-governance.md",
      "crates/jj-store-cli/tests/native_rpc.rs",
      "spec/src/verification.md"
    ],
    "tests": [
      "crates/jj-store-adapters/tests/delivery.rs#adapter_replays_only_selected_records_and_bootstraps_after_artifact_loss",
      "crates/jj-store-adapters/tests/delivery.rs#adapter_effect_crashes_and_poison_never_advance_or_skip_source",
      "crates/jj-store-adapters/tests/delivery.rs#adapter_cursor_crashes_reconcile_durable_artifact_after_restart",
      "crates/jj-store-adapters/tests/delivery.rs#adapter_restored_older_artifact_catches_up_without_remote_regression_and_config_rebuilds",
      "crates/jj-store-adapters/tests/delivery.rs#adapter_denial_limits_and_corrupt_selected_data_fail_without_advancing",
      "crates/jj-store-adapters/tests/delivery.rs#adapter_timeout_keeps_writer_and_admission_until_background_work_exits",
      "crates/jj-store-adapters/tests/delivery.rs#adapter_polling_retries_poison_with_health_and_stops_on_cancellation",
      "crates/jj-store-adapters/tests/delivery.rs#adapter_corrupt_native_payload_and_noncanonical_or_foreign_cursor_are_rejected",
      "crates/jj-store-adapters/tests/delivery.rs#adapter_bootstrap_rejects_corrupt_selected_tail_before_destination_effect",
      "crates/jj-store-cli/tests/native_lifecycle/adapters.rs#native_adapter_source_verifies_committed_objects_and_rechecks_revocation",
      "crates/jj-store-wal/tests/publication.rs#selected_reader_keeps_exact_committed_refs_and_inventory_across_checkpoint",
      "crates/jj-store-cli/tests/native_rpc.rs#rpc_stream_protocol_deadlines_and_permits_are_bounded"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication selected_reader --locked",
        "result": "fail",
        "failures": 1,
        "note": "Selected-record acceptance cannot compile: five missing-method diagnostics in one test target."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication selected_reader --locked",
        "result": "pass",
        "failures": 0,
        "note": "Exact WAL regression passes: orphan exclusion, exact references, captured history and record-specific inventory survive checkpoints and reject oversized/future traversal."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_replays_only_selected_records_and_bootstraps_after_artifact_loss --locked",
        "result": "fail",
        "failures": 1,
        "note": "Adapter crate does not yet exist; baseline acceptance target cannot run."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-adapters",
        "result": "pass",
        "failures": 0,
        "note": "New crate compiles; removed one unused import warning. Lockfile records the workspace crate."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_replays_only_selected_records_and_bootstraps_after_artifact_loss",
        "result": "pass",
        "failures": 0,
        "note": "Initial ordered replay and artifact-loss acceptance passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery --locked",
        "result": "fail",
        "failures": 1,
        "note": "During fixture cleanup the artifact deletion was accidentally removed before a rebuild assertion. Restored restart assertions and explicit file-loss step; no product assertion weakened."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery",
        "result": "pass",
        "failures": 0,
        "note": "Six initial crash, recovery, poison, scope, ownership and limit tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_adapter_source --locked",
        "result": "pass",
        "failures": 0,
        "note": "Native JJ objects decode through current read-token binding; staged content is denied; revoked token cannot advance the projection."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery --locked",
        "result": "pass",
        "failures": 0,
        "note": "Eight expanded fixtures pass, including polling/backoff/cancellation and corrupt native/cursor data."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_bootstrap_rejects_corrupt_selected_tail_before_destination_effect --locked",
        "result": "fail",
        "failures": 1,
        "note": "Bootstrap incorrectly acknowledges a corrupt selected WAL tail because only inventory was read."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_bootstrap_rejects_corrupt_selected_tail_before_destination_effect --locked",
        "result": "pass",
        "failures": 0,
        "note": "Exact regression passes after current tail/receipt validation; no destination effect or cursor is written."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery --locked",
        "result": "pass",
        "failures": 0,
        "note": "All nine final adapter fault tests pass; remote pointer readback also rejects an unexpected competing cursor."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_replays_only_selected_records_and_bootstraps_after_artifact_loss --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original acceptance rerun with exact locked command passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_adapter_source --locked",
        "result": "pass",
        "failures": 0,
        "note": "Final source boundary including bootstrap tail/receipt checks passes native token, integrity and revocation fixture."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Normal checks pass: all-target Clippy, rustfmt, ten package boundaries, progress, generated contracts and Svelte checks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "312 Rust tests pass, zero failures; six external deployment tests ignored. Existing governance, harness, contracts and browser unit tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Locked Rust workspace and unchanged browser assets build successfully."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 1,
        "note": "Repeated Rust lane hits RPC admission fixture race at native_rpc.rs:640: no occupied permit observed before watchdog. 144 native lifecycle and nine adapter fixtures pass; specification stops before final build lanes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_rpc rpc_stream_protocol_deadlines_and_permits_are_bounded --locked",
        "result": "pass",
        "failures": 0,
        "note": "Narrow baseline rerun passes, exposing the nondeterministic nature of the fixture: heads polling can acquire the sole permit before the upload or begin after its short deadline."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_rpc rpc_stream_protocol_deadlines_and_permits_are_bounded --locked",
        "result": "pass",
        "failures": 0,
        "note": "After replacing competing polling with server-side upload-admission synchronization and connecting the observer first, exact deadline/permit/release acceptance passes. The 400 ms product deadline and rejection assertions remain unchanged."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_rpc --locked",
        "result": "pass",
        "failures": 0,
        "note": "All seven tests consuming the native RPC harness pass after synchronization repair."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Final all-target checks pass after RPC synchronization repair."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "312 Rust tests pass, zero failures, six opt-in external tests ignored; governance, contracts, harness and existing browser unit lanes pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Locked workspace and existing browser assets build."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 lanes pass, including existing cold browser interactions/accessibility, Rust tests, rustfmt, mdBook and spec-first."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.",
    "formats": "New adapter checkpoint V1 with identity, exact selected source reference and durable artifact descriptor under repos/{repo}/adapters/v1/{name}/cursor.json. WAL/native/RPC formats unchanged. Adapter crate and service binding added to committed workspace locks; no UI files changed. Enclosing Jujutsu change is the revision reference. Tantivy and real RustFS/TLS qualification remain separate gates."
  },
  {
    "id": "E-M6-002",
    "tasks": ["M6", "M6.2"],
    "date": "2026-09-27",
    "kind": "local-tantivy",
    "files": [
      "Cargo.lock",
      "Cargo.toml",
      "crates/jj-store-tantivy/Cargo.toml",
      "crates/jj-store-tantivy/src/disk.rs",
      "crates/jj-store-tantivy/src/document.rs",
      "crates/jj-store-tantivy/src/lib.rs",
      "crates/jj-store-tantivy/src/projection.rs",
      "crates/jj-store-tantivy/src/search.rs",
      "crates/jj-store-tantivy/tests/durability.rs",
      "crates/jj-store-tantivy/tests/support/mod.rs",
      "docs/evidence/m6-tantivy-generations.md",
      "docs/licenses/NOTICE.md",
      "docs/upstream-lock.md",
      "spec/src/adapters.md",
      "spec/src/architecture.md",
      "spec/src/compatibility.md",
      "spec/src/milestones.md",
      "spec/src/security.md"
    ],
    "tests": [
      "crates/jj-store-tantivy/tests/durability.rs#tantivy_recovers_committed_payload_without_repeating_effects",
      "crates/jj-store-tantivy/tests/durability.rs#tantivy_crashes_before_pointer_replay_and_after_reader_recover",
      "crates/jj-store-tantivy/tests/durability.rs#tantivy_generation_pagination_retains_old_files_and_checks_each_authorization",
      "crates/jj-store-tantivy/tests/durability.rs#tantivy_missing_corrupt_artifacts_and_config_changes_rebuild",
      "crates/jj-store-tantivy/tests/durability.rs#tantivy_limits_and_writer_owner_fail_without_advancing",
      "crates/jj-store-tantivy/tests/durability.rs#tantivy_scope_changes_remove_stale_content_and_replay_upserts",
      "crates/jj-store-tantivy/tests/durability.rs#tantivy_remote_acknowledgement_loss_recovers_real_index",
      "crates/jj-store-tantivy/tests/durability.rs#tantivy_retained_generation_budget_recovers_after_reader_release",
      "crates/jj-store-tantivy/tests/durability.rs#tantivy_restored_older_generation_replays_without_cursor_regression",
      "crates/jj-store-tantivy/tests/durability.rs#tantivy_rechecks_authorization_before_returning_query_results"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability tantivy_recovers_committed_payload_without_repeating_effects --locked",
        "result": "fail",
        "failures": 1,
        "note": "Named initial target cannot run: Tantivy crate does not exist."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p jj-store-tantivy",
        "result": "fail",
        "failures": 1,
        "note": "Initial product compile target: four diagnostics for Tantivy 0.26 collector API and extractor Sync bounds."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability tantivy_recovers_committed_payload_without_repeating_effects --locked",
        "result": "fail",
        "failures": 1,
        "note": "One remaining product compile diagnostic: TopDocs requires order_by_score in Tantivy 0.26."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability tantivy_recovers_committed_payload_without_repeating_effects --locked",
        "result": "fail",
        "failures": 1,
        "note": "Fixture compile target: ambiguous Checkpoint import, three Arc inference errors and async closure borrow; compiler reports six diagnostics."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability tantivy_recovers_committed_payload_without_repeating_effects --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original durable-pointer/reload/restart acceptance passes with real Tantivy files and recovered commit payload."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked",
        "result": "pass",
        "failures": 0,
        "note": "Initial six durability, loss/config rebuild, scope, reader, authorization and limit tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-tantivy --all-targets --locked -- -D warnings",
        "result": "fail",
        "failures": 1,
        "note": "One too-many-arguments lint in private commit helper; grouped commit inputs without suppressing the lint."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked",
        "result": "pass",
        "failures": 0,
        "note": "Expanded ten fixtures pass, including before/after remote cursor write, restored older local generation, retained-generation budget and post-query revocation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-tantivy --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0,
        "note": "Focused all-target Clippy passes after input grouping and literal bounded query implementation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked",
        "result": "pass",
        "failures": 0,
        "note": "All ten final fixtures pass after streamed file hashing/copy, complete output bounds and retained-generation limits."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-tantivy --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0,
        "note": "Final focused all-target Clippy passes without suppressions."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability tantivy_recovers_committed_payload_without_repeating_effects --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original target rerun with the exact locked command passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Normal checks pass, including all-target Clippy, rustfmt, eleven package boundaries, ledger/spec, generated contracts and existing Svelte analysis."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "322 Rust tests pass, zero failures; six external deployment tests remain explicitly ignored. Governance, storage harness, contracts and existing browser unit checks pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Locked Rust workspace and existing browser assets build successfully."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 specification lanes pass, including existing cold browser interaction/accessibility, full Rust tests, rustfmt, mdBook and spec-first."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and all repository Prettier checks pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Implement native extraction and configured source scopes, native view visibility filtering, server/HTTP search and freshness/minimum-position waits, generated contracts and immutable remote index snapshots.",
      "Run cold process/deployment and measured search acceptance; UI remains deferred and real RustFS/TLS qualification remains externally blocked."
    ],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; Tantivy 0.26.2 with mmap/LZ4 locked from crates.io. Tantivy source revision from released .cargo_vcs_info.json: 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.",
    "formats": "New disposable local Tantivy schema V1, canonical commit payload and checksummed generation descriptor plus atomic local active pointer. Uses existing adapter checkpoint V1; native/WAL/RPC/HTTP formats unchanged. Enclosing Jujutsu change is the revision reference; no UI changes."
  },
  {
    "id": "E-M6-003",
    "tasks": ["M6", "M6.2"],
    "date": "2026-09-27",
    "kind": "native-tantivy-extraction",
    "files": [
      "Cargo.lock",
      "crates/jj-store-adapters/src/source.rs",
      "crates/jj-store-adapters/tests/delivery.rs",
      "crates/jj-store-cli/Cargo.toml",
      "crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs",
      "crates/jj-store-cli/tests/native_lifecycle/content.rs",
      "crates/jj-store-service/src/adapter.rs",
      "crates/jj-store-service/src/adapter/read.rs",
      "crates/jj-store-tantivy/Cargo.toml",
      "crates/jj-store-tantivy/src/disk.rs",
      "crates/jj-store-tantivy/src/document.rs",
      "crates/jj-store-tantivy/src/lib.rs",
      "crates/jj-store-tantivy/src/native.rs",
      "crates/jj-store-tantivy/src/search.rs",
      "crates/jj-store-tantivy/tests/support/mod.rs",
      "crates/jj-store-wal/src/publication/reader.rs",
      "docs/evidence/m6-native-extraction.md",
      "spec/src/adapters.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/milestones.md",
      "spec/src/mutations.md",
      "spec/src/publication.md",
      "spec/src/security.md"
    ],
    "tests": [
      "crates/jj-store-adapters/tests/delivery.rs#adapter_history_and_prior_inventory_stop_at_the_delivered_snapshot",
      "crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs#native_adapter_projection_reader_decodes_committed_trees_and_files",
      "crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs#native_tantivy_extracts_committed_metadata_and_explicit_source_scopes",
      "crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs#native_tantivy_diffs_replay_equivalently_and_keep_exact_scope_context",
      "crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs#native_tantivy_clears_ambiguous_scopes_and_respects_source_disable"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_history_and_prior_inventory_stop_at_the_delivered_snapshot --locked",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: compile failure with four missing history/at method diagnostics."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_adapter_projection_reader_decodes_committed_trees_and_files --locked",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: missing ProjectionRead and inaccessible fixture; four compiler diagnostics. An intermediate filtered run after moving the fixture matched zero tests and is not acceptance evidence."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_adapter_projection_reader_decodes_committed_trees_and_files --locked",
        "result": "pass",
        "failures": 0,
        "note": "Named reader fixture passes after placement under content_acceptance; committed trees/files, virtual roots and authorization bridge."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_extracts_committed_metadata_and_explicit_source_scopes --locked",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: native extractor module absent."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_extracts_committed_metadata_and_explicit_source_scopes --locked",
        "result": "fail",
        "failures": 1,
        "note": "One runtime failure: native tree diff rejects inconsistent copy history in the codec fixture. Diagnostic rerun confirms the same failure. The shared codec fixture remains unchanged; search publishes a valid nested copy lineage."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_extracts_committed_metadata_and_explicit_source_scopes --locked",
        "result": "fail",
        "failures": 1,
        "note": "Valid fixture helper initially failed compilation: nonexistent Tree::set and collection type mismatch; two diagnostics. Fixed sorted tree entry construction."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_extracts_committed_metadata_and_explicit_source_scopes --locked",
        "result": "pass",
        "failures": 0,
        "note": "Real native metadata, invisible predecessor, actor and explicit source scopes pass with the valid search fixture."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_diffs_replay_equivalently_and_keep_exact_scope_context --locked",
        "result": "fail",
        "failures": 1,
        "note": "Exact operation-ID query returned one file instead of two after a scope update: unchanged file terms retained stale context."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_diffs_replay_equivalently_and_keep_exact_scope_context --locked",
        "result": "pass",
        "failures": 0,
        "note": "Schema 2 derives file context and exact commit/operation-ID scope matches from the pinned payload. The original two-file assertion passes; incremental hits equal rebuild."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy --locked",
        "result": "fail",
        "failures": 1,
        "note": "Two tests passed; multi-head fixture failed because its sibling publication provided an empty removed-head set. Service correctly returned native transition heads."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_clears_ambiguous_scopes_and_respects_source_disable --locked",
        "result": "pass",
        "failures": 0,
        "note": "Named multi-head fixture passes after using the original parent operation as removed head; no service guard relaxation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery --locked",
        "result": "pass",
        "failures": 0,
        "note": "All ten delivery fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked",
        "result": "pass",
        "failures": 0,
        "note": "All ten real local Tantivy durability fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_adapter_projection_reader_decodes_committed_trees_and_files --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original named native read bridge target passes after final changes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_history_and_prior_inventory_stop_at_the_delivered_snapshot --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original selected-history boundary target passes after final changes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy --locked",
        "result": "pass",
        "failures": 0,
        "note": "All three native extraction/replay/context/multi-head fixtures pass, zero failures."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Normal checks pass, including all-target Clippy, rustfmt, package boundaries, ledger/spec, generated contracts and existing Svelte analysis."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "327 Rust tests pass, zero failures; six opt-in external tests ignored. Governance, contracts, storage harness and existing browser unit lanes pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Locked workspace and existing browser assets build successfully."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 lanes pass, including existing cold browser interaction/accessibility, Rust tests, rustfmt, mdBook and spec-first."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and repository Prettier checks pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Implement current-visible versus historical search, server lifecycle and configuration, HTTP signed generation cursors, freshness/minimum-position waits and generated API contracts.",
      "Implement immutable remote snapshots and restore; qualify cold process/deployment and measured indexing/rebuild performance. Real RustFS/TLS remains externally blocked; UI stays deferred."
    ],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.",
    "formats": "Disposable local Tantivy schema advances to 2: unchanged file context is bound to scope payload and exact-ID scope terms. Schema-1 generations rebuild. Native extraction config identity is domain-separated native-extraction-v1. Native/WAL/receipt/adapter cursor and HTTP formats unchanged. Enclosing Jujutsu change is the revision reference; no UI changes."
  },
  {
    "id": "E-M6-004",
    "tasks": ["M6", "M6.2"],
    "date": "2026-09-27",
    "kind": "native-search-visibility",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs",
      "crates/jj-store-cli/tests/native_lifecycle/search_visibility.rs",
      "crates/jj-store-jj/src/query/content.rs",
      "crates/jj-store-jj/src/query/content/operations.rs",
      "crates/jj-store-service/src/reads.rs",
      "crates/jj-store-tantivy/src/lib.rs",
      "crates/jj-store-tantivy/src/search.rs",
      "crates/jj-store-tantivy/tests/durability.rs",
      "docs/evidence/m6-search-visibility.md",
      "spec/src/adapters.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/jj-integration.md",
      "spec/src/milestones.md",
      "spec/src/mutations.md",
      "spec/src/security.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/search_visibility.rs#native_tantivy_selected_view_filters_hidden_commits_before_pagination",
      "crates/jj-store-cli/tests/native_lifecycle/search_visibility.rs#native_search_visibility_excludes_predecessors_and_enforces_limits",
      "crates/jj-store-cli/tests/native_lifecycle/search_visibility.rs#native_tantivy_rejects_foreign_view_and_changed_epoch_filters"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_selected_view_filters_hidden_commits_before_pagination --locked",
        "result": "fail",
        "failures": 1,
        "note": "Initial named target cannot compile: 26 diagnostics for absent visibility API and explicit query request types."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_selected_view_filters_hidden_commits_before_pagination --locked",
        "result": "fail",
        "failures": 1,
        "note": "One fixture compiler error: ambiguous glob import of Limits. Explicit Tantivy imports fix it."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle search_visibility --locked",
        "result": "pass",
        "failures": 0,
        "note": "Initial two fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle search_visibility --locked",
        "result": "pass",
        "failures": 0,
        "note": "Expanded three fixtures pass with native all() oracle, actual token revocation, foreign repository/epoch filters and limit checks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_tantivy_selected_view_filters_hidden_commits_before_pagination --locked",
        "result": "pass",
        "failures": 0,
        "note": "Exact original acceptance target passes after final changes, with native graph oracle and revocation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle adapter_projection --locked",
        "result": "pass",
        "failures": 0,
        "note": "All seven native projection and visibility fixtures pass, zero failures."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked",
        "result": "pass",
        "failures": 0,
        "note": "All ten real local Tantivy durability fixtures pass with explicit historical search mode."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-tantivy -p jj-store-jj -p jj-store-service --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0,
        "note": "Focused all-target Clippy passes with no suppressed warnings."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Normal checks pass, including rustfmt, all-target Clippy, ledger/spec, package boundaries, generated contracts and existing Svelte analysis."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "330 Rust tests pass, zero failures; six opt-in external tests ignored. Governance, contracts, storage harness and existing browser unit lanes pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Locked Rust workspace and existing browser assets build successfully."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 specification lanes pass, including existing cold browser interaction/accessibility, Rust tests, rustfmt, mdBook and spec-first."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and repository Prettier checks pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Implement server search lifecycle/configuration, HTTP generation cursors and freshness/minimum-position waits with generated contracts.",
      "Implement immutable remote index snapshots/restore and measured cold process/deployment indexing/rebuild acceptance. RustFS/TLS qualification remains externally blocked; UI remains deferred."
    ],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.",
    "formats": "Core search requests explicitly select historical metadata or native-view commit visibility; results include the selected read snapshot. Local Tantivy schema remains 2. Native/WAL/receipt/adapter cursor and HTTP formats unchanged. Enclosing Jujutsu change is the revision reference; no UI changes."
  },
  {
    "id": "E-M6-005",
    "tasks": ["M6", "M6.2"],
    "date": "2026-09-27",
    "kind": "native-search-serving",
    "files": [
      "Cargo.lock",
      "crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs",
      "crates/jj-store-cli/tests/native_lifecycle/search_paging.rs",
      "crates/jj-store-tantivy/Cargo.toml",
      "crates/jj-store-tantivy/src/document.rs",
      "crates/jj-store-tantivy/src/lib.rs",
      "crates/jj-store-tantivy/src/paging.rs",
      "crates/jj-store-tantivy/src/paging/cursors.rs",
      "crates/jj-store-tantivy/src/search.rs",
      "docs/evidence/m6-search-sessions.md",
      "spec/src/adapters.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/milestones.md",
      "spec/src/security.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_pages_pin_generation_view_and_query_across_writes",
      "crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_freshness_waits_catch_up_and_reauthorize_under_admission",
      "crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_cursor_expiry_releases_idle_generations_and_restart_resets",
      "crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_scope_filters_truncation_and_view_retention_limits",
      "crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_cursor_epoch_and_request_changes_fail_closed",
      "crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_reports_served_position_during_degraded_reader_recovery",
      "crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_requires_head_selection_and_keeps_historical_mode_explicit",
      "crates/jj-store-cli/tests/native_lifecycle/search_paging.rs#native_search_page_output_failure_does_not_exhaust_cursor_pins"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_search_pages_pin_generation_view_and_query_across_writes --locked",
        "result": "fail",
        "failures": 1,
        "note": "Initial target cannot compile: paging module absent; four diagnostics."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_search_pages_pin_generation_view_and_query_across_writes",
        "result": "pass",
        "failures": 0,
        "note": "Initial named serving acceptance passes; existing pinned signing/schema dependencies are added to the committed lockfile."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle search_paging --locked",
        "result": "fail",
        "failures": 1,
        "note": "Expanded fixtures cannot compile: missing reads import, two diagnostics."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle search_paging --locked",
        "result": "pass",
        "failures": 0,
        "note": "Seven expanded native serving fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle adapter_projection --locked",
        "result": "fail",
        "failures": 1,
        "note": "13 passed, one failure: the catch-up fixture required full native index construction under a one-second wait during concurrent tests. Service correctly returned IndexLag. The fixture now prepares durable files separately, uses the configured five-second wait for reader recovery and retains short-wait expiry assertions."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_search_freshness_waits_catch_up_and_reauthorize_under_admission --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original failing freshness/admission/revocation target passes after separating artifact preparation from reader recovery."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_search_page_output_failure_does_not_exhaust_cursor_pins --locked",
        "result": "pass",
        "failures": 0,
        "note": "Encoding failure after cursor retention releases its pin; repeated failing outputs cannot exhaust the one-pin registry."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle adapter_projection --locked",
        "result": "pass",
        "failures": 0,
        "note": "All 15 native projection/visibility/serving fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked",
        "result": "pass",
        "failures": 0,
        "note": "All ten local Tantivy durability fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-tantivy --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0,
        "note": "Focused all-target Clippy passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle native_search_pages_pin_generation_view_and_query_across_writes --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original acceptance passes after final changes; one passed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle search_paging --locked",
        "result": "pass",
        "failures": 0,
        "note": "All eight search serving fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "All checks pass, including rustfmt, Clippy, generated contracts, progress and existing UI checks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "338 Rust tests passed, zero failures, six external tests ignored; governance, storage harness, contracts and existing web tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Existing web build and all workspace binaries pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 specification validation lanes pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      },
      {
        "command": "pnpm check:progress",
        "result": "fail",
        "failures": 1,
        "note": "Evidence finalization incorrectly used pass with remaining milestone gaps; checker rejected it. A subsequent partial value was also rejected (one failure); restored supported pending milestone result while keeping passing slice runs."
      },
      {
        "command": "pnpm check:progress && pnpm spec:validate && pnpm spec:build && pnpm spec:first",
        "result": "pass",
        "failures": 0,
        "note": "Final evidence checks pass after retaining pending milestone scope; 58 requirements and 13-lane product validation recorded above."
      }
    ],
    "result": "pending",
    "remaining": [
      "Register HTTP search/status routes and generated contracts; integrate configured server runners and lifecycle/diagnostics.",
      "Implement immutable remote index snapshots/restore and measured cold process/deployment indexing/rebuild acceptance. RustFS/TLS qualification remains externally blocked; UI remains deferred."
    ],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.",
    "formats": "New in-memory search serving request/status/page contracts and ephemeral jss1 HMAC cursor profile. Canonical native/WAL/receipt/adapter cursor, local Tantivy schema 2 and registered HTTP formats remain unchanged. Enclosing Jujutsu change is the revision reference; no UI changes."
  },
  {
    "id": "E-M6-006",
    "tasks": ["M6", "M6.2"],
    "date": "2026-09-27",
    "kind": "http-search",
    "files": [
      "Cargo.lock",
      "crates/jj-store-cli/tests/native_lifecycle/http_search.rs",
      "crates/jj-store-cli/tests/native_lifecycle/search_paging.rs",
      "crates/jj-store-http/Cargo.toml",
      "crates/jj-store-http/src/contract.rs",
      "crates/jj-store-http/src/contract/tests.rs",
      "crates/jj-store-http/src/dto.rs",
      "crates/jj-store-http/src/error.rs",
      "crates/jj-store-http/src/lib.rs",
      "crates/jj-store-http/src/routes.rs",
      "crates/jj-store-http/src/search.rs",
      "docs/evidence/m6-http-search.md",
      "schemas/openapi.json",
      "scripts/contracts.mjs",
      "spec/src/adapters.md",
      "spec/src/api.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/milestones.md",
      "spec/src/spec-governance.md",
      "web/src/lib/api/generated.ts"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/http_search.rs#http_search_pins_results_across_writes_and_captures_contracts",
      "crates/jj-store-cli/tests/native_lifecycle/http_search.rs#http_search_rejects_bad_inputs_and_unconfigured_repositories",
      "crates/jj-store-cli/tests/native_lifecycle/http_search.rs#http_search_revocation_and_repository_isolation_precede_query_details",
      "crates/jj-store-cli/tests/native_lifecycle/http_search.rs#http_search_reports_degradation_and_head_runs_without_publication",
      "crates/jj-store-cli/tests/native_lifecycle/http_search.rs#http_search_requires_head_selection_and_reports_missing_reader",
      "crates/jj-store-http/src/contract/tests.rs#search_contract_requires_query_and_exposes_freshness_and_resets"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_search_pins_results_across_writes_and_captures_contracts --locked",
        "result": "fail",
        "failures": 1,
        "note": "Initial acceptance cannot compile: missing SearchIndex and HttpApi::with_search (two diagnostics)."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_search_pins_results_across_writes_and_captures_contracts --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original named acceptance passes after implementation and again with the committed dependency lock."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_search --locked",
        "result": "pass",
        "failures": 0,
        "note": "Initial four HTTP search fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --lib --locked",
        "result": "pass",
        "failures": 0,
        "note": "Six HTTP contract tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:generate",
        "result": "pass",
        "failures": 0,
        "note": "OpenAPI and TypeScript declarations regenerated."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:check",
        "result": "pass",
        "failures": 0,
        "note": "47 actual HTTP/SSE responses conform; six contract validators and TypeScript checks pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_search --locked",
        "result": "pass",
        "failures": 0,
        "note": "All five HTTP search fixtures pass, including multiple-head selection and an absent reader."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "All checks pass: rustfmt, boundaries, progress, generated contracts, existing Svelte and all-target Clippy."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "344 Rust tests passed, zero failures, six external tests ignored; governance, harness, contract and existing web unit checks pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Existing browser bundle and Rust workspace build successfully."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 specification lanes pass, including existing cold browser interaction/accessibility, Rust tests, mdBook and spec-first."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and repository Prettier formatting pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Configure server search runners, credentials, lifecycle and diagnostics.",
      "Implement immutable remote index snapshots and restore; record cold deployment and measured performance evidence. RustFS/TLS external qualification is blocked; UI remains deferred."
    ],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.",
    "formats": "Adds registered V1 HTTP search and status routes, per-repository search capability and optional search_status error data; generated OpenAPI and client declarations updated. Local schema 2, jss1 cursors and authoritative native/WAL/receipt formats unchanged. Enclosing Jujutsu change is the revision reference."
  },
  {
    "id": "E-M6-007",
    "tasks": ["M6", "M6.2"],
    "date": "2026-09-27",
    "kind": "search-config-recovery",
    "files": [
      "crates/jj-store-adapters/src/runner.rs",
      "crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs",
      "crates/jj-store-cli/tests/native_lifecycle/search_identity.rs",
      "crates/jj-store-tantivy/src/native.rs",
      "crates/jj-store-tantivy/src/projection.rs",
      "docs/evidence/m6-search-identity.md",
      "spec/src/adapters.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/milestones.md",
      "spec/src/security.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/search_identity.rs#native_search_rejects_mislabeled_extractor_configuration_before_recovery",
      "crates/jj-store-cli/tests/native_lifecycle/search_identity.rs#native_search_config_change_hides_incompatible_recovery_until_rebuild"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle search_identity --locked",
        "result": "fail",
        "failures": 2,
        "note": "Both regressions fail: mislabeled native extractor accepted and incompatible recovered generation served after failed rebuild."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle search_identity --locked",
        "result": "pass",
        "failures": 0,
        "note": "Both original regressions pass after early identity validation and recovery filtering."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle search_identity --locked",
        "result": "pass",
        "failures": 0,
        "note": "Both final regressions pass; mismatched config, implementation, schema and name are rejected before ownership."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle adapter_projection --locked",
        "result": "pass",
        "failures": 0,
        "note": "All 22 native projection/search/HTTP/recovery fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery --locked",
        "result": "pass",
        "failures": 0,
        "note": "All ten adapter delivery/recovery fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked",
        "result": "pass",
        "failures": 0,
        "note": "All ten local Tantivy durability fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Normal checks pass, including rustfmt, all-target Clippy, contract drift/capture, boundaries, progress and existing Svelte checks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "346 Rust tests passed, zero failures, six external tests ignored; governance, harness, contract and existing web unit lanes pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Rust workspace and existing browser bundle build successfully."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 specification lanes pass, including existing cold browser interaction/accessibility, Rust tests, mdBook, rustfmt and spec-first."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and repository Prettier checks pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Configure server search runners, credentials, lifecycle and diagnostics.",
      "Implement immutable remote index snapshots/restore and measured cold deployment evidence. Real RustFS/TLS qualification remains blocked externally; UI stays deferred."
    ],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.",
    "formats": "Projection/Extractor owned interfaces add identity validation. No native, WAL, receipt, adapter checkpoint, local index schema 2 or HTTP format changes. Enclosing Jujutsu change is the revision reference."
  },
  {
    "id": "E-M6-008",
    "tasks": ["M6", "M6.2"],
    "date": "2026-09-27",
    "kind": "server-search-lifecycle",
    "files": [
      "Cargo.lock",
      "crates/jj-store-adapters/src/runner.rs",
      "crates/jj-store-adapters/tests/delivery.rs",
      "crates/jj-store-cli/src/server_commands.rs",
      "crates/jj-store-cli/tests/native_lifecycle/adapter_projection.rs",
      "crates/jj-store-cli/tests/native_lifecycle/server_search.rs",
      "crates/jj-store-cli/tests/server_executable.rs",
      "crates/jj-store-server/Cargo.toml",
      "crates/jj-store-server/src/config.rs",
      "crates/jj-store-server/src/lib.rs",
      "crates/jj-store-server/src/process.rs",
      "crates/jj-store-server/src/search.rs",
      "crates/jj-store-server/tests/configuration.rs",
      "crates/jj-store-server/tests/search_diagnostics.rs",
      "crates/jj-store-tantivy/src/native.rs",
      "crates/jj-store-tantivy/src/paging.rs",
      "docs/cli/server-config.md",
      "docs/cli/server-doctor.md",
      "docs/evidence/m6-server-search.md",
      "scripts/check-boundaries.mjs",
      "scripts/governance.test.mjs",
      "spec/src/adapters.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/milestones.md",
      "spec/src/security.md",
      "spec/src/spec-governance.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_runs_recovers_and_rebuilds_after_cache_loss",
      "crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_revoked_worker_degrades_without_revoking_other_readers",
      "crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_diagnostics_use_authorized_live_status_and_handle_unavailability",
      "crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_drop_cancels_workers_and_releases_owned_state",
      "crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_rejects_state_symlink_escape_and_invalid_programmatic_limits",
      "crates/jj-store-server/tests/search_diagnostics.rs#search_diagnostics_bound_streams_and_reject_foreign_or_invalid_status",
      "crates/jj-store-server/tests/search_diagnostics.rs#search_diagnostics_do_not_forward_credentials_through_redirects",
      "crates/jj-store-cli/tests/server_executable.rs#cold_search_server_requires_injected_credentials_without_echoing_secrets",
      "crates/jj-store-cli/tests/server_executable.rs#cold_search_worker_polls_failed_storage_and_stops_with_foreground_server",
      "crates/jj-store-server/tests/configuration.rs#search_configuration_is_explicit_bounded_and_contains_no_credentials",
      "crates/jj-store-adapters/tests/delivery.rs#adapter_shutdown_waits_for_retained_worker_contexts",
      "scripts/governance.test.mjs#server_composition_cannot_depend_on_cli_or_native_upstream"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration search_configuration_is_explicit_bounded_and_contains_no_credentials --locked",
        "result": "fail",
        "failures": 1,
        "note": "Initial named acceptance cannot compile: server search configuration absent (three diagnostics)."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration search_configuration_is_explicit_bounded_and_contains_no_credentials",
        "result": "pass",
        "failures": 0,
        "note": "Original configuration fixture passes after implementation and lockfile update."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search --locked",
        "result": "fail",
        "failures": 1,
        "note": "First lifecycle fixture expansion cannot compile: wrong fixture token field (two diagnostics), plus one unused import warning."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search --locked",
        "result": "fail",
        "failures": 1,
        "note": "One passed, one failed: exact ordering was assumed across rebuilt generations with equal-score ties. Corrected comparison preserves all document fields and exact same-generation ordering."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_shutdown_waits_for_retained_worker_contexts --locked",
        "result": "fail",
        "failures": 1,
        "note": "Required idle-drain API absent; two compiler diagnostics."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search_drop_cancels_workers_and_releases_owned_state --locked",
        "result": "fail",
        "failures": 1,
        "note": "Stopped runner retained an apparently healthy last watch value. Pager now marks closed health channels degraded."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search --locked",
        "result": "pass",
        "failures": 0,
        "note": "Five lifecycle fixtures pass, including closed health, symlink escape and invalid programmatic limits."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery adapter_shutdown_waits_for_retained_worker_contexts --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original idle-drain target passes; outstanding callback context delays idle completion."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --locked",
        "result": "pass",
        "failures": 0,
        "note": "Seven server configuration tests and two diagnostic transport tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_search --locked",
        "result": "pass",
        "failures": 0,
        "note": "Both cold executable fixtures pass, including standalone and serve-local background requests against isolated failing storage."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search --locked",
        "result": "fail",
        "failures": 1,
        "note": "Four passed, one failed after adding source verification: fixture expected source body in search response, but the existing owned contract indexes body terms and stores file IDs. Replaced with unique term matching and exact native HTTP file readback, preserving source-content verification."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search_runs_recovers_and_rebuilds_after_cache_loss --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original rebuild target passes with complete document equivalence, exact same-generation order and native source readback."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search_drop_cancels_workers_and_releases_owned_state --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original closed-health/drop target passes; stopped runner degrades and releases owned state."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search --locked",
        "result": "pass",
        "failures": 0,
        "note": "All five final server-search lifecycle fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration search_configuration_is_explicit_bounded_and_contains_no_credentials --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original configuration acceptance passes again with locked dependencies."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test search_diagnostics --locked",
        "result": "pass",
        "failures": 0,
        "note": "Both diagnostic transport fixtures pass, including foreign/invalid/oversized streamed responses and redirect rejection."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle adapter_projection --locked",
        "result": "pass",
        "failures": 0,
        "note": "All 27 native extraction/visibility/paging/HTTP/configuration/server projection fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable --locked",
        "result": "pass",
        "failures": 0,
        "note": "All ten regular cold executable fixtures pass; one real RustFS qualification remains ignored."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test delivery --locked",
        "result": "pass",
        "failures": 0,
        "note": "All eleven delivery/recovery/shutdown fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked",
        "result": "pass",
        "failures": 0,
        "note": "All ten local Tantivy durability fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "All normal checks pass, including rustfmt, Clippy, boundaries, progress, generated contracts and existing Svelte checks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "All normal tests pass: 357 Rust tests, zero failures, six external fixtures ignored; governance, storage harness, contracts and existing client unit tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Workspace binaries and existing browser assets build successfully."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 specification lanes pass, including existing cold browser interactions/accessibility, contracts, rustfmt, governance, native tests, mdBook and mapped spec-first."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and all matched Prettier files pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Implement immutable remote index snapshots/restore and measured indexing/rebuild performance.",
      "Qualify real RustFS/TLS server search and SSE. The original Docker volume-create harness is still live and stalled; UI remains deferred."
    ],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.",
    "formats": "TOML format 1 adds explicit search entries; doctor JSON adds structured live search diagnosis. Owned adapter API adds idle drain; search status treats closed runner channels as degraded. No native/WAL/receipt/adapter cursor/local index schema 2 or registered HTTP format changes. Enclosing Jujutsu change is the revision reference."
  },
  {
    "id": "E-M6-009",
    "tasks": ["M6", "M6.2"],
    "date": "2026-09-27",
    "kind": "authorized-immutable-snapshot-artifacts",
    "files": [
      "crates/jj-store-adapters/Cargo.toml",
      "crates/jj-store-adapters/src/artifacts.rs",
      "crates/jj-store-adapters/src/lib.rs",
      "crates/jj-store-adapters/src/source.rs",
      "crates/jj-store-adapters/tests/artifacts.rs",
      "docs/evidence/m6-snapshot-artifacts.md",
      "spec/src/adapters.md",
      "spec/src/compatibility.md",
      "spec/src/milestones.md"
    ],
    "tests": [
      "crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_streams_verify_before_pointer_and_preserve_authority",
      "crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_missing_or_corrupt_files_never_select_a_pointer",
      "crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_unknown_upload_descriptor_and_pointer_outcomes_reconcile",
      "crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_rechecks_revocation_and_confines_identity_and_lineage",
      "crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_rejects_unsafe_names_forged_selected_records_and_rollback",
      "crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_rejects_corrupt_and_noncanonical_descriptors_and_pointers",
      "crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_bounds_file_bytes_and_shared_transfer_budgets",
      "crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_competing_pointer_candidates_preserve_one_complete_selection",
      "crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_revocation_during_download_returns_no_content",
      "crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_cancellation_and_deadline_bound_streaming_work",
      "crates/jj-store-adapters/tests/artifacts.rs#adapter_snapshot_epoch_changes_and_request_exhaustion_stop_access"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test artifacts adapter_snapshot_streams_verify_before_pointer_and_preserve_authority --locked",
        "result": "fail",
        "failures": 1,
        "note": "Initial named target cannot compile: four missing artifact capability/type diagnostics."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test artifacts adapter_snapshot_streams_verify_before_pointer_and_preserve_authority --locked",
        "result": "pass",
        "failures": 0,
        "note": "Original target passes after implementation. Shared fixture imports initially emit three dead-code warnings; the isolated fixture import is now annotated."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --test artifacts --locked",
        "result": "pass",
        "failures": 0,
        "note": "All eleven expanded snapshot storage fixtures pass, including competing CAS candidates, every write-stage unknown outcome, corruption, missing dependencies, namespace/epoch checks, revocation, shared budgets, cancellation and deadline."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-adapters --locked",
        "result": "pass",
        "failures": 0,
        "note": "All 22 adapter delivery and snapshot fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle adapter_projection --locked",
        "result": "pass",
        "failures": 0,
        "note": "All 27 native search/extraction/server projection fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-tantivy --test durability --locked",
        "result": "pass",
        "failures": 0,
        "note": "All ten local Tantivy durability fixtures pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Normal checks pass, including rustfmt, all-target Clippy, progress, boundaries, contracts and existing Svelte checks."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "Full suite passes: 368 Rust tests, zero failures, six external fixtures ignored; governance, storage harness, generated contracts and existing client unit checks pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Workspace binaries and existing browser assets build successfully."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 13 lanes pass, including existing cold browser interaction/accessibility, native tests, contracts, rustfmt, governance, mdBook and mapped spec-first."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and all matched Prettier files pass."
      }
    ],
    "result": "pending",
    "remaining": [
      "Integrate committed Tantivy generation export and compatible restore with complete payload/schema/index validation and rebuild fallback.",
      "Configure foreground snapshot scheduling and record measured indexing/rebuild and real RustFS/TLS deployment acceptance. UI remains deferred; the original external Docker qualification remains stalled."
    ],
    "versions": "Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.",
    "formats": "Additive optional adapter snapshot namespace v1: SHA-256 blobs, canonical V1 bundle descriptors and conditional snapshot.json pointers scoped to repository/epoch/full adapter identity. No native, WAL, receipt, replay cursor, local Tantivy schema 2 or HTTP format changes. Enclosing Jujutsu change is the revision reference."
  },
  {
    "id": "E-UI-001",
    "tasks": ["M4.4"],
    "date": "2026-09-27",
    "kind": "storybook-foundation",
    "files": [
      "catalog/foundation.test.ts",
      ".storybook/main.ts",
      "docs/catalog.md",
      "package.json",
      "web/src/lib/ui/community-chip/model.ts",
      "scripts/governance.test.mjs",
      "scripts/check-progress.mjs",
      "scripts/check-catalog.mjs",
      "spec-validator.config.mjs"
    ],
    "tests": [
      "catalog/foundation.test.ts#recognizes_svelte_diagnostics",
      "catalog/foundation.test.ts#portable_canvas_is_scoped",
      "scripts/governance.test.mjs#requires_catalog_evidence"
    ],
    "runs": [
      {
        "command": "pnpm test:storybook",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: command absent; no stories executed."
      },
      {
        "command": "pnpm build-storybook",
        "result": "fail",
        "failures": 3,
        "note": "Initial static build lacked an explicit Svelte Vite config; three raw Svelte parse errors. Corrected catalog builder config."
      },
      {
        "command": "pnpm catalog:check",
        "result": "pass",
        "failures": 0,
        "note": "Zero Svelte errors or warnings; presentation boundary passes."
      },
      {
        "command": "pnpm catalog:test",
        "result": "pass",
        "failures": 0,
        "note": "2 tests pass."
      },
      {
        "command": "pnpm test:storybook",
        "result": "pass",
        "failures": 0,
        "note": "1 story passes, including blocking axe checks."
      },
      {
        "command": "pnpm build-storybook",
        "result": "pass",
        "failures": 0,
        "note": "Cold static build passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "All normal checks pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "Governance, web, catalog and native suites pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Storybook, existing browser and Rust builds pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 17 lanes pass, including cold existing browser acceptance and rustfmt."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Pinned rustfmt and Prettier pass."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Storybook 10.5.7; Svelte CSF 5.1.2; Vitest 4.1.11; Design Core ^0.2.0; Community d859f33a484a9238bb11f974f4ead8e9be729ec6.",
    "formats": "No production formats or APIs changed. Enclosing Jujutsu change is evidence reference."
  },
  {
    "id": "E-UI-002",
    "tasks": ["M4.5"],
    "date": "2026-09-27",
    "kind": "storybook-shell",
    "files": [
      "web/src/lib/ui/store-application/StoreApplication.svelte",
      "catalog/components/store-application/StoreApplication.stories.svelte",
      "docs/catalog.md",
      ".storybook/vite.config.ts",
      "catalog/vitest.config.ts",
      "catalog/components/community-app-shell/shell-header-geometry.test.ts"
    ],
    "tests": [
      "catalog/components/community-app-shell/shell-header-geometry.test.ts#coalesces_and_cancels_header_measurement"
    ],
    "runs": [
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: no matching story file; shell acceptance absent."
      },
      {
        "command": "pnpm catalog:check",
        "result": "fail",
        "failures": 8,
        "note": "Initial adaptation: unsupported Close side prop and invalid test query exact options; fixed."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Suite import failed with four dependency optimizer errors. Reused Community source-package exclusions for rune compilation."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte",
        "result": "fail",
        "failures": 2,
        "note": "Expanded selector and compact-picker keyboard setup mismatch; corrected control queries and actual opening interaction."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Expanded story asserted before collapse animation settled; added waitFor."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Expanded details correctly dismissed on focus leaving constrained overlay; test now closes details before resizing navigation."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "5 shell stories pass: Default, Expanded, CompactRepositoryPicker, Mobile, HoverReveal; blocking axe passes."
      },
      {
        "command": "pnpm catalog:test",
        "result": "pass",
        "failures": 0,
        "note": "3 tests pass, including copied header geometry cancellation."
      },
      {
        "command": "pnpm catalog:check",
        "result": "pass",
        "failures": 0,
        "note": "Zero errors or warnings; presentation boundaries pass."
      },
      {
        "command": "pnpm test:storybook",
        "result": "pass",
        "failures": 0,
        "note": "6 stories pass including Chip; final cold process after optimizer configuration."
      },
      {
        "command": "pnpm build-storybook",
        "result": "pass",
        "failures": 0,
        "note": "Final cold static shell build passes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "17 lanes pass, including existing browser acceptance."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Normal checks pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "All suites pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Catalog, served browser and Rust builds pass."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Same pinned catalog dependencies and Community source as E-UI-001.",
    "formats": "Presentation props only; no storage or API changes. Enclosing Jujutsu change is the evidence reference."
  },
  {
    "id": "E-UI-003",
    "tasks": ["M4.6"],
    "date": "2026-09-27",
    "kind": "storybook-repository",
    "files": [
      ".storybook/vite.config.ts",
      "AGENTS.md",
      "web/src/lib/community/avatar-content.ts",
      "catalog/components/community-chip/CommunityChip.stories.svelte",
      "web/src/lib/ui/community-commit-diff/CommunityCommitDiff.svelte",
      "web/src/lib/ui/community-diff-controls/CommunityDiffControls.svelte",
      "web/src/lib/ui/community-member-avatar/CommunityMemberAvatar.svelte",
      "web/src/lib/ui/community-member-avatar/community-member-avatar.css",
      "web/src/lib/ui/community-member-avatar/model.ts",
      "web/src/lib/ui/community-navigation/model.ts",
      "web/src/lib/ui/community-profile-menu/CommunityProfileMenu.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsGitAuthorAvatar.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryBranchSelector.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryFileTree.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte",
      "catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogRail.svelte",
      "catalog/components/community-projects/CommunityProjectsRepositoryLogDiff.stories.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogEmpty.svelte",
      "catalog/components/community-projects/CommunityProjectsRepositoryLogFiles.stories.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogFiles.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogHeader.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogRail.svelte",
      "catalog/components/community-projects/CommunityProjectsRevisionGraph.stories.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphCopy.svelte",
      "catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphIds.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphMessage.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsVirtualRows.svelte",
      "catalog/components/community-projects/repository-browser.test.ts",
      "web/src/lib/ui/community-projects/repository-browser.ts",
      "web/src/lib/ui/community-projects/repository-files.css",
      "web/src/lib/projects/repository-log-canvas.ts",
      "web/src/lib/ui/community-projects/virtual-project-row-estimates.ts",
      "web/src/lib/ui/community-relative-time/CommunityRelativeTime.svelte",
      "web/src/lib/ui/community-relative-time/model.ts",
      "catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte",
      "web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte",
      "web/src/lib/ui/community-workspace/model.ts",
      "catalog/components/store-application/RepositoryApplication.stories.svelte",
      "catalog/components/store-application/StoreApplication.stories.svelte",
      "web/src/lib/ui/store-application/StoreApplication.svelte",
      "catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
      "web/src/lib/ui/store-application/StoreRepositoryApplication.svelte",
      "web/src/lib/controller/contracts.ts",
      "catalog/controller/in-memory-store-controller.svelte.ts",
      "catalog/controller/store-controller.test.ts",
      "catalog/fixtures/DiffFixture.svelte",
      "catalog/fixtures/FilesFixture.svelte",
      "catalog/fixtures/RepositoryFixture.svelte",
      "catalog/fixtures/repository.ts",
      "catalog/fixtures/snapshots.ts",
      "catalog/index.ts",
      "catalog/projects/native-presentation.test.ts",
      "web/src/lib/projects/presentation.ts",
      "web/src/lib/projects/repository-log-canvas.ts",
      "web/src/lib/projects/repository-log-collapse.ts",
      "web/src/lib/projects/repository-log-files.ts",
      "catalog/projects/repository-log-graph.test.ts",
      "web/src/lib/projects/repository-log-graph.ts",
      "web/src/lib/projects/repository-log-message.ts",
      "web/src/lib/projects/repository-log.ts",
      "catalog/vitest.config.ts",
      "docs/catalog.md",
      "docs/implementation-plan.md",
      "package.json",
      "pnpm-lock.yaml",
      "scripts/catalog-browser.test.mjs",
      "spec/src/architecture.md",
      "spec/src/milestones.md",
      "spec/src/spec-governance.md",
      "spec/src/ui.md",
      "spec/src/verification.md",
      "spec-validator.config.mjs"
    ],
    "tests": [
      "catalog/projects/native-presentation.test.ts#preserves_native_divergence_and_bookmark_targets",
      "catalog/projects/native-presentation.test.ts#native_ids_remain_complete_in_messages",
      "catalog/projects/native-presentation.test.ts#operation_siblings_keep_separate_graph_lanes",
      "scripts/catalog-browser.test.mjs#cold_catalog_contract"
    ],
    "runs": [
      {
        "command": "pnpm test:storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: no matching stories."
      },
      {
        "command": "pnpm catalog:check",
        "result": "fail",
        "failures": 5,
        "note": "Initial fixture state name shadowed the Svelte rune: 5 errors and 6 warnings; renamed it."
      },
      {
        "command": "pnpm test:storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Initial suite import/reoptimization failure; source compilation exclusions retained."
      },
      {
        "command": "pnpm test:storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte",
        "result": "fail",
        "failures": 4,
        "note": "CSF harness children were mounted as component children: 4 failed, 4 passed, 8 unhandled errors; fixed template snippets."
      },
      {
        "command": "pnpm test:storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte",
        "result": "fail",
        "failures": 2,
        "note": "Radio controls were queried as buttons; corrected public roles."
      },
      {
        "command": "pnpm test:storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte",
        "result": "fail",
        "failures": 2,
        "note": "Highlighted code spans split the text query; assert the complete diff textContent. Repeated once before final correction."
      },
      {
        "command": "pnpm test:storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "8 repository stories pass after corrections."
      },
      {
        "command": "node --test scripts/catalog-browser.test.mjs",
        "result": "fail",
        "failures": 1,
        "note": "Geometry regression exposed wrapped native bookmark labels beyond fixed row height; measured row heights now feed original graph paths."
      },
      {
        "command": "node --test scripts/catalog-browser.test.mjs",
        "result": "fail",
        "failures": 1,
        "note": "Narrow bookmark labels overlapped inline IDs; IDs now wrap beneath content in narrow graph columns."
      },
      {
        "command": "node --test scripts/catalog-browser.test.mjs",
        "result": "fail",
        "failures": 1,
        "note": "Keyboard branch input queried by a label replaced by public Command context; switched to its placeholder. First unbounded attempt interrupted; regression now has bounded timeouts."
      },
      {
        "command": "pnpm catalog:test:browser",
        "result": "fail",
        "failures": 1,
        "note": "Autodocs includes its primary preview plus named stories; corrected explicit iframe count."
      },
      {
        "command": "pnpm catalog:test:browser",
        "result": "pass",
        "failures": 0,
        "note": "Cold static build, desktop/mobile geometry, keyboard selection/resize, Docs canvases, state reset, font/icon contracts and no external/API/WebSocket requests pass."
      },
      {
        "command": "pnpm test:storybook",
        "result": "pass",
        "failures": 0,
        "note": "33 interaction/axe tests across 10 story files pass before the additional Full app story."
      },
      {
        "command": "pnpm catalog:test",
        "result": "pass",
        "failures": 0,
        "note": "15 unit tests pass before copied file-browser helper tests were added."
      },
      {
        "command": "pnpm catalog:check",
        "result": "fail",
        "failures": 17,
        "note": "Final review found unsupported Testing Library exact options in two story files. Removed the unsupported options; string role names are already exact."
      },
      {
        "command": "pnpm catalog:check",
        "result": "fail",
        "failures": 1,
        "note": "One multiline exact option remained; removed it before the passing rerun."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 17,
        "note": "Stopped at catalog-check with the same 17 story type errors; later passed all 18 lanes."
      },
      {
        "command": "pnpm format:check",
        "result": "fail",
        "failures": 2,
        "note": "Prettier parsed literal script tags inside two CSF play attributes as Svelte script blocks. Moved the unsafe-source fixture string to the fixture module; formatter then passed."
      },
      {
        "command": "pnpm catalog:check",
        "result": "pass",
        "failures": 0,
        "note": "71 catalog modules; 0 Svelte errors and 0 warnings."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "18 lanes pass including 19 catalog unit tests, 34 interaction/axe stories, cold static catalog, 8 existing browser cases, Rust tests and rustfmt. Before final author-spacing and shutdown fixes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "All checks and Clippy pass before the final author-spacing and shutdown fixes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "fail",
        "failures": 1,
        "note": "UI/catalog lanes passed; existing cold_search_worker_polls_failed_storage_and_stops_with_foreground_server exited with SIGTERM before handler installation. Source repair and focused evidence recorded in E-M3-015."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "fail",
        "failures": 1,
        "note": "New Full app cold assertion used the Testing Library toHaveTextContent matcher in Playwright; corrected to toContainText. 19 unit tests and 34 interaction/axe stories passed; the cold script failed one test."
      },
      {
        "command": "node --test scripts/catalog-browser.test.mjs",
        "result": "fail",
        "failures": 1,
        "note": "Regex text assertion compared raw DOM whitespace; the rendered author spacing was correct. Switched to Playwright string matching with its standard whitespace normalization, retaining the required separator."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 1,
        "note": "Concurrent final validation had already loaded that same raw-whitespace assertion and stopped at catalog-cold-browser."
      },
      {
        "command": "node --test scripts/catalog-browser.test.mjs",
        "result": "pass",
        "failures": 0,
        "note": "Cold static catalog acceptance passes, including the dedicated Full app initial view and author-spacing assertion; screenshots captured, with no unexpected requests or runtime errors."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Final source checks pass: all canonical/boundary/progress/contracts checks, rustfmt, Svelte checks with 0 errors/warnings and workspace Clippy."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "Final 18 lanes pass: 19 catalog unit tests, 34 interaction/axe stories, cold static catalog, 8 existing browser cases, 369 Rust tests (6 external-service/credential tests ignored), rustfmt, contracts, governance, boundaries and mdBook."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "Final aggregate passes: governance, storage harness self-tests, contracts, browser unit tests, 19 catalog unit tests, 34 stories, cold catalog and 369 Rust tests; 6 external-service/credential tests ignored."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Static Storybook, existing browser bundle/manifest and all locked Rust workspace binaries build successfully."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Community d859f33a484a9238bb11f974f4ead8e9be729ec6; Storybook 10.5.7; Svelte CSF 5.1.2; Vitest 4.1.11; Design Core 0.2.0 via ^0.2.0; Tailwind and Tailwind Vite 4.3.3 via ^4.3.3.",
    "formats": "Presentation-only contracts; native/server/API formats unchanged. Enclosing Jujutsu change is the evidence reference."
  },
  {
    "id": "E-UI-004",
    "tasks": ["M4.7"],
    "date": "2026-09-27",
    "kind": "storybook-complete-application",
    "files": [
      "web/src/lib/controller/contracts.ts",
      "catalog/controller/in-memory-store-controller.svelte.ts",
      "catalog/controller/store-controller.test.ts",
      "web/src/lib/ui/store-application/StoreRepositoryApplication.svelte",
      "catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
      "catalog/components/store-application/RepositoryApplication.stories.svelte",
      "web/src/lib/ui/community-profile-menu/CommunityProfileMenu.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryBranchSelector.svelte",
      "web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte",
      "docs/catalog.md"
    ],
    "tests": [
      "catalog/controller/store-controller.test.ts#controller_restores_snapshot_history",
      "catalog/controller/store-controller.test.ts#controller_isolates_instances_and_conflicts"
    ],
    "runs": [
      {
        "command": "pnpm catalog:test catalog/controller/store-controller.test.ts",
        "result": "fail",
        "failures": 1,
        "note": "Before implementation: controller module absent; one failed suite, no tests executed."
      },
      {
        "command": "pnpm catalog:test catalog/controller/store-controller.test.ts",
        "result": "pass",
        "failures": 0,
        "note": "2 tests pass: snapshot/history restoration, instance and conflict isolation."
      },
      {
        "command": "pnpm catalog:check",
        "result": "fail",
        "failures": 2,
        "note": "Local view named state shadowed the Svelte rune; renamed the derived view."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "fail",
        "failures": 4,
        "note": "Two file selectors used basename instead of public full-path labels, menu close animation required pointer restoration, and empty breadcrumb had no accessible name. Same count reproduced on prebundle rerun."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Profile menu unmounted before global pointer restoration; wait for actual interactive state."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "4 stories and blocking axe pass, including full change-ID search and divergent target selection."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "6 stories pass: repository files, conflicts, profile login/logout, mobile files, branch card filtering and empty cards."
      },
      {
        "command": "pnpm test:storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Bookmark cards regression reproduced joined author text (byMaya). Empty bookmarks passed."
      },
      {
        "command": "pnpm test:storybook catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "2 stories pass after preserving explicit whitespace between relative time and author."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "Final 18 lanes pass: 19 catalog unit tests, 34 interaction/axe stories, cold static catalog, 8 existing browser cases, 369 Rust tests (6 external-service/credential tests ignored), rustfmt, contracts, governance, boundaries and mdBook."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "Final aggregate passes: governance, storage harness self-tests, contracts, browser unit tests, 19 catalog unit tests, 34 stories, cold catalog and 369 Rust tests; 6 external-service/credential tests ignored."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Static Storybook, existing browser bundle/manifest and all locked Rust workspace binaries build successfully."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Community d859f33a484a9238bb11f974f4ead8e9be729ec6; Storybook 10.5.7; Svelte CSF 5.1.2; Vitest 4.1.11; Design Core 0.2.0 via ^0.2.0; Tailwind and Tailwind Vite 4.3.3 via ^4.3.3.",
    "formats": "Typed local presentation/controller contracts only. No API, server, credential or storage formats changed; enclosing Jujutsu change is the evidence reference."
  },
  {
    "id": "E-M3-015",
    "tasks": ["M3", "M3.3"],
    "date": "2026-09-27",
    "kind": "foreground-shutdown-regression",
    "files": [
      "crates/jj-store-server/src/process.rs",
      "crates/jj-store-cli/src/server_commands.rs",
      "crates/jj-store-cli/tests/server_executable.rs",
      "spec/src/cli.md"
    ],
    "tests": [
      "crates/jj-store-cli/tests/server_executable.rs#cold_server_registers_shutdown_before_reporting_listener",
      "crates/jj-store-cli/tests/server_executable.rs#cold_search_worker_polls_failed_storage_and_stops_with_foreground_server"
    ],
    "runs": [
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "fail",
        "failures": 1,
        "note": "Existing search-worker shutdown acceptance exited with SIGTERM. Listening output preceded polling the async handler-registration function."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_search_worker_polls_failed_storage_and_stops_with_foreground_server -- --exact",
        "result": "pass",
        "failures": 0,
        "note": "1 focused retry passed before the fix, confirming timing sensitivity rather than resolving the ordering defect."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_server_registers_shutdown_before_reporting_listener --locked -- --exact",
        "result": "fail",
        "failures": 1,
        "note": "New immediate-termination regression fails before the source fix. Both native run and serve-local are covered by eight cold starts each."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_server_registers_shutdown_before_reporting_listener --locked -- --exact",
        "result": "pass",
        "failures": 0,
        "note": "Immediate termination passes for all 16 cold run/serve-local starts after eager handler registration."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable --locked",
        "result": "pass",
        "failures": 0,
        "note": "11 executable tests pass including the original search-worker regression; 1 external RustFS case remains intentionally ignored."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "Final 18 lanes pass: 19 catalog unit tests, 34 interaction/axe stories, cold static catalog, 8 existing browser cases, 369 Rust tests (6 external-service/credential tests ignored), rustfmt, contracts, governance, boundaries and mdBook."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "Final aggregate passes: governance, storage harness self-tests, contracts, browser unit tests, 19 catalog unit tests, 34 stories, cold catalog and 369 Rust tests; 6 external-service/credential tests ignored."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Static Storybook, existing browser bundle/manifest and all locked Rust workspace binaries build successfully."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "JJ 0.45.1; Rust 1.97.1; existing locked dependencies.",
    "formats": "No wire or storage changes. Unix shutdown handlers now install synchronously before the listening event; enclosing Jujutsu change is the evidence reference."
  },
  {
    "id": "E-UI-005",
    "tasks": ["M4.8"],
    "date": "2026-09-27",
    "kind": "catalog-review-refinements",
    "files": [
      ".storybook/preview.css",
      ".storybook/vite.config.ts",
      ".storybook/vite.config.ts",
      "AGENTS.md",
      "web/src/lib/ui/community-navigation/CommunityNavigation.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectory.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte",
      "web/src/lib/ui/community-projects/repository-directory.css",
      "catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte",
      "web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte",
      "catalog/components/store-application/RepositoryApplication.stories.svelte",
      "catalog/components/store-application/RepositoryReview.stories.svelte",
      "catalog/components/store-application/StoreApplication.stories.svelte",
      "web/src/lib/ui/store-application/StoreApplication.svelte",
      "catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
      "web/src/lib/ui/store-application/StoreRepositoryApplication.svelte",
      "web/src/lib/ui/store-application/repository-breadcrumb.css",
      "web/src/lib/controller/contracts.ts",
      "catalog/controller/in-memory-store-controller.svelte.ts",
      "catalog/controller/store-controller.test.ts",
      "docs/catalog.md",
      "docs/implementation-plan.md",
      "scripts/catalog-browser.test.mjs",
      "spec/src/architecture.md",
      "spec/src/milestones.md",
      "spec/src/spec-governance.md",
      "spec/src/ui.md",
      "spec/src/verification.md"
    ],
    "tests": [
      "catalog/controller/store-controller.test.ts#controller_restores_directory_history",
      "scripts/catalog-browser.test.mjs#cold_catalog_contract"
    ],
    "runs": [
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte",
        "result": "fail",
        "failures": 2,
        "note": "Before implementation: redundant sidebar header and missing directory table reproduced in two review stories."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte",
        "result": "fail",
        "failures": 4,
        "note": "First rerun: two story failures and two unhandled errors from newly imported folder icon triggering optimizer reload. Added the icon to startup prebundling."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte",
        "result": "fail",
        "failures": 2,
        "note": "Header breadcrumb label assertion and duplicate breadcrumb navigation axe landmark failure. Public List now composes within the existing application navigation landmark."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Two review stories passed before extending the assertions to right rail and branch dividers."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Rail border assertion expected zero width; Community uses a transparent border with retained width. Copied the source rail styles and asserted visible color."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Every branch card matched last-child inside its own command wrapper, removing all dividers. Reproduced in the live full-app DOM."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte",
        "result": "fail",
        "failures": 3,
        "note": "Expanded regression: both card/divider stories failed and the requested body-title removal failed; two other stories passed."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "All five stories pass after targeting the final populated wrapper and removing redundant desktop titles. Mobile keeps navigation controls without duplicate title."
      },
      {
        "command": "pnpm catalog:test",
        "result": "pass",
        "failures": 0,
        "note": "20 catalog unit tests passed, including controller_restores_directory_history."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application catalog/components/community-repository-branch-command",
        "result": "pass",
        "failures": 0,
        "note": "18 affected stories pass, including graph alignment, sticky bookmark heading, branch/revision/diff/operation-details secondary headers, directory drill-down, transparent rail, filtered card separators and redundant-title removal."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "fail",
        "failures": 4,
        "note": "Four TypeScript diagnostics in new review assertions used Playwright-only exact options with Testing Library getByRole. Removed invalid options; exact name matching is the Testing Library default."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 4,
        "note": "Cold catalog passed; catalog type lane reported the same four getByRole option diagnostics, then stopped. No runtime or accessibility failures."
      },
      {
        "command": "pnpm catalog:check",
        "result": "pass",
        "failures": 0,
        "note": "Focused rerun after assertion fix: zero errors/warnings; 73 catalog modules pass the boundary audit."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "62 requirements, 37-task bookkeeping, boundaries, contracts, web/catalog types, rustfmt and all-target Clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 18 lanes pass: 20 catalog unit tests, 37 Storybook interactions with blocking axe, cold static catalog, 8 existing browser acceptance cases, 369 Rust tests (6 external-service/credential cases intentionally ignored), contracts, governance, rustfmt, mdBook and mapped spec-first."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass before final evidence-only updates."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "Aggregate passes: governance, storage harness, contracts, browser unit tests, 20 catalog unit tests, 37 Storybook interaction/axe cases, cold catalog and 369 Rust tests. Six external-service/credential cases remain intentionally ignored."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Static Storybook, existing browser bundle/manifest and locked Rust workspace build pass."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Community d859f33a484a9238bb11f974f4ead8e9be729ec6; released Design Core 0.2.0 via ^0.2.0; Storybook 10.5.7; Svelte CSF 5.1.2; Vitest 4.1.11.",
    "formats": "Local in-memory presentation route adds directory. No API, storage or credential changes. Enclosing Jujutsu change is the version-control evidence reference."
  },
  {
    "id": "E-UI-006",
    "tasks": ["M4.8"],
    "date": "2026-09-27",
    "kind": "catalog-log-and-files-navigation-polish",
    "files": [
      ".storybook/preview.css",
      ".storybook/vite.config.ts",
      "web/src/lib/ui/community-app-shell/CommunityAppShell.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsMarkdownPreview.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectory.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryFileTree.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte",
      "catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogDiff.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRepositoryLogRail.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte",
      "catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphIds.svelte",
      "web/src/lib/ui/community-projects/repository-browser.ts",
      "web/src/lib/ui/community-projects/repository-directory.css",
      "web/src/lib/ui/community-projects/repository-files.css",
      "web/src/lib/ui/community-projects/repository-source-tokenizer.ts",
      "catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte",
      "web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte",
      "catalog/components/store-application/RepositoryReview.stories.svelte",
      "web/src/lib/ui/store-application/StoreApplication.svelte",
      "catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
      "web/src/lib/ui/store-application/StoreRepositoryApplication.svelte",
      "catalog/controller/in-memory-store-controller.svelte.ts",
      "web/src/lib/projects/repository-log-canvas.ts",
      "scripts/catalog-browser.test.mjs",
      "spec/src/milestones.md",
      "spec/src/spec-governance.md",
      "spec/src/ui.md"
    ],
    "tests": ["scripts/catalog-browser.test.mjs#cold_catalog_contract"],
    "runs": [
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Before fix: malformed StoreApplication style block caused Svelte unexpected_eof before interaction tests loaded."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Three review stories passed after moving Files breadcrumb/snapshot selector into the page nav, using the body file-tree sidebar, moving the tree toggle next to the avatar row and removing file-browser card borders."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Connected application story still asserted the removed Files heading. The rendered app exposed Files through the body breadcrumb and toolbar instead."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Final focused review pass: verifies toolbar branch selector, Files-rooted breadcrumb, toolbar bottom divider matching the sidebar border, padded file-tree body sidebar, full-height divider, moved tree toggle, borderless directory surface and directory drill-down."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Four connected app stories pass with the Files breadcrumb replacing the removed heading and the snapshot picker remaining in the page nav."
      },
      {
        "command": "pnpm format:check",
        "result": "fail",
        "failures": 3,
        "note": "Rustfmt passed; Prettier reported three Svelte files after the initial Files layout edit. Ran targeted Prettier over touched source/spec files."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass after formatting the touched Svelte files."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "File action-row assertion found left sidebar padding remained on the inner listing instead of the body-sidebar container."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Action-row/header alignment assertion found the shared header height variable only existed in the right content subtree, leaving the left action row shorter."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Three review stories pass with action-row toggles, aligned file-tree/commit row height and bottom edge, compact table header and reduced commit row left padding."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Four connected app stories still pass after adding the action row, shared file-listing mode toggle and compact header density."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Three review stories pass after increasing action-row left padding; Files in the body file-tree action row aligns with the page navigation Files label."
      },
      {
        "command": "pnpm test:storybook catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Four connected app stories still pass after the final action-row left padding alignment."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "fail",
        "failures": 3,
        "note": "Extended review reproduced two failed assertions plus one unhandled operation file-selection error: op-log changed files rendered but route validation rejected non-snapshot operation paths, and collapsed-column assertions used the internal revisions id after restoring body padding."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "15 focused stories pass: body padding restored, collapsed columns fill the padded content box, operation-head rows use muted grey, log graphs remove internal top inset, Op Log opens changed files and file diffs, and connected Files navigation still passes."
      },
      {
        "command": "pnpm format:check",
        "result": "fail",
        "failures": 1,
        "note": "Rustfmt passed; Prettier reported spec/src/spec-governance.md after the spec wording edits."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass after formatting spec/src/spec-governance.md."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 6,
        "note": "Executable spec lanes passed through browser, contracts, rustfmt and boundaries, then progress bookkeeping rejected five Storybook story-name references in the tests field plus the partial result token."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm catalog:check",
        "result": "pass",
        "failures": 0,
        "note": "Focused progress-blocker rerun after HTMLElement typing fixes: 0 Svelte diagnostics and catalog boundaries ok for 73 modules."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "15 focused stories still pass after sharing --jj-store-row-background across Bookmarks, Column Canvas headers and the Op Log info row for neutral theme-grey surfaces."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 1,
        "note": "Run was intentionally interrupted during the long spec lane after a new review request required Column Canvas full-height behavior; no validation failure had been reported before interruption."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "15 focused stories pass with Column Canvas filling the available body viewport on Branches, JJ Log and Op Log while preserving padded canvas content and collapsed rail height."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "15 focused stories pass after graph rows keep bottom dividers through the final item, final graph rows stay square, and the Op Log info row uses a neutral grey distinct from Column Canvas headers."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "fail",
        "failures": 1,
        "note": "Rustfmt passed; Prettier reported CommunityProjectsRevisionGraph.svelte after removing the final-row divider exception. Ran targeted Prettier over touched source and spec files."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass after formatting the graph component and touched specification files."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 18 specification lanes pass, including cold Storybook/browser contract, 20 catalog unit tests, 37 Storybook interaction/axe cases, catalog type/boundary checks, browser contracts, rustfmt, Rust tests, mdBook and mapped spec-first coverage."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Normal check passed before the follow-up Op Log details relocation: spec validation, boundaries, progress, rustfmt, contracts, web/catalog Svelte checks and workspace Clippy."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "15 focused stories pass after moving selected Op Log operation details into a collapsible card at the top of the diff column, preserving parent navigation and removing the standalone Operation details column."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "First unified-column rerun failed only the Op Log story: the test clicked a stale detached operation details region after collapse/re-expand before asserting parent navigation. Re-queried the live region."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "15 focused stories pass with Op Log operation summary, metadata and selected file diff acting as stacked sections in one Column Canvas column; file diff controls remain in the file header and no standalone Operation details column renders."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass after the unified Op Log column layout updates."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 1,
        "note": "Cold static Storybook/browser contract passed, then catalog:check failed because the fileHeaderTrailing snippet was declared as a direct ColumnCanvas.Body child and Svelte typed it as an unsupported Body prop."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "15 focused stories still pass after moving the fileHeaderTrailing snippet inside the diff component invocation."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 18 specification lanes pass with the unified Op Log column: cold static Storybook/browser contract, 20 catalog unit tests, 37 Storybook interaction/axe cases, catalog type/boundary checks, browser contracts, rustfmt, Rust tests, mdBook and mapped spec-first coverage."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Normal check passed before the final equal-width stacked-card adjustment: spec validation, boundaries, progress, rustfmt, contracts, web/catalog Svelte checks and workspace Clippy."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "fail",
        "failures": 2,
        "note": "First equal-width card rerun failed because the operation card and metadata subcomponent shared the Selected operation region name, and the old collapse assertion expected the whole operation component to disappear."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Second equal-width card rerun failed only the old collapse assertion. The operation card correctly stayed visible while the metadata row collapsed."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "15 focused stories pass with Op Log operation and file diff rendered as equal-width stacked cards in one Column Canvas column; collapse hides metadata only, parent navigation remains available, and no standalone Operation details column renders."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass after the equal-width Op Log stacked-card adjustment."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "fail",
        "failures": 2,
        "note": "First empty-state polish rerun loaded 7 tests before Vite reoptimized the new file-search/file-warning icon imports, causing one dynamic-import suite failure; the card close-button hover assertion also stayed unchanged because the test runner did not expose CSS :hover through userEvent.hover."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "fail",
        "failures": 2,
        "note": "After prebundling the new Lucide icons and retargeting global data-attribute styles, all 15 stories loaded; both Op Log stories still failed only the computed-style hover comparison, so the hover CSS stayed in product code and the regression assertion changed to visible themed close-action styling."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "15 focused stories pass with shadcn Empty icon states for no selected Op Log file and binary/unavailable diffs, single-border operation metadata, themed copy/parent pills, styled close action and stable full-app drill-down."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "fail",
        "failures": 4,
        "note": "Rustfmt passed; Prettier reported four touched Svelte files after the Empty and operation metadata styling edits. Ran targeted Prettier over touched source/spec/config files."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass after formatting the touched Storybook and repository-log Svelte files."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 1,
        "note": "Run was intentionally interrupted after the cold spec lane started because new review comments changed the branch-card divider, operation-card radius and split native-ID copy contract."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "19 focused stories pass with final bookmark-card dividers and square lower corners, Column Canvas radius on stacked Op Log cards, split commit/change copy controls for revisions, operation-only ID buttons unchanged, Empty icon states and connected full-app drill-down intact."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass after the branch-card, stacked-card radius and split-ID copy control updates."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 18 specification lanes pass after the latest Storybook shell polish: cold Storybook/browser contract, catalog check, 20 catalog unit tests, 37 Storybook interaction/axe cases, static build, browser contracts, rustfmt, Rust tests, mdBook and mapped spec-first coverage."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Two native-ID stories pass after wrapping each split copy target in shadcn Tooltip; hovering each half exposes the full commit or change value before copy."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "19 focused stories still pass with the split ID shadcn tooltip contract plus full app, repository log and branch-card refinements."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass after adding shadcn tooltips to split ID copy controls."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "fail",
        "failures": 2,
        "note": "First selected-file source preview rerun failed because forcing CodeBlock span mode for all files exposed duplicate plain-space keys in Design Core CodeBlock, and the source preview story could not complete while the Svelte error was unhandled."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Second selected-file source preview rerun passed rendering but kept a stale exact three-line stats assertion for the merged Rust fixture, which correctly has four non-empty lines."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Seven focused stories pass with selected-file metadata, deterministic file statistics, filename source headers, theme-background source body, Rust source tokens via the public CodeBlock custom tokenizer, and existing Files navigation preserved."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Landing-page rerun after moving the snapshot selector into the directory card failed one review assertion: the new summary row height no longer matched the file-tree action row exactly; the removed toolbar-controls CSS also emitted an unused-selector warning."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "fail",
        "failures": 1,
        "note": "Second landing-page rerun failed only the README tab styling assertion because Design Core tab triggers still computed centered content after adding the document icon and table-header background split."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Seven focused stories pass with the branch picker inside the directory commit/action row, bordered/radiused file table, distinct table-header grey, GitHub-style README preview tabs, safe markdown preview and markdown source toggle."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test:storybook catalog/components/store-application/RepositoryReview.stories.svelte catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/StoreRepositoryApplication.stories.svelte catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte catalog/components/community-repository-branch-command/CommunityRepositoryBranchCommand.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Nineteen focused stories pass after the repository landing updates, including review refinements, repository log interactions, connected application files, split native-ID controls and branch cards."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rustfmt and Prettier pass after the repository landing table, tabs, markdown preview and ledger/spec updates."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 1,
        "note": "Cold catalog browser lane failed because its README flow still expected clicking README.md to render a CodeBlock immediately; markdown files now open safe preview mode first."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm catalog:test:browser",
        "result": "pass",
        "failures": 0,
        "note": "Cold catalog browser contract passes after checking README markdown preview, verifying script text is inert, switching to Source mode and then asserting the CodeBlock and font contract."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 18 specification lanes pass after the repository landing markdown/table updates: cold catalog browser, catalog check, 20 catalog unit tests, 37 Storybook interaction/axe stories, static catalog build, Rust tests, mdBook and mapped spec-first coverage."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Normal check passes after repository landing updates: spec validation, boundaries, progress bookkeeping, rustfmt, contracts, web/catalog Svelte checks and workspace Clippy."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "Aggregate test passes after repository landing updates: governance, RustFS harness self-test, contract tests, 8 web tests, 20 catalog unit tests, 37 Storybook interaction/axe stories, cold catalog browser and the locked Rust workspace tests. Six external-service/credential cases remain intentionally ignored."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Static Storybook, existing web bundle/manifest and locked Rust workspace build pass after repository landing markdown/table updates."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Community d859f33a484a9238bb11f974f4ead8e9be729ec6; released Design Core 0.2.0 via ^0.2.0; Storybook 10.5.7; Svelte CSF 5.1.2; Vitest 4.1.11.",
    "formats": "Storybook-only presentation layout change. Selected-file Rust highlighting uses CodeBlock custom-tokenizer presentation data only; markdown previews render deterministic fixture text without executing HTML. No API, storage, native object, credential or backend format changes. Enclosing Jujutsu change is the version-control evidence reference."
  },
  {
    "id": "E-UI-007",
    "tasks": ["M4", "M4.3", "M4.9"],
    "date": "2026-09-27",
    "kind": "architecture",
    "files": [
      "AGENTS.md",
      "package.json",
      "scripts/web-architecture.mjs",
      "scripts/web-architecture-baseline.json",
      "scripts/web-architecture.test.mjs",
      "spec/src/architecture.md",
      "spec/src/spec-governance.md",
      "spec/src/ui.md",
      "spec/src/verification.md",
      "spec/src/milestones.md",
      "docs/implementation-plan.md"
    ],
    "tests": [
      "scripts/web-architecture.test.mjs#rejects a new oversized production module",
      "scripts/web-architecture.test.mjs#rejects production imports from catalog and fixtures",
      "scripts/web-architecture.test.mjs#allows stories to consume production presentation modules"
    ],
    "runs": [
      {
        "command": "node --test scripts/web-architecture.test.mjs && node scripts/web-architecture.mjs",
        "result": "fail",
        "failures": 1,
        "note": "The first focused run proved counted-line enforcement but exposed that the import audit did not recognize side-effect imports."
      },
      {
        "command": "node --test scripts/web-architecture.test.mjs && node scripts/web-architecture.mjs",
        "result": "pass",
        "failures": 0,
        "note": "Four focused architecture tests and the 104-module repository scan pass with Community-derived production/story/test limits, exact hotspot no-growth budgets and production isolation from catalog fixtures."
      },
      {
        "command": "pnpm test:governance",
        "result": "pass",
        "failures": 0,
        "note": "All 21 governance and architecture tests pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Type checking, architecture enforcement, contract drift, progress and source audits pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "Governance, RustFS harness, generated contracts, 8 web tests, 20 catalog unit tests, 37 Storybook interaction/accessibility tests, cold catalog browser acceptance and the full Rust workspace pass; three OS credential tests remain intentionally ignored."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Cold Storybook, production web and locked Rust workspace builds pass. Existing Lightning CSS :global(svg) and large Storybook chunk warnings remain non-fatal."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "rustfmt and Prettier checks pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 18 canonical validation lanes pass, including the cold catalog browser, Storybook interactions/accessibility, full Rust workspace and mdBook."
      }
    ],
    "result": "pending",
    "remaining": [
      "Promote the reviewed presentation modules into production ownership and introduce the shared asynchronous controller contract."
    ],
    "versions": "Community architecture metric and limits reviewed at d859f33a484a9238bb11f974f4ead8e9be729ec6; jj-store parent e187e24b1b48db7ee8486847596056b8c84183b9.",
    "formats": "No storage, native identity, HTTP wire, credential or browser persistence format changes. New architecture checks are wired into normal validation; the enclosing Jujutsu change is the version-control evidence reference."
  },
  {
    "id": "E-UI-008",
    "tasks": ["M4", "M4.3", "M4.9"],
    "date": "2026-09-27",
    "kind": "implementation",
    "files": [
      ".storybook/preview.css",
      "catalog/controller/in-memory-store-controller.svelte.ts",
      "catalog/fixtures/StoreApplicationFixture.svelte",
      "catalog/fixtures/StoreRepositoryApplicationFixture.svelte",
      "catalog/index.ts",
      "docs/catalog.md",
      "scripts/check-catalog.mjs",
      "scripts/web-architecture-baseline.json",
      "spec/src/architecture.md",
      "spec/src/ui.md",
      "spec/src/milestones.md",
      "web/src/lib/controller/contracts.ts",
      "web/src/lib/projects/repository-log.ts",
      "web/src/lib/ui/store-application/StoreApplication.svelte",
      "web/src/lib/ui/store-application/StoreRepositoryApplication.svelte"
    ],
    "tests": [
      "catalog/controller/store-controller.test.ts#controller_restores_snapshot_history",
      "catalog/controller/store-controller.test.ts#controller_isolates_instances_and_conflicts",
      "scripts/web-architecture.test.mjs#allows stories to consume production presentation modules"
    ],
    "runs": [
      {
        "command": "pnpm test:storybook",
        "result": "fail",
        "failures": 11,
        "note": "The first cold interaction import exposed that the moved top-level typography stylesheet had not moved with component subdirectories; no story executed."
      },
      {
        "command": "pnpm catalog:test",
        "result": "pass",
        "failures": 0,
        "note": "All 20 catalog unit tests pass against production-owned presentation modules and the catalog-owned in-memory controller."
      },
      {
        "command": "pnpm test:storybook",
        "result": "pass",
        "failures": 0,
        "note": "All 37 Chromium interaction and accessibility tests pass after moving the shared typography asset."
      },
      {
        "command": "pnpm web:check && pnpm catalog:check && pnpm check:architecture",
        "result": "pass",
        "failures": 0,
        "note": "Production/catalog type checks, the transport-free catalog audit and the 106-module architecture scan pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Specification validation, package and web architecture boundaries, progress, rustfmt, generated contracts, production/catalog types and locked workspace clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "Governance, storage harness, contract, 8 web, 20 catalog, 37 Storybook, cold catalog and full Rust workspace lanes pass; external OS credential/RustFS tests remain intentionally ignored."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Cold Storybook, served web bundle and locked Rust workspace builds pass. Existing Lightning CSS :global(svg) and large Storybook chunk warnings remain non-fatal."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "rustfmt and Prettier checks pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 2,
        "note": "The first aggregate canonical run exposed missing mapped api.md and spec-governance.md updates after promoting modules into web ownership; all functional lanes passed before spec-first stopped the run."
      },
      {
        "command": "pnpm spec:first",
        "result": "pass",
        "failures": 0,
        "note": "The focused spec-first regression passes for 144 protected files and five mapped canonical chapters."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 18 canonical validation lanes pass after recording the web ownership and catalog boundary in api.md and spec-governance.md."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Community source d859f33a484a9238bb11f974f4ead8e9be729ec6; Design Core ^0.2.0; jj-store parent bce792e4.",
    "formats": "Presentation components, models and styles moved from catalog ownership to web/src/lib without changing their rendered contract. The shared asynchronous controller route now separates snapshot operation, inspected operation, commit, change, comparison-parent and comparison-group identities. Catalog fixtures remain transport-free. No HTTP, storage, credential or browser persistence format changes."
  },
  {
    "id": "E-M4-018",
    "tasks": ["M4", "M4.3", "M4.10"],
    "date": "2026-09-27",
    "kind": "implementation",
    "files": [
      "crates/jj-store-cli/tests/native_lifecycle/browsing.rs",
      "crates/jj-store-cli/tests/native_lifecycle/http_browsing.rs",
      "crates/jj-store-cli/tests/native_lifecycle.rs",
      "crates/jj-store-http/src/contract/tests.rs",
      "crates/jj-store-http/src/dto.rs",
      "crates/jj-store-http/src/lib.rs",
      "crates/jj-store-http/src/routes.rs",
      "crates/jj-store-jj/src/query/content/diff_summary.rs",
      "crates/jj-store-jj/src/query/content/operation_changes.rs",
      "crates/jj-store-jj/src/query/content/operations.rs",
      "crates/jj-store-jj/src/query/content/paths.rs",
      "crates/jj-store-jj/src/query/content/references.rs",
      "crates/jj-store-jj/src/query/content.rs",
      "crates/jj-store-service/src/pagination/browsing.rs",
      "crates/jj-store-service/src/pagination/cursors.rs",
      "crates/jj-store-service/src/pagination/operation_browsing.rs",
      "crates/jj-store-service/src/pagination.rs",
      "crates/jj-store-service/src/reads.rs",
      "docs/implementation-plan.md",
      "schemas/openapi.json",
      "scripts/contracts.mjs",
      "scripts/contracts.types.ts",
      "spec/src/api.md",
      "spec/src/architecture.md",
      "spec/src/cli.md",
      "spec/src/compatibility.md",
      "spec/src/jj-integration.md",
      "spec/src/mutations.md",
      "spec/src/spec-governance.md",
      "spec/src/ui.md",
      "spec/src/milestones.md",
      "web/src/lib/api/generated.ts"
    ],
    "tests": [
      "crates/jj-store-cli/tests/native_lifecycle/browsing.rs#native_browser_windows_preserve_references_paths_groups_and_exact_diffs",
      "crates/jj-store-cli/tests/native_lifecycle/browsing.rs#signed_browser_cursors_bind_queries_support_backward_windows_and_reset_on_restart",
      "crates/jj-store-cli/tests/native_lifecycle/http_browsing.rs#http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured",
      "crates/jj-store-http/src/contract/tests.rs#browsing_contracts_keep_complete_reads_and_expose_bound_windows"
    ],
    "runs": [
      {
        "command": "cargo test -p jj-store-cli --test native_lifecycle browsing_acceptance::native_browser_windows_preserve_references_paths_groups_and_exact_diffs --locked -- --exact",
        "result": "fail",
        "failures": 1,
        "note": "The first exact native regression exposed an invalid even-term test bookmark merge; the production reader was not relaxed and the fixture was corrected to preserve native alternating terms."
      },
      {
        "command": "cargo test -p jj-store-cli --test native_lifecycle browsing_acceptance::native_browser_windows_preserve_references_paths_groups_and_exact_diffs --locked -- --exact",
        "result": "pass",
        "failures": 0,
        "note": "Native bookmark/change lookup, conflicted targets, recursive path windows, bounded last-change attribution, exact operation rewrite grouping, metadata-only operations and changed-path summaries pass."
      },
      {
        "command": "cargo test -p jj-store-cli --test native_lifecycle browsing_acceptance::signed_browser_cursors_bind_queries_support_backward_windows_and_reset_on_restart --locked -- --exact",
        "result": "pass",
        "failures": 0,
        "note": "Signed continuations preserve the exact snapshot, support backward windows, reject changed filters and reset after a serving-process restart."
      },
      {
        "command": "cargo test -p jj-store-cli --test native_lifecycle http_browsing_acceptance::http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured --locked -- --exact",
        "result": "pass",
        "failures": 0,
        "note": "All four registered HTTP browsing routes enforce repository authorization, retain the selected operation, reject cross-query cursors, publish nothing and emit contract samples."
      },
      {
        "command": "cargo test -p jj-store-http contract::tests::browsing_contracts_keep_complete_reads_and_expose_bound_windows --locked -- --exact",
        "result": "pass",
        "failures": 0,
        "note": "OpenAPI exposes required operation parents, explicit diff snapshots and nullable bidirectional cursors while leaving the complete diff route cursor-free."
      },
      {
        "command": "pnpm check:architecture && pnpm test:contracts",
        "result": "pass",
        "failures": 0,
        "note": "The 106-module architecture scan and generated TypeScript/schema consumer tests pass."
      },
      {
        "command": "pnpm contracts:check",
        "result": "pass",
        "failures": 0,
        "note": "Generated OpenAPI and TypeScript match; 51 actual HTTP/SSE responses, including the four new browsing handlers, conform."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "fail",
        "failures": 1,
        "note": "The first aggregate check exposed a clippy large-enum-variant error in LastChange; its two signatures were boxed without changing JSON or schema shape."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "fail",
        "failures": 1,
        "note": "The next aggregate check reached a clippy too-many-arguments error in the browser pager; cursor/query inputs were consolidated into an owned internal request."
      },
      {
        "command": "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "result": "pass",
        "failures": 0,
        "note": "The exact workspace lint boundary passes after both internal ownership fixes."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Specification, 11-package boundaries, 106-module architecture, progress, rustfmt, 51-response contract drift, web/catalog diagnostics and workspace clippy pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "21 governance/architecture, 3 storage-harness, 6 contract, 8 web, 20 catalog-unit, 37 Storybook interaction/accessibility and cold catalog tests pass; the complete Rust workspace passes, including 174 native-lifecycle tests."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Cold Storybook, production browser and complete Rust workspace builds pass. Existing Design Core :global minifier and Storybook chunk-size warnings remain non-failing."
      },
      {
        "command": "pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Tracked rustfmt and Prettier checks pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 18 canonical lanes pass, including spec-first coverage for 23 protected files across eight chapters, catalog interaction/accessibility, cold catalog and the complete Rust workspace."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "fail",
        "failures": 1,
        "note": "After marking M4.10 done, spec-first required the task's bounded-data versus production-wiring boundary in ui.md; no executable lane failed."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Native profile jj-0.45.1; cursor envelope jsr1; OpenAPI 3.1; generated openapi-typescript 7.13.0; enclosing Jujutsu change is the revision reference.",
    "formats": "Additive browsing HTTP routes and signed continuation kinds for references, recursive paths, operation groups and diff summaries. Existing complete graph/tree/operation-diff/file-diff responses, native object envelopes, WAL, receipts, credentials and storage formats remain unchanged."
  },
  {
    "id": "E-UI-009",
    "tasks": ["M4.8"],
    "date": "2026-09-27",
    "kind": "implementation",
    "files": [
      "catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
      "catalog/components/community-projects/revision-id.test.ts",
      "scripts/catalog-browser.test.mjs",
      "scripts/catalog-split-copy.mjs",
      "spec/src/api.md",
      "spec/src/milestones.md",
      "spec/src/spec-governance.md",
      "spec/src/ui.md",
      "web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphIds.svelte",
      "web/src/lib/ui/community-projects/revision-graph-ids.css",
      "web/src/lib/ui/community-projects/revision-id.ts"
    ],
    "tests": [
      "catalog/components/community-projects/revision-id.test.ts#abbreviates native identifiers with a prefix and final two characters",
      "scripts/catalog-browser.test.mjs#cold_catalog_contract"
    ],
    "runs": [
      {
        "command": "pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Before change: the two existing identity stories passed but did not cover the requested split-control behavior."
      },
      {
        "command": "pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
        "result": "fail",
        "failures": 7,
        "note": "New acceptance scenarios fail against the original component."
      },
      {
        "command": "pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
        "result": "fail",
        "failures": 2,
        "note": "Synthetic Storybook hover did not activate CSS hover. Keyboard assertions remain in the stories; real pointer checks were added to cold browser acceptance."
      },
      {
        "command": "pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Nine focused interaction/accessibility stories pass, including full copy values, independent feedback, failure retry, keyboard input and optional identifiers."
      },
      {
        "command": "pnpm catalog:test",
        "result": "pass",
        "failures": 0,
        "note": "All 23 catalog unit tests pass, including three abbreviation regressions."
      },
      {
        "command": "pnpm web:check; pnpm catalog:check; pnpm check:architecture",
        "result": "pass",
        "failures": 0,
        "note": "No Svelte errors or warnings; catalog and 109-module architecture checks pass."
      },
      {
        "command": "pnpm catalog:test:browser",
        "result": "fail",
        "failures": 1,
        "note": "Cold build succeeds; cold_catalog_contract finds row 2 bookmark overlap with the wider identifiers before reaching the new native pointer/touch checks."
      }
    ],
    "result": "pending",
    "remaining": [
      "Fix graph bookmark overlap and complete native pointer/touch/reduced-motion acceptance, full check/test/build/spec and formatting lanes. Stories remain visual-pending."
    ],
    "versions": "Existing registry Design Core ^0.2.0, Svelte 5.57.1, Storybook 10.5.7, Vitest 4.1.11. Enclosing Jujutsu change is the revision reference.",
    "formats": "Presentation-only prefix/suffix abbreviation and fixed-width split-copy interaction. Complete underlying clipboard identities retained. No service, schema, storage or credential format changes. User requested a checkpoint commit before continuing validation."
  },
  {
    "id": "E-UI-010",
    "tasks": ["M4.8"],
    "date": "2026-09-27",
    "kind": "implementation",
    "files": [
      "catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
      "catalog/components/community-projects/revision-id.test.ts",
      "scripts/catalog-split-copy.mjs",
      "spec/src/api.md",
      "spec/src/milestones.md",
      "spec/src/ui.md",
      "web/src/lib/ui/community-projects/CommunityProjectsRevisionGraph.svelte",
      "web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphIds.svelte",
      "web/src/lib/ui/community-projects/revision-graph-ids.css",
      "web/src/lib/ui/community-projects/revision-id.ts",
      "spec/src/spec-governance.md",
      "spec/src/verification.md"
    ],
    "tests": [
      "catalog/components/community-projects/revision-id.test.ts#abbreviates native identifiers with a prefix and final two characters",
      "scripts/catalog-browser.test.mjs#cold_catalog_contract"
    ],
    "runs": [
      {
        "command": "pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Nine focused interaction/accessibility stories pass with five-character prefixes, two-dot separators and eight-character expanded prefixes."
      },
      {
        "command": "pnpm catalog:test",
        "result": "pass",
        "failures": 0,
        "note": "All 23 catalog unit tests pass including updated abbreviation boundary/invalid-input tests."
      },
      {
        "command": "pnpm catalog:test:browser",
        "result": "pass",
        "failures": 0,
        "note": "Fresh static build and cold_catalog_contract pass. Bookmark overlap is resolved by wrapping at the graph metadata basis. Native pointer/touch copying, independent feedback, constant outer width, unclipped suffixes and reduced motion pass."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "All normal checks pass, including contracts, architecture, Svelte diagnostics and locked Rust clippy."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "fail",
        "failures": 2,
        "note": "42 of 44 stories pass; OpLog and ShellAndGraphAlignment encounter multiple status roles because the new clipboard live region also rendered inside legacy operation-ID controls."
      },
      {
        "command": "pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRepositoryLog.stories.svelte catalog/components/store-application/RepositoryReview.stories.svelte -t 'OpLog|ShellAndGraphAlignment'",
        "result": "pass",
        "failures": 0,
        "note": "Both exact regressions pass after scoping the clipboard live region to the Git/JJ control; legacy operation-ID presentation stays unchanged."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "Full suite passes after the Op Log correction: 23 catalog unit and 44 interaction/accessibility stories, cold native-pointer/touch catalog acceptance, web/governance/contracts/storage harness and full Rust workspace. This run used the intermediate five-character labels; final four-character at-rest refinement is verified below."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check",
        "result": "pass",
        "failures": 0,
        "note": "Rust and repository-wide Prettier formatting pass."
      },
      {
        "command": "pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Final requested display: all nine focused stories pass with only four characters at rest and prefix/dots/suffix revealed by hover or keyboard focus."
      },
      {
        "command": "pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Nine stories pass after reclaiming reserved width: four characters at rest, six-character expanded prefix with two dots and suffix, and compact independent check/warning feedback."
      },
      {
        "command": "pnpm catalog:test:browser",
        "result": "pass",
        "failures": 0,
        "note": "Intermediate reclaimed-width control passed cold pointer/touch acceptance."
      },
      {
        "command": "pnpm catalog:test:browser",
        "result": "fail",
        "failures": 1,
        "note": "After making the inactive half chip-only, the geometry helper waited for an absent code element. The helper now inspects all visible chip, identifier and feedback boxes, and preserves minimum trailing padding assertions."
      },
      {
        "command": "pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "All nine final stories pass with chip, tick and Copied feedback in the expanded half, four-character resting prefixes and five-character expanded prefixes with dots/suffix."
      },
      {
        "command": "pnpm catalog:test:browser",
        "result": "pass",
        "failures": 0,
        "note": "Fresh build and cold native-pointer/touch acceptance pass: default width <=122px, height <=27px, constant outer width, and at least 4px measured edge clearance for every visible chip/text/feedback box."
      },
      {
        "command": "pnpm web:check; pnpm catalog:check; pnpm check:architecture",
        "result": "pass",
        "failures": 0,
        "note": "Final component has zero Svelte errors/warnings and passes the 110-module architecture boundary."
      },
      {
        "command": "pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/community-projects/CommunityProjectsRevisionGraphIds.stories.svelte",
        "result": "pass",
        "failures": 0,
        "note": "Latest single-identifier refinement passes all nine focused interaction/accessibility stories. Git-only and JJ-only now assert the five-character prefix, two dots and two-character suffix immediately at rest, before interaction; paired controls retain four-character resting labels."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check",
        "result": "pass",
        "failures": 0,
        "note": "Final source passes all normal checks: 11-package boundaries, 110-module architecture, contracts, progress, rustfmt, zero web/catalog Svelte errors or warnings, and locked workspace clippy."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test",
        "result": "pass",
        "failures": 0,
        "note": "Final source passes all 23 catalog unit tests, 44 Storybook interaction/accessibility stories, fresh-build cold_catalog_contract pointer/touch/geometry acceptance, web/governance/contracts/storage-harness tests, and the complete Rust workspace. Environment-gated ignored RustFS tests were not executed; no new live-service claim is made."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build",
        "result": "pass",
        "failures": 0,
        "note": "Final source builds the static Storybook catalog, production browser and locked Rust workspace. Existing non-failing Design Core CSS minifier and Storybook chunk-size warnings remain."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check",
        "result": "pass",
        "failures": 0,
        "note": "All 18 canonical lanes pass on final source, including cold catalog, 44 stories, eight production browser scenarios, web/catalog types and units, contracts, Rust workspace, mdBook and spec-first. This completes the graph overlap, native pointer/touch, reduced-motion and aggregate-validation gaps recorded at checkpoint E-UI-009."
      },
      {
        "command": "DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check; pnpm check:progress; pnpm spec:validate; pnpm spec:first",
        "result": "pass",
        "failures": 0,
        "note": "Final repository-wide formatting and evidence/specification consumers pass: 43 tasks, 65 requirements, seven protected files and four canonical chapters. Visual baselines remain unapproved."
      }
    ],
    "result": "pass",
    "remaining": [],
    "versions": "Registry Design Core ^0.2.0; Svelte 5.57.1; Storybook 10.5.7; Vitest 4.1.11. Enclosing Jujutsu change is the revision reference.",
    "formats": "Final user refinement: paired controls show four leading characters alone at rest; hover/focus reveals five leading characters, two dots and the final two characters. A single Git or JJ identifier shows that expanded abbreviation immediately at rest. Inactive halves retain their chip. Balanced 5px inline padding; 11px resting IDs, 10px expanded IDs, 9px chips and 8px Copied text with tick. Default paired control fits 122px by 27px with constant outer width. Feedback expands the tapped side and preserves independent feedback tint when the other side expands. CSS animates grid tracks; pointer/focus/feedback select the active half. Graph rows wrap before bookmark/ID overlap. No transport, schema or stored format changes. Stories remain visual-pending; screenshots are not approved baselines. Functional acceptance completes the pending E-UI-009 checkpoint; next action is optional human visual review."
  }
]
```
