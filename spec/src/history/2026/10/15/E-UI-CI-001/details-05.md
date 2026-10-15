---
schema: 1
id: E-UI-CI-001
---

# E-UI-CI-001 details 5

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
pnpm check:progress
```

**Notes**

Final ledger validation correctly rejected overall pass because this record retains failed diagnostic runs and a hosted-verification gap. Kept the record pending while preserving all successful final local results; M30 stays In progress.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Hosted PR verification is pending at this local evidence revision. Local
  acceptance is not a hosted CI pass.
- "Full qualification remains failed and idle: existing backend
  identity/cursor/capture failures and the cold fixture JJ-workspace setup
  failure are unresolved."
- Visual baseline approval, cache experiments and release publication remain
  outside this slice.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Local Node v24.10.0, pnpm 10.32.1, Playwright 1.63.0, Vitest 4.1.11; Linux ARM64 Chromium from mcr.microsoft.com/playwright:v1.63.0-noble at sha256:eff16c30e6f3f4af0a03fa4b706120d5e9b0891c344a27d64559aff5900a4a27. Existing local preview/docs tools and captured HTTP fixtures reused; hosted producer builds fresh inputs.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No public Rust API, data or wire format changes. Artifact receipts remain format 1 with a separate same-run UI role. Source, run, attempt and payload checksums are verified. UI inputs never qualify release artifact reuse. Existing JUnit/timing formats and seven UI jobs remain.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Create the reviewed PR and verify its hosted UI and required gates. Repair other qualification suites independently before any full release qualification.
<!-- /evidence-field -->

<!-- evidence-field:contract_review yaml -->
## Contract review

```yaml
status: changed
reason: JST-ARCH-005 promotes core UI into selected standard checks with an
  independent fresh-input entry point. UI requirements explicitly retain
  completions during refresh, disclosure reachability and compact geometry.
  JST-GOV-003/004/011 record the changed gate and retained qualification gaps.
  Layout restoration, focus, padding and accessibility assertions are preserved
  or strengthened. Queued automatic refresh now defers during explicit reference
  pagination and resumes without blocking Projects or Threads freshness.

```
<!-- /evidence-field -->

<!-- evidence-field:milestone_review yaml -->
## Milestone review

```yaml
status: unchanged
reason: M30 remains In progress. Core UI and orchestration proof cannot qualify
  unresolved backend/cold failures, publication or visual baselines.

```
<!-- /evidence-field -->

