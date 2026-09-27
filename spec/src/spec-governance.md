# Specification governance

Use the parent workspace test escalation ladder. The archived attachment is
non-normative; requirement changes belong here before or with implementation.
Generated book output is ignored. Only milestones.md carries implementation
status; reports hold immutable evidence rather than competing status summaries.

## Requirements

| ID          | Requirement                                                                                                                                                                                               |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| JST-GOV-001 | The canonical specification MUST live under spec/src, build with mdBook, and assign unique JST requirement IDs with exactly one verification row each.                                                    |
| JST-GOV-002 | Protected implementation, interface, tooling, and agent-guidance changes MUST update their mapped canonical chapter in the same Jujutsu change.                                                           |
| JST-GOV-003 | Every implementation slice MUST update milestones.md with actual commands, results, failures, affected files, versions/formats, gaps, and next action in the same change.                                 |
| JST-GOV-004 | Progress checks MUST validate task/dependency references, evidence links/test names, completion gates, and same-change ledger updates; passing bookkeeping MUST NOT substitute for passing product tests. |
| JST-GOV-005 | Verified slices MUST be committed with Jujutsu while unrelated work and local workspace configuration remain uncommitted.                                                                                 |
| JST-GOV-006 | Rust formatting MUST use the pinned toolchain component and tracked rustfmt.toml, and MUST be checked by both normal and specification checks.                                                            |

## Change map

| Protected area                | Required chapter     |
| ----------------------------- | -------------------- |
| `^rustfmt\\.toml$`            | `spec-governance.md` |
| `^crates/jj-store-model/`     | `identity.md`        |
| `^crates/jj-store-object/`    | `object-store.md`    |
| `^crates/jj-store-wal/`       | `publication.md`     |
| `^crates/jj-store-jj/`        | `jj-integration.md`  |
| `^crates/jj-store-service/`   | `mutations.md`       |
| `^crates/jj-store-rpc/`       | `jj-integration.md`  |
| `^(?:crates/jj-store-http/    | web/)`               | `api.md`                     |
| `^crates/jj-store-(?:adapters | tantivy)/`           | `adapters.md`                |
| `^(?:crates/jj-store-cli/     | bins/                | docs/cli/)`                  | `cli.md`         |
| `^(?:crates/jj-store-testkit/ | fixtures/            | schemas/                     | proto/           | docs/upstream-lock | docs/codec   | docs/compatibility   | docs/licenses/                | docs/evidence/    | scripts/.*(?:rustfs | spike | upstream) | deploy/)` | `compatibility.md` |
| `^(?:scripts/                 | AGENTS\.md$          | spec-validator\.config\.mjs$ | spec/book\.toml$ | \.gitignore$       | \.prettier)` | `spec-governance.md` |
| `^(?:Cargo\.(?:toml           | lock)$               | rust-toolchain\.toml$        | package\.json$   | pnpm-lock\.yaml$   | README\.md$  | LICENSE\.md$         | docs/implementation-plan\.md$ | docs/reference/)` | `architecture.md`   |

The M0 contract harness is validated in both normal tests and spec checks. Real
storage qualification remains an explicit lane and cannot be inferred from the
harness self-tests or a source audit.

## M0 extension qualification

Native compatibility probes are normal Rust test targets. Their source/fixture
updates require canonical chapter changes and same-change ledger evidence.

The external compiled-storage lane is explicitly opt-in with the isolated RustFS
harness. Its immutable report records the command and actual test output; normal
test output must report this deployment test as ignored, not passed.

Publication faults are exercised in the normal WAL tests. The opt-in deployment
lane records both the initial compiled native publication run and a separate
post-kill recovery process; neither is inferred from normal ignored tests.

## Rust formatting

The pinned Rust toolchain includes rustfmt. The root rustfmt.toml fixes Rust and
formatting editions to 2024 and uses Unix line endings. `pnpm format:rust`
formats every workspace crate, including examples and tests;
`pnpm format:rust:check` checks those files without changing them. `pnpm format`
and `pnpm format:check` include Rust alongside Prettier. Both `pnpm check` and
`pnpm spec:check` reject Rust formatting drift.
