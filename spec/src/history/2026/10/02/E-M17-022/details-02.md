---
schema: 1
id: E-M17-022
---

# E-M17-022 details 2

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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_resume admitted_resume_uses_current_repository_when_original_header_is_revoked --locked -- --exact > /tmp/jj-store-agent-capture-m172-resume-before.log 2>&1
```

**Notes**

New exact regression fails on the absent staged preparation exports. Existing header-only admitted acquisition cannot resume a current linked repository after original-header revocation.
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
cargo fmt -p jj-store-capture -p jj-store-server
```

**Notes**

Focused formatting repeated after the new lookup, current-context probe, staged acquisition and race/rotation fixtures.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_resume admitted_resume_uses_current_repository_when_original_header_is_revoked --locked -- --exact > /tmp/jj-store-agent-capture-m172-resume-after.log 2>&1
```

**Notes**

Compilation exposed an incorrect SourceInstanceId import from the service facade. Reexported the already-public source channel type from its owning capture registry; dependency boundaries remain unchanged.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_resume --locked > /tmp/jj-store-agent-capture-m172-resume-server.log 2>&1
```

**Notes**

An intermediate all-case attempt retained that unresolved import. Corrected the owned capture-registry import before the next run.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_resume --locked > /tmp/jj-store-agent-capture-m172-resume-server-fixed.log 2>&1
```

**Notes**

Three initial staged native preparation cases pass: current-directory binding, bounded cursor-free reconstruction and Write-before-rotated-enrollment.
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
cargo test -p jj-store-capture --test codex_resume --locked > /tmp/jj-store-agent-capture-m172-resume-runtime.log 2>&1
```

**Notes**

Five direct probe/lookup cases pass, including a fresh-process read-only admitted-session lookup and offline complete-context reconstruction; the ignored child fixture is explicitly invoked by its parent.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_resume --locked > /tmp/jj-store-agent-capture-m172-resume-server-expanded.log 2>&1
```

**Notes**

All five expanded preparation cases pass. Lost native Write leaves rotation aliases/locks untouched; restored authority enrolls exactly one new physical lease and preserves channel ID. File changes during admission withhold before acquisition; catalog/spool changes remain independently current. One-use completion and partial final context are qualified.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-capture --lib --tests --locked > /tmp/jj-store-agent-capture-m172-resume-runtime-direct.log 2>&1
```

**Notes**

All 76 primary runtime cases pass; subprocess fixture cases execute from their parents. Complete direct context, activity, registry, leased reader, cursor, queued-proof and sanitation consumers qualify the shared lookup and context visibility boundary.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_resume --test capture_recovery --test capture_source --test capture_activity --test capture_authority --test capture_admission --locked > /tmp/jj-store-agent-capture-m172-resume-server-direct.log 2>&1
```

**Notes**

All 19 direct native source/resume/recovery/admission/activity/authority cases pass with the existing pinned scanner cache.
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
cargo check -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked > /tmp/jj-store-agent-capture-m172-resume-check.log 2>&1
```

**Notes**

Affected runtime/native and direct CLI consumers compile across all targets.
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
cargo clippy -p jj-store-capture -p jj-store-server -p jj-store-cli --all-targets --locked -- -D warnings > /tmp/jj-store-agent-capture-m172-resume-clippy.log 2>&1
```

**Notes**

Affected and direct-consumer strict lint passes.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt -p jj-store-capture -p jj-store-server -- --check > /tmp/jj-store-agent-capture-m172-resume-rust-format.log 2>&1
```

**Notes**

Affected Rust formatting passes.
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
JJ_STORE_TRANSCRIPT_CACHE=/tmp/jj-store-agent-capture-m172-native-kingfisher cargo test -p jj-store-server --test capture_resume admitted_resume_uses_current_repository_when_original_header_is_revoked --locked -- --exact > /tmp/jj-store-agent-capture-m172-resume-exact-final.log 2>&1
```

**Notes**

Original exact revoked-header regression rerun passes after all final acquisition revalidation and race fixes.
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
pnpm check:boundaries > /tmp/jj-store-agent-capture-m172-resume-boundaries.log 2>&1
```

**Notes**

All 17 package boundaries pass; no new service/server dependency on domain or upstream JJ crates.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md > /tmp/jj-store-agent-capture-m172-resume-doc-write.log 2>&1
```

**Notes**

Owning canonical requirements formatted.
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
pnpm exec prettier --write spec/src/collaboration.md spec/src/cli.md spec/src/verification.md spec/src/milestones.md > /tmp/jj-store-agent-capture-m172-resume-doc-final-write.log 2>&1
```

**Notes**

Final canonical chapters, verification and appended ledger formatting passes.
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
pnpm check:progress > /tmp/jj-store-agent-capture-m172-resume-progress.log 2>&1
```

**Notes**

129 tasks validate; 277 evidence entries preserve all 276 parent structures.
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
pnpm spec:validate > /tmp/jj-store-agent-capture-m172-resume-spec.log 2>&1
```

**Notes**

Five validators, 21 canonical chapters and 160 requirements pass.
<!-- /run -->
<!-- /evidence-field -->

