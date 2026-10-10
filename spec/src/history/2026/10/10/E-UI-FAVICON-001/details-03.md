---
schema: 1
id: E-UI-FAVICON-001
---

# E-UI-FAVICON-001 details 3

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
pnpm spec:first --base 53788616 --head 573e34b68fca10af29f7c3f568ba3dc50eb14db1
```

**Notes**

Documentation coverage/manual edits also require their mapped canonical delivery chapter under JST-GOV-002. Added explicit unchanged documentation-review boundary under JST-DOC-002; no gate bypassed.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Corrected evidence passes: 146 tasks, 733 evidence records. /tmp/favicon-progress-final.log. Final verification runs again after this record is saved.
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
pnpm spec:first --base 53788616 --head 487cf8fbc418f32701e8317665c21bda6930c75e
```

**Notes**

Protected change gate passes with canonical UI/API/documentation updates, fresh evidence and unchanged M24 status. /tmp/favicon-first-retry.log. Uses an explicit checked current commit, not its future final commit hash.
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

macOS ARM64; JJ 0.45.1; Node v26.9.0; pnpm 10.32.1; system sips and Python 3 stdlib; released Design Core/Mira dependencies unchanged.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Static SVG embedding a 64x64 RGBA PNG; existing version-1 web asset-manifest. No server DTO, schema, API, preference, repository or publication format changes.
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

Static favicon generation/HTML registration using approved app artwork and existing agent-icons SVG inventory. Before: no icon link in web/index.html. After: explicit same-origin image/svg+xml icon plus manifest-selected built asset. UI-only generated-asset slice; focused web check, manifest tests and cold built static browser decoding. Root aggregates, catalog stories/accessibility, Rust checks, execution/publication/restart flows, cross-browser/platform qualification and visual baselines are out of scope. Native asset loader source confirms unchanged SVG admission/MIME/CSP; no Rust code changed or native qualification claimed. Existing live Vite 7460 / backend 7462 and ignored demo state preserved.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit verified default-workspace change vpxrwszm. Retain independent M24 execution/platform/ResizeObserver gates and explicit visual-baseline approval.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: "JST-UI-018: UI 08-device-appearance.md now specifies accepted local
  favicon registration and built inventory. JST-API-001 unchanged-authority
  confirmation in API 26-browser-appearance-state.md: existing static SVG
  media/path/CSP boundaries remain intact; no new API. Documentation
  03-delivery-and-validation.md explicitly retains source review and strict
  staging under JST-DOC-002; JST-GOV-002 protected-change ownership updated."

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M24 remains In progress. agents-presentation-review.md reviews
  E-UI-FAVICON-001 in enclosing change vpxrwszm. Evidence table/history updated;
  visual baselines remain unapproved. This narrow icon run did not emit
  ResizeObserver warnings, but independent prior warnings remain unresolved.

```
<!-- /evidence-field -->

<!-- evidence-field:notes markdown -->
## Notes

Accepted artwork source: desktop/icons/icon.png and desktop/icons/README.md. Original accepted JPEG SHA256 7df0fb3f0b6d9b6366db9a6e1ad6d36ef706d183ac77a8a012834da24874e3ce. Favicon retains transparent surround and pale silhouette edge; ignored preview .local/favicon-proof/favicon-64.png was visually inspected. Compact self-contained SVG avoids serving the 229KB traced vector. Provenance records generation and reuse. No favicon.ico route or external host request added. Temporary browser tabs closed; user tab remains untouched.
<!-- /evidence-field -->

