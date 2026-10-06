---
schema: 1
id: E-READ-067
---

# E-READ-067 details 3

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
pnpm contracts:check
```

**Notes**

705.23s; .local/bounded-reads/performance/kind-contracts-067.log
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
pnpm exec prettier --check spec/src/mutations.md spec/src/jj-integration.md spec/src/cli.md spec/src/architecture.md spec/src/milestones.md docs/implementation-plan.md
```

**Notes**

3.69s; .local/bounded-reads/performance/kind-document-format-067.log
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:notes list -->
## Notes

- "Before warm native lookup: 304 reads/260 descriptors; before complete RPC:299
  reads/260 descriptors. Both exact regressions fail the fixed
  zero-descriptor/<=16-read warm bound. After:10 fresh authorization reads/zero
  descriptors for native/RPC operation and empty/positive copy results."
- "Increment after real operation publication plus four newly selected files:
  native37/RPC35 reads and exactlyfive descriptors, fixed<=80-read bound
  unchanged. Copy addition reads25/exactlyone descriptor; staged eighth copy
  excluded before selection and included only after the test-selected manifest
  root advances."
- Cache retains only matching IDs plus checked manifest data under source
  capability, repository, epoch, inventory root and kind; max32 roots,
  fixedfive-minute expiry, conservative192-byte ID and1MiB manifest allowance
  under64MiB total. Charges remain owned beyond eviction. Cache saturation falls
  back to ordinary bounded request-local work.
- Eight work slots queue within the original whole-call deadline; fresh
  authorization/epoch checks and caller/session result bounds guard all results.
  Complete baseline advancement consumes additions from None; exact retention
  loss discards that baseline and restarts cold without renewing total scan
  allowance or deadline. Other corruption/unknown reads fail closed.
- Client complete convenience uses existing single selected Inventory RPC,
  validating bounded canonical sorted IDs and explicit monotonic whole-call
  deadline. InventoryPage wire, signed pins, sparse/empty continued windows,
  inspected counts, replay and expiry remain unchanged; legacy request bytes are
  identical.
- 255 distinct Rust tests pass:18 native RPC,231 native lifecycle,three cache
  andthree inventory wire units. No milestone promoted. Cold exact-kind
  preparation still scans unrelated descriptors under explicit total bounds;
  this evidence does not claim cold large-root lookup solved.
- macOS debug linker unwind-size warnings remain recorded; release qualification
  remains deferred. Verified main E-READ-062 landing, temporary workspace, Linux
  source, fixtures, services, installed CLIs and unrelated workspaces preserved.
- All 507 prior evidence entries remain JSON-identical; Rust format, boundaries,
  architecture, contracts, changed-document format, spec-first, spec validation
  and progress pass. Contract validation includes actual HTTP/SSE fixtures and
  remains distinct from complete release qualification.
- A premature bookkeeping read attempted the result JSON before contract
  validation finished and failed without modifying the ledger; waited for
  completion and then recorded all passing results. No functional verification
  lane failed after implementation.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Browser query ReadOnlyAuthority copy/operation enumeration still uses
  ReadAccess::inventory; preparation Access refuses that full inventory. Add
  pinned kind/provider lookup and relevant copy reverse/path index; do not use
  current NativeSession selection for a historical browser pin.
- Cold native kind construction for very large roots remains explicit bounded
  preparation work. Current disposable warm reuse cannot alone make a root
  beyond the cold total-scan allowance loadable.
- General filtered log continuation with exact upstream indirect/missing edge
  semantics, uncached/ambiguous evolution, indexed code/local application
  search, retained oversized comparisons/change grouping/extreme filters remain
  delivery work.
- Complete qualification and Help/generated reference/source-review refresh
  remain separately deferred; inherited installation-reference drift remains a
  failing qualification gate.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit E-READ-067. Continue highest-priority pinned browser kind/copy and cold preparation delivery, then exact filtered-history continuations; retain temporary workspace and defer complete qualification.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

