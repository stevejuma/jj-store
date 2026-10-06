---
schema: 1
id: E-M5-002
---

# E-M5-002 details 3

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

11 protected files mapped to six canonical chapters.
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

32 task records validate; bookkeeping does not close remaining M5 gates.
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
jj --no-pager diff --name-only -- web
```

**Notes**

Empty: no UI files changed.
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

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds application intent V1 and selected StoredResult V1 in existing WAL receipt result bytes. Existing native/token request digest profile, native object codecs, WAL envelopes, RPC and HTTP formats are unchanged. No UI files changed. Enclosing Jujutsu change is the revision reference. M5.1 retains HTTP exposure, capability registry and reconciliation; M5.2 abandon and M5.3 conflict resolution remain. M4.2 RustFS/SSE qualification awaits the stopped shared Docker engine; browser work is Deferred under M4.3 per user instruction.
<!-- /evidence-field -->

