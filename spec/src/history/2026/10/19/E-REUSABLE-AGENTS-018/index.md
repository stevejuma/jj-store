---
schema: 1
id: E-REUSABLE-AGENTS-018
---

# E-REUSABLE-AGENTS-018

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-018
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.2
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-007
- JST-COLLAB-002
- JST-COLLAB-006
- JST-COLLAB-019
- JST-COLLAB-031
- JST-DOC-001
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-19
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-capture/README.md
- crates/jj-store-capture/src/codex_transcript.rs
- crates/jj-store-capture/src/codex_transcript/snapshot.rs
- crates/jj-store-capture/src/transcript_sanitization.rs
- crates/jj-store-capture/tests/transient_snapshot.rs
- docs/maintenance/reusable-agent-workflows-progress.md
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/collaboration/07-readers-durability-identity-and-supervision.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/progress/M33/history/M33.2/index.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/index.md
- spec/src/history/2026/10/19/E-REUSABLE-AGENTS-018/SUMMARY.nav.md
- spec/src/history/2026/10/19/E-REUSABLE-AGENTS-018/index.md
- spec/src/history/2026/10/19/SUMMARY.nav.md
- spec/src/history/2026/10/19/index.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-capture/tests/transient_snapshot.rs#transient_snapshot_spans_batches_without_durable_ack_and_preserves_exact_bytes
- crates/jj-store-capture/tests/transient_snapshot.rs#snapshot_progress_does_not_acknowledge_an_independent_durable_reader
- crates/jj-store-capture/tests/transient_snapshot.rs#long_record_scan_yields_bounded_reading_steps_and_an_explicit_safe_omission
- crates/jj-store-capture/tests/transient_snapshot.rs#complete_snapshot_requires_complete_tail_and_stays_failed_after_restoration
- crates/jj-store-capture/tests/transient_snapshot.rs#source_growth_rewrite_and_truncation_invalidate_the_whole_tentative_snapshot
- crates/jj-store-capture/tests/transient_snapshot.rs#replacement_and_symlink_rebinding_cannot_complete_a_prior_snapshot
- crates/jj-store-capture/tests/transient_snapshot.rs#explicit_bounds_and_exact_surface_are_checked_before_polling
- crates/jj-store-capture/tests/transient_snapshot.rs#neutral_native_sanitization_requires_no_repository_and_matches_the_publication_adapter
- crates/jj-store-capture/tests/headless_capture.rs#headless_reader_retains_exact_pending_identity_until_host_acknowledgement
- crates/jj-store-capture/tests/transcript_sanitization.rs#transcript_sanitization_precedes_durability_and_covers_path_keys_and_values
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
Clean-state/canonical/source review; pnpm dev:disk
```

**Notes**

Clean source and Notes checkouts start on a5019093/6081be31. The old reader exposes only durable-ACK advancement, and sanitizer wrapping requires a repository ID. Explicit previews cannot fabricate either. Before Rust compilation41GiB is free; managed builds remain above the30GiB reserve. No cleanup or application-data operation.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- test -p jj-store-capture --no-default-features --test transient_snapshot
```

**Notes**

Six cases pass; privacy assertion incorrectly inspects domain payload serialized byte arrays as textual JSON. Replace that assertion with actual payload.bytes() inspection. It now proves root/secret absence and the expected workspace placeholder, then compares exact bytes with the existing publication adapter. No production policy is weakened. .local/import-snapshot-tests.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- test -p jj-store-capture --no-default-features --test transient_snapshot
```

**Notes**

All8 exact cases pass:700records/3batches, separate durable reader stays pending, incomplete tail, append/rewrite/truncation, physical replacement/symlink, exact CLI/Desktop surface/bounds,3MiB oversized-record bounded Reading/safe omission, and neutral native sanitation equivalence. Source file remains byte-identical and no spool file is created. Tests explicitly cfg(unix). .local/import-snapshot-bounded.log.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- test -p jj-store-capture; pnpm dev:cargo -- test -p jj-store-capture --no-default-features
```

**Notes**

Escalation is limited to the whole affected crate in both supported feature profiles because shared reader advancement and privacy conversion serve durable publication and headless consumers. Default195passes/3inherited ignored child fixtures; headless75passes/1inherited ignored child fixture. Parent tests execute those child fixtures; nested subprocess receipts are not double-counted. Existing durable publication, ACK/replay, context, recovery and sanitation tests pass. Earlier headless74case run predates the long-record addition. .local/import-capture-default.log and import-capture-headless-final.log.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- clippy -p jj-store-capture --all-targets --no-default-features -- -D warnings; pnpm dev:cargo -- clippy -p jj-store-capture --all-targets -- -D warnings; package fmt --check
```

**Notes**

Both feature-profile package/all-target checks pass under strict Clippy; Rust formatting passes. Root workspace, UI, actual native Notes/provider, other-platform, release and visual qualification are out of scope for this Rust-library slice.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- tree -p jj-store-capture --no-default-features --edges normal
```

**Notes**

The managed build wrapper does not accept the read-only tree command. No build bypass or cleanup is attempted.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
cargo tree --manifest-path Cargo.toml -p jj-store-capture --no-default-features --edges normal --locked --offline; exact graph assertions
```

**Notes**

Read-only dependency inspection excludes jj-lib, service, RPC, WAL, Tauri and Turso. No raw Cargo build/check/test/run or cleanup is used. The existing application feature remains enabled by default. .local/import-capture-tree-readonly.log.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm spec:first before evidence/ledger closure
```

**Notes**

The governing fresh-evidence/ledger gate refuses the unfinished change. Add this source record and select it on M33/M33.2; keep original milestone fields. Final document/ownership checks follow below.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:progress; pnpm spec:first after initial evidence creation
```

**Notes**

Plain Rust filenames are not valid evidence test references. Replace them with existing file#function references, including every new snapshot acceptance case. Fresh progress/contract rules are unchanged. A later intermediate record puts out-of-scope native adoption/UI items in Remaining and is refused by the pass/no-gaps rule. Keep those explicit in Scope/Next and the failed UI run; this result qualifies only the completed Rust prerequisite.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:boundaries; exact parent/current comparison for existing web architecture diagnostics
```

**Notes**

Package boundaries pass for26packages. The UI architecture command is outside this Rust slice and fails on3unrelated files: StoreStackWorkspaceApplication story401/400, StoreApplicationMain313/300 and StoreStackWorkspace310/300. All3files are byte-identical to the parent under the same countedLines function. No UI source or baseline is changed. This is an inherited UI-only failure, not a passing aggregate architecture lane.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:architecture
```

**Notes**

The unchanged3UI count failures above are retained. No repository-wide UI or architecture success is claimed. Rust package checks and source ownership remain independently qualified; this unrelated UI repair stays outside the current library slice.
<!-- /run -->


<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
Original M33 task/history comparison; scoped Prettier and Rust fmt; authored page bounds; pnpm check:progress; pnpm spec:first; pnpm spec:validate
```

**Notes**

All nine original task definitions retain statuses, dependencies, requirements, gates and next actions; only selected evidence changes. Every historical evidence ID remains. Progress179tasks/940records, source-first6protected files/3canonical chapters and seven validators/4366chapters/208requirements pass. Authored pages stay within400lines/32768bytes. The source Rust prerequisite is qualified independently of the unchanged failed UI-count lane and pending native Notes adoption. IMPORT-01B remains Active.
<!-- /run -->

<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Base a5019093; macOS ARM64/Unix, Rust1.99.0, Node24.10.0/pnpm10.32.1. Existing Codex CLI0.156.1 and Desktop0.159.2 profiles and policy IDs are unchanged. Sibling Lapis AI96ced413 and Agent Redaction public Rust boundary are unchanged. No dependency manifest/lockfile, package archive, runtime bundle or native Notes build change.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persisted format changes. SnapshotSummary is transient complete byte count, source record count and SHA256. Existing batches, observation IDs, cursors, metadata and sanitized payload bytes are preserved; the publication adapter still requires the real selected repository. Snapshot mode has no ACK, source registration, resume or publication API.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Enclosing Jujutsu change yxkmyqsm implements the reusable native read-only snapshot prerequisite for IMPORT-01B, with source/contract/README/evidence only. It does not wire Notes native commands, import real sessions into Notes or enable JJ persistence.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Finish IMPORT-01B in Notes: admitted worker-owned source catalog and preview, bounded normalization across batches, truthful unknown tool state, cancellation/drain and actual native-file import/reopen. Keep raw paths/headers native, return sanitized snapshots only after completion, then refresh the preserved ordinary review app. Automatic capture and original platform/JJ/release/visual gates remain pending. The unchanged UI architecture count failures are retained for their owning UI slice.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-007 and JST-COLLAB-002/006/019 now explicitly distinguish
  read-only transient snapshot acquisition from durable capture progress.
  Sanitized observations can be produced without an invented repository ID.
  Existing native publication, source identities, payload bytes, durable
  acknowledgement and JST-COLLAB-031 execution/recovery contracts are unchanged.
  JST-DOC-001 keeps source-only qualification separate from Notes adoption.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: IMPORT-01B remains Active. Its reusable reader/privacy prerequisite is
  qualified; native Notes discovery/admission/UI import and automatic capture
  are pending. Original M33 and M33.2 remain In progress with their original
  gates and next actions; all other original statuses/dependencies remain.
  Unix fixtures do not complete other-platform or actual-provider acceptance.
```
<!-- /evidence-field -->

