# Implementation progress

This is the single status ledger. Done requires original-gate evidence; fake or
analog fixtures cannot satisfy a real-storage gate. Every implementation slice
adds evidence here in its enclosing Jujutsu change.

## Tasks and gates

| ID          | Status      | Depends   | Requirements                                                | Gate                                                                | Next                                             | Evidence   |
| ----------- | ----------- | --------- | ----------------------------------------------------------- | ------------------------------------------------------------------- | ------------------------------------------------ | ---------- |
| Bootstrap   | Done        | -         | JST-GOV-001,JST-GOV-002,JST-GOV-003,JST-GOV-004,JST-GOV-005 | Spec, ledger, skeleton, gates and independent commit                | Begin M0 compatibility proofs                    | E-BOOT-001 |
| Bootstrap.1 | Done        | -         | JST-GOV-001,JST-GOV-004                                     | Reference checksum, requirements, progress checker negative tests   | Governance maintained each slice                 | E-BOOT-001 |
| M0          | In progress | Bootstrap | JST-COMPAT-001,JST-COMPAT-002                               | Compiled extension and exact RustFS contract qualified              | Complete compiled extension/context/query proofs | E-M0-001   |
| M0.1        | Done        | Bootstrap | JST-COMPAT-001                                              | Pin references/licenses and complete trait/codec matrix             | Audit upstream                                   | E-M0-001   |
| M0.2        | Planned     | M0.1      | JST-JJ-001,JST-JJ-002,JST-ID-002                            | Compile custom CLI/RPC context and query spikes; golden IDs         | Compile integration                              | -          |
| M0.3        | Done        | M0.1      | JST-OBJ-002                                                 | Real image create/CAS/range/restart/fault qualification             | Test isolated RustFS                             | E-M0-001   |
| M1          | Planned     | M0        | JST-ID-001,JST-ID-003,JST-OBJ-001,JST-OBJ-003               | Complete lossless objects and streamed storage                      | Implement native objects                         | -          |
| M1.1        | Planned     | M0        | JST-ID-003,JST-ID-004,JST-ID-005                            | All native variants and golden identities                           | Implement codec matrix                           | -          |
| M1.2        | Planned     | M0        | JST-OBJ-001,JST-OBJ-003,JST-OBJ-004                         | Fault fake and real streamed immutable object transport             | Implement object contract                        | -          |
| M2          | Planned     | M1        | JST-PUB-001,JST-PUB-006                                     | Publication fault matrix and cold replay pass                       | Implement coordinator                            | -          |
| M2.1        | Planned     | M1        | JST-PUB-001,JST-PUB-002                                     | Complete inventory and unique candidate CAS                         | Implement selected references                    | -          |
| M2.2        | Planned     | M2.1      | JST-PUB-003,JST-PUB-004,JST-PUB-005                         | Retry receipts and unknown outcome recovery                         | Inject late/unknown races                        | -          |
| M2.3        | Planned     | M2.2      | JST-PUB-006                                                 | Checkpoint equivalence and cache-free restart                       | Verify retained history                          | -          |
| M3          | Planned     | M2        | JST-JJ-001,JST-JJ-002,JST-CLI-001,JST-SEC-001               | Native CLI/server vertical slice with authentication                | Integrate native stores                          | -          |
| M3.1        | Planned     | M2        | JST-SEC-001,JST-SEC-002,JST-SEC-003,JST-SEC-004             | Revocable tokens and transport authorization                        | Implement auth boundaries                        | -          |
| M3.2        | Planned     | M3.1      | JST-JJ-001,JST-JJ-004,JST-JJ-005                            | Create edit describe log restart op-log evolog; multi-workspace     | Implement factories and RPC                      | -          |
| M3.3        | Planned     | M3.2      | JST-CLI-001,JST-CLI-002,JST-CLI-003,JST-CLI-004             | Custom command presentation and capability parity                   | Exercise CLI family matrix                       | -          |
| M4          | Planned     | M3        | JST-API-001,JST-API-005                                     | Structured reads and basic served UI match native semantics         | Implement read surface                           | -          |
| M4.1        | Planned     | M3        | JST-API-001,JST-API-002,JST-JJ-003                          | Pinned three-graph reads with zero GET publications                 | Implement bounded graph sessions                 | -          |
| M4.2        | Planned     | M4.1      | JST-API-003,JST-API-004,JST-API-005                         | SSE, safe UI, generated contracts and cold interaction tests        | Build Svelte UI                                  | -          |
| M5          | Planned     | M4        | JST-MUT-001,JST-MUT-002,JST-MUT-005                         | Authenticated typed commands and shared RPC policy                  | Implement commands                               | -          |
| M5.1        | Planned     | M4        | JST-MUT-001,JST-MUT-002,JST-MUT-005                         | Basic changes/refs/reconcile; preview receipt and semantic policy   | Implement command transaction facade             | -          |
| M5.2        | Planned     | M5.1      | JST-MUT-003                                                 | Full abandon descendant bookmark workspace/evolution parity         | Implement abandon                                | -          |
| M5.3        | Planned     | M5.1      | JST-MUT-004                                                 | Atomic stale-safe resolution with native conflict variants          | Implement resolve                                | -          |
| M6          | Planned     | M5        | JST-ADAPT-001,JST-ADAPT-002,JST-ADAPT-003,JST-ADAPT-004     | Replay-equivalent searchable projection with freshness              | Implement adapters and search                    | -          |
| M6.1        | Planned     | M5        | JST-ADAPT-001,JST-ADAPT-002                                 | Ordered committed reader and durable replay-safe checkpoints        | Implement runner                                 | -          |
| M6.2        | Planned     | M6.1      | JST-ADAPT-003,JST-ADAPT-004                                 | Tantivy crash/rebuild/generation and revocation tests               | Implement projection and UI                      | -          |
| M8a         | Planned     | M6        | JST-COMPAT-003,JST-SEC-005                                  | Pinned release, restore, security, performance and cold UI evidence | Harden first release                             | -          |
| M8a.1       | Planned     | M6        | JST-COMPAT-003                                              | Local/shared examples, diagnostics, restore and request budgets     | Run release acceptance                           | -          |
| M7          | Deferred    | M8a       | JST-ADAPT-005                                               | Complete read-only replica through applied position                 | After first release                              | -          |
| M8b         | Deferred    | M8a       | JST-COMPAT-004                                              | Verified lossless import with source safety                         | Separate migration release                       | -          |

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
  }
]
```
