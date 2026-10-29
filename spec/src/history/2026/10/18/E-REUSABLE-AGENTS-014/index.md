---
schema: 1
id: E-REUSABLE-AGENTS-014
---

# E-REUSABLE-AGENTS-014

<!-- evidence-field:id markdown -->
## Id

E-REUSABLE-AGENTS-014
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M33
- M33.2
- M33.3
- M33.4
<!-- /evidence-field -->

<!-- evidence-field:requirements list -->
## Requirements

- JST-ARCH-007
- JST-COLLAB-006
- JST-COLLAB-029
- JST-DOC-001
- JST-GOV-002
- JST-GOV-011
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-10-18
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

verification
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-conversations/Cargo.toml
- crates/jj-store-conversations/README.md
- crates/jj-store-conversations/src/lib.rs
- crates/jj-store-conversations/src/native_privacy.rs
- crates/jj-store-conversations/tests/native_privacy.rs
- docs/maintenance/coverage.json
- docs/site/technical/conversations.md
- docs/maintenance/reusable-agent-workflows-progress.md
- scripts/check-boundaries.mjs
- scripts/ci-packages.mjs
- scripts/ci-selection.test.mjs
- scripts/conversations-features.test.mjs
- scripts/release.test.mjs
- spec/src/current/contracts/architecture/26-reusable-agent-workflows.md
- spec/src/current/contracts/collaboration/21-capture-sanitization-boundary.md
- spec/src/current/governance/23-shared-ai-clients.md
- spec/src/current/contracts/documentation/01-application-documentation.md
- spec/src/current/progress/M33/index.md
- spec/src/current/progress/M33/history/M33/index.md
- spec/src/current/progress/M33/history/M33.2/index.md
- spec/src/current/progress/M33/history/M33.3/index.md
- spec/src/current/progress/M33/history/M33.4/index.md
- spec/src/history/2026/10/18/index.md
- spec/src/history/2026/10/18/SUMMARY.nav.md
- spec/src/history/2026/10/18/E-REUSABLE-AGENTS-014/index.md
- spec/src/history/2026/10/18/E-REUSABLE-AGENTS-014/SUMMARY.nav.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

[]
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm dev:cargo -- test -p jj-store-conversations --features native-privacy --test native_privacy
```

**Notes**

Completed before state: package has no native-privacy feature (Cargo 101), so the new public-profile test cannot compile. Log native-privacy-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown
```

```shell
node --test --test-name-pattern='only optional conversation privacy|conversation native privacy admits' scripts/conversations-features.test.mjs scripts/ci-selection.test.mjs
```

**Notes**

Both exact admission regressions fail before the change: dependency boundary refuses the optional bridge and CI rejects its exact sibling source. Log native-privacy-boundaries-before.log.
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
pnpm dev:cargo -- check -p jj-store-conversations --features native-privacy
pnpm dev:cargo -- test -p jj-store-conversations --features native-privacy --test native_privacy
```

**Notes**

Opt-in check passes; all five real-scanner cases pass. Nested keys/values, registered exact secrets, built-in GitHub detection, all UTF-8 splits, key collision, payload refusal and uncertain safe receipts are covered. Test caches are disposable private temporary directories. Logs native-privacy-check.log/native-privacy-focused.log.
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
pnpm dev:cargo -- test -p jj-store-conversations --features native-privacy,serde_json/preserve_order --locked
pnpm dev:cargo -- test -p jj-store-conversations --no-default-features --locked
pnpm dev:cargo -- clippy -p jj-store-conversations --all-targets --all-features --locked -- -D warnings
```

**Notes**

Opt-in ordered-JSON profile passes 25 tests; default passes its original 20. Counts overlap and are not 45 distinct cases. Strict all-feature lint passes. Existing receipt/progress and uncertain-owner tests retain their assertions. Logs native-privacy-crate.log/native-privacy-default.log/native-privacy-clippy.log.
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
node --test --test-name-pattern='only optional conversation privacy|conversation native privacy admits' scripts/conversations-features.test.mjs scripts/ci-selection.test.mjs
pnpm check:boundaries
pnpm test:governance
```

**Notes**

Both exact regressions pass, 25 package boundaries pass and all 90 governance cases pass. Actual Cargo trees prove the scanner-free default and opt-in native scanner closure without application storage/runtime authority. Widened because shared dependency/CI selection guards affect all package cohorts; unrelated product suites remain out of scope.
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
pnpm test:release
```

**Notes**

Initial full release/CI helper lane: 58 pass, one release-graph fixture failure for the newly approved external edge. Added only that exact owner/dependency to the expected graph; production optional/path checks remain enforced. Log native-privacy-ci.log.
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
pnpm test:release
```

**Notes**

All 59 release/CI helper cases pass, including source-path/mandatory-edge refusal and real acyclic package inventory. No publication or hosted CI qualification is inferred. Log native-privacy-ci-final.log.
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
pnpm docs:check
```

**Notes**

Manual check correctly requires a source review for conversations after adding the new module. Documentation had been updated, but its source fingerprint still needed acknowledgement. Log native-privacy-docs.log.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown
```

```shell
pnpm docs:review conversations 'Reviewed the optional native privacy bridge, exact policy selection, default dependency isolation, admitted cache/root/secret ownership, receipt fencing and unchanged Notes/JJ publication gates against the source and real-scanner tests.'
pnpm docs:check
```

**Notes**

Refreshed only the conversation source fingerprint after reviewing the implementation and manual. Strict manual build passes, nine Mermaid diagrams render offline and the 144-file/26,703,333-byte closure has valid links/anchors. Log native-privacy-docs-final.log.
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
pnpm evidence:new -- --id E-REUSABLE-AGENTS-014 --task M33 --task M33.2 --task M33.3 --task M33.4 --requirement JST-ARCH-007 --requirement JST-COLLAB-006 --requirement JST-COLLAB-029 --requirement JST-DOC-001 --requirement JST-GOV-002 --requirement JST-GOV-011
pnpm progress:history
pnpm dev:cargo -- package -p jj-store-conversations --list --allow-dirty
pnpm check:progress && pnpm spec:validate && pnpm spec:first
pnpm exec prettier --check scripts/check-boundaries.mjs scripts/ci-packages.mjs scripts/ci-selection.test.mjs scripts/conversations-features.test.mjs scripts/release.test.mjs crates/jj-store-conversations/README.md docs/maintenance/coverage.json docs/site/technical/conversations.md docs/maintenance/reusable-agent-workflows-progress.md spec/src/current/contracts/architecture/26-reusable-agent-workflows.md spec/src/current/contracts/collaboration/21-capture-sanitization-boundary.md spec/src/current/contracts/documentation/01-application-documentation.md spec/src/current/governance/23-shared-ai-clients.md spec/src/current/progress/M33/index.md && pnpm format:rust:check
```

**Notes**

Created fresh evidence and day navigation. Restored 532 initial and 539 history-generation projections, retaining the implementation, owners and exact record navigation. Regenerated 171 task histories. Package inventory has 17 inputs including the public native module/test. Final document checks pass: 171 tasks/884 records, seven validators/4,241 chapters/207 requirements and spec-first 14 protected files/four canonical chapters. Selected formatting and complete Rust formatting pass; aggregate product lanes remain out of scope.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown
```

```shell
pnpm check:progress && pnpm spec:validate && pnpm spec:first
```

**Notes**

Progress passes 171 tasks/884 records and specification seven validators/4,241
chapters/207 requirements. Spec-first correctly requires the canonical
documentation owner for the manual/source review. Added its native bridge
boundary without changing documentation or product gates.
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

Enclosing Jujutsu change qlksymqx, based on e1b09266. Agent Redaction
6b4b94e6 (clean source); scanner 1.4.0/rules 1.2.1/core 1.0.3. macOS ARM64,
Rust 1.99.0, Node 24.10.0 and pnpm 10.34.5.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No persistence schema, request identity or policy version changes. Opt-in
NativeCapture::with_native_privacy selects
kingfisher-scanner@1.4.0/builtin-medium/transcript-paths/v1. Native sanitized
canonical JSON, bounds and stopped-owner receipt behavior remain unchanged.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Add a public opt-in bridge from the already admitted Agent Redaction sanitizer
to NativeCapture; default ports retain their independent dependency closure.
Only optional conversation redaction at the pinned sibling path is added to
boundary/CI graphs. This is a shared Rust/library and graph-governance slice.
Root UI, full Rust workspace, native Notes, provider, platform and release
acceptance are out of scope; the bridge cannot select a backend or target.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Adopt the real privacy capability through Notes native ownership, with current
physical vault/window admission and drained closure. Complete AgentStore/
ThreadStore sink adapters before selecting actual JJ durability. Add automatic
optional-profile CI execution during standalone qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-007 exposes an explicit native-privacy feature and typed
  NativeCapture constructor; default builds remain scanner-free. JST-COLLAB-006
  and JST-COLLAB-029 retain the existing qualified policy, host cache/root/secret
  admission and pre-durability scanning. JST-GOV-002/011 admit only the optional
  pinned conversation edge. JST-DOC-001 has fresh usage/source review.
  JST-ARCH-005 publication and qualification gates remain unchanged.
```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M32 stays Done; M33/M33.1-M33.4 In progress and M33.5-M33.8 Planned.
  NOTES-06C4A completes only the shared native privacy bridge prerequisite;
  NOTES-06C remains the sole Active task. Notes adoption, exact native scope,
  actual JJ durability, restart and optional-profile CI execution remain open.
  Existing M30 publication/platform and visual/provider gates are unchanged.
```
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


