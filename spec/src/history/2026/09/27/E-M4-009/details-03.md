---
schema: 1
id: E-M4-009
---

# E-M4-009 details 3

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

190 Rust tests pass / 6 external cases intentionally ignored; 15 governance and 3 storage-harness tests pass. No unresolved failures.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Locked workspace build passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 9 lanes pass, including 190 Rust tests, rustfmt, mdBook and mapped spec-first checks.
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
pnpm format:check
```

**Notes**

Rustfmt and Prettier pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Adversarial merge/tree amplification and large content memory admission,
  controlled long-index rebuild tasks/status, HTTP JSON/media/SSE, generated
  contracts and browser integration. Fixed-width graph measurements do not
  qualify whole-server deployment budgets.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; Tokio 1.53.1; release measurement host and binary/lock checksums recorded in linked evidence.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

New owned Canceled query error; no native identity, envelope, publication, RPC or persisted authority format change. Qualification JSON is evidence only.
<!-- /evidence-field -->

