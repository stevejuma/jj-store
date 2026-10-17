---
schema: 1
id: E-INJECTABLE-STORES-002
---

# E-INJECTABLE-STORES-002 details 3

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
node scripts/package-check.mjs
```

**Notes**

All 23 coordinated Cargo packages have portable registry dependencies and contained source compilation inputs.
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
pnpm check:boundaries
```

**Notes**

22 workspace packages passed the updated owned dependency allowlist.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 26
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Draft evidence needed named test references and removed paths included in both files and removed_files; corrected the draft without touching committed E001.
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
pnpm spec:validate
```

**Notes**

Seven validators, 4102 chapters and 206 requirements. This validates document structure and does not qualify pending Rust implementation.
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
rustfmt --edition 2024 --config skip_children=true <changed existing Rust files>; pnpm exec prettier --write <changed Markdown and JavaScript files>
```

**Notes**

Only changed sources and governed documents were formatted; historical evidence is excluded by repository formatter policy. No build was run.
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
pnpm check:progress
```

**Notes**

162 tasks, 851 evidence records after fixing fresh removed-path coverage and named tests. E002 remains pending; this is bookkeeping validation only.
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
pnpm dev:stop
```

**Notes**

Explicit user authorization received. Stopped this workspace Vite/backend/RustFS processes; preserved the complete .local/dev-demo fixture and saved previous runtime mode.
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
pnpm dev:clean -- --dry-run; pnpm dev:clean -- --apply
```

**Notes**

After stop, cleanup selected eligible embedded runtime outputs and recovered about5.2GiB (29.0-&gt;34.2GiB). Main target and protected fixtures retained. Dev stack will be restored in Vite mode.
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
pnpm dev:cargo -- test -p jj-store-git --test factories --locked --message-format=short
```

**Notes**

All five factory regressions passed after correcting the new version assertions; includes exact reopening/replay across all domains, atomic-bound publication, fresh authorization, staged invisibility and missing evidence validation.
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
pnpm dev:cargo -- test -p jj-store-core -p jj-store-git --locked --message-format=short
```

**Notes**

14 passed: core7 including independent replacement of each store, Git7 including original descriptor/canonical tests and new factory scenarios.
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
pnpm dev:cargo -- test -p jj-store-jj --no-default-features --features companion --lib --test local_evidence --test local_metadata --locked --message-format=short
```

**Notes**

20 passed: evidence6, metadata14. Exact source codecs, root sentinel, retained closures, GC/undo, receipts, process exit, races and unknown ref outcomes passed without native features.
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
pnpm dev:cargo -- test -p jj-store-service --test local_collab local_commands_keep_exact_history_preserve_siblings_and_replay_after_gc --locked -- --exact
```

**Notes**

After extraction: exact original regression1passed. Before state was also1passed.
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
cargo tree -p jj-store-git --edges normal --prefix none --format '{p}' --locked --offline; cargo tree -p jj-store-jj --no-default-features --features companion --edges normal --prefix none --format '{p}' --locked --offline; python3 <dependency-name audit>
```

**Notes**

Read-only dependency audit: Git161normal packages, companion232. Git excludes upstream JJ/CLI and both exclude native object/WAL, service/RPC/HTTP/server, search/adapters, reqwest and redb. Graphs logged separately; dev-only test transports are excluded. Independent external manifest/runtime proof remains a later M32 gate.
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
pnpm dev:cargo -- test -p jj-store-service --lib --test local_agent --test local_agent_binding --test local_agent_pages --test local_collab --test local_review --test local_work --test conversion --test conversion_agent_discussion --test native_collab --test review_threads --test work_store --locked --message-format=short
```

**Notes**

149 passed across the affected service families. The shared Git persistence extraction and pure resolver have several independent domain consumers, so these complete families are the required wider Rust impact surface; unrelated UI and workspace suites remain out of scope.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-store-core -p jj-store-git -p jj-store-jj -p jj-store-wal -p jj-store-service -p jj-store -p jj-store-capture --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Two redundant same-type conversions remained in extracted Git stage_root methods. Removed them. Postflight also reached the40GiB target cap, while free space remained above reserve; completed targeted maintenance before retry.
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
pnpm dev:clean -- --apply
```

**Notes**

With the authorized dev stack still stopped and Rust processes idle, eligible main-workspace artifacts were cleaned. Free space31.1-&gt;51.6GiB. External dependencies, fixtures, archived targets and sources preserved.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-store-core -p jj-store-git -p jj-store-jj -p jj-store-wal -p jj-store-service -p jj-store -p jj-store-capture --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Attempt 2: Error-mapping correction exposed two compile errors, corrected without changing error semantics.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm dev:cargo -- clippy -p jj-store-core -p jj-store-git -p jj-store-jj -p jj-store-wal -p jj-store-service -p jj-store -p jj-store-capture --all-targets --locked --message-format=short -- -D warnings
```

**Notes**

Attempt 3: Six redundant borrows after introducing readonly graph accessors, removed.
<!-- /run -->
<!-- /evidence-field -->

