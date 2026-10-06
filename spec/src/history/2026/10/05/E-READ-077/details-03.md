---
schema: 1
id: E-READ-077
---

# E-READ-077 details 3

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
pnpm format:rust:check
```

**Notes**

4.42s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-rust-format-077.log
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
pnpm format:rust:check
```

**Notes**

4.2s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-rust-format-final-077.log
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
pnpm spec:first
```

**Notes**

2.51s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-spec-first-077.log
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
pnpm spec:first
```

**Notes**

2.57s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-spec-first-final-077.log
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
pnpm spec:validate
```

**Notes**

3.48s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-spec-validate-077.log
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
pnpm spec:validate
```

**Notes**

4.43s; 0 Rust tests passed. .local/bounded-reads/performance/remote-family-spec-validate-final-077.log
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
cargo test --locked -p jj-store-rpc --lib copy_families
```

**Notes**

62.36s; 2 Rust tests passed. .local/bounded-reads/performance/remote-family-wire-077.log
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test --locked -p jj-store-rpc --lib
```

**Notes**

14.01s; 10 Rust tests passed. .local/bounded-reads/performance/remote-family-wire-ceilings-077.log
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
cargo test --locked -p jj-store-rpc --lib
```

**Notes**

37.16s; 10 Rust tests passed. .local/bounded-reads/performance/remote-family-wire-final-077.log
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:notes list -->
## Notes

- Main E-READ-076 is committed as ymrvkrow/66a17841 with
  codex/bounded-reads-landed advanced. Its381-test focused combined landing
  verification and174 byte-identical source evidence files remain preserved.
  Rebase this retained temporary workspace empty mzmylqwx onto it without
  conflicts; main stays clean and unchanged during this follow-up.
- "Before state: real remote factories request a five-copy family among96
  unrelated copies and96 files and fail with native ResourceExhausted. The exact
  regression now returns only the five family bodies in deterministic
  children-before-parents order; a repeat reads zero inventory descriptors and
  exactly five bodies. Cold relation construction retains its independent
  descriptor/cache ceilings."
- Owned transport DTOs retain no upstream JJ types, source or credentials. Admit
  bounded sorted seed flags before set growth; required missing seeds fail and
  optional uncommitted seeds return no IDs. Clamp server/session defaults
  independently of client allowances, including a trusted endpoint ceiling
  exposed to the RPC decoder before seed-set growth. Final review tightened this
  admission and repeated both affected complete RPC consumer suites and
  all-target checks. One checked current root serves the ordinary remote backend
  call; explicit historical browser providers retain their original pins. Result
  identities, work, allocation, required-seed inclusion and deadlines fail
  closed on decode.
- "Actual older-peer loopback layer suppresses only CopyFamily: explicit
  Unimplemented alone performs one bounded Inventory call. PermissionDenied,
  ResourceExhausted, DeadlineExceeded, DataLoss, Unavailable and
  FailedPrecondition each remain errors with zero Inventory fallback calls."
- Client-local acknowledged staged ancestors and descendants retain the existing
  captured-sequence implementation. Reacknowledged selected identities preserve
  committed siblings; foreign and unknown uploads are excluded. An identical
  acknowledged retry joins the family. No staged record enters the server
  projection or grants publication power.
- "Intermediate focused compile fails with four diagnostics because the parent
  fixture imports auth::Result and the generic peer service needs
  std::result::Result; qualify the two signatures. The next focused run has two
  assertion failures: incremental construction also performs one checked seed
  membership read, so exact descriptor evidence is three (membership plus
  operation/copy additions), and tonic local timeout maps to Cancelled. A
  diagnostic rerun confirms the deadline assertion remains failing. Preserve
  these logs. Normalize only a concrete tonic::TimeoutExpired source or an
  already-expired admitted deadline, keeping an independent remote Cancelled
  visible. Exact pending-I/O regression then passes and proves source read exit
  plus sole RPC admission release. No assertion is weakened to allow arbitrary
  cancellation."
- Complete affected consumer verification is recorded in runs. The initial
  passing strict-wire two-test target is followed by the complete RPC library
  (ten tests). Six new real-loopback regressions plus the existing native family
  regression pass in the exact family target. Final complete consumer results
  are51 Rust tests (28 native RPC,10 RPC library and13 native inventory); all
  three affected package all-target checks pass. Fresh fixture servers exercise
  new RPC startup and teardown; complete cold browser/Linux/RustFS qualification
  is deferred.
- Preserve all517 earlier ledger entries verbatim and JSON-identically. No
  installed binary replacement, dev service restart, user
  fixture/reset/migration, publication/push, Linux checkout or visual baseline
  change.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Native application mutation providers and app kind preparation recovery/remote
  preparation task delivery still need integration. Indexed get_copy_records DAG
  range/path selection and larger copy relation chunk/disk support remain.
- General filtered-log continuation preserving indirect/missing edges,
  uncached/ambiguous evolution, indexed code/local application search, oversized
  comparisons/change grouping and extreme-filter candidate selection remain.
- Complete scale/RSS/latency/fault/cold-browser/RustFS/release qualification,
  Help/generated reference/source-review refresh and inherited
  installation-reference drift stay open.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit this verified remote capability slice in the retained temporary workspace, then continue high-priority app preparation recovery and applicable native mutation capabilities. Keep full qualification separate.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

