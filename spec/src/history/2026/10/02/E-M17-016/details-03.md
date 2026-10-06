---
schema: 1
id: E-M17-016
---

# E-M17-016 details 3

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
jj version
```

**Notes**

Verified JJ 0.45.1.
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
uname -m
```

**Notes**

Verified ARM64 host.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-activity-clippy.log 2>&1
```

**Notes**

Strict affected/direct-consumer all-target lint passes with no suppression.
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
pnpm format:rust:check > /tmp/jj-store-agent-capture-m172-activity-format.log 2>&1
```

**Notes**

Rust workspace and embedded-runtime formatting checks pass; no unrelated product-suite claim.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-activity-doc-write.log 2>&1
```

**Notes**

Changed canonical documents formatted; previous 270 evidence structures remain unchanged.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-activity-progress.log 2>&1
```

**Notes**

129 tasks validate; 271 evidence entries retain all 270 parent entries.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-activity-spec.log 2>&1
```

**Notes**

Five validators, 21 chapters and 160 requirements pass.
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
pnpm spec:first > /tmp/jj-store-agent-capture-m172-activity-first.log 2>&1
```

**Notes**

15 protected source/test files and three canonical chapters pass the spec-first gate.
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
pnpm exec prettier --check spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-activity-doc-format.log 2>&1
```

**Notes**

Changed canonical document formatting passes.
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

Rust 1.98.1; JJ 0.45.1; macOS ARM64; independent synthetic Codex CLI 0.156.1 and Desktop 0.159.2 headers and task-start records. Fresh OS process resumes admitted offline final flush after sanitized safe-gap durability; actual native socket/directory verifier acceptance remains separate from real harness execution and RustFS.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Private source catalog V1 adds optional admitted_sessions digests; empty field omitted to preserve legacy bytes. At most 4096 admission digests within existing 2 MiB catalog/file bounds; no eviction. Domain-separated lowercase Codex native UUID digests bind source eligibility, not repository authority. Existing native/companion objects, pending/prepared requests, source cursor journals, receipts and aliases retain their encodings and interpretation. Raw probe snapshots/directories/partial records are memory-only and have no Debug or serialization.
<!-- /evidence-field -->

<!-- evidence-field:revision markdown -->
## Revision

enclosing Jujutsu change rsqoyznp
<!-- /evidence-field -->

<!-- evidence-field:scope markdown -->
## Scope

One active-source eligibility behavior spans read-only capture probing, private source enrollment and direct server directory admission. Exact new regression precedes complete direct reader/context/discovery/lease/journal binaries, three private catalog/lease unit targets, direct server admission and the exact actual native CLI workspace verifier consumer; affected/direct-consumer all-target checks and strict lint qualify the Rust boundary. The activity probe creates no IDs, registry entry/lock, durable cursor or publication. Initial changing baselines are conservatively historical; only a supported task_started after stable arming with verified per-record context supplies activity. Growth/completions/text/unfinished historical starts do not. Prefix verification is bounded and changed/rebound/truncated sources rearm rather than inherit activity. Snapshot races refuse active acquisition until verified repoll. Automatic admission flags are distinct from generic registration, selected with channel identity under the existing atomic private catalog replacement after successful leases. Read-only existing lookup does not create/clean state, refuses foreign/corrupt/unsafe/ambiguous state and checks known names/file/temp bounds. Offline admitted sessions retain backfill/resume capability including same-session rotation; a different native session at an existing location requires fresh activity. Server checks the activity directory and current native/socket/catalog proof before allocating and repeats current binding afterward; publication write authorization is separate. Fresh-process tail proof uses constant sanitized gaps and exact durable reader progress, not real native transcript bodies or a real model execution. This slice does not connect automatic startup, live status/hooks, capture principal, supervised worker/publication, pause/backpressure controls, Cursor/OpenCode readers or UI. Enabled server preparation remains guarded; M17.2 and aggregate M17 stay In progress and the full M17.3-M17.7/M17.5 real harness roadmap remains required. No model/wire, native/WAL, UI lifecycle, installed service, real RustFS or deployment changed; root aggregate and unrelated wider suites are out of scope.
<!-- /evidence-field -->

