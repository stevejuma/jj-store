---
schema: 1
id: E-READ-094
---

# E-READ-094 details 3

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
CUA fresh hidden tab on pinned Linux CREDITS; click Blame/Source; CDP Network response observation
```

**Notes**

Original user tab13 CDP focus timed out twice; kept it intact and used fresh14. Blame enabled, source and origin skeletons present, original pin preserved. Source navigation stopped further Blame responses. First live trace found background admission starvation (repeated503 busy after4 successful windows); switched on-demand Blame to ordinary read admission. Screenshot .local/blame-live.png records presentation; final admission result follows.
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
pnpm dev:backend:build; preview fixture pnpm dev:backend:restart (same explicit environment as above)
```

**Notes**

blame-backend-build-final.log/blame-preview-restart-final.log:ordinary on-demand admission rebuilt and running on7472,PID92236;UI PID302 remains. Both production reader regressions repeated and pass in blame-local-admission.log/blame-native-admission.log.
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
pnpm exec tsx --test web/tests/store-blame.test.ts; pnpm web:check; cold StoreBlame.stories.svelte; pnpm docs:review; pnpm check:architecture/boundaries/ui-ownership
```

**Notes**

Final model4/4,types0 diagnostics,cold stories7/7,documentation fingerprints,1155 UI modules,17 Rust boundaries and ownership pass. Exact logs blame-model-verified,blame-web-last,blame-stories-last,blame-docs-review-final and final architecture/boundaries/ownership logs.
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
jj --no-pager st (main)
```

**Notes**

Main was initially clean but acquired uncommitted Complete native stack discovery and branch boundaries work in sktnmmmv. Generated contracts/specs overlap. Stopped main integration as required; temporary change and preview remain isolated above committed fe3cb82c.
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
CUA fresh Linux CREDITS Blame after rebuilt on-demand admission; CDP Network.responseReceived/getResponseBody; return to Source
```

**Notes**

Live7470/7472:preparation/reset409s recovered on the same operation after restart,then7 consecutive200 responses with advancing cursors. Response header latencies2305,2367,2367,2295,2245,2605,2312ms;latest6720 history commits,4582 lines,0 proven origins,not complete. Existing source remains visible;no invented origins or full-completion claim. Source navigation removes Blame and prior cancellation check observed0 later Blame responses. Saved .local/blame-live-response.json and .local/blame-live.png;temporary test tab retained no writes to Linux.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress; pnpm spec:first; pnpm spec:validate; parse old/current ledger JSON
```

**Notes**

blame-progress-final.log/bookkeeping142 tasks;spec-first/validate pass before final live evidence append. All607 historical entries preserved;E-READ-094 appended. Final repeat in blame-governance-final.log/blame-progress-last.log.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:verification_scope markdown -->
## Verification scope

Focused shared annotation tests, native/local production readers, both registered HTTP consumers and response captures, affected Rust/type/contract checks, query/navigation/model tests, cold Chromium story interactions, original Linux read-only probes, live preview and governance/documentation checks. No milestone promotion.
<!-- /evidence-field -->

<!-- evidence-field:limitations list -->
## Limitations

- Upstream synchronous line comparisons cannot be preempted. Cancellation
  prevents subsequent work and late publication, retaining worker admission
  until exit.
- "Service bounds: 256 KiB selected/historical blobs, 32768 selected lines, 32
  pending sources, 16 parents per visited commit, 64 topological windows of 64
  commits per cooperative one-second request, existing hard read/work/output
  budgets; memory admission reserves trial scratch plus retained estimates.
  These are application safeguards, not JJ defaults."
- Eight idle-LRU cache entries with ten-minute inactivity expiry; active
  requests renew expiry. Eviction/restart returns explicit same-selection reset.
- Linux progress probes qualified response delivery and source preservation, not
  complete attribution latency. The warm five-second target was exceeded on some
  development samples; no fast full-attribution claim.
<!-- /evidence-field -->

<!-- evidence-field:out_of_scope list -->
## Out of scope

- Full Linux completion/first-proven-origin benchmark, release-scale/RustFS/RSS
  pressure qualification, full root suites and visual baselines.
- Rename/copy tracing, working-copy blame, ignore revisions and C syntax
  highlighting beyond released Design Core support.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- "Main landing blocked by active overlapping Stacks edits in sktnmmmv: shared
  generated contracts, canonical chapters and generated API documentation.
  AGENTS requires stopping on overlapping work; main left intact."
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Keep the verified change and preview available. Land after the active Stacks change is committed or coordination explicitly resolves the overlap.
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-jj/src/query/blame/tests.rs#annotation_matches_jj_merges_repeated_lines_empty_and_missing_newline
- crates/jj-store-jj/src/query/blame/tests.rs#retries_cancellation_reset_and_selection_binding
- crates/jj-store-jj/src/query/blame/tests.rs#cache_expiry_and_capacity_preserve_active_progress
- crates/jj-store-jj/src/query/blame_probe.rs#incremental_domains_match_complete_annotation_for_edits_and_merges
- crates/jj-store-cli/tests/local_browser/managed_code.rs#progressive_blame_preserves_local_source_and_rejects_stale_generation
- crates/jj-store-cli/tests/native_lifecycle/path_metadata.rs#path_metadata_crosses_merges_and_retains_the_ancestry_bound
- crates/jj-store-cli/tests/native_lifecycle/http_browsing.rs#http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured
- crates/jj-store-cli/tests/local_browser/managed_http.rs#managed_local_http_activates_without_subscribers_resumes_and_captures_tasks
- web/tests/store-blame.test.ts#blame admission explains UTF-8 byte and line
  bounds
<!-- /evidence-field -->

