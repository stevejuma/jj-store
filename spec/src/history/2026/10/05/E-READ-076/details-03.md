---
schema: 1
id: E-READ-076
---

# E-READ-076 details 3

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
pnpm spec:first
```

**Notes**

0.66s; 0 Rust tests passed. .local/bounded-reads/performance/main-spec-first-076.log
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
pnpm spec:validate
```

**Notes**

1.4s; 0 Rust tests passed. .local/bounded-reads/performance/main-spec-validate-076.log
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
pnpm web:check
```

**Notes**

23.93s; 0 Rust tests passed. .local/bounded-reads/performance/main-web-check-076.log
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:notes list -->
## Notes

- Main was clean at empty sqsrltsw/3fb1a57e above E-READ-071 xvyloluq/a6c7b0de.
  It had no intervening source changes or overlap. Advance it through
  ysoumkvm/e874c6a2, kxupxzzm/69379656, ptnmxplv/6cc59c74 and xnwpnypq/0c09be9e,
  retaining all original histories. The enclosing ymrvkrow landing records new
  combined verification; it does not repeat E-READ-071.
- All381 focused Rust tests pass from main's actual sources:7 copy primitive,66
  query,15 native backend,22 native RPC,235 native lifecycle,13 native
  inventory,1 independent preparation pin and22 HTTP tests. Seven
  generated-contract Node tests and TypeScript acceptance pass; main web has
  zero errors/warnings. Affected six package all-target checks and canonical
  checks pass.
- 174 original E-READ-072 through075 log/metadata files copied into main's
  ignored evidence directory and verified byte-identically, with SHA256/byte
  proofs in .local/bounded-reads/performance/source-evidence-076.json. Preserve
  all516 earlier evidence entries and their historical text
  verbatim/JSON-identically.
- Main production files and generated contracts are unchanged from the verified
  E-READ-075 parent. Its complete277-response registered HTTP/SSE capture proof
  travels byte-identically with the source; main reruns HTTP contract shape,
  native lifecycle transport consumers and generated types rather than repeating
  unchanged capture fixtures or release qualification.
- Rebuildable source projections, original root/operation pins, fresh
  authorization/epoch/deadline checks, staged-acknowledgement cutoffs and shared
  preparation admission retain their existing contracts. The initial main
  progress checker rejects eleven bookkeeping references (whole files and a
  directory instead of named tests); replace them with exact existing regression
  identifiers without changing any product or test behavior. No manifest
  publication, source migration, dev fixture, installed executable, running dev
  service or Linux checkout changes. Keep the temporary workspace open and
  continue there after rebasing its empty change onto this landing.
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Selected copy-family RPC/client and applicable mutation providers still
  require delivery; the remote CLI copy authority remains on complete kind
  enumeration. Indexed get_copy_records DAG range/path selection and larger copy
  relation chunk/disk support remain.
- App recovery and remote kind task delivery, general filtered-log continuation
  preserving indirect/missing edges, uncached/ambiguous evolution, indexed
  code/local application search and oversized comparison/change
  grouping/extreme-filter candidates remain.
- Complete scale/RSS/latency/fault/cold-browser/RustFS/release qualification,
  Help/generated-reference/source-review refresh and inherited
  installation-reference drift remain open. No milestone promotion.
<!-- /evidence-field -->

<!-- evidence-field:next markdown -->
## Next

Commit the verified main landing, advance codex/bounded-reads-landed, rebase the retained temporary workspace's empty continuation onto it and continue selected family remote delivery followed by app preparation recovery. Full qualification stays separate.
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

