---
schema: 1
id: E-M17-002
---

# E-M17-002 details 4

<!-- evidence-field:remaining list -->
## Remaining

- "M17.1 remains In progress: bounded transcript append is unimplemented."
- Explicit bound-gap handling remains pending; exact bound capture still
  requires a real session.
- Automatic server configuration, directory/workspace matching, source readers
  and worker supervision remain M17.2; no watcher or real harness qualification
  claimed.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

JJ 0.45.1; Rust/Cargo 1.98.1; Node.js 26.9.0; pnpm 10.32.1; existing V1 capture models and version-bound normalizers; shared native WAL and local Git companion engine.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

V1 event batches now admit evidence-only empty activity with unchanged field framing. Existing nonempty bytes, request identities and receipts remain unchanged. Tool snapshots retain selected prior content/evidence and terminal state; conflict diagnostics contribute gap coverage.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

Enclosing Jujutsu change kouymyzw; specification parent mxmuvnrt (4aa40300).
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Shared agent model/projection changes affect native WAL and local Git provider consumers, so all jj-agent domain tests and the complete direct capture, provenance, binding, paging and checkpoint test files run. Broader Rust workspace, root aggregates, UI, real harness and real RustFS lanes are out of scope. Public field/framing shape is unchanged; no generated schema update required. Strict clippy exposed four existing no-op lint issues in direct dependencies; remove redundant ancestry binding and flatten equivalent content-search guards, then run the direct content-search test file. No search/UI behavior or schema change.
<!-- /evidence-field -->

