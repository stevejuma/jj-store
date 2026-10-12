---
schema: 1
id: E-PROFILE-AVATAR-002
---

# E-PROFILE-AVATAR-002 details 4

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 18
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

First record validation exposed 13 new test references lacking the required file#test-name format, alongside the five inherited npm evidence errors. Corrected the new references before final validation. /tmp/jj-settings-progress.log
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 18
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Same evidence-reference schema error and inherited diagnostics; canonical specification validation itself passes. New record references corrected without touching immutable history. /tmp/jj-settings-spec-first.log
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Final new evidence is valid. Only five inherited E-NPM-REGISTRY-001 diagnostics remain: missing .pnpmfile.cjs, a prose test reference and its missing file, invalid historical run/result. /tmp/jj-settings-progress-final.log
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Only the same inherited progress evidence errors block spec-first; no new governing-topic or evidence finding remains. /tmp/jj-settings-spec-first-final.log
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_CHECK_REVISION=@- pnpm check:progress
```

**Notes**

Fresh parent-revision check reproduces exactly the same five errors, confirming inheritance. /tmp/jj-settings-progress-parent.log
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- 18 inherited source-budget audit findings in unchanged sources remain outside
  this slice.
- Five inherited E-NPM-REGISTRY-001 evidence diagnostics remain in
  progress/spec-first validation; no history is rewritten to conceal them.
- Visual baseline approval, release/platform distribution, full unrelated
  server/Rust workspace and replication qualification remain separate.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; JJ 0.45.1; Node 26.9.0; pnpm 10.32.1; Vite 8.3.1; Vitest 4.1.11. The globally changed pnpm 12.6.0 stalled during package-manager bootstrap; focused verification uses the already installed repository-pinned pnpm 10.32.1 via PATH.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

DirectoryCommand adds UploadImage and POST /v1/directory/image-uploads/{request_id} with application/octet-stream, retaining old Upload receipt encoding. Source limit 20 MiB, 8192 px/side and 64 MiB decoded RGBA. Retained media remains bounded by the existing 512 × 512 / 2 MiB ImageAssetV1 format; only normalized assets/previews are persisted. Immutable manifest/WAL/CAS and version-1 directory documents remain unchanged. Compute memory stays integer GB or null, 1–1024; slider bounds are presentation-only.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Enclosing Jujutsu change pklqtuzx. Server normalizes oversized uploaded raster media with aspect ratio, orientation and transparency; resized animations use first frame, already-bounded media retains exact bytes. Profile and custom emoji use owned shared Settings cards, icons, footers and scrolling. Avatar lower-right action clears an explicit avatar or edits an absent one; identity is copied within Profile info. Emoji shortcode colons are decorative addons, image choice previews locally and uploads only with Save; request recovery preserves original intent. Global Settings has Personal/App groups and bottom version. Compute uses the shared GB integer row and a 1–64 slider that expands for larger valid typed values without clamping or altering automatic values. Direct UI and Rust consumer lanes were selected; root aggregate check/test/build/spec:check, unrelated Rust/UI suites and visual baselines were not run under the targeted policy.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Retain focused functional/cold acceptance. Address independent architecture and historical npm evidence diagnostics with their owners, and select visual/platform qualification separately.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-SEC-008, JST-API-010, JST-UI-021 and JST-OBJ-006 add bounded binary
  image normalization; owning
  security/API/mutation/object/compatibility/CLI/documentation pages are updated
  in this change. JST-UI-016 now specifies the Compute convenience slider and
  retained automatic semantics. JST-GOV-008 remains unchanged: all presentation
  is repository-owned and released primitives are used; no credential,
  repository permission, format migration or publication authority change."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: updated
reason: "M26/M26.1/M26.2/M26.4 remain Partial: focused normalizer, recovery,
  Settings interaction/axe and cold real-RustFS evidence is appended without
  promoting remaining visual or wider lanes. M26.3 and M15 publication/snapshot
  gates are unchanged. M25 stays In progress: Compute presentation tests do not
  qualify signed macOS/WKWebView/Linux distributions or real paired inference.
  The enclosing revision pklqtuzx owns this record."

```
<!-- /evidence-field -->

