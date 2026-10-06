---
schema: 1
id: E-M15-013
---

# E-M15-013 details 2

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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test agent_capture_publication bound_capture_ --locked (before)
```

**Notes**

Zero tests matched; exact bound worker regression absent. This is before-state evidence, not acceptance; /tmp/jj-bound-transport-before.log.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo check -p jj-store-service -p jj-store-rpc -p jj-store-cli --all-targets --locked (initial)
```

**Notes**

NativeRequest forwarding wrapper was missing. Added binding-aware service forwarding; /tmp/jj-bound-transport-check.log.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test agent_capture_publication bound_capture_ --locked (initial)
```

**Notes**

Same missing NativeRequest forwarding method; no assertions ran; /tmp/jj-bound-transport-drain.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test agent_capture_publication bound_capture_ --locked (fixed)
```

**Notes**

Bound captured spool survives reopen before preparation and unknown publication, then retries the complete original request; /tmp/jj-bound-transport-drain-fixed.log. Later direct-consumer run also checks dropped binding refusal.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test agent_capture_spool --locked
```

**Notes**

Four spool regressions including absent/prepared mismatch rejection and retained binding; /tmp/jj-bound-transport-spool.log.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test -p jj-store-rpc --test collab explicit_capture_binding_ --locked (initial)
```

**Notes**

Test used ThreadEventV1.kind instead of its event field; corrected to the canonical event field; /tmp/jj-bound-transport-rpc.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-rpc --test collab explicit_capture_binding_ --locked (fixed)
```

**Notes**

Real owner-admitted Unix gRPC preparation stages only, publication selects both roots, replay after rename and transport reconstruction keeps receipt, changed retry aborts; /tmp/jj-bound-transport-rpc-fixed.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-rpc --locked
```

**Notes**

Complete relevant RPC package: 18 tests, zero doctests; /tmp/jj-bound-transport-rpc-suite.log.
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
CARGO_INCREMENTAL=0 cargo test -p jj-store-service --test agent_sessions --test agent_checkpoints --test agent_relations --test native_collab --test collab_pagination --locked
```

**Notes**

24 direct service read/capture/collaboration consumers; /tmp/jj-bound-transport-service-consumers.log.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt -p jj-store-service -p jj-store-rpc -p jj-store-cli
```

**Notes**

Focused Rust formatting, repeated after test additions and wrapper repair.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm exec prettier --write spec/src/api.md spec/src/cli.md spec/src/collaboration.md spec/src/jj-integration.md
```

**Notes**

Canonical transport, capture and spool contracts formatted.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (initial)
```

**Notes**

Service forwarding is mapped to mutations.md; added the explicit preparation/staging/authorization contract there. No policy exemption; /tmp/jj-bound-transport-spec-first.log.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test agent_capture_spool --test agent_capture_publication --test agent_hook --test agent_integrations --test agent_setup --test local_agent --test local_agent_recovery --locked
```

**Notes**

26 direct CLI tests including cold sanitizer/hook/local capture and both profile drain/recovery paths; /tmp/jj-bound-transport-cli-consumers.log.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CARGO_INCREMENTAL=0 cargo test -p jj-store-cli --test native_lifecycle local_trusted_http_lists_root_pinned_agent_sessions_without_bearer_credentials --locked
```

**Notes**

Existing unlinked HTTP capture consumer passes; /tmp/jj-bound-transport-http-consumer.log.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first (fixed)
```

**Notes**

13 protected files; six canonical chapters.
<!-- /run -->

<!-- run -->
### Run 16

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

Four validators; 21 chapters; 140 requirements.
<!-- /run -->

<!-- run -->
### Run 17

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

111 tasks; evidence bookkeeping only.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries
```

**Notes**

16 package boundaries.
<!-- /run -->
<!-- /evidence-field -->

